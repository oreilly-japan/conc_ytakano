targets = 3_4_semaphore 3_4_2_posix_semaphore
CFLAGS = -O3
LDFLAGS = -pthread

PLATFORM = $(shell uname -m)

ifeq ($(PLATFORM),arm64)
	targets += 3_4_semaphore_llsc 3_4_1_semaphore_llsc.o
else
ifeq ($(PLATFORM),aarch64)
	targets += 3_4_semaphore_llsc 3_4_1_semaphore_llsc.o
endif
endif

all: $(targets)

3_4_semaphore_llsc: 3_4_semaphore_llsc.c 3_4_1_semaphore_llsc.o
	$(CC) $(CFLAGS) $(LDFLAGS) $^ -o $@

plat:
	echo $(PLATFORM)

.PHONY: clean
clean:
	rm -f $(targets)
