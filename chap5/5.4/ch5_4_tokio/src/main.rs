use tokio::io::{AsyncBufReadExt, AsyncWriteExt}; // <1>
use tokio::io;
use tokio::net::TcpListener; // <2>

#[tokio::main] // <3>
async fn main() -> io::Result<()> {
    // 10000番ポートでTCPリッスン <4>
    let listener = TcpListener::bind("127.0.0.1:10000").await.unwrap();

    loop {
        // TCPコネクションアクセプト <5>
        let (mut socket, addr) = listener.accept().await?;
        println!("accept: {}", addr);

        // 非同期タスク生成 <6>
        tokio::spawn(async move {
            // バッファ読み書き用オブジェクト生成 <7>
            let (r, w) = socket.split(); // <8>
            let mut reader = io::BufReader::new(r);
            let mut writer = io::BufWriter::new(w);

            let mut line = String::new();
            loop {
                line.clear(); // <9>
                match reader.read_line(&mut line).await { // <10>
                    Ok(0) => { // コネクションクローズ
                        println!("closed: {}", addr);
                        return;
                    }
                    Ok(_) => {
                        print!("read: {}, {}", addr, line);
                        writer.write_all(line.as_bytes()).await.unwrap();
                        writer.flush().await.unwrap();
                    }
                    Err(e) => { // エラー
                        println!("error: {}, {}", addr, e);
                        return;
                    }
                }
            }
        });
    }
}