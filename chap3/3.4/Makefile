targets = 3_4_semaphore 3_4_semaphore_llsc.o 3_4_1_semaphore_llsc.o 3_4_2_posix_semaphore 3_4_semaphore_llsc
CFLAGS = -O3
LDFLAGS = -pthread

all: $(targets)

3_4_semaphore_llsc: 3_4_semaphore_llsc.o 3_4_1_semaphore_llsc.o
	$(CC) $(CFLAGS) $(LDFLAGS) $^ -o $@

.PHONY: clean
clean:
	rm -f $(targets)
