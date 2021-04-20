section .text
  global _start

_start:
L2:
  mov ecx, '1'
  sub ecx, '0'
  mov edx, '2'
  sub edx, '0'
  mov eax, ecx
  add eax, edx
  add eax, '0'
  mov [OenuQQL5Mv], eax
  mov ecx, OenuQQL5Mv
  mov edx, 2
  mov ebx, 1
  mov eax, 4
  int 0x80
  mov ecx, '2'
  cmp ecx, 1
  je L2
  mov eax, 1
  int 0x80
section .data

segment .bss
  OenuQQL5Mv resb 1
  exLeRBEQws resb 1
  D3BpzEatPk resb 1