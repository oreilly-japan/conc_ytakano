# 7.3 ロックフリーデータ構造とアルゴリズム

各ディレクトリにCargo用のリポジトリがあるので、ディレクトリに移動後cargoでコンパイル・実行して下さい。
ビルド時に```--release```を指定すると正しく効果が分かります。

## コンパイルと実行例

以下のようにディレクトリに移動後実行。AArch64環境のみ実行可能です。インラインアセンブリを利用しているため、nightlyのみ対応です。

```sh
$ cd ch7_3_lockfree
$ cargo +nightly run --release
```
