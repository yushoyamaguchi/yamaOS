use crate::drivers::vga::*;
use crate::drivers::kbc::*;
use crate::drivers::serial::*;


macro_rules! print {
    ($($arg:tt)*) => ({
        unsafe {
            VGA_BUFFER.print(format_args!($($arg)*));
        }
    });
}
macro_rules! printk {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}


pub fn cons_init() {
    vga_init();
    serial_init();
}

pub fn cons_getc() -> char {
    getc()
}
