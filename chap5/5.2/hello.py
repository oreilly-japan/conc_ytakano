def hello():
    print('Hello,', end='')
    yield  # ここで中断、再開 <1>
    print('World!')
    yield  # ここで中断、再開 <2>

h = hello()  # 1まで実行
h.__next__() # 1から再開し2まで実行
h.__next__() # 2から再開