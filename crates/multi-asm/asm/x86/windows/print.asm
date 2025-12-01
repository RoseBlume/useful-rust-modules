; 32-bit x86 Windows assembly for Rust FFI
; stdcall calling convention
; void _asm_print(const char* msg);
.386
extrn _GetStdHandle@4:proc 
;:proc
extrn _WriteConsoleA@20:proc

STD_OUTPUT_HANDLE equ -11
.MODEL FLAT
.DATA
charsWritten dd ?          ; DWORD for number of chars written

.CODE
PUBLIC _asm_print

_asm_print PROC
    push    ebp
    mov     ebp, esp
    sub     esp, 8              ; local space (charsWritten already in .DATA)

    mov     esi, [ebp+8]        ; msg pointer from stack

    ; GetStdHandle(STD_OUTPUT_HANDLE)
    push    STD_OUTPUT_HANDLE
    call    _GetStdHandle@4
    mov     ebx, eax            ; store handle in ebx

    ; strlen(msg)
    xor     ecx, ecx
strlen_loop:
    cmp     byte ptr [esi+ecx], 0
    je      strlen_done
    inc     ecx
    jmp     strlen_loop
strlen_done:
    mov     edx, ecx            ; length in edx

    ; WriteConsoleA(hConsole, lpBuffer, nChars, lpWritten, lpReserved)
    push    0                   ; lpReserved = NULL
    lea     eax, charsWritten
    push    eax                 ; LPDWORD written
    push    edx                 ; nChars
    push    esi                 ; buffer
    push    ebx                 ; handle
    call    _WriteConsoleA@20

    mov     esp, ebp
    pop     ebp
    ret
_asm_print ENDP

END