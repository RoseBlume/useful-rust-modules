    .text
    .global q_rsqrt
    .align 2
q_rsqrt:
    // Windows AArch64: float arg in v0.s[0] (s0). Return in v0.s[0] (s0).

    // x2 = number * 0.5f
    fmov    s1, s0            // s1 = number
    ldr     s2, =__half       // s2 = 0.5
    fmul    s1, s1, s2        // s1 = x2

    // i = *(int*)&number  (bitcast float -> uint32)
    fmov    w3, s0            // w3 = bit-pattern of input float

    // i = 0x5f3759df - (i >> 1)
    lsr     w4, w3, #1        // w4 = i >> 1

    // Load 0x5F3759DF into w5 (use movz/movk to assemble 32-bit imm)
    movz    w5, #0x5f37, lsl #16
    movk    w5, #0x59df

    sub     w5, w5, w4        // w5 = magic - (i >> 1)

    // y = *(float*)&i  (bitcast uint32 -> float)
    fmov    s0, w5            // s0 = initial approximation y

    // Newton iteration: y = y * (1.5 - x2 * y * y)
    fmov    s3, s0            // s3 = y
    fmul    s3, s3, s3        // s3 = y*y
    fmul    s3, s3, s1        // s3 = x2 * y*y
    ldr     s4, =__threehalfs // s4 = 1.5
    fsub    s4, s4, s3        // s4 = 1.5 - x2*y*y
    fmul    s0, s0, s4        // s0 = y * (1.5 - x2*y*y)

    ret
    .seh_endfunclet
	.seh_endproc

    .data
    .align 2
__half:
    .float 0.5
__threehalfs:
    .float 1.5
