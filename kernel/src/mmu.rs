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

pub fn pte_addr(pte: usize) -> usize {
    pte & !0xFFF
}