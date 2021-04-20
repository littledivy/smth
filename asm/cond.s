section .text
  global _start

_start:
  cmp '1', 1
  je L1
L1:
  mov ecx, '1'
  sub ecx, '0'
  mov edx, '2'
  sub edx, '0'
  mov eax, ecx
  add eax, edx
  add eax, '0'
  mov [i1mMBjOeuI], eax
  mov ecx, i1mMBjOeuI
  mov edx, 2
  mov ebx, 1
  mov eax, 4
  int 0x80
  mov eax, 1
  int 0x80
section .data

segment .bss
  i1mMBjOeuI resb 1
  xwxtYPBFiG resb 1
  vqXOHD3onk resb 1