.att_syntax prefix
.equ KERNBASE, 0x80000000
.equ CR4_PSE,  0x00000010  # Page size extension

.global entry_kernel
entry_kernel:
    # Turn on page size extension for 4Mbyte pages
    movl    %cr4, %eax
    orl     $(CR4_PSE), %eax
    movl    %eax, %cr4
    # Set page directory
    movl    $(entrypgdir - KERNBASE), %eax
    movl    %eax, %cr3
    movl $0x2f6f2f4c, 0xb8000
    movl $0x2f642f61, 0xb8004
    movl $0x2f642f65, 0xb8008
    movl $0x2f212f21, 0xb800c
    hlt
    movl $kernel_main, %eax
    jmp *%eax