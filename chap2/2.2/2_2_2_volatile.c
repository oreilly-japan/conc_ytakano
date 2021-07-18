void wait_while_0(volatile int *p) {
    while (*p == 0) {}
}