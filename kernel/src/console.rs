use crate::drivers::vga::*;
use crate::drivers::kbc::*;


pub fn cons_init() {
    vga_init();
}

pub fn cons_getc() -> char {
    getc()
}