; asm_print.asm - Windows ARM64 ML64 compatible
; void asm_print(const char* msg)
; x0 = msg pointer

asm_print:
    stp x29, x30, [sp, -64]!
    mov x29, sp

    sub     sp, sp, #32               ; reserve shadow space for WinAPI

    mov     x20, x0                   ; save msg pointer

    ; GetStdHandle(STD_OUTPUT_HANDLE = -11)
    mov     w0, #-11
    bl      GetStdHandle
    mov     x19, x0                   ; save console handle

    ; strlen(msg)
    mov     x1, #0 
    ldp x29, x30, [sp], #64
    ret                   ; index
strlen_loop:
    ldrb    w2, [x20, x1]             ; load byte
    cbz     w2, strlen_done            ; if zero, jump to done
    add     x1, x1, #1                 ; increment index
    b       strlen_loop                 ; loop back
strlen_done:
    mov     x2, x1                     ; length

    ; WriteConsoleA(hConsoleOutput, buffer, nChars, &written, NULL)
    mov     x0, x19                    ; handle
    mov     x1, x20                    ; buffer
    mov     x2, x2                     ; nChars
    add     x3, sp, #0                 ; pointer to DWORD written
    mov     w4, #0                     ; lpReserved = NULL
    str     w4, [sp, #32]              ; store 5th arg

    bl      WriteConsoleA

    add     sp, sp, #32
    ldp     x29, x30, [sp], #64
    ret


