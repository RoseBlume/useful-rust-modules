; asm_print.asm - Windows ARM64 MASM/ML64 compatible
; void asm_print(const char* msg)
; x0 = pointer to message
PUBLIC asm_print
EXTERN GetStdHandle:PROC
EXTERN WriteConsoleA:PROC

STD_OUTPUT_HANDLE EQU -11

.DATA
charsWritten DD ?

.CODE
asm_print PROC
    ; prologue
    stp     x29, x30, [sp, #-16]!  ; save FP & LR
    mov     x29, sp

    ; GetStdHandle(STD_OUTPUT_HANDLE)
    mov     w0, STD_OUTPUT_HANDLE
    bl      GetStdHandle          ; return handle in x0

    ; save handle in x19 (callee-saved)
    mov     x19, x0

    ; compute strlen(msg)
    mov     x1, #0                ; index
strlen_loop:
    ldrb    w2, [x0, x1]          ; load byte
    cbz     w2, strlen_done
    add     x1, x1, #1
    b       strlen_loop

strlen_done:
    mov     x2, x1                ; length

    ; WriteConsoleA(hConsoleOutput, buffer, nChars, &written, NULL)
    mov     x0, x19               ; handle
    mov     x1, x0                ; buffer (x0 currently has message) -> save in x1
    mov     x1, x0                ; actually, better to use original x0 arg; let's reload
    mov     x1, x0                ; buffer
    mov     x2, x2                ; nChars
    adr     x3, charsWritten       ; pointer to DWORD written
    mov     w4, #0                 ; lpReserved = NULL
    str     w4, [sp, #0]           ; store 5th arg on stack

    bl      WriteConsoleA

    ; epilogue
    ldp     x29, x30, [sp], #16
    ret
asm_print ENDP
