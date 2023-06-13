.att_syntax prefix
.global entry_kernel
entry_kernel:
    movl $0x2f6f2f4c, 0xb8000
    movl $0x2f642f61, 0xb8004
    movl $0x2f642f65, 0xb8008
    movl $0x2f212f21, 0xb800c
    movl $kernel_main, %eax
    jmp *%eax