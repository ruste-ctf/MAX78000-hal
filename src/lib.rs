#![no_std]
pub mod bits;
pub mod error;
pub mod i2c;
pub mod memory_map;
pub mod registers;
pub mod timer;

/// # Const Assert
/// Assert in a const context, useful for making sure that
/// provided constants fall in expected range.
#[macro_export]
macro_rules! const_assert {
    (STRUCT, $($tt:tt)*) => {
        #[allow(unused)]
        const CONST_ASSERT_VALUE: () = assert!($($tt)*);
    };
    ($($tt:tt)*) => {
        const _: () = assert!($($tt)*);
    };
}
