cas = 3_2_1_cas_1.s 3_2_1_cas_2.s 3_2_2_tas.s

all: $(cas)

%.s: %.c
	$(CC) -O3 -S $<

.PHONY: clean
clean:
	rm -f *.s
