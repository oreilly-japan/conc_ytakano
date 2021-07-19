use tokio::sync::oneshot; // <1>

// 将来のどこかで値が決定される関数 <2>
async fn set_val_later(tx: oneshot::Sender<i32>) {
    let ten_secs = std::time::Duration::from_secs(10);
    tokio::time::sleep(ten_secs).await;
    if let Err(_) = tx.send(100) { // <3>
        println!("failed to send");
    }
}

#[tokio::main]
pub async fn main() {
    let (tx, rx) = oneshot::channel(); // <4>

    tokio::spawn(set_val_later(tx)); // <5>

    match rx.await { // 値読み込み <6>
        Ok(n) => {
            println!("n = {}", n);
        }
        Err(e) => {
            println!("failed to receive: {}", e);
            return;
        }
    }
}