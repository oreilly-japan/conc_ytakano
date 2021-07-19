use std::sync::{Arc, Mutex};

const NUM_TASKS: usize = 4; // タスク数
const NUM_LOOP: usize = 100000; // ループ数

#[tokio::main]
async fn main() -> Result<(), tokio::task::JoinError> {
    let val = Arc::new(Mutex::new(0)); // 共有変数 <1>
    let mut v = Vec::new();
    for _ in 0..NUM_TASKS {
        let n = val.clone();
        let t = tokio::spawn(async move { // タスク生成 <2>
            for _ in 0..NUM_LOOP {
                let mut n0 = n.lock().unwrap();
                *n0 += 1; // インクリメント <3>
            }
        });

        v.push(t);
    }

    for i in v {
        i.await?;
    }

    println!("COUNT = {} (expected = {})",
        *val.lock().unwrap(), NUM_LOOP * NUM_TASKS);
    Ok(())
}