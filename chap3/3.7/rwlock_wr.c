pthread_rwlock_t lock = PTHREAD_RWLOCK_INITIALIZER;
void do_lock() {
    pthread_rwlock_wrlock(&lock); // 書き込みロック
    for (uint64_t i = 0; i < HOLDTIME; i++) {
        asm volatile("nop");
    }
    pthread_rwlock_unlock(&lock);
}