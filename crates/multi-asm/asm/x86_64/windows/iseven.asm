
.CODE                ; ADD THIS

PUBLIC is_asm_even

; int isasmeven(int x)
is_asm_even PROC
    mov     eax, ecx
    and     eax, 1
    xor     eax, 1
    ret
is_asm_even ENDP

END