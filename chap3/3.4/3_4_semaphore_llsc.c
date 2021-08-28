// LL/SCを用いたセマフォのテストコードです
// このコードは書籍には記載ありません

#define NUM 4

void semaphore_acquire_llsc(volatile int *cnt);

void semaphore_acquire(int *cnt) {
    semaphore_acquire_llsc(cnt);
}

void semaphore_release(int *cnt) {
    __sync_fetch_and_sub(cnt, 1);
}

#include "semtest.c"