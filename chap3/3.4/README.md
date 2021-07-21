# 3.4 セマフォ

## ファイル

- 3_4_semaphore.c: p70 + セマフォのテストコード（書籍には記載なし）
- 3_4_semaphore_llsc.c: LL/SC版セマフォのテストコード（書籍には記載なし）
- 3_4_1_semaphore_llsc.S: p71下
- 3_4_2_posix_semaphore.c: p72

## コンパイル

makeを実行すると、.oファイル、または実行ファイルが生成されます。LL/SC版はAArch64環境でのみコンパイルします。

```sh
$ make
$ ls *.o
3_4_1_semaphore_llsc.o  3_4_semaphore.o
$ ls 3_4_semaphore 3_4_semaphore_llsc 3_4_2_posix_semaphore
3_4_2_posix_semaphore* 3_4_semaphore*         3_4_semaphore_llsc*
```

各実行ファイルを実行するとテストが行われます。
