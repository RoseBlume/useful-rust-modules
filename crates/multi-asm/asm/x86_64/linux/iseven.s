; x86_64 Windows, NASM syntax
; argument: first integer in RCX
; return: RAX

global is_asm_even
section .text

is_asm_even:
    mov rax, rcx
    and rax, 1
    xor rax, 1
    ret
