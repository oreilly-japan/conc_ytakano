#define NUM 4

void semaphore_acquire(volatile int *cnt) { // <1>
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

#include "semtest.c"