pub fn memset(s: *mut u8, c: u8, size: usize) {
    unsafe {
        for i in 0..size {
            *s.offset(i as isize) = c;
        }
    }
}