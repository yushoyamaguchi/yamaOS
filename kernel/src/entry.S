.att_syntax prefix
.equ KERNBASE, 0xF0000000
.equ CR4_PSE,  0x00000010  # Page size extension
.equ CR0_WP,     0x00010000  # Write Protect
.equ CR0_PG,     0x80000000  # Paging
.equ CR0_PE,     0x00000001  # Protection Enable
.equ BSTACKSIZE, 4096*9    # Size of per-process kernel stack
.equ PGSHIFT,    12      # log2(PGSIZE)

.global entry_kernel
entry_kernel:
    # Turn on page size extension for 4Mbyte pages
    movl    %cr4, %eax
    orl     $(CR4_PSE), %eax
    movl    %eax, %cr4
    # Set page directory
    movl    $(entrypgdir - KERNBASE), %eax
    movl    %eax, %cr3
    # Turn on paging.
    movl    %cr0, %eax
    orl     $(CR0_PE|CR0_PG|CR0_WP), %eax
    movl    %eax, %cr0
    # Set up the stack pointer.
    movl    $(bootstacktop), %esp
    # VGA test message
    movl    $0x2f6f2f4c, 0xb8000
    movl    $0x2f642f61, 0xb8004
    movl    $0x2f642f65, 0xb8008
    movl    $0x2f212f21, 0xb800c
    movl    $kernel_main, %eax
    jmp     *%eax


.data
###################################################################
# boot stack
###################################################################
	.p2align	PGSHIFT		# force page alignment
	.globl		bootstack
bootstack:
	.space		BSTACKSIZE
	.globl		bootstacktop   
bootstacktop:   