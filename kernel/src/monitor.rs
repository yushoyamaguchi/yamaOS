use crate::console::*;

pub fn monitor(){
    let mut c = ' ';
    cons_putc('K');
    cons_putc('>');
    unsafe { 
        CONS.wpos=2 ;
        CONS.rpos=2 ;
    }
    while 1==1  {
        if c=='\n' {
            cons_putc('K');
            cons_putc('>');
            unsafe { 
                CONS.wpos=2 ;
                CONS.rpos=2 ;
            }
        }
        
        c = getc();
        //  ToDo : Now, we don't consider input buffer overflow
        if c==b'\x08' as char && unsafe { CONS.rpos  > 3 } {
            unsafe{
                CONS.rpos -= 2;
                CONS.wpos -= 2;
            }
        }
        else if c==b'\x08' as char && unsafe { CONS.rpos  <= 3 } {
            unsafe{
                CONS.rpos -= 1;
                CONS.wpos -= 1;
            }
            continue;
        }
        cons_putc(c);
    }
}