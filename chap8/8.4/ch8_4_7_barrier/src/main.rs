use std::sync::mpsc::{channel, Sender}; // <1>

fn main() {
    let mut v = Vec::new();

    // チャネルを作成 <2>
    let (tx, rx) = channel::<Sender<()>>();

    // バリア同期用スレッド <3>
    let barrier = move || {
        let x = rx.recv().unwrap();
        let y = rx.recv().unwrap();
        let z = rx.recv().unwrap();
        println!("send!");
        x.send(()).unwrap();
        y.send(()).unwrap();
        z.send(()).unwrap();
    };
    let t = std::thread::spawn(barrier);
    v.push(t);

    // クライアントスレッド <4>
    for _ in 0..3 {
        let tx_c = tx.clone(); // <5>
        let node = move || {
            // バリア同期 <6>
            let (tx0, rx0) = channel();
            tx_c.send(tx0).unwrap();
            rx0.recv().unwrap();
            println!("received!");
        };
        let t = std::thread::spawn(node);
        v.push(t);
    }

    for t in v {
        t.join().unwrap();
    }
}