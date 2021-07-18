targets = 2_2_1_1_pthreads_1 2_2_1_2_pthreads_2 2_2_2_wo_volatile.s 2_2_2_volatile.s

all: $(targets)

%.s: %.c
	$(CC) -O3 -S $<

$*: %.c
	$(CC) -lpthread $(CFLAGS) $< -o $@

.PHONY: clean
clean:
	rm -f $(targets)
