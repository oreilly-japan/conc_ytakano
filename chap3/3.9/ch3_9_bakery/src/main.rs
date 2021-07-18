// 最適化抑制読み書き用
use std::ptr::{read_volatile, write_volatile}; // <1>
// メモリバリア用
use std::sync::atomic::{fence, Ordering}; // <2>
use std::thread;

const NUM_THREADS: usize = 4;   // スレッド数
const NUM_LOOP: usize = 100000; // 各スレッドでのループ数

// volatile用のマクロ <3>
macro_rules! read_mem {
    ($addr: expr) => { unsafe { read_volatile($addr) } };
}

macro_rules! write_mem {
    ($addr: expr, $val: expr) => {
        unsafe { write_volatile($addr, $val) }
    };
}

// パン屋のアルゴリズム用の型 <4>
struct BakeryLock {
    entering: [bool; NUM_THREADS],
    tickets: [Option<u64>; NUM_THREADS],
}

impl BakeryLock {
    // ロック関数。idxはスレッド番号
    fn lock(&mut self, idx: usize) -> LockGuard {
        // ここからチケット取得処理 <5>
        fence(Ordering::SeqCst);
        write_mem!(&mut self.entering[idx], true);
        fence(Ordering::SeqCst);

        // 現在配布されているチケットの最大値を取得 <6>
        let mut max = 0;
        for i in 0..NUM_THREADS {
            if let Some(t) = read_mem!(&self.tickets[i]) {
                max = max.max(t);
            }
        }
        // 最大値+1を自分のチケット番号とする <7>
        let ticket = max + 1;
        write_mem!(&mut self.tickets[idx], Some(ticket));

        fence(Ordering::SeqCst);
        write_mem!(&mut self.entering[idx], false); // <8>
        fence(Ordering::SeqCst);

        // ここから待機処理 <9>
        for i in 0..NUM_THREADS {
            if i == idx {
                continue;
            }

            // スレッドiがチケット取得中なら待機
            while read_mem!(&self.entering[i]) {} // <10>

            loop {
                // スレッドiと自分の優先順位を比較して
                // 自分の方が優先順位が高いか、
                // スレッドiが処理中でない場合に待機を終了 <11>
                match read_mem!(&self.tickets[i]) {
                    Some(t) => {
                        // スレッドiのチケット番号より
                        // 自分の番号の方が若いか、
                        // チケット番号が同じでかつ、
                        // 自分の方がスレッド番号が若い場合に
                        // 待機終了
                        if ticket < t ||
                           (ticket == t && idx < i) {
                            break;
                        }
                    }
                    None => {
                        // スレッドiが処理中でない場合は
                        // 待機終了
                        break;
                    }
                }
            }
        }

        fence(Ordering::SeqCst);
        LockGuard { idx }
    }
}

// ロック管理用の型 <12>
struct LockGuard {
    idx: usize,
}

impl Drop for LockGuard {
    // ロック解放処理 <13>
    fn drop(&mut self) {
        fence(Ordering::SeqCst);
        write_mem!(&mut LOCK.tickets[self.idx], None);
    }
}

// グローバル変数 <14>
static mut LOCK: BakeryLock = BakeryLock {
    entering: [false; NUM_THREADS],
    tickets: [None; NUM_THREADS],
};

static mut COUNT: u64 = 0;

fn main() {
    // NUM_THREADSだけスレッドを生成
    let mut v = Vec::new();
    for i in 0..NUM_THREADS {
        let th = thread::spawn(move || {
            // NUM_LOOPだけループし、COUNTをインクリメント
            for _ in 0..NUM_LOOP {
                // ロック獲得
                let _lock = unsafe { LOCK.lock(i) };
                unsafe {
                    let c = read_volatile(&COUNT);
                    write_volatile(&mut COUNT, c + 1);
                }
            }
        });
        v.push(th);
    }

    for th in v {
        th.join().unwrap();
    }

    println!(
        "COUNT = {} (expected = {})",
        unsafe { COUNT },
        NUM_LOOP * NUM_THREADS
    );
}