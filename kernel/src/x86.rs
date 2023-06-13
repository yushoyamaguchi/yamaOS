use core::arch::asm;

pub fn read_io_port(port: u16) -> u8 {
    let mut data: u8;
    unsafe {
        asm!("inb %dx, %al",
            out("al") data,
            in("dx") port,
            options(att_syntax))
    }
    data
}