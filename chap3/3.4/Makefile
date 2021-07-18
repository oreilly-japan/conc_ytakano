targets = 3_4_semaphore.o 3_4_1_semaphore_llsc.o 3_4_2_posix_semaphore
CFLAGS = -O3
LDFLAGS = -lpthread

all: $(targets)

.PHONY: clean
clean:
	rm -f $(targets)
