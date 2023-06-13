#![feature(panic_handler)]
#![feature(global_asm)]
#![no_std]

mod drivers;
#[macro_use]
mod printk;
mod x86;
mod kbc;

use drivers::vga::VGA_BUFFER;
use core::panic::PanicInfo;
use core::arch::global_asm;

#[panic_handler]
#[no_mangle]
pub extern "C" fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

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
