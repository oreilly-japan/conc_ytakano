# 2.2 C言語

## ファイル

- 2_2_1_1_pthreads_1.c: p20
- 2_2_1_2_pthreads_2.c: p22
- 2_2_2_wo_volatile.c: p24上
- 2_2_2_volatile.c: p24下

## コンパイル or アセンブリファイル生成

makeを実行するとコンパイル、もしくはアセンブリファイルを生成します。

```sh
$ make
$ ls *.s 2_2_1_1_pthreads_1 2_2_1_2_pthreads_2
2_2_1_1_pthreads_1*  2_2_1_2_pthreads_2*  2_2_2_volatile.s     2_2_2_wo_volatile.s
```
