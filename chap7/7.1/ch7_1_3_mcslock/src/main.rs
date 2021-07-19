use std::sync::Arc;

const NUM_LOOP: usize = 100000;
const NUM_THREADS: usize = 4;

mod mcs;

fn main() {
    let n = Arc::new(mcs::MCSLock::new(0));
    let mut v = Vec::new();

    for _ in 0..NUM_THREADS {
        let n0 = n.clone();
        let t = std::thread::spawn(move || {
            // ノードを作成してロック
            let mut node = mcs::MCSNode::new();
            for _ in 0..NUM_LOOP {
                let mut r = n0.lock(&mut node);
                *r += 1;
            }
        });

        v.push(t);
    }

    for t in v {
        t.join().unwrap();
    }

    // ノードを作成してロック
    let mut node = mcs::MCSNode::new();
    let r = n.lock(&mut node);
    println!(
        "COUNT = {} (expected = {})",
        *r,
        NUM_LOOP * NUM_THREADS
    );
}