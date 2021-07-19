all: fizzbuzz

fizzbuzz: fizzbuzz.o
	ld fizzbuzz.o -o fizzbuzz

fizzbuzz.o: fizzbuzz.S
	as fizzbuzz.S -o fizzbuzz.o

clean:
	rm -f *.o fizzbuzz
