.MODEL FLAT
.CODE                ; ADD THIS

PUBLIC _is_asm_even

; int is_asm_even(int x)
_is_asm_even PROC
    mov     eax, ecx
    and     eax, 1
    xor     eax, 1
    ret
_is_asm_even ENDP

END
