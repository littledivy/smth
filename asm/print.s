section .text
  global _start

_start:
  mov ecx, '1'
  sub ecx, '0'
  mov edx, '1'
  sub edx, '0'
  mov eax, ecx
  add eax, edx
  add eax, '0'
  mov [BwpXqIc4E8], eax
  mov ecx, BwpXqIc4E8
  mov edx, 2
  mov ebx, 1
  mov eax, 4
  int 0x80
  mov eax, 1
  int 0x80

segment .bss
  BwpXqIc4E8 resb 1
 