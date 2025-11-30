    .globl is_asm_even
    .type  is_asm_even, @function

is_asm_even:
    movl 4(%esp), %eax   # load argument
    andl $1, %eax
    xorl $1, %eax
    ret
