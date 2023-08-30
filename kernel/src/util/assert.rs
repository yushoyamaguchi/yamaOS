
#[macro_export]
macro_rules! assert {
    ($x:expr $(,)?) => (
        if !$x {
            panic!(concat!("assertion failed: ", stringify!($x)));
        }
    );
}