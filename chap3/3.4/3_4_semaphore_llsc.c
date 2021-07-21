// LL/SCを用いたセマフォのテストコードです
// このコードは書籍には記載ありません

#define NUM 4

void semaphore_aqcuire_llsc(volatile int *cnt);

void semaphore_aqcuire(int *cnt) {
    semaphore_aqcuire_llsc(cnt);
}

void semaphore_release(int *cnt) {
    __sync_fetch_and_sub(cnt, 1);
}

#include "semtest.c"