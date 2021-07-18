use crate::semaphore::Semaphore;
use std::collections::LinkedList;
use std::sync::{Arc, Condvar, Mutex};

// 送信端のための型 <1>
#[derive(Clone)]
pub struct Sender<T> {
    sem: Arc<Semaphore>, // 有限性を実現するセマフォ
    buf: Arc<Mutex<LinkedList<T>>>, // キュー
    cond: Arc<Condvar>, // 読み込み側の条件変数
}

impl<T: Send> Sender<T> { // <2>
    // 送信関数
    pub fn send(&self, data: T) {
        self.sem.wait(); // キューの最大値に到達したら待機 <3>
        let mut buf = self.buf.lock().unwrap();
        buf.push_back(data); // エンキュー
        self.cond.notify_one(); // 読み込み側へ通知 <4>
    }
}

// 受信端のための型 <1>
pub struct Receiver<T> {
    sem: Arc<Semaphore>, // 有限性を実現するセマフォ
    buf: Arc<Mutex<LinkedList<T>>>, // キュー
    cond: Arc<Condvar>, // 読み込み側の条件変数
}

impl<T> Receiver<T> {
    pub fn recv(&self) -> T {
        let mut buf = self.buf.lock().unwrap();
        loop {
            // キューから取り出し <2>
            if let Some(data) = buf.pop_front() {
                self.sem.post(); // <3>
                return data;
            }
            // 空の場合待機 <4>
            buf = self.cond.wait(buf).unwrap();
        }
    }
}

pub fn channel<T>(max: isize) -> (Sender<T>, Receiver<T>) {
    assert!(max > 0);
    let sem = Arc::new(Semaphore::new(max));
    let buf = Arc::new(Mutex::new(LinkedList::new()));
    let cond = Arc::new(Condvar::new());
    let tx = Sender {
        sem: sem.clone(),
        buf: buf.clone(),
        cond: cond.clone(),
    };
    let rx = Receiver { sem, buf, cond };
    (tx, rx)
}