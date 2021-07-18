#include <pthread.h>
#include <stdio.h>

void barrier(volatile int *cnt, int max) { // <1>
    __sync_fetch_and_add(cnt, 1); // <2>
    while (*cnt < max); // <3>
}

volatile int num = 10; // 共有変数

void *worker(void *arg) { // スレッド用関数
    barrier(&num, 10); // 全スレッドがここまで到達するまで待つ <1>
    // 何らかの処理

    return NULL;
}

int main(int argc, char *argv[]) {
    // スレッド生成
    pthread_t th[10];
    for (int i = 0; i < 10; i++) {
        if (pthread_create(&th[i], NULL, worker, NULL) != 0) {
            perror("pthread_create"); return -1;
        }
    }
    // joinは省略
    return 0;
}