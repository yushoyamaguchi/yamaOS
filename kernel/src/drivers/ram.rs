use crate::x86::*;

// RTC port
pub const IO_RTC: u8 = 0x070;

// Start of NVRAM: offset 14
pub const MC_NVRAM_START: u8 = 0xe;

// 50 bytes of NVRAM
pub const MC_NVRAM_SIZE: u8 = 50;

// NVRAM bytes 7 & 8: base memory size
pub const NVRAM_BASELO: u8 = MC_NVRAM_START + 7; // low byte; RTC off. 0x15
pub const NVRAM_BASEHI: u8 = MC_NVRAM_START + 8; // high byte; RTC off. 0x16

// NVRAM bytes 9 & 10: extended memory size (between 1MB and 16MB)
pub const NVRAM_EXTLO: u8 = MC_NVRAM_START + 9; // low byte; RTC off. 0x17
pub const NVRAM_EXTHI: u8 = MC_NVRAM_START + 10; // high byte; RTC off. 0x18

// NVRAM bytes 38 and 39: extended memory size (between 16MB and 4G)
pub const NVRAM_EXT16LO: u8 = MC_NVRAM_START + 38; // low byte; RTC off. 0x34
pub const NVRAM_EXT16HI: u8 = MC_NVRAM_START + 39; // high byte; RTC off. 0x35


pub fn mc146818_read(reg: u32) -> u32 {
    outb(IO_RTC as u16, reg as u8);
    inb(IO_RTC as u16 +1) as u32
}

pub fn mc146818_write(reg: u32, datum: u8) {
    outb(IO_RTC as u16, reg as u8);
    outb(IO_RTC as u16+1, datum);
}
