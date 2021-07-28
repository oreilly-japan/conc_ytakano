# 5.3 async/await

各ディレクトリにCargo用のリポジトリがあるので、ディレクトリに移動後cargoでコンパイル・実行して下さい。
ビルド時に```--release```を指定すると正しく効果が分かります。

## コンパイルと実行例

以下のようにディレクトリに移動後実行。epollを利用しているため、Linuxのみ対象です。

```sh
$ cd ch5_3_2_ioselect
$ cargo run --release
```
