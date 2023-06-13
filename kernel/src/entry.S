.att_syntax prefix
.global entry_kernel
entry_kernel:
    movl $kernel_main, %eax
    jmp *%eax