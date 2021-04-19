package main

import (
	"fmt"
	"strconv"
)

var source = `
section	.text
	global _start
`

func declare(x, y int) string {
	return `
	mov	eax,'` + strconv.Itoa(x) + `'
	sub eax, '0'
	 
	mov ebx, '` + strconv.Itoa(y) +`'
	sub ebx, '0'
`
}

func sub() string {
	return `	
	sub eax, ebx
	sub	eax, '0'

	mov [result], eax
`
}

func mul(x, y int) string {
	return `
	mov	al,'` + strconv.Itoa(x) + `'
	sub al, '0'
	 
	mov bl, '` + strconv.Itoa(y) +`'
	sub bl, '0'
	mul 	bl
	add	al, '0'	

	mov [result], al
`
}

func add() string {
	return `	
	add eax, ebx
	add	eax, '0'

	mov [result], eax
`
}

func main() {
	source += `
_start: 
`
	source += declare(3, 4)
	source += add()
	source += mul(1, 2)

	// Print result
	source += `
	mov	ecx, result
	mov	edx, 2
	mov	ebx, 1	
	mov	eax, 4	
	int	0x80
`
	// Call sys_exit
	source += `
	mov	eax, 1  ; exit
	int	0x80
`

	// Define result variable
	source += `
segment .bss
	result resb 1
`
	fmt.Println(source)
}
