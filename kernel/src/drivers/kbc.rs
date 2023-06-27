use crate::x86::*;
use crate::console::*;

const KBC_DATA_ADDR: u16 = 0x0060;
const KBC_DATA_BIT_IS_BRAKE: u8 = 0x80;
const KBC_STATUS_ADDR: u16 = 0x0064;
const KBC_STATUS_BIT_OBF: u8 = 0x01;

const ASCII_ESC: char = '\x1B';
const ASCII_BS: char = '\x08';
const ASCII_HT: char = '\x09';

const KEYMAP: [char; 128] = [
    '\0', ASCII_ESC, '1', '2', '3', '4', '5', '6',
    '7', '8', '9', '0', '-', '^', ASCII_BS, ASCII_HT,
    'q', 'w', 'e', 'r', 't', 'y', 'u', 'i',
	'o', 'p', '@', '[', '\n', 0x00 as char, 'a', 's',
	'd', 'f', 'g', 'h', 'j', 'k', 'l', ';',
	':', 0x00 as char, 0x00 as char, ']', 'z', 'x', 'c', 'v',
	'b', 'n', 'm', ',', '.', '/', 0x00 as char, '*',
	0x00 as char, ' ', 0x00 as char, 0x00 as char, 0x00 as char, 0x00 as char, 0x00 as char, 0x00 as char,
	0x00 as char, 0x00 as char, 0x00 as char, 0x00 as char, 0x00 as char, 0x00 as char, 0x00 as char, '7',
	'8', '9', '-', '4', '5', '6', '+', '1',
	'2', '3', '0', '.', 0x00 as char, 0x00 as char, 0x00 as char, 0x00 as char,
	0x00 as char, 0x00 as char, 0x00 as char, 0x00 as char, 0x00 as char, 0x00 as char, 0x00 as char, 0x00 as char,
	0x00 as char, 0x00 as char, 0x00 as char, 0x00 as char, 0x00 as char, 0x00 as char, 0x00 as char, 0x00 as char,
	0x00 as char, 0x00 as char, 0x00 as char, 0x00 as char, 0x00 as char, 0x00 as char, 0x00 as char, 0x00 as char,
	0x00 as char, 0x00 as char, 0x00 as char, '_', 0x00 as char, 0x00 as char, 0x00 as char, 0x00 as char,
	0x00 as char, 0x00 as char, 0x00 as char, 0x00 as char, 0x00 as char, '\\', 0x00 as char, 0x00 as char
];

fn get_kbc_data() -> Option<u8> {
    // Wait until the OBF bit is set in the status register.
    if inb(KBC_STATUS_ADDR)  & KBC_STATUS_BIT_OBF == 0 {
        None
    } else {
        Some(inb(KBC_DATA_ADDR))
    }
}

fn get_keycode() -> Option<u8> {
    let keycode;
    // Wait until the brake bit is not set (i.e., make state).
    let keycode_option = get_kbc_data();
    if keycode_option  == None  {
        return None;
    }
    else{
        keycode = keycode_option.unwrap();
    }
    if keycode & KBC_DATA_BIT_IS_BRAKE != 0 {
        None
    } else {
        Some(keycode)
    }
}


pub fn kbc_intr()  {
    match get_keycode() {
        Some(c) => {
            unsafe {
                ConsoleStruct.buf [ConsoleStruct.wpos as usize] = KEYMAP[c as usize] as u8;
                ConsoleStruct.wpos = (ConsoleStruct.wpos+1)%CONSBUFSIZE as u32;
            }
        },
        None => {}
    }
}