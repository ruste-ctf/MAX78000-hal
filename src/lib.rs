#![no_std]

pub mod error;
pub mod i2c;
pub mod memory_map;

/// # Const Assert
/// Assert in a const context, useful for making sure that
/// provided constants fall in expected range.
#[macro_export]
macro_rules! const_assert {
    ($($tt:tt)*) => {
        const _: () = assert!($($tt)*);
    };
}
