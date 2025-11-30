; option casemap:none

.CODE

; float q_rsqrt(float number)
; Windows x64 ABI:
;   RCX = float argument (bit-pattern in low 32 bits)
;   Return float in XMM0
PUBLIC q_rsqrt

q_rsqrt PROC
    ; Move float from RCX to XMM0 (bitwise move)
    movd        xmm0, ecx

    ; x2 = number * 0.5f
    movss       xmm1, xmm0
    mulss       xmm1, DWORD PTR __half

    ; i = *(int*)&number
    movd        eax, xmm0

    ; i = 0x5f3759df - (i >> 1)
    shr         eax, 1
    mov         edx, 05F3759DFh
    sub         edx, eax

    ; y = *(float*)&i
    movd        xmm0, edx

    ; Newton iteration: y = y*(1.5 - x2*y*y)
    movss       xmm2, xmm0
    mulss       xmm2, xmm2        ; y^2
    mulss       xmm2, xmm1        ; x2*y^2
    movss       xmm3, DWORD PTR __threehalfs
    subss       xmm3, xmm2
    mulss       xmm0, xmm3

    ret
q_rsqrt ENDP

.data
__half        REAL4 0.5
__threehalfs  REAL4 1.5

END
