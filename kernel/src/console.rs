use crate::drivers::kbc;
use crate::drivers::vga::*;
use crate::drivers::kbc::*;
use crate::drivers::uart::*;


macro_rules! print {
    ($($arg:tt)*) => ({
        unsafe {
            VGA_BUFFER.print(format_args!($($arg)*));
            UART.print(format_args!($($arg)*));
        }
    });
}
macro_rules! printk {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub const CONSBUFSIZE: usize = 512;

pub struct ConsStruct{
    pub buf: [u8; CONSBUFSIZE],
    pub rpos: u32,
    pub wpos: u32,
}

pub static mut CONS: ConsStruct= ConsStruct{
    buf: ['0' as u8; CONSBUFSIZE],
    rpos: 0,
    wpos: 0,
};


pub fn cons_init() {
    vga_init();
    uart_init();
    if unsafe { ! UART.serial_exists } {
        printk!("serial not exists");
    }
}

pub fn cons_putc(c: char) {
    unsafe{
        VGA_BUFFER.putc(c);
        UART.putc(c);
    }
}

pub fn getc() -> char {
    let c;
    loop {
        match cons_getc() {
            Some(cc) => {
                c = cc;
                break;
            }
            None => {}
        }
    }
    return c;
}

pub fn cons_getc() -> Option<char> {
    serial_intr();
    kbc_intr();
    unsafe{
        if CONS.rpos != CONS.wpos {
            let c=CONS.buf[CONS.rpos as usize];
            CONS.rpos = (CONS.rpos+1)%CONSBUFSIZE as u32;
            return Some(c as char);
        }
    }
    return None;
}

pub fn serial_intr(){
    unsafe{
        if UART.serial_exists {
            UART.handle_interrupt();
        }
    }
}
