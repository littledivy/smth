example:
	make clean
	nasm -f elf64 asm/print.s -o add.o
	ld add.o -o example 

clean:
	rm -rf example add.o