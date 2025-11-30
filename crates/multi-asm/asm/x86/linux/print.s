    .globl asm_print
    .type  asm_print, @function

# void asm_print(const char* msg)
asm_print:
    pushl %ebp
    movl %esp, %ebp

    movl 8(%ebp), %ecx    # msg pointer
    movl %ecx, %esi       # pointer for length loop

strlen_loop:
    cmpb $0, (%esi)
    je strlen_done
    incl %esi
    jmp strlen_loop

strlen_done:
    subl %ecx, %esi       # esi = len

    movl $4, %eax         # write syscall
    movl $1, %ebx         # stdout
    movl %ecx, %ecx       # buffer in ecx
    movl %esi, %edx       # len
    int $0x80

    movl %ebp, %esp
    popl %ebp
    ret
