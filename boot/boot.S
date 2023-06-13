.section .text

.extern entry_kernel


boot:
    # like xv6
    movl $entry_kernel, %eax
    jmp *%eax
    hlt
