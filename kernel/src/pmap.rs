use core::ptr::null_mut;

use crate::drivers::ram::*;
use crate::mmu::*;
use crate::memlayout::*;
use crate::drivers::vga::*;
use crate::drivers::uart::*;
use crate::util::assert;
use crate::util::mem::*;
use crate::util::types::*;

const ALLOC_ZERO: u32 = 0x1;

pub static mut NPAGES: usize = 0;
pub static mut NPAGES_BASEMEM: usize = 0;
pub static mut KERN_PGDIR: *mut PdeT = null_mut();

static mut PAGE_FREE_LIST: *mut PageInfo = null_mut();
static mut PAGES: *mut PageInfo = null_mut();

fn page2pa(page: *mut PageInfo) -> PhysaddrT {
    unsafe{
        (page as PhysaddrT - PAGES as PhysaddrT) << PGSHIFT
    }
}

fn page2kva(page: *mut PageInfo) -> *mut u32 {
    unsafe{
        kaddr(page2pa(page))
    }
}

unsafe fn kaddr(pa: PhysaddrT) -> *mut u32 {
    if pgnum(pa as usize) >= NPAGES  {
        panic!("KADDR called with invalid pa {:08x}", pa);
    }
    (pa + KERNBASE as u32) as *mut u32
}

pub fn paddr( kva: u32) -> u32 {
    if kva < KERNBASE as u32 {
        panic!(concat!("assertion failed: "));
    }
    kva - KERNBASE as u32
}

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
        KERN_PGDIR=boot_alloc( PGSIZE as usize );
        memset(KERN_PGDIR as *mut u8, 0, PGSIZE);
        let kern_pgdir_slice = core::slice::from_raw_parts_mut(KERN_PGDIR, PGSIZE); // replace SIZE with the actual size
        kern_pgdir_slice[pdx(UVPT)] = paddr(KERN_PGDIR as u32) | PTE_U | PTE_P;
        PAGES=boot_alloc( NPAGES * core::mem::size_of::<PageInfo>() ) as *mut PageInfo;
        memset(PAGES as *mut u8, 0, NPAGES * core::mem::size_of::<PageInfo>());
    }
    page_init();
    check_page_alloc();
}


fn page_init(){
    let mut addr:PhysaddrT;
    unsafe{
        PAGE_FREE_LIST = null_mut();
        let pages_slice = core::slice::from_raw_parts_mut(PAGES, NPAGES);
        pages_slice[0].pp_ref = 1;
        pages_slice[0].pp_link = null_mut();
        for i in 1 .. NPAGES_BASEMEM{
            let addr=page2pa(PAGES.offset(i as isize));
            if addr >= IOPHYSMEM as PhysaddrT && addr < EXTPHYSMEM as PhysaddrT{
                pages_slice[i].pp_ref = 1;
                pages_slice[i].pp_link = null_mut();
                continue;
            }
            pages_slice[i].pp_ref = 0;
            pages_slice[i].pp_link = PAGE_FREE_LIST;
            PAGE_FREE_LIST = PAGES.offset(i as isize);
        }
    }
}

fn page_alloc(alloc_flags:u32)->*mut PageInfo{
    unsafe{
        if PAGE_FREE_LIST.is_null(){
            return null_mut();
        }
        let mut ret:*mut PageInfo=PAGE_FREE_LIST;
        let mut addr:*mut u32=page2kva(ret);
        if alloc_flags & ALLOC_ZERO != 0{
            memset(addr as *mut u8, 0, PGSIZE);
        }
        PAGE_FREE_LIST=(*ret).pp_link;
        (*ret).pp_link=null_mut();
        (*ret).pp_ref=0;
        return ret;
    }
}

fn page_free(pp: *mut PageInfo){
    unsafe{
        if (*pp).pp_ref != 0{
            panic!("page_free: pp->pp_ref isn't zero");
        }
        if (*pp).pp_link != null_mut(){
            panic!("page_free: pp->pp_link is not null");
        }
        (*pp).pp_link=PAGE_FREE_LIST;
        PAGE_FREE_LIST=pp;
    }
}

fn page_decref(pp: *mut PageInfo){
    unsafe{
        if (*pp).pp_ref == 0{
            panic!("page_decref: pp->pp_ref is zero");
        }
        (*pp).pp_ref-=1;
        if (*pp).pp_ref == 0{
            page_free(pp);
        }
    }
}

fn check_page_alloc(){
    let mut pp: *mut PageInfo;
    let mut pp1: *mut PageInfo;
    let mut pp2: *mut PageInfo;
    let mut pp3: *mut PageInfo;
    let mut fl: *mut PageInfo;
    let mut nfree:u32=0;
    unsafe{
        if PAGES.is_null(){
            panic!("check_page_alloc: PAGES is a null pointer");
        }
        pp=PAGE_FREE_LIST;
        while pp != null_mut(){
            nfree+=1;
            pp=(*pp).pp_link;
        }
        pp1=0 as *mut PageInfo;
        pp2=0 as *mut PageInfo;
        pp3=0 as *mut PageInfo;
        pp1=page_alloc(0);
        assert!(pp1 != null_mut());
        pp2=page_alloc(0);
        assert!(pp2 != null_mut());
        pp3=page_alloc(0);
        assert!(pp3 != null_mut());
        assert!(pp2 != pp1);
        assert!(pp3 != pp1);
        assert!(pp3 != pp2);
        assert!(page2pa(pp1)<(NPAGES as u32*PGSIZE as u32) );
        assert!(page2pa(pp2)<(NPAGES as u32*PGSIZE as u32) );
        assert!(page2pa(pp3)<(NPAGES as u32*PGSIZE as u32) );
        fl=PAGE_FREE_LIST;
        PAGE_FREE_LIST=0 as *mut PageInfo;
        assert!(page_alloc(0) == null_mut());
        page_free(pp1);
        page_free(pp2);
        page_free(pp3);

    }
}

fn check_page_free_list(){
    let mut pp: *mut PageInfo;
    let pdx_limit:usize= NPDENTRIES;
    let nfree_basemem:u64;
    let nfree_extmem:u64;
    let first_free_page:*mut u32;

    unsafe{
        if PAGE_FREE_LIST.is_null(){
            panic!("check_page_free_list: page free list is empty");
        }
        pp=PAGE_FREE_LIST;
        while pp != null_mut(){
            if pdx(page2pa(pp) as usize )<pdx_limit{
                memset(page2kva(pp) as *mut u8,0x97,128);
            }
            pp=(*pp).pp_link;
        }
        first_free_page=boot_alloc(0);
        pp=PAGE_FREE_LIST;
        while pp != null_mut(){
            assert!(pp>=PAGES);
            assert!(pp<PAGES.wrapping_add(NPAGES));
        }
    }

}