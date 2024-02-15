#![no_std]
pub mod aes;
pub mod bits;
pub mod error;
pub mod i2c;
pub mod memory_map;
pub mod registers;
pub mod uart;
pub mod timer;
pub mod trng;

extern "C" {
    #[link_name = "SystemCoreClock"]
    pub(crate) static SYSTEM_CORE_CLOCK: u32;
}

/// # Core Peripheral Clock
/// Get the peripheral clock used for timing things like I2C and UART for the CPU.
pub(crate) fn core_peripheral_clock() -> u32 {
    unsafe { SYSTEM_CORE_CLOCK / 2 }
}




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
