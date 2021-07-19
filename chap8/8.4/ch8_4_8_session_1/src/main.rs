extern crate session_types;
use session_types as S; // <1>
use std::thread;

type Client = S::Send<u64, S::Choose<S::Recv<u64, S::Eps>, S::Recv<bool, S::Eps>>>; // クライアントの端点の型 <2>
type Server = <Client as S::HasDual>::Dual; // サーバの端点の型 <3>

enum Op {
    Square, // 2乗命令
    Even,   // 偶数判定命令
}

fn server(c: S::Chan<(), Server>) {
    let (c, n) = c.recv(); // データ受信 <1>
    match c.offer() {
        S::Branch::Left(c) => { // 2乗命令 <2>
            c.send(n * n).close(); // <3>
        }
        S::Branch::Right(c) => { // 偶数判定命令 <4>
            c.send(n & 1 == 0).close(); // <5>
        }
    }
}

fn client(c: S::Chan<(), Client>, n: u64, op: Op) {
    let c = c.send(n); // <1>
    match op {
        Op::Square => {
            let c = c.sel1();        // 1番目の選択肢を選択 <2>
            let (c, val) = c.recv(); // データ受信 <3>
            c.close();               // セッション終了 <4>
            println!("{}^2 = {}", n, val);
        }
        Op::Even => {
            let c = c.sel2();        // 2番目の選択肢を選択 <5>
            let (c, val) = c.recv(); // データ受信 <6>
            c.close();               // セッション終了 <7>
            if val {
                println!("{} is even", n);
            } else {
                println!("{} is odd", n);
            }
        }
    };
}

fn main() {
    // Evenの例
    let (server_chan, client_chan) = S::session_channel();
    let srv_t = thread::spawn(move || server(server_chan));
    let cli_t = thread::spawn(move || client(client_chan, 11, Op::Even));
    srv_t.join().unwrap();
    cli_t.join().unwrap();

    // Squareの例
    let (server_chan, client_chan) = S::session_channel();
    let srv_t = thread::spawn(move || server(server_chan));
    let cli_t = thread::spawn(move || client(client_chan, 11, Op::Square));
    srv_t.join().unwrap();
    cli_t.join().unwrap();
}