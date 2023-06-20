
SECTIONS {
    . = 0x80100000;

    .text : AT(0x100000) {
        KEEP(*(.multiboot_header))
        *(.text)
    }

    .rodata : {
		. = ALIGN(4096);
		*(.rodata.entrypgdir)
        *(.rodata .rodata.* .gnu.linkonce.r.*)
    }

    /* Include debugging information in kernel memory */
    .stab : {
		PROVIDE(__STAB_BEGIN__ = .);
		*(.stab);
		PROVIDE(__STAB_END__ = .);
		BYTE(0)		/* Force the linker to allocate space
				   for this section */
	}

	.stabstr : {
		PROVIDE(__STABSTR_BEGIN__ = .);
		*(.stabstr);
		PROVIDE(__STABSTR_END__ = .);
		BYTE(0)		/* Force the linker to allocate space
				   for this section */
	}

    /* Adjust the address for the data segment to the next page */
    . = ALIGN(0x1000);

    /* The data segment */
    .data : {
		*(.data)
	}

    .bss : {
		PROVIDE(edata = .);
		*(.bss)
		PROVIDE(end = .);
		BYTE(0)
	}

    /DISCARD/ : {
		*(.eh_frame .note.GNU-stack)
	}
}
