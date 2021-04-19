Tiny compiler. Compiles down to x86 assembly.

```python
print(1 + 2)
```

Generated asm:
```asm
section .text
    global _start

_start:
    mov ecx, '1'
    sub ecx, '0'
    mov edx, '2'
    sub edx, '0'
    mov eax, ecx
    add eax, edx
    add eax, '0'
    mov [vP30pEpztB], eax
    mov ecx, vP30pEpztB
    mov edx, 2
    mov ebx, 1
    mov eax, 4
    int 0x80
    mov eax, 1
    int 0x80

segment .bss
    vP30pEpztB resb 1
    NfrWQ1Xylg resb 1
```