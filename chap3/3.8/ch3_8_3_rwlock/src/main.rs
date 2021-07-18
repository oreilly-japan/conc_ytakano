use std::sync::RwLock; // <1>

fn main() {
    let lock = RwLock::new(10); // <2>
    {
        // immutableな参照を取得 <3>
        let v1 = lock.read().unwrap();
        let v2 = lock.read().unwrap();
        println!("v1 = {}", v1);
        println!("v2 = {}", v2);
    }

    {
        // mutableな参照を取得 <4>
        let mut v = lock.write().unwrap();
        *v = 7;
        println!("v = {}", v);
    }
}