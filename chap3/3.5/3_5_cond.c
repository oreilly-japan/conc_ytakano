#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

pthread_mutex_t mut = PTHREAD_MUTEX_INITIALIZER; // <1>
pthread_cond_t cond = PTHREAD_COND_INITIALIZER;  // <2>

volatile bool ready = false; // <3>
char buf[256]; // スレッド間でデータを受け渡すためのバッファ

void* producer(void *arg) { // データ生成スレッド <4>
    printf("producer: ");
    fgets(buf, sizeof(buf), stdin); // 入力を受け取る

    pthread_mutex_lock(&mut);
    ready = true; // <5>

    if (pthread_cond_broadcast(&cond) !=0) { // 全体に通知 <6>
        perror("pthread_cond_broadcast"); exit(-1);
    }

    pthread_mutex_unlock(&mut);
    return NULL;
}

void* consumer(void *arg) { // データ消費スレッド <7>
    pthread_mutex_lock(&mut);

    while (!ready) { // ready変数の値がfalseの場合に待機
        // ロック解放と待機を同時に実行
        if (pthread_cond_wait(&cond, &mut) != 0) { // <8>
            perror("pthread_cond_wait"); exit(-1);
        }
    }

    pthread_mutex_unlock(&mut);
    printf("consumer: %s\n", buf);
    return NULL;
}

int main(int argc, char *argv[]) {
    // スレッド生成
    pthread_t pr, cn;
    pthread_create(&pr, NULL, producer, NULL);
    pthread_create(&cn, NULL, consumer, NULL);

    // スレッドの終了を待機
    pthread_join(pr, NULL);
    pthread_join(cn, NULL);

    // ミューテックスオブジェクトを解放
    pthread_mutex_destroy(&mut);

    // 条件変数オブジェクトを解放 <9>
    if (pthread_cond_destroy(&cond) != 0) {
        perror("pthread_cond_destroy"); return -1;
    }

    return 0;
}