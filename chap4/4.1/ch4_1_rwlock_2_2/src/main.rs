use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let val = Arc::new(RwLock::new(true));

    let t = thread::spawn(move || {
        let _ = val.read().unwrap(); // <1>
        *val.write().unwrap() = false; // <2>
        println!("not deadlock");
    });

    t.join().unwrap();
}