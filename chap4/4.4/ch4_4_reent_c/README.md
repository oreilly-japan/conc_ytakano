## 書籍との修正点

[issue](https://github.com/oreilly-japan/conc_ytakano/issues/61)の指摘を受け、
`lock->id`の読み込みと代入に、`_atomic`系のビルトイン関数を利用しています。

1刷、2刷で掲載されているソースコードが若干異なっている可能性がありますが、基本的なアルゴリズムは異なりません。