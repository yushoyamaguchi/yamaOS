use crate::x86::read_io_port;

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

fn get_kbc_data() -> u8 {
    // Wait until the OBF bit is set in the status register.
    while  read_io_port(KBC_STATUS_ADDR)  & KBC_STATUS_BIT_OBF == 0 {}
    read_io_port(KBC_DATA_ADDR) 
}

fn get_keycode() -> u8 {
    let mut keycode;
    // Wait until the brake bit is not set (i.e., make state).
    while {
        keycode = get_kbc_data();
        keycode & KBC_DATA_BIT_IS_BRAKE != 0
    } {}
    keycode
}

pub fn getc() -> char {
    KEYMAP[get_keycode() as usize]
}