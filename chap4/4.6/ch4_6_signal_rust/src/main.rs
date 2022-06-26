use signal_hook::{iterator::Signals}; // <1>
use std::{error::Error, process, thread, time::Duration};
use libc::SIGUSR1;

fn main() -> Result<(), Box<dyn Error>> {
    // プロセスIDを表示
    println!("pid: {}", process::id());

    let mut signals = Signals::new(&[SIGUSR1])?; // <2>
    thread::spawn(move || {
        // シグナルを受信
        for sig in signals.forever() { // <3>
            println!("received signal: {:?}", sig);
        }
    });

    // 10秒スリープ
    thread::sleep(Duration::from_secs(10));
    Ok(())
}