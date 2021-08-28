// テストコード

#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>

#define NUM_THREADS 10 // スレッド数
#define NUM_LOOP 10000 // スレッド内のループ数

int cnt = 0; // 共有変数

void *th(void *arg) {
    for (int i = 0; i < NUM_LOOP; i++) {
        semaphore_acquire(&cnt);
        if (cnt > 4) {
            printf("cnt = %d\n", cnt);
            exit(1);
        }
        semaphore_release(&cnt);
    }

    return NULL;
}

int main(int argc, char *argv[]) {
    // スレッド生成
    pthread_t v[NUM_THREADS];
    for (int i = 0; i < NUM_THREADS; i++) {
        pthread_create(&v[i], NULL, th, NULL);
    }

    printf("OK!\n");

    return 0;
}