use std::ptr::null_mut;
use std::sync::atomic::{AtomicPtr, Ordering};

// スタックのノード。リスト構造で管理 <1>
struct Node<T> {
    next: AtomicPtr<Node<T>>,
    data: T,
}

// スタックの先頭
pub struct StackBad<T> {
    head: AtomicPtr<Node<T>>,
}

impl<T> StackBad<T> {
    pub fn new() -> Self {
        StackBad {
            head: AtomicPtr::new(null_mut()),
        }
    }

    pub fn push(&self, v: T) { // <2>
        // 追加するノードを作成
        let node = Box::new(Node {
            next: AtomicPtr::new(null_mut()),
            data: v,
        });

        // Box型の値からポインタを取り出す
        let ptr = Box::into_raw(node);

        unsafe {
            // アトミックにヘッドを更新 <3>
            loop {
                // headの値を取得
                let head = self.head.load(Ordering::Relaxed);

                // 追加するノードのnextをheadに設定
                (*ptr).next.store(head, Ordering::Relaxed);

                // headの値が更新されていなければ、追加するノードに更新
                if let Ok(_) =
                    self.head
                        .compare_exchange_weak(
                            head, // 値がheadなら
                            ptr,  // ptrに更新
                            Ordering::Release, // 成功時のオーダー
                            Ordering::Relaxed  // 失敗時のオーダー
                ) {
                    break;
                }
            }
        }
    }

    pub fn pop(&self) -> Option<T> { // <4>
        unsafe {
            // アトミックにヘッドを更新
            loop {
                // headの値を取得 <5>
                let head = self.head.load(Ordering::Relaxed);
                if head == null_mut() {
                    return None; // headがヌルの場合にNone
                }

                // head.nextを取得 <6>
                let next = (*head).next.load(Ordering::Relaxed);

                // headの値が更新されていなければ、
                // head.nextを新たなheadに更新 <7>
                if let Ok(_) = self.head.compare_exchange_weak(
                    head, // 値がheadなら
                    next, // nextに更新
                    Ordering::Acquire, // 成功時のオーダー
                    Ordering::Relaxed, // 失敗時のオーダー
                ) {
                    // ポインタをBoxに戻して、中の値をリターン
                    let h = Box::from_raw(head);
                    return Some((*h).data);
                }
            }
        }
    }
}

impl<T> Drop for StackBad<T> {
    fn drop(&mut self) {
        // データ削除
        let mut node = self.head.load(Ordering::Relaxed);
        while node != null_mut() {
            // ポインタをBoxに戻す操作を繰り返す
            let n = unsafe { Box::from_raw(node) };
            node = n.next.load(Ordering::Relaxed)
        }
    }
}