mod banker;

use banker::Banker;
use std::thread;

const NUM_LOOP: usize = 100000;

fn main() {
    // 利用可能な箸の数と、哲学者の利用する最大の箸の数を設定
    let banker = Banker::<2, 2>::new([1, 1], [[1, 1], [1, 1]]);
    let banker0 = banker.clone();

    let philosopher0 = thread::spawn(move || {
        for _ in 0..NUM_LOOP {
            // 箸0と1を確保
            while !banker0.take(0, 0) {}
            while !banker0.take(0, 1) {}

            println!("0: eating");

            // 箸0と1を解放
            banker0.release(0, 0);
            banker0.release(0, 1);
        }
    });

    let philosopher1 = thread::spawn(move || {
        for _ in 0..NUM_LOOP {
            // 箸1と0を確保
            while !banker.take(1, 1) {}
            while !banker.take(1, 0) {}

            println!("1: eating");

            // 箸1と0を解放
            banker.release(1, 1);
            banker.release(1, 0);
        }
    });

    philosopher0.join().unwrap();
    philosopher1.join().unwrap();
}