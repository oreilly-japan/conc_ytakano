#include <stdint.h>
#include <stdbool.h>

bool compare_and_swap(uint64_t *p, uint64_t val, uint64_t newval)
{
    if (*p != val) { // <1>
        return false;
    }
    *p = newval; // <2>
    return true;
}