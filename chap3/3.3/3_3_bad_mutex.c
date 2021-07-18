#include <stdbool.h>

bool lock = false; // 共有変数 <1>

void some_func() {
retry:
    if (!lock) { // <2>
        lock = true; // ロック獲得
        // クリティカルセクション
    } else {
        goto retry;
    }
    lock = false; // ロック解放 <3>
}