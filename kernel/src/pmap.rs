use core::mem::size_of;
use core::ptr::null_mut;
use core::cmp::min;

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
        if page < PAGES  {
            panic!("page2pa called with invalid page {:08x} , {:08x}", page as PhysaddrT, PAGES as PhysaddrT);
        }
        (page as PhysaddrT - PAGES as PhysaddrT) << PGSHIFT
    }
}

fn page2kva(page: *mut PageInfo) -> *mut u32 {
    unsafe{
        kaddr(page2pa(page))
    }
}

fn pa2page(pa: PhysaddrT) -> *mut PageInfo {
    unsafe{
        if pgnum(pa as usize) >= NPAGES {
            panic!("pa2page called with invalid pa {:08x}", pa);
        }
        PAGES.offset(pgnum(pa as usize) as isize)
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
        panic!("PADDR called with invalid kva {:08x}", kva);
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

extern  {
    static end: *mut u32;
    static bootstack:  *mut u32;
    static bootstacktop: *mut u32;
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
    relocate_page_free_list(true);
    check_page_free_list();
    //check_page_alloc();

    unsafe{
        boot_map_region(KERN_PGDIR, UPAGES as u32, roundup((NPAGES*size_of::<PageInfo>()) as u32, PGSIZE as u32) as usize, paddr(PAGES as u32), PTE_U );
        let stack_paddr=paddr(rounddown(bootstacktop as u32-(KSTKSIZE as u32),PGSIZE as u32));
        //printk!("stack_paddr: {:08x}", stack_paddr);
        boot_map_region(KERN_PGDIR as *mut u32, (KSTACKTOP-KSTKSIZE) as u32, KSTKSIZE , stack_paddr, PTE_W );
        boot_map_region(KERN_PGDIR as *mut u32, KERNBASE as u32, (!KERNBASE) +1 , 0, PTE_W );
    }
}


fn page_init(){
    let mut addr:PhysaddrT;
    unsafe{
        PAGE_FREE_LIST = null_mut();
        let pages_slice = core::slice::from_raw_parts_mut(PAGES, NPAGES);
        pages_slice[0].pp_ref = 1;
        pages_slice[0].pp_link = null_mut();
        for i in 1 .. NPAGES_BASEMEM{
            addr=page2pa(PAGES.offset(i as isize));
            if addr >= IOPHYSMEM as PhysaddrT && addr < EXTPHYSMEM as PhysaddrT{
                pages_slice[i].pp_ref = 1;
                pages_slice[i].pp_link = null_mut();
                continue;
            }
            pages_slice[i].pp_ref = 0;
            pages_slice[i].pp_link = PAGE_FREE_LIST;
            PAGE_FREE_LIST = PAGES.offset(i as isize);
        }

        let first_free_page_pa=paddr(boot_alloc(0) as u32);
        assert!(first_free_page_pa % PGSIZE as u32 == 0);
        assert!(first_free_page_pa >= IOPHYSMEM as u32);
        let first_free_page_index=pa2page(first_free_page_pa).offset_from(PAGES);
        let hole_start_page_index=min(NPAGES_BASEMEM, IOPHYSMEM  / PGSIZE );
        for i in hole_start_page_index .. first_free_page_index as usize{ //This area cannot be allocated
            pages_slice[i].pp_ref = 1;
            pages_slice[i].pp_link = null_mut();
        }

        for i in first_free_page_index as usize .. NPAGES{
            addr=page2pa(PAGES.offset(i as isize));
            if addr >= IOPHYSMEM as PhysaddrT && addr < EXTPHYSMEM as PhysaddrT{ //Maybe this area don't exist in free_page
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
            printk!("page_alloc: no memory");
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
        if PAGE_FREE_LIST.is_null(){
            printk!("page_alloc2: no memory");
        }
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
        if PAGE_FREE_LIST.is_null(){
            panic!("page_free: PAGE_FREE_LIST is null");
        }
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

pub fn boot_map_region(pgdir: *mut u32, va: u32, size: usize, pa: u32, perm: u32) {

    assert!(va % PGSIZE as u32 == 0);
    assert!(pa % PGSIZE as u32 == 0);
    assert!(size % PGSIZE == 0);

    let mut i = 0;
    while i < size {
        let table_entry = pgdir_walk(pgdir, (va + i as u32) , true);
        if table_entry==null_mut() {
            panic!("boot_map_region: pgdir_walk returned null {:8x} size={:8x}, va={:8x}",i,size,va);
        }
        unsafe{
            *table_entry = pa + i as u32 | perm as u32 | PTE_P;
        }
        i += PGSIZE;
    }
}

fn pgdir_walk(pgdir: *mut u32, va: u32, create:bool) -> *mut u32 {
    let pdx = pdx(va as usize);
    let pgdir_slice = unsafe { core::slice::from_raw_parts_mut(pgdir, NPDENTRIES) };
    if pgdir_slice[pdx]&PTE_P != 0{
        unsafe{
            let pgtable = kaddr(pte_addr(pgdir_slice[pdx] as usize )as u32) as *mut u32;
            return pgtable.offset(ptx(va as usize) as isize);
        }
    } else if create {
        unsafe{
            let mut new_pgtable_pp=page_alloc(ALLOC_ZERO) ;
            if new_pgtable_pp == null_mut(){
                printk!("pgdir_walk: page_alloc failed");
                return null_mut();
            }
            (*new_pgtable_pp).pp_ref+=1;
            let new_pgtable_pa:PhysaddrT=page2pa(new_pgtable_pp) as PhysaddrT;
            pgdir_slice[pdx]=new_pgtable_pa as u32 | PTE_P | PTE_W | PTE_U;
            let new_pgtable_va=kaddr(new_pgtable_pa as u32) as *mut u32;
            let ret=new_pgtable_va.offset(ptx(va as usize) as isize);
            /*if ret.is_null(){
                printk!("pgdir_walk null: va={:8x} ",va);
            }*/
            return ret;
        }
    } else {
        null_mut()
    }
}



fn relocate_page_free_list(only_lowmem: bool){
    let mut pp: *mut PageInfo;
    let pdx_limit=if only_lowmem {1} else {NPDENTRIES};
    unsafe{
        if PAGE_FREE_LIST.is_null(){
            panic!("check_page_free_list: PAGE_FREE_LIST is a null pointer");
        }
        if only_lowmem {
            let (mut pp1, mut pp2): (*mut PageInfo, *mut PageInfo) = (null_mut(), null_mut());
            let mut tp: [*mut *mut PageInfo; 2] = [&mut pp1, &mut pp2 ];
            pp=PAGE_FREE_LIST;
            while pp != null_mut(){
                let page_type= if pdx(page2pa(pp) as usize) >= pdx_limit { 1 } else { 0 };
                *tp[page_type] = pp;
                if !(*pp).pp_link.is_null() {
                    tp[page_type] = &mut (*pp).pp_link;
                }
                pp=(*pp).pp_link;
            }
            *tp[1] = 0 as *mut PageInfo;
            *tp[0] = pp2;
            PAGE_FREE_LIST = pp1;
            if PAGE_FREE_LIST.is_null(){
                panic!("check_page_free_list: PAGE_FREE_LIST is a null pointer");
            }
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
    let mut c:*mut u8;
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
        PAGE_FREE_LIST=fl;

        memset(page2kva(pp1) as *mut u8, 1, PGSIZE);
        page_free(pp1);
        pp=page_alloc(ALLOC_ZERO);
        assert!(pp != null_mut());
        /*c=page2kva(pp) as *mut u8;
        for i in 0..PGSIZE{
            c=c.offset(i as isize);
            assert!(*c == 0);
        }*/

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
        /*while pp != null_mut(){
            if pdx(page2pa(pp) as usize )<pdx_limit{
                memset(page2kva(pp) as *mut u8,0x97,128);
            }
            pp=(*pp).pp_link;
        }*/
        pp=PAGE_FREE_LIST;
        while pp != null_mut(){
            assert!(pp>=PAGES);
            assert!(pp<PAGES.wrapping_add(NPAGES));
            pp=(*pp).pp_link;
        }
    }

}