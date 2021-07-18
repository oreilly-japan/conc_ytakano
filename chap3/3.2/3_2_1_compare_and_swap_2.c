#include <stdint.h>
#include <stdbool.h>

bool compare_and_swap(uint64_t *p, uint64_t val, uint64_t newval)
{
    return __sync_bool_compare_and_swap(p, val, newval);
}