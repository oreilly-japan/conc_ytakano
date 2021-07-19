use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpListener;

fn main() {
    // TCPの10000番ポートをリッスン
    let listener = TcpListener::bind("127.0.0.1:10000").unwrap(); // <1>

    // コネクション要求をアクセプト
    while let Ok((stream, _)) = listener.accept() { // <2>
        // 読み込み、書き込みオブジェクトを生成 <3>
        let stream0 = stream.try_clone().unwrap();
        let mut reader = BufReader::new(stream0);
        let mut writer = BufWriter::new(stream);

        // 1行読み込んで、同じものを書き込み <4>
        let mut buf = String::new();
        reader.read_line(&mut buf).unwrap();
        writer.write(buf.as_bytes()).unwrap();
        writer.flush().unwrap(); // <5>
    }
}