#[macro_export]
macro_rules! addr_of_func {
    ($target:expr) => {
        ($target as *const ()) as usize
    };
}
