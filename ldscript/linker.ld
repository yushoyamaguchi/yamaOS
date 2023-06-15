
SECTIONS {
    . = 0x80100000;

    .text : AT(0x100000) {
        KEEP(*(.multiboot_header))
        *(.text)
    }

    .rodata : ALIGN(4K) {
        *(.rodata)
    }
}
