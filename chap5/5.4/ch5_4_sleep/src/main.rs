// ブロックしてしまうsleep
// use std::{thread, time}; // <1>
//
// #[tokio::main]
// async fn main() {
//     // joinで終了を待機
//     tokio::join!(async move { // <2>
//         // 10秒スリープ <3>
//         let ten_secs = time::Duration::from_secs(10);
//         thread::sleep(ten_secs);
//     });
// }

// Tokioの関数を持用いたスリープ
use std::time;

#[tokio::main]
async fn main() {
    // joinで終了を待機
    tokio::join!(async move {
        // 10秒スリープ
        let ten_secs = time::Duration::from_secs(10);
        tokio::time::sleep(ten_secs).await; // <1>
    });
}