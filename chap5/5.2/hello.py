def hello():
    print('Hello,', end='')
    yield  # ここで中断、再開 <1>
    print('World!')
    yield  # ここまで実行 (2)

h = hello()  # イテレータを生成
h.__next__() # 1まで実行し中断
h.__next__() # 1から再開し2まで実行
