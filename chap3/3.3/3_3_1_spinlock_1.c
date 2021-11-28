#include "../3.2/3_2_2_tas.c"

void spinlock_acquire(bool *lock) {
    while (!test_and_set(lock)); // <1>
}

void spinlock_release(bool *lock) {
    tas_release(lock); // <2>
}
