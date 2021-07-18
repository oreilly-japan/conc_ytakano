#include "../3.2/3_2_2_tas.c"

void spinlock_aqcuire(volatile bool *lock) { // <1>
    for (;;) {
        while(*lock); // <2>
        if (!test_and_set(lock))
            break;
    }
}

void spinlock_release(bool *lock) {
    tas_release(lock);
}