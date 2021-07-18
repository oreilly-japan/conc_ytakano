use std::sync::{Arc, Mutex}; // <1>
use std::thread;

fn some_func(lock: Arc<Mutex<u64>>) { // <2>
    loop {
        // ロックしないとMutex型の中の値は参照不可
        let mut val = lock.lock().unwrap(); // <3>
        *val += 1;
        println!("{}", *val);
    }
}

fn main() {
    // Arcはスレッドセーフな参照カウンタ型のスマートポインタ
    let lock0 = Arc::new(Mutex::new(0)); // <4>

    // 参照カウンタがインクリメントされるのみで
    // 中身はクローンされない
    let lock1 = lock0.clone(); // <5>

    // スレッド生成
    // クロージャ内変数へmove
    let th0 = thread::spawn(move || { // <6>
        some_func(lock0);
    });

    // スレッド生成
    // クロージャ内変数へmove
    let th1 = thread::spawn(move || {
        some_func(lock1);
    });

    // 待ち合わせ
    th0.join().unwrap();
    th1.join().unwrap();
}