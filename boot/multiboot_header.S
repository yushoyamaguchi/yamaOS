.section .multiboot_header

header_start:
    .int 0xe85250d6                   # magic number
    .int 0                            # architecture (i386)
    .int header_end - header_start    # header size
    .int 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))    # checksum

    # required end tag
    .short 0    # type
    .short 0    # flags
    .int 8    # size
header_end: