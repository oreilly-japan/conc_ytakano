#[macro_use]
extern crate session_types;
use session_types as S; // <1>
use std::thread;
use std::collections::HashMap;

type Put = S::Recv<u64, S::Recv<u64, S::Var<S::Z>>>;
type Get = S::Recv<u64, S::Send<Option<u64>, S::Var<S::Z>>>;

type DBServer = S::Rec<S::Offer<Put, S::Offer<Get, S::Eps>>>;
type DBClient = <DBServer as S::HasDual>::Dual;

fn db_server_macro(c: S::Chan<(), DBServer>) {
    let mut c_enter = c.enter();
    let mut db = HashMap::new();

    loop {
        let c = c_enter;
        offer! {c, // <1>
            Put => { // <2>
                let (c, key) = c.recv();
                let (c, val) = c.recv();
                db.insert(key, val);
                c_enter = c.zero();
            },
            Get => {
                let (c, key) = c.recv();
                    let c = if let Some(val) = db.get(&key) {
                        c.send(Some(*val))
                    } else {
                        c.send(None)
                    };
                    c_enter = c.zero();
            },
            Quit => {
                c.close();
                return;
            }
        }
    }
}

fn db_server(c: S::Chan<(), DBServer>) {
    let mut c_enter = c.enter(); // <1>
    let mut db = HashMap::new(); // DBデータ

    loop {
        match c_enter.offer() { // Putが選択された <2>
            S::Branch::Left(c) => {
                let (c, key) = c.recv();
                let (c, val) = c.recv();
                db.insert(key, val); // DBへデータ挿入
                c_enter = c.zero();  // Recへジャンプ <3>
            }
            S::Branch::Right(c) => match c.offer() { // Get or 終了 <4>
                S::Branch::Left(c) => { // Getが選択された <5>
                    let (c, key) = c.recv();
                    let c = if let Some(val) = db.get(&key) {
                        c.send(Some(*val))
                    } else {
                        c.send(None)
                    };
                    c_enter = c.zero(); // Recへジャンプ <6>
                }
                S::Branch::Right(c) => { // 終了が選択 <7>
                    c.close(); // セッションクローズ <8>
                    return;
                }
            },
        }
    }
}

fn db_client(c: S::Chan<(), DBClient>) {
    let c = c.enter(); // Recの中へ処理を移行
    // Putを2回実施
    let c = c.sel1().send(10).send(4).zero();
    let c = c.sel1().send(50).send(7).zero();

    // Get
    let (c, val) = c.sel2().sel1().send(10).recv();
    println!("val = {:?}", val); // Some(4)

    let c = c.zero(); // Recへジャンプ

    // Get
    let (c, val) = c.sel2().sel1().send(20).recv();
    println!("val = {:?}", val); // None

    // 終了
    let _ = c.zero().sel2().sel2().close();
}

type SChan = S::Chan<(), S::Send<(), S::Eps>>; // <1>
type ChanRecv = S::Recv<SChan, S::Eps>; // <2>
type ChanSend = <ChanRecv as S::HasDual>::Dual;

fn chan_recv(c: S::Chan<(), ChanRecv>) {
    let (c, cr) = c.recv(); // チャネルの端点を受信 <3>
    c.close();
    let cr = cr.send(()); // 受信した端点に対して送信 <4>
    cr.close();
}

fn chan_send(c: S::Chan<(), ChanSend>) {
    let (c1, c2) = S::session_channel(); // チャネルの生成
    let c = c.send(c1); // チャネルの端点を送信 <5>
    c.close();
    let (c2, _) = c2.recv(); // 送信した端点の反対側より受信 <6>
    c2.close();
}

fn main() {
    let (server_chan, client_chan) = S::session_channel();
    let srv_t = thread::spawn(move || db_server(server_chan));
    let cli_t = thread::spawn(move || db_client(client_chan));
    srv_t.join().unwrap();
    cli_t.join().unwrap();

    println!("--------------------");

    // マクロの利用例
    let (server_chan, client_chan) = S::session_channel();
    let srv_t = thread::spawn(move || db_server_macro(server_chan));
    let cli_t = thread::spawn(move || db_client(client_chan));
    srv_t.join().unwrap();
    cli_t.join().unwrap();

    println!("--------------------");

    // チャネル送受信の利用例
    let (server_chan, client_chan) = S::session_channel();
    let srv_t = thread::spawn(move || chan_recv(server_chan));
    let cli_t = thread::spawn(move || chan_send(client_chan));
    srv_t.join().unwrap();
    cli_t.join().unwrap();
}