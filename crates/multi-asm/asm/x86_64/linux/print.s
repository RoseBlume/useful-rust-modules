; x86_64 Windows, NASM syntax
; arguments: RCX = handle, RDX = string pointer, R8 = length, R9 = pointer for written
; We’ll use RCX = string pointer, and call GetStdHandle inside

global asm_print
extern GetStdHandle
extern WriteConsoleA

STD_OUTPUT_HANDLE equ -11

section .text

asm_print:
    mov rdx, rcx              ; string pointer

    mov rcx, STD_OUTPUT_HANDLE
    call GetStdHandle          ; returns handle in RAX
    mov rcx, rax               ; console handle
    mov r8, rdx                ; string pointer

    ; compute length
    mov rax, r8
strlen_loop:
    cmp byte [rax], 0
    je strlen_done
    inc rax
    jmp strlen_loop
strlen_done:
    sub rax, r8                 ; length in RAX

    mov rdx, rax                ; number of chars
    sub rsp, 32                 ; shadow space, align stack
    lea r9, [rsp+8]             ; DWORD written
    xor rax, rax
    call WriteConsoleA
    add rsp, 32
    ret
