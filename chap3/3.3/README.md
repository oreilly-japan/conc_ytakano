# 3.3 ミューテックス

## ファイル

- 3_3_bad_mutex.c: p64
- 3_3_good_mutex.c: p66
- 3_3_1_spinlock_1.c: p67上
- 3_3_1_spinlock_2.c: p67下
- 3_3_1_use_spinlock.c: p68上
- 3_3_2_pthreads_mutex.c: p68下

## コンパイル

makeを実行すると、.oファイル、または実行ファイルが生成されます。

```sh
$ make
$ ls *.o
3_3_1_spinlock_1.o    3_3_1_spinlock_2.o    3_3_1_use_spinlock.o  3_3_bad_mutex.o       3_3_good_mutex.o
$ ls 3_3_2_pthreads_mutex
3_3_2_pthreads_mutex*
```
