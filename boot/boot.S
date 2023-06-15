.section .text

.extern entry_kernel
.global start, _start
_start = (start-0x80000000)

start:
_start:
    # like xv6
    movl $(entry_kernel-0x80000000), %eax
    jmp *%eax
    hlt
