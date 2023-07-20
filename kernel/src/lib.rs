#![no_std]

mod drivers;
#[macro_use]
mod util;
mod x86;
mod mmu;
mod memlayout;
mod monitor;
#[macro_use]
mod console;
mod pmap;

use drivers::vga::*;
use drivers::uart::*;
use util::types::*;
use core::panic::PanicInfo;
use core::arch::global_asm;
use mmu::*;
use memlayout::*;
use console::*;
use monitor::*;
use pmap::*;

macro_rules! assigned_array {
    ($def:expr; $len:expr; $([$idx:expr] = $val:expr),*) => {{
        let mut tmp = [$def; $len];
        $(tmp[$idx] = $val;)*
        tmp
    }};
}

#[panic_handler]
#[no_mangle]
pub extern "C" fn panic(_info: &PanicInfo) -> ! {
    printk!("{}", _info);
    loop {}
}

#[used]
#[no_mangle]
#[link_section = ".rodata.entrypgdir"]
pub static entrypgdir: [PageDirEntry; NPDENTRIES] = assigned_array![
    0; NPDENTRIES;
    // Map VA's [0, 4MB) to PA's [0, 4MB)
    [0] = 0x000 | PTE_P | PTE_W | PTE_PS,
    // Map VA's [KERNBASE, KERNBASE+4MB) to PA's [0, 4MB)
    [(KERNBASE as usize) >> PDXSHIFT] = 0x000 | 0x001 | 0x002 | 0x080
    // 0x80 means the size of the page is 4MiB
];

global_asm!(include_str!("entry.S"));

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    cons_init();
    let test_str="This is yamaOS";
    printk!("Hello World!");
    printk!("{}", test_str);
    assert!(1 == 1);
    mem_init();
    while 1==1  {
        monitor();
    }
    loop {}
}
