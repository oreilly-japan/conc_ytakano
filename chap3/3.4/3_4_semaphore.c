#define NUM 4

void semaphore_aqcuire(volatile int *cnt) { // <1>
    for (;;) {
        while (*cnt >= NUM); // <2>
        __sync_fetch_and_add(cnt, 1); // <3>
        if (*cnt <= NUM) // <4>
            break;
        __sync_fetch_and_sub(cnt, 1); // <5>
    }
}

void semaphore_release(int *cnt) {
    __sync_fetch_and_sub(cnt, 1); // <6>
}

int cnt = 0; // 共有変数

void some_func() {
    for (;;) {
        semaphore_aqcuire(&cnt); // ロック獲得
        // 何らかの処理
        semaphore_release(&cnt); // ロック解放
    }
}