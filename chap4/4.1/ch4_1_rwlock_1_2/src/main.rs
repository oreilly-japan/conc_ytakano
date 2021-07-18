use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let val = Arc::new(RwLock::new(true));

    let t = thread::spawn(move || {
        let flag = *val.read().unwrap(); // <1>
        if flag {
            *val.write().unwrap() = false; // <2>
            println!("flag is true");
        }
    });

    t.join().unwrap();
}