use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::atomic::{fence, AtomicU64, Ordering};

// ストライプのサイズ
const STRIPE_SIZE: usize = 8; // u64, 8バイト

// メモリの合計サイズ
const MEM_SIZE: usize = 512; // 512バイト

// メモリの型
pub struct Memory {
    mem: Vec<u8>,             // メモリ
    lock_ver: Vec<AtomicU64>, // lock & version
    global_clock: AtomicU64,  // global version-clock

    // アドレスからストライプ番号に変換するためのシフト量
    shift_size: u32,
}

impl Memory {
    pub fn new() -> Self { // <1>
        // メモリ領域を生成
        let mem = [0].repeat(MEM_SIZE);

        // アドレスからストライプ番号へ変換するためのシフト量を計算
        // ストライプのサイズは2^nにアラインメントされている必要あり
        let shift = STRIPE_SIZE.trailing_zeros(); // <2>

        // lock & versionを初期化 <3>
        let mut lock_ver = Vec::new();
        for _ in 0..MEM_SIZE >> shift {
            lock_ver.push(AtomicU64::new(0));
        }

        Memory {
            mem,
            lock_ver,
            global_clock: AtomicU64::new(0),
            shift_size: shift,
        }
    }

    // global version-clockをインクリメント <4>
    fn inc_global_clock(&mut self) -> u64 {
        self.global_clock.fetch_add(1, Ordering::AcqRel)
    }

    // 対象のアドレスのバージョンを取得 <5>
    fn get_addr_ver(&self, addr: usize) -> u64 {
        let idx = addr >> self.shift_size;
        let n = self.lock_ver[idx].load(Ordering::Relaxed);
        n & !(1 << 63)
    }

    // 対象のアドレスのバージョンがrv以下でロックされていないかをテスト <6>
    fn test_not_modify(&self, addr: usize, rv: u64) -> bool {
        let idx = addr >> self.shift_size;
        let n = self.lock_ver[idx].load(Ordering::Relaxed);
        // ロックのビットは最上位ビットとするため、
        // 単にrvと比較するだけでテスト可能
        n <= rv
    }

    // 対象アドレスのロックを獲得 <7>
    fn lock_addr(&mut self, addr: usize) -> bool {
        let idx = addr >> self.shift_size;
        match self.lock_ver[idx].fetch_update( // <8>
            Ordering::Relaxed, // 書き込み時のオーダー
            Ordering::Relaxed, // 読み込み時のオーダー
            |val| {
                // 最上位ビットの値をテスト&セット
                let n = val & (1 << 63);
                if n == 0 {
                    Some(val | (1 << 63))
                } else {
                    None
                }
            },
        ) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    // 対象アドレスのロックを解放 <9>
    fn unlock_addr(&mut self, addr: usize) {
        let idx = addr >> self.shift_size;
        self.lock_ver[idx].fetch_and(!(1 << 63),
                                     Ordering::Relaxed);
    }
}

pub struct ReadTrans<'a> { // <1>
    read_ver: u64,   // read-version
    is_abort: bool,  // 競合を検知した場合に真
    mem: &'a Memory, // Memory型への参照
}

impl<'a> ReadTrans<'a> {
    fn new(mem: &'a Memory) -> Self { // <2>
        ReadTrans {
            is_abort: false,

            // global version-clock読み込み
            read_ver: mem.global_clock.load(Ordering::Acquire),

            mem,
        }
    }

    // メモリ読み込み関数 <3>
    pub fn load(&mut self, addr: usize) -> Option<[u8; STRIPE_SIZE]> {
        // 競合を検知した場合終了 <4>
        if self.is_abort {
            return None;
        }

        // アドレスがストライプのアラインメントに沿っているかチェック
        assert_eq!(addr & (STRIPE_SIZE - 1), 0); // <5>

        // 読み込みメモリがロックされておらず、read-version以下か判定 <6>
        if !self.mem.test_not_modify(addr, self.read_ver) {
            self.is_abort = true;
            return None;
        }

        fence(Ordering::Acquire);

        // メモリ読み込み。単なるコピー <7>
        let mut mem = [0; STRIPE_SIZE];
        for (dst, src) in mem
            .iter_mut()
            .zip(self.mem.mem[addr..addr + STRIPE_SIZE].iter())
        {
            *dst = *src;
        }

        fence(Ordering::SeqCst);

        // 読み込みメモリがロックされておらず、read-version以下か判定 <8>
        if !self.mem.test_not_modify(addr, self.read_ver) {
            self.is_abort = true;
            return None;
        }

        Some(mem)
    }
}

pub struct WriteTrans<'a> {
    read_ver: u64,            // read-version
    read_set: HashSet<usize>, // read-set
    write_set: HashMap<usize, [u8; STRIPE_SIZE]>, // write-set
    locked: Vec<usize>,  // ロック済みアドレス
    is_abort: bool,      // 競合を検知した場合に真
    mem: &'a mut Memory, // Memory型への参照
}

impl<'a> Drop for WriteTrans<'a> {
    fn drop(&mut self) {
        // ロック済みアドレスのロックを解放
        for addr in self.locked.iter() {
            self.mem.unlock_addr(*addr);
        }
    }
}

impl<'a> WriteTrans<'a> {
    fn new(mem: &'a mut Memory) -> Self { // <1>
        WriteTrans {
            read_set: HashSet::new(),
            write_set: HashMap::new(),
            locked: Vec::new(),
            is_abort: false,

            // global version-clock読み込み
            read_ver: mem.global_clock.load(Ordering::Acquire),

            mem,
        }
    }

    // メモリ書き込み関数 <2>
    pub fn store(&mut self, addr: usize, val: [u8; STRIPE_SIZE]) {
        // アドレスがストライプのアラインメントに沿っているかチェック
        assert_eq!(addr & (STRIPE_SIZE - 1), 0);
        self.write_set.insert(addr, val);
    }

    // メモリ読み込み関数 <3>
    pub fn load(&mut self, addr: usize) -> Option<[u8; STRIPE_SIZE]> {
        // 競合を検知した場合終了
        if self.is_abort {
            return None;
        }

        // アドレスがストライプのアラインメントに沿っているかチェック
        assert_eq!(addr & (STRIPE_SIZE - 1), 0);

        // 読み込みアドレスを保存
        self.read_set.insert(addr);

        // write-setにあればそれを読み込み
        if let Some(m) = self.write_set.get(&addr) {
            return Some(*m);
        }

        // 読み込みメモリがロックされておらず、read-version以下か判定
        if !self.mem.test_not_modify(addr, self.read_ver) {
            self.is_abort = true;
            return None;
        }

        fence(Ordering::Acquire);

        // メモリ読み込み。単なるコピー
        let mut mem = [0; STRIPE_SIZE];
        for (dst, src) in mem
            .iter_mut()
            .zip(self.mem.mem[addr..addr + STRIPE_SIZE].iter())
        {
            *dst = *src;
        }

        fence(Ordering::SeqCst);

        // 読み込みメモリがロックされておらず、read-version以下か判定
        if !self.mem.test_not_modify(addr, self.read_ver) {
            self.is_abort = true;
            return None;
        }

        Some(mem)
    }

    // write-set中のアドレスをロック
    // すべてのアドレスをロック獲得できた場合は真をリターンする <4>
    fn lock_write_set(&mut self) -> bool {
        for (addr, _) in self.write_set.iter() {
            if self.mem.lock_addr(*addr) {
                // ロック獲得できた場合は、lockedに追加
                self.locked.push(*addr);
            } else {
                // できなかった場合はfalseを返して終了
                return false;
            }
        }
        true
    }

    // read-setの検証 <5>
    fn validate_read_set(&self) -> bool {
        for addr in self.read_set.iter() {
            // write-set中にあるアドレスの場合は
            // 自スレッドがロック獲得しているはず
            if self.write_set.contains_key(addr) {
                // バージョンのみ検査
                let ver = self.mem.get_addr_ver(*addr);
                if ver > self.read_ver {
                    return false;
                }
            } else {
                // 他のスレッドがロックしていないかとバージョンを検査
                if !self.mem.test_not_modify(*addr, self.read_ver) {
                    return false;
                }
            }
        }
        true
    }

    // コミット <6>
    fn commit(&mut self, ver: u64) {
        // すべてのアドレスに対して書き込み。単なるメモリコピー
        for (addr, val) in self.write_set.iter() {
            let addr = *addr as usize;
            for (dst, src) in self.mem.mem[addr..addr + STRIPE_SIZE].iter_mut().zip(val) {
                *dst = *src;
            }
        }

        fence(Ordering::Release);

        // すべてのアドレスのロック解放&バージョン更新
        for (addr, _) in self.write_set.iter() {
            let idx = addr >> self.mem.shift_size;
            self.mem.lock_ver[idx].store(ver, Ordering::Relaxed);
        }

        // ロック済みアドレス集合をクリア
        self.locked.clear();
    }
}

pub enum STMResult<T> {
    Ok(T),
    Retry, // トランザクションをリトライ
    Abort, // トランザクションを中止
}

pub struct STM {
    mem: UnsafeCell<Memory>, // 実際のメモリ
}

// スレッド間で共有可能に設定。チャネルで送受信可能に設定。
unsafe impl Sync for STM {}
unsafe impl Send for STM {}

impl STM {
    pub fn new() -> Self {
        STM {
            mem: UnsafeCell::new(Memory::new()),
        }
    }

    // 読み込みトランザクション <1>
    pub fn read_transaction<F, R>(&self, f: F) -> Option<R>
    where
        F: Fn(&mut ReadTrans) -> STMResult<R>,
    {
        loop {
            // 1. global version-clock読み込み <2>
            let mut tr = ReadTrans::new(unsafe { &*self.mem.get() });

            // 2. 投機的実行 <3>
            match f(&mut tr) {
                STMResult::Abort => return None, // 中断
                STMResult::Retry => {
                    if tr.is_abort {
                        continue; // リトライ
                    }
                    return None; // 中断
                }
                STMResult::Ok(val) => {
                    if tr.is_abort == true {
                        continue; // リトライ
                    } else {
                        return Some(val); // 3. コミット
                    }
                }
            }
        }
    }

    // 書き込みトランザクション <4>
    pub fn write_transaction<F, R>(&self, f: F) -> Option<R>
    where
        F: Fn(&mut WriteTrans) -> STMResult<R>,
    {
        loop {
            // 1. global version-clock読み込み <5>
            let mut tr = WriteTrans::new(unsafe { &mut *self.mem.get() });

            // 2. 投機的実行 <6>
            let result;
            match f(&mut tr) {
                STMResult::Abort => return None,
                STMResult::Retry => {
                    if tr.is_abort {
                        continue;
                    }
                    return None;
                }
                STMResult::Ok(val) => {
                    if tr.is_abort {
                        continue;
                    }
                    result = val;
                }
            }

            // 3. write-setのロック <7>
            if !tr.lock_write_set() {
                continue;
            }

            // 4. global version-clockのインクリメント <8>
            let ver = 1 + tr.mem.inc_global_clock();

            // 5. read-setの検証 <9>
            if tr.read_ver + 1 != ver && !tr.validate_read_set() {
                continue;
            }

            // 6. コミットとリリース <10>
            tr.commit(ver);

            return Some(result);
        }
    }
}

// メモリ読み込み用のマクロ <1>
#[macro_export]
macro_rules! load {
    ($t:ident, $a:expr) => {
        if let Some(v) = ($t).load($a) {
            v
        } else {
            // 読み込みに失敗したらリトライ
            return tl2::STMResult::Retry;
        }
    };
}

// メモリ書き込み用のマクロ <2>
#[macro_export]
macro_rules! store {
    ($t:ident, $a:expr, $v:expr) => {
        $t.store($a, $v)
    };
}