use core::ptr::null_mut;

use crate::drivers::ram::*;
use crate::mmu::*;
use crate::memlayout::*;
use crate::drivers::vga::*;
use crate::drivers::uart::*;
use crate::util::mem::*;
use crate::util::types::*;

pub static mut NPAGES: usize = 0;
pub static mut NPAGES_BASEMEM: usize = 0;
pub static mut kern_pgdir: *mut PdeT = null_mut();

fn nvram_read(r: u32) -> u32 {
    return mc146818_read(r) | (mc146818_read(r + 1) << 8);
}

fn i386_detect_memory() {
    let mut basemem: usize;
    let mut extmem: usize;
    let mut ext16mem: usize;
    let mut totalmem: usize;

    // Use CMOS calls to measure available base & extended memory.
    // (CMOS calls return results in kilobytes.)
    basemem = nvram_read(NVRAM_BASELO as u32) as usize;
    extmem = nvram_read(NVRAM_EXTLO as u32) as usize;
    ext16mem = nvram_read(NVRAM_EXT16LO as u32) as usize * 64;

    // Calculate the number of physical pages available in both base
    // and extended memory.
    if ext16mem > 0 {
        totalmem = 16 * 1024 + ext16mem;
    } else if extmem > 0 {
        totalmem = 1 * 1024 + extmem;
    } else {
        totalmem = basemem;
    }

    unsafe{
        NPAGES = totalmem / (PGSIZE / 1024);
        NPAGES_BASEMEM = basemem / (PGSIZE / 1024);
    }

    printk!("Physical memory: {}K available, base = {}K, extended = {}K",
        totalmem, basemem, totalmem - basemem);
}


#[no_mangle]
static mut NEXT_FREE: *mut u32 = null_mut();

extern "C" {
    static end: *mut u32;
}

unsafe fn nextfree_init() {
    NEXT_FREE = roundup((&end as *const _ as usize) as u32, PGSIZE as u32) as *mut u32;
}

#[no_mangle]
unsafe fn boot_alloc(n: usize) -> *mut u32 {
    if NEXT_FREE.is_null() {
        nextfree_init();
    }

    let result = NEXT_FREE;
    NEXT_FREE = roundup((result as usize + n) as u32, PGSIZE as u32) as *mut u32;

    result
}

pub fn mem_init(){
    i386_detect_memory();
    unsafe {
        kern_pgdir=boot_alloc( PGSIZE as usize );
        memset(kern_pgdir as *mut u8, 0, PGSIZE);
    }
    
}
