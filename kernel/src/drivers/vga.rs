use core::fmt;
use crate::memlayout::KERNBASE;
use crate::x86::*;
use crate::console::*;

const VGA_BUFFER_WIDTH: usize = 80;
const VGA_BUFFER_HIGHT: usize = 25;

const VGA_BUFFER_ADDR: usize=0xb8000 +KERNBASE;

pub static mut VGA_BUFFER: VGABuffer = VGABuffer {
    buffer: [[
        VGACharacter{
            character: b' ',
            color: (ColorCode::DarkGray as u8) << 4 | ColorCode::DarkGray as u8};
        VGA_BUFFER_WIDTH]; VGA_BUFFER_HIGHT],
    x_pos: 0,
    y_pos: 0,
};

enum ColorCode {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xa,
    LightCyan = 0xb,
    LightRed = 0xc,
    LightMangenta = 0xd,
    Yellow = 0xe,
    White = 0xf,
}

#[derive(Clone, Copy)]
struct VGACharacter {
    character: u8,
    color: u8,
}

impl VGACharacter {
    // VGA Character Attribute
    // |   7   |   6   |   5   |   4   |  3  |  2  |  1  |  0  |
    // | blink |    Background color   |    Foreground color   |
    fn new(character: u8, foreground: ColorCode, background: ColorCode) -> Self {
        VGACharacter {
            character: character,
            color: (background as u8) << 4 | foreground as u8
        }
    }
}

pub struct VGABuffer {
    buffer: [[VGACharacter; VGA_BUFFER_WIDTH]; VGA_BUFFER_HIGHT],
    x_pos: usize,
    y_pos: usize,
}

impl VGABuffer{
    pub fn clear_screen(&mut self) {
        // Turn off the blinking light in the upper left corner.
        unsafe {
            let port = 0x3D4;
            outb(port, 0x0A);
            let value = inb(port + 1);
            outb(port + 1, value | 0x20);
        }
    }
    pub fn new_line(&mut self) {
        if self.y_pos<VGA_BUFFER_HIGHT-1 {
            self.x_pos = 0;
            self.y_pos += 1;
        }
        else {
            self.scroll_one_line();
        }
    }

    pub fn scroll_one_line(&mut self) {
        for y in 0..VGA_BUFFER_HIGHT-1 {
            for x in 0..VGA_BUFFER_WIDTH {
                self.buffer[y][x] = self.buffer[y+1][x];
            }
        }
        self.clear_line(VGA_BUFFER_HIGHT-1);
        self.x_pos = 0;
        self.y_pos = VGA_BUFFER_HIGHT-1;
    }

    pub fn clear_line(&mut self, line: usize) {
        let blank = VGACharacter::new(b' ', ColorCode::DarkGray, ColorCode::DarkGray);
        for x in 0..VGA_BUFFER_WIDTH {
            self.buffer[line][x] = blank;
        }
    }

    pub fn delete_last_char(&mut self) {
        if self.x_pos>0 {
            self.x_pos -= 1;
            self.buffer[self.y_pos][self.x_pos] = VGACharacter::new(b' ', ColorCode::White, ColorCode::DarkGray);
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            b'\x08' | b'\x7f'  => {
                self.delete_last_char();
            }
            byte => {
                if self.x_pos<VGA_BUFFER_WIDTH {
                    self.buffer[self.y_pos][self.x_pos] = VGACharacter::new(byte, ColorCode::White, ColorCode::DarkGray);
                    self.x_pos += 1;
                }
                else {
                    self.new_line();
                    self.buffer[self.y_pos][self.x_pos] = VGACharacter::new(byte, ColorCode::White, ColorCode::DarkGray);
                    self.x_pos += 1;
                }
            }
        }
    }

    pub fn putc(&mut self, c: char) {
        self.write_byte(c as u8);
        self.flush();
    }

    pub fn write_str(&mut self, s: &str) -> () {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }

    pub fn clear(&mut self) -> () {
        for y in 0..VGA_BUFFER_HIGHT {
            for x in 0..VGA_BUFFER_WIDTH {
                self.buffer[y][x] = VGACharacter::new(b' ', ColorCode::DarkGray, ColorCode::DarkGray);
            }
        }
        self.x_pos = 0;
        self.y_pos = 0;
    }

    pub fn print(&mut self, args: fmt::Arguments) -> () {
        use core::fmt::Write;
        self.write_fmt(args).unwrap();
        self.flush();
    }

    pub fn flush(&mut self) -> () { // Output the contents of the structure to the screen through VGA
        let vga_text_buffer = VGA_BUFFER_ADDR as *mut u8;
        for y in 0..VGA_BUFFER_HIGHT {
            for x in 0..VGA_BUFFER_WIDTH {
                unsafe {
                    *vga_text_buffer.offset((x + y * VGA_BUFFER_WIDTH) as isize * 2) =
                        self.buffer[y][x].character;
                    *vga_text_buffer.offset((x + y * VGA_BUFFER_WIDTH) as isize * 2 + 1) =
                        self.buffer[y][x].color;
                }
            }
        }
    }
}


impl fmt::Write for VGABuffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
        Ok(())
    }
}

pub fn vga_init() {
    unsafe {
        VGA_BUFFER.clear_screen();
    }
}


