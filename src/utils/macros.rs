
/// Convert from mb to b
#[macro_export]
macro_rules! MB {
    ($a:expr) => {
       $a * (2 << 20)
    };
}

#[macro_export]
macro_rules! align_down{
    ($addr:expr, $align:expr) => {
       $addr & !((1<<$align) - 1)
    };
}

#[macro_export]
macro_rules! align_up{
    ($addr:expr, $align:expr) => {
       align_down!($addr, $align) + $align
    };
}

#[macro_export]
macro_rules! is_aligned{
    ($addr:expr, $align:expr) => {
       $addr & ($align - 1) == 0
    };
}
