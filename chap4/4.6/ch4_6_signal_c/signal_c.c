#include <pthread.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER;
sigset_t set;

void *handler(void *arg) { // <1>
    pthread_detach(pthread_self()); // デタッチ <2>

    int sig;
    for (;;) {
        if (sigwait(&set, &sig) != 0) { // <3>
            perror("sigwait");
            exit(1);
        }
        printf("received signal: %d\n", sig);
        pthread_mutex_lock(&mutex);
        // 何らかの処理
        pthread_mutex_unlock(&mutex);
    }

    return NULL;
}

void *worker(void *arg) { // <4>
    for (int i = 0; i < 10; i++) {
        pthread_mutex_lock(&mutex);
        // 何らかの処理
        sleep(1);
        pthread_mutex_unlock(&mutex);
        sleep(1);
    }
    return NULL;
}

int main(int argc, char *argv[]) {
    // プロセスIDを表示
    pid_t pid = getpid();
    printf("pid: %d\n", pid);

    // SIGUSR1シグナルをブロックに設定
    // この設定は、後に作成されるスレッドにも引き継がれる <5>
    sigemptyset(&set);
    sigaddset(&set, SIGUSR1);
    if (pthread_sigmask(SIG_BLOCK, &set, NULL) != 0) {
        perror("pthread_sigmask");
        return 1;
    }

    pthread_t th, wth;
    pthread_create(&th, NULL, handler, NULL);
    pthread_create(&wth, NULL, worker, NULL);
    pthread_join(wth, NULL);

    return 0;
}