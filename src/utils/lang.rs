pub const STACK_OVERFLOW: &str = "Stack overflow!";

#[macro_export]
macro_rules! stack_overflow {
    () => {
        panic!("{}", STACK_OVERFLOW)
    };
}