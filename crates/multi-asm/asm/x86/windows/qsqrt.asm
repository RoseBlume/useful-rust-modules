; option casemap:none
.model flat, stdcall
.CODE

; float q_rsqrt(float number)
; x86 stdcall / cdecl: float argument on stack at [esp + 4]
; Return float in XMM0
PUBLIC _q_rsqrt

_q_rsqrt PROC
    ; load float argument from stack into xmm0
    fld DWORD PTR [esp + 4]      ; load onto FPU stack
    sub esp, 4                   ; reserve temporary stack slot
    fstp DWORD PTR [esp]         ; store temporarily on stack
    movss xmm0, DWORD PTR [esp]  ; move into xmm0
    add esp, 4                   ; free temporary stack slot

    ; x2 = number * 0.5f
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
    mulss xmm2, xmm2        ; y^2
    mulss xmm2, xmm1        ; x2*y^2
    movss xmm3, DWORD PTR __threehalfs
    subss xmm3, xmm2
    mulss xmm0, xmm3

    ret 4                  ; clean up stack argument
_q_rsqrt ENDP

.data
__half        REAL4 0.5
__threehalfs  REAL4 1.5

END
