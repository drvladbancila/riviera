PREFIX=riscv64-unknown-linux-gnu-
ROOTDIR=..
DISTDIR=$(ROOTDIR)/compiled
CC=$(PREFIX)gcc -march=rv64g -nostdlib

bubblesort: main.o lfsr.o
	$(CC) -o $(DISTDIR)/bubblesort main.o lfsr.o
clean_obj:
	rm -rf *.o

main.o: main.c
	$(CC) -c main.c

lfsr.o: $(ROOTDIR)/lfsr/lfsr.c $(ROOTDIR)/lfsr/lfsr.h
	$(CC) -c $(ROOTDIR)/lfsr/lfsr.c