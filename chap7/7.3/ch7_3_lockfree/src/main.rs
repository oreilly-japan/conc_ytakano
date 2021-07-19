#![feature(asm)]

use std::sync::Arc;

mod stack;

const NUM_LOOP: usize = 1000000; // ループ回数
const NUM_THREADS: usize = 4;    // スレッド数

use stack::Stack;

fn main() {
    let stack = Arc::new(Stack::<usize>::new());
    let mut v = Vec::new();

    for i in 0..NUM_THREADS {
        let stack0 = stack.clone();
        let t = std::thread::spawn(move || {
            if i & 1 == 0 {
                // 偶数スレッドはpush
                for j in 0..NUM_LOOP {
                    let k = i * NUM_LOOP + j;
                    stack0.get_mut().push(k);
                    println!("push: {}", k);
                }
                println!("finished push: #{}", i);
            } else {
                // 奇数スレッドはpop
                for _ in 0..NUM_LOOP {
                    loop {
                        // pop、Noneの場合やり直し
                        if let Some(k) = stack0.get_mut().pop() {
                            println!("pop: {}", k);
                            break;
                        }
                    }
                }
                println!("finished pop: #{}", i);
            }
        });
        v.push(t);
    }

    for t in v {
        t.join().unwrap();
    }

    assert!(stack.get_mut().pop() == None);
}