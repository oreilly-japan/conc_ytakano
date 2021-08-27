#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>

pthread_mutex_t barrier_mut = PTHREAD_MUTEX_INITIALIZER;
pthread_cond_t barrier_cond = PTHREAD_COND_INITIALIZER;

void barrier(volatile int *cnt, int max) {
    if (pthread_mutex_lock(&barrier_mut) != 0) {
        perror("pthread_mutex_lock"); exit(-1);
    }

    (*cnt)++; // <1>

    if (*cnt == max) { // <2>
        // 全プロセスが揃ったので通知 <3>
        if (pthread_cond_broadcast(&barrier_cond) != 0) {
            perror("pthread_cond_broadcast"); exit(-1);
        }
    } else {
        do { // 全プロセスが揃うまで待機 <4>
            if (pthread_cond_wait(&barrier_cond,
                                  &barrier_mut) != 0) {
                perror("pthread_cond_wait"); exit(-1);
            }
        } while (*cnt < max); // 擬似覚醒のための条件
    }

    if (pthread_mutex_unlock(&barrier_mut) != 0) {
        perror("pthread_mutex_unlock"); exit(-1);
    }
}
