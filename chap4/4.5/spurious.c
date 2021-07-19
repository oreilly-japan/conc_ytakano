#include <pthread.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <unistd.h>

pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER;
pthread_cond_t cond = PTHREAD_COND_INITIALIZER;

// シグナルハンドラ <1>
void handler(int sig) { printf("received signal: %d\n", sig); }

int main(int argc, char *argv[]) {
    // プロセスIDを表示 <2>
    pid_t pid = getpid();
    printf("pid: %d\n", pid);

    // シグナルハンドラ登録
    signal(SIGUSR1, handler); // <3>

    // waitしているが、誰もnotifyしないので止まったままのはず <4>
    pthread_mutex_lock(&mutex);
    if (pthread_cond_wait(&cond, &mutex) != 0) {
        perror("pthread_cond_wait");
        exit(1);
    }
    printf("sprious wake up\n");
    pthread_mutex_unlock(&mutex);

    return 0;
}