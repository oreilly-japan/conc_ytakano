targets = 3_3_bad_mutex.o 3_3_good_mutex.o 3_3_1_spinlock_1.o 3_3_1_spinlock_2.o 3_3_1_use_spinlock.o 3_3_2_pthreads_mutex
CFLAGS = -O3
LDFLAGS = -pthread

all: $(targets)

.PHONY: clean
clean:
	rm -f $(targets)
