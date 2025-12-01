option casemap:none
.MODEL FLAT
.DATA
__half        REAL4 0.5
__threehalfs  REAL4 1.5

.CODE

; float q_rsqrt(float number)
; 32-bit: float argument passed on stack [esp+4], return in xmm0
PUBLIC _q_rsqrt

_q_rsqrt PROC
    ; Load argument into xmm0
    movss   xmm0, DWORD PTR [esp+4]   ; number

    ; Compute x2 = number * 0.5
    movaps  xmm1, xmm0                ; reuse xmm1 for x2
    mulss   xmm1, DWORD PTR __half

    ; Fast inverse sqrt bit hack
    movd    eax, xmm0                 ; treat float as int
    shr     eax, 1
    mov     edx, 05F3759DFh
    sub     edx, eax
    movd    xmm0, edx                 ; convert back to float

    ; Newton-Raphson iteration
    movaps  xmm2, xmm0                ; y
    mulss   xmm2, xmm2                 ; y*y
    mulss   xmm2, xmm1                 ; x2*y*y
    movss   xmm3, DWORD PTR __threehalfs
    subss   xmm3, xmm2
    mulss   xmm0, xmm3                 ; final y

    ret
_q_rsqrt ENDP

END
