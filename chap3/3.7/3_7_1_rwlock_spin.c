#include "../3.3/3_3_1_spinlock_2.c"

// Reader用ロック獲得関数 <1>
void rwlock_read_acquire(int *rcnt, volatile int *wcnt) {
    for (;;) {
        while (*wcnt); // Writerがいるなら待機 <2>
        __sync_fetch_and_add(rcnt, 1); // <3>
        if (*wcnt == 0) // Writerがいない場合にロック獲得 <4>
            break;
        __sync_fetch_and_sub(rcnt, 1);
    }
}

// Reader用ロック解放関数 <5>
void rwlock_read_release(int *rcnt) {
    __sync_fetch_and_sub(rcnt, 1);
}

// Writer用ロック獲得関数 <6>
void rwlock_write_acquire(bool *lock, volatile int *rcnt, int *wcnt) {
    __sync_fetch_and_add(wcnt, 1); // <7>
    while (*rcnt); // Readerがいるなら待機
    spinlock_acquire(lock); // <8>
}

// Writer用ロック解放関数 <9>
void rwlock_write_release(bool *lock, int *wcnt) {
    spinlock_release(lock);
    __sync_fetch_and_sub(wcnt, 1);
}

// 共有変数
int  rcnt = 0;
int  wcnt = 0;
bool lock = false;

void reader() { // Reader用関数
    for (;;) {
        rwlock_read_acquire(&rcnt, &wcnt);
        // クリティカルセクション（読み込みのみ）
        rwlock_read_release(&rcnt);
    }
}

void writer () { // Writer用関数
    for (;;) {
        rwlock_write_acquire(&lock, &rcnt, &wcnt);
        // クリティカルセクション（読み書き）
        rwlock_write_release(&lock, &wcnt);
    }
}