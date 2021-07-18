# 3.7 Readers-Writerロック

## ファイル

- 3_7_1_rwlock_spin.c: p81, p82
- 3_7_2_rwlock_pthreads.c: p83
- 3_7_3_performance.c: p84下
  - empty.c: p86下
  - mutex.c: p87の1番目
  - rwlock.c: p87の2番目
  - rwlock_wr.c: p87の3番目

## コンパイル

makeを実行すると、.oファイル、または実行ファイルが生成されます。

```sh
$ make
$ ls *.o 3_7_2_rwlock_pthreads 3_7_3_performance_RWLOCK 3_7_3_performance_RWLOCK_WR 3_7_3_performance_MUTEX 3_7_3_performance_EMPTY
3_7_1_rwlock_spin.o          3_7_2_rwlock_pthreads*       3_7_3_performance_EMPTY*     3_7_3_performance_MUTEX*     3_7_3_performance_RWLOCK*    3_7_3_performance_RWLOCK_WR*
```
