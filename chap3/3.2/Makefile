cas = 3_2_1_compare_and_swap_1.s 3_2_1_compare_and_swap_2.s

all: $(cas)

%.s: %.c
	$(CC) -O3 -S $<

.PHONY: clean
clean:
	rm -f *.s
