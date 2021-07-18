#include <stdlib.h>

void barrier(volatile int *cnt, int max) { // <1>
    __sync_fetch_and_add(cnt, 1); // <2>
    while (*cnt < max); // <3>
}
