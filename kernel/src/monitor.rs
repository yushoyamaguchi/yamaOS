use crate::console::*;

pub fn monitor(){
    let mut c = ' ';
    cons_putc('K');
    cons_putc('>');
    while 1==1  {
        if c=='\n' {
            cons_putc('K');
            cons_putc('>');
        }
        c = getc();
        if c==b'\x08' as char && unsafe { CONS.wpos  <= 1 } {
            continue;
        }
        cons_putc(c);
    }
}