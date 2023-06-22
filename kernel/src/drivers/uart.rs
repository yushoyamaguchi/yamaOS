use core::fmt;
use crate::x86::*;

pub static mut UART: Uart = Uart{
    serial_exists: false,
};

const COM1: u16 = 0x3F8;

// ref. https://en.wikibooks.org/wiki/Serial_Programming/8250_UART_Programming#UART_Registers
// TODO: validation(e.g. DLAB, read or write only)
enum Register {
    Rbr,
    Thr,
    Dll,
    Dlm,
    Ier,
    Iir,
    Fcr,
    Lcr,
    Mcr,
    Lsr,
}

use core::convert::Into;
impl Into<u16> for Register {
    fn into(self) -> u16 {
        match self {
            Self::Rbr => 0,
            Self::Thr => 0,
            Self::Dll => 0,
            Self::Dlm => 1,
            Self::Ier => 1,
            Self::Iir => 2,
            Self::Fcr => 2,
            Self::Lcr => 3,
            Self::Mcr => 4,
            Self::Lsr => 5,
        }
    }
}

impl Register {
    unsafe fn write(self, data: u8) {
        outb(COM1 + self as u16, data)
    }

    unsafe fn read(self) -> u8 {
        inb(COM1 as u16 + self as u16)
    }

    unsafe fn dlab_on(other: Option<u8>) {
        outb(COM1 + Self::Lcr as u16, 0x80 & other.unwrap_or(0xFF))
    }

    unsafe fn dlab_off(other: Option<u8>) {
        outb(COM1 + Self::Lcr as u16, !0x80 & other.unwrap_or(0xFF))
    }
}

pub struct Uart {
    serial_exists: bool,
}

impl Uart {

    // https://wiki.osdev.org/Serial_Ports#Example_Code
    pub fn init(&mut self){
        unsafe{
            Register::Fcr.write(0);

            Register::dlab_on(None);
            Register::Dll.write((115200 / 9600) as u8);
            Register::Dlm.write(0);

            Register::dlab_off(Some(0x03));

            Register::Mcr.write(0);
            Register::Ier.write(0x01);

            let serial_exists = Register::Lsr.read() != 0xFF;
            Register::Iir.read();
            Register::Rbr.read();
        }
    }

    pub fn read_byte(&self) -> Option<u8> {
        if !self.serial_exists {
            return None;
        }
        while Self::serial_received() {}
        None
    }

    fn serial_received() -> bool {
        unsafe { Register::Ier.read() & 0x01 == 0 }
    }

    pub fn write_byte(&self, value: u8) {
        while Self::is_transmit_empty() {}
        outb(COM1, value) 
    }

    pub fn putc(&mut self, c: char) {
        self.write_byte(c as u8)
    }

    fn is_transmit_empty() -> bool {
        unsafe { Register::Lsr.read() & 0x20 == 0 }
    }

    pub fn print(&mut self, args: fmt::Arguments) -> () {
        use core::fmt::Write;
        self.write_fmt(args).unwrap();
    }
}


impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
        Ok(())
    }
}

pub fn uart_init(){
    unsafe{
        UART.init();
    }
}
