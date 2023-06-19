// Page directory and page table constants.
pub const NPDENTRIES: usize = 1024; // page directory entries per page directory
pub const NPTENTRIES: usize = 1024; // page table entries per page table
pub const PTXSHIFT: usize = 12; // offset of PTX in a linear address
pub const PDXSHIFT: usize = 22; // offset of PDX in a linear address

// Page table/directory entry flags.
pub const PTE_P: usize = 0x001; // Present
pub const PTE_W: usize = 0x002; // Writeable
pub const PTE_U: usize = 0x004; // User
pub const PTE_PS: usize = 0x080; // Page Size

