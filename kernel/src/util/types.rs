fn rounddown(a: u32, n: u32) -> u32 {
    a - a % n
}

pub fn roundup(a: u32, n: u32) -> u32 {
    rounddown(a + n - 1, n)
}