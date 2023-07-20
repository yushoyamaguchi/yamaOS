
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
		*(.stab);
	}

	.stabstr : {
		*(.stabstr);
	}

    /* Adjust the address for the data segment to the next page */
    . = ALIGN(0x1000);

    /* The data segment */
    .data : {
		*(.data)
	}

    .bss : {
		*(.bss)
	}
  PROVIDE(end = .);

    /DISCARD/ : {
		*(.eh_frame .note.GNU-stack)
	}
}
