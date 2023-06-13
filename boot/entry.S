.set KERNBASE,   0x80000000  # First kernel virtual address

.section .text

.extern kernel_main

.globl		_start
_start = (entry-KERNBASE)

.globl entry

entry:
    movl $0x2f6f2f4c, 0xb8000
    movl $0x2f642f61, 0xb8004
    movl $0x2f642f65, 0xb8008
    movl $0x2f212f21, 0xb800c
    # like xv6
    movl $kernel_main, %eax
    jmp *%eax
    hlt
