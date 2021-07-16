targets = pthreads_1 pthreads_2

all: $(targets)

$*: %.c
	$(CC) -lpthread $(CFLAGS) %< -o $@

.PHONY: clean
clean:
	rm -f $(targets)
