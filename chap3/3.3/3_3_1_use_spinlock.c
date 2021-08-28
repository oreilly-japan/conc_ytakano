#include "3_3_1_spinlock_2.c"

bool lock = false; // 共有変数

void some_func() {
    for (;;) {
        spinlock_acquire(&lock); // ロック獲得 <1>
        // クリティカルセクション <2>
        spinlock_release(&lock); // ロック解放 <3>
    }
}