extrn GetStdHandle:proc
extrn WriteConsoleA:proc

STD_OUTPUT_HANDLE equ -11

.DATA
charsWritten dq ?

.CODE
PUBLIC asm_print

; void asm_print(const char* msg)
asm_print PROC
    push    rbp
    mov     rbp, rsp
    sub     rsp, 40                 ; shadow space + local

    mov     rsi, rcx                ; msg ptr

    ; GetStdHandle(STD_OUTPUT_HANDLE)
    mov     ecx, STD_OUTPUT_HANDLE
    call    GetStdHandle
    mov     rbx, rax

    ; strlen(msg)
    xor     rcx, rcx
strlen_loop:
    cmp     byte ptr [rsi+rcx], 0
    je      strlen_done
    inc     rcx
    jmp     strlen_loop
strlen_done:
    mov     rdx, rcx                ; save length in RDX temporarily
                                    ; (RCX will be overwritten below)

    ; WriteConsoleA(h, msg, len, &written, NULL)
    mov     rcx, rbx                ; HANDLE hConsoleOutput
    mov     r8,  rdx                ; DWORD nChars
    lea     r9, charsWritten        ; LPDWORD written
    mov     rdx, rsi                ; LPCVOID buffer

    ; 5th argument must be on stack (aligned)
    mov     qword ptr [rsp+32], 0   ; lpReserved = NULL

    call    WriteConsoleA

    mov     rsp, rbp
    pop     rbp
    ret
asm_print ENDP

END
