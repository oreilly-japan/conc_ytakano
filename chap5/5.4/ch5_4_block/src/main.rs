// ブロッキング関数
fn do_block(n: u64) -> u64 {
    let ten_secs = std::time::Duration::from_secs(10);
    std::thread::sleep(ten_secs);
    n
}

// async関数
async fn do_print() {
    let sec = std::time::Duration::from_secs(1);
    for _ in 0..20 {
        tokio::time::sleep(sec).await;
        println!("wake up");
    }
}

#[tokio::main]
pub async fn main() {
    // ブロッキング関数呼び出し
    let mut v = Vec::new();
    for n in 0..32 {
        let t = tokio::task::spawn_blocking(move || do_block(n)); // <1>
        v.push(t);
    }

    // async関数呼び出し。
    let p = tokio::spawn(do_print()); // <2>

    for t in v {
        let n = t.await.unwrap();
        println!("finished: {}", n);
    }

    p.await.unwrap()
}