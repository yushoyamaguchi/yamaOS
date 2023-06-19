#![no_std]

mod drivers;
#[macro_use]
mod printk;
mod x86;
mod kbc;
mod mmu;
mod memlayout;

use drivers::vga::VGA_BUFFER;
use core::panic::PanicInfo;
use core::arch::global_asm;
use mmu::NPDENTRIES;
use mmu::PDXSHIFT;
use memlayout::PageDirEntry;

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
    loop {}
}

#[used]
#[no_mangle]
#[link_section = ".rodata.entrypgdir"]
pub static entrypgdir: [PageDirEntry; NPDENTRIES] = assigned_array![
    0; NPDENTRIES;
    // Map VA's [0, 4MB) to PA's [0, 4MB)
    [0] = 0x000 | 0x001 | 0x002 | 0x080,
    // Map VA's [KERNBASE, KERNBASE+4MB) to PA's [0, 4MB)
    [0x80000000 >> PDXSHIFT] = 0x000 | 0x001 | 0x002 | 0x080
    // 0x80 means the size of the page is 4MiB
];

global_asm!(include_str!("entry.S"));

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    printk!("Hello {}", "World");
    printk!("{} + {} = {}", 1, 2, 3);
    while 1==1  {
        let c = kbc::getc();
        printk!("{}", c);
    }
    loop {}
}
