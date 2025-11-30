; iseven.asm - ARM64 Windows (armasm64.exe syntax)

; int is_asm_even(int x)
; Windows ARM64: x0 = int arg, return value in x0

is_asm_even:
    and     w0, w0, #1       ; w0 = x & 1
    eor     w0, w0, #1       ; w0 = (x & 1) ^ 1
    ret
