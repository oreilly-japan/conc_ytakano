#include "../3.2/3_2_2_tas.c"

bool lock = false; // 共有変数

void some_func() {
retry:
    if (!test_and_set(&lock)) { // 検査とロック獲得
        // クリティカルセクション
    } else {
        goto retry;
    }
    tas_release(&lock); // ロック解放
}