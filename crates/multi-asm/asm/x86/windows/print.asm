; option casemap:none
; q_rsqrt.asm
.CODE

PUBLIC q_rsqrt   ; MASM automatically adds _ prefix for x86

q_rsqrt PROC
    ; RCX is not used on x86; float arg comes via stack
    ; For __fastcall (default in MASM), first arg is in ECX
    movd xmm0, ecx       ; move int bits into XMM0

    ; x2 = number * 0.5
    movss xmm1, xmm0
    mulss xmm1, DWORD PTR __half

    ; i = *(int*)&number
    movd eax, xmm0

    ; i = 0x5f3759df - (i >> 1)
    shr eax, 1
    mov edx, 05F3759DFh
    sub edx, eax

    ; y = *(float*)&i
    movd xmm0, edx

    ; Newton iteration: y = y*(1.5 - x2*y*y)
    movss xmm2, xmm0
    mulss xmm2, xmm2
    mulss xmm2, xmm1
    movss xmm3, DWORD PTR __threehalfs
    subss xmm3, xmm2
    mulss xmm0, xmm3

    ret
q_rsqrt ENDP

.data
__half        REAL4 0.5
__threehalfs  REAL4 1.5

END
