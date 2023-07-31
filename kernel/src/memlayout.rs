use crate::mmu::*;

pub type PageTableEntry = u32;
pub type PageDirEntry = u32;
pub const KERNBASE: usize = 0xF0000000;


pub type PteT = u32;
pub type PdeT = u32;


pub const IOPHYSMEM: usize = 0x0A0000;
pub const EXTPHYSMEM: usize = 0x100000;

pub const KSTACKTOP: usize = KERNBASE;
pub const KSTKSIZE: usize = 8 * PGSIZE;  // size of a kernel stack
pub const KSTKGAP: usize = 8 * PGSIZE;   // size of a kernel stack guard

pub const MMIOLIM: usize = KSTACKTOP - PTSIZE;
pub const MMIOBASE: usize = MMIOLIM - PTSIZE;

pub const ULIM: usize = MMIOBASE;

// User read-only mappings! Anything below here til UTOP are readonly to user.
// They are global pages mapped in at env allocation time.

// User read-only virtual page table (see 'uvpt' below)
pub const UVPT: usize = ULIM - PTSIZE;
// Read-only copies of the Page structures
pub const UPAGES: usize = UVPT - PTSIZE;
// Read-only copies of the global env structures
pub const UENVS: usize = UPAGES - PTSIZE;



pub struct PageInfo {
    pub pp_link: *mut PageInfo,
    pub pp_ref: u16,
}