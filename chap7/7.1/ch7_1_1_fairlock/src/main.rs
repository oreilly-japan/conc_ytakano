use std::sync::Arc;

const NUM_LOOP: usize = 100000;
const NUM_THREADS: usize = 4;

mod fairlock;

fn main() {
    let lock = Arc::new(fairlock::FairLock::new(0));
    let mut v = Vec::new();

    for i in 0..NUM_THREADS {
        let lock0 = lock.clone();
        let t = std::thread::spawn(move || {
            for _ in 0..NUM_LOOP {
                // スレッド番号を渡してロック
                let mut data = lock0.lock(i);
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
        *lock.lock(0),
        NUM_LOOP * NUM_THREADS
    );
}