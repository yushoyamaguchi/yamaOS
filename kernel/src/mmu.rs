// Page directory and page table constants.
pub const NPDENTRIES: usize = 1024; // page directory entries per page directory
pub const NPTENTRIES: usize = 1024; // page table entries per page table
pub const PTXSHIFT: usize = 12; // offset of PTX in a linear address
pub const PDXSHIFT: usize = 22; // offset of PDX in a linear address

// Page table/directory entry flags.
pub const PTE_P: u32 = 0x001; // Present
pub const PTE_W: u32 = 0x002; // Writeable
pub const PTE_U: u32 = 0x004; // User
pub const PTE_PS: u32 = 0x080; // Page Size

//controle register flags
pub const CR0_PE: u32 = 0x00000001;  // Protection Enable
pub const CR0_MP: u32 = 0x00000002;  // Monitor coProcessor
pub const CR0_EM: u32 = 0x00000004;  // Emulation
pub const CR0_TS: u32 = 0x00000008;  // Task Switched
pub const CR0_ET: u32 = 0x00000010;  // Extension Type
pub const CR0_NE: u32 = 0x00000020;  // Numeric Error
pub const CR0_WP: u32 = 0x00010000;  // Write Protect
pub const CR0_AM: u32 = 0x00040000;  // Alignment Mask
pub const CR0_NW: u32 = 0x20000000;  // Not Writethrough
pub const CR0_CD: u32 = 0x40000000;  // Cache Disable
pub const CR0_PG: u32 = 0x80000000;  // Paging


pub const PGSIZE: usize = 4096; // bytes mapped by a page
pub const PGSHIFT: usize = 12; // log2(PGSIZE)

pub const PTSIZE: usize = PGSIZE * NPTENTRIES; // bytes mapped by a page directory entry
pub const PTSHIFT: usize = PTXSHIFT + PGSHIFT; // log2(PTSIZE)

pub fn pdx(la: usize) -> usize {
    (la >> PDXSHIFT) & 0x3FF
}

pub fn ptx(la: usize) -> usize {
    (la >> PTXSHIFT) & 0x3FF
}

pub fn pgnum(la: usize) -> usize {
    la >> PTXSHIFT
}

pub fn pte_addr(pte: u32) -> u32 {
    pte & !0xFFF
}