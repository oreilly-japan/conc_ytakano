use std::cell::UnsafeCell; // <1>
use std::ops::{Deref, DerefMut}; // <2>
use std::sync::atomic::{AtomicBool, Ordering}; // <3>
use std::sync::Arc;

const NUM_THREADS: usize = 4;
const NUM_LOOP: usize = 100000;

// スピンロック用の型 <4>
struct SpinLock<T> {
    lock: AtomicBool,    // ロック用共有変数
    data: UnsafeCell<T>, // 保護対象データ
}

// ロックの解放および、ロック中に保護対象データを操作するための型 <5>
struct SpinLockGuard<'a, T> {
    spin_lock: &'a SpinLock<T>,
}

impl<T> SpinLock<T> {
    fn new(v: T) -> Self {
        SpinLock {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(v),
        }
    }

    // ロック関数 <6>
    fn lock(&self) -> SpinLockGuard<T> {
        loop {
            // ロック用共有変数がfalseとなるまで待機
            while self.lock.load(Ordering::Relaxed) {}

            // ロック用共有変数をアトミックに書き込み
            if let Ok(_) =
                self.lock
                    .compare_exchange_weak(
                        false, // falseなら
                        true,  // trueを書き込み
                        Ordering::Acquire, // 成功時のオーダー
                        Ordering::Relaxed) // 失敗時のオーダー
            {
                break;
            }
        }
        SpinLockGuard { spin_lock: self } // <7>
    }
}

// SpinLock型はスレッド間で共有可能と指定
unsafe impl<T> Sync for SpinLock<T> {} // <8>
unsafe impl<T> Send for SpinLock<T> {} // <9>

// ロック獲得後に自動で解放されるようにDropトレイトを実装 <10>
impl<'a, T> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        self.spin_lock.lock.store(false, Ordering::Release);
    }
}

// 保護対象データのimmutableな参照外し <11>
impl<'a, T> Deref for SpinLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.spin_lock.data.get() }
    }
}

// 保護対象データのmutableな参照外し <12>
impl<'a, T> DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.spin_lock.data.get() }
    }
}

fn main() {
    let lock = Arc::new(SpinLock::new(0));
    let mut v = Vec::new();

    for _ in 0..NUM_THREADS {
        let lock0 = lock.clone();
        // スレッド生成
        let t = std::thread::spawn(move || {
            for _ in 0..NUM_LOOP {
                // ロック
                let mut data = lock0.lock();
                *data += 1;
            }
        });
        v.push(t);
    }

    for t in v {
        t.join().unwrap();
    }

    println!(
        "COUNT = {} (expected = {})",
        *lock.lock(),
        NUM_LOOP * NUM_THREADS
    );
}