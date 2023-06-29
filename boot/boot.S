.equ KERNBASE, 0x80000000
.section .text

.extern entry_kernel
.global start, _start
_start = (start-KERNBASE)

start:
_start:
    # like xv6
    movl $entry_kernel, %eax
    subl $KERNBASE, %eax
    jmp *%eax
    hlt
