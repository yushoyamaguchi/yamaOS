use core::arch::asm;

pub unsafe fn inb(port: u16) -> u8 {
    let ret: u8;
    asm!("inb %dx, %al", out("al") ret, in("dx") port, options(nostack, preserves_flags));
    ret
}


