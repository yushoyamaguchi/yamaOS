use core::arch::asm;

pub fn inb(port: u16) -> u8 {
    let mut data: u8;
    unsafe {
        asm!("inb %dx, %al",
            out("al") data,
            in("dx") port,
            options(att_syntax))
    }
    data
}

pub fn outb(port: u16, value: u8) {
    unsafe {
        asm!("outb %al, %dx",
            in("al") value,
            in("dx") port,
            options(att_syntax))
    }
}