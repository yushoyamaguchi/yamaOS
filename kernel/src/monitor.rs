use crate::console::*;

pub fn monitor(){
    let mut c = ' ';
    cons_putc('K');
    cons_putc('>');
    while 1==1  {
        if(c=='\n'){
            cons_putc('K');
            cons_putc('>');
        }
        c = cons_getc();
        cons_putc(c);
    }
}