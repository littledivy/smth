example:
	make clean
	nasm -f elf64 asm/cond.s -o add.o
	ld add.o -o example 

clean:
	rm -rf example add.o