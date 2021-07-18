#include <stdbool.h>

bool test_and_set(volatile bool *p) {
    return __sync_lock_test_and_set(p, 1);
}

void tas_release(volatile bool *p) {
    return __sync_lock_release(p);
}