example:
	make clean
	nasm -f elf64 asm/loop.s -o add.o
	ld add.o -o example 

clean:
	rm -rf example add.o