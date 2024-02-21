#![no_std]
pub mod aes;
pub mod bits;
pub mod debug;
pub mod error;
pub mod gcr;
pub mod gpio;
pub mod i2c;
pub mod memory_map;
pub mod timer;
pub mod trng;
pub mod uart;

#[cfg(test)]
pub mod tests;

extern "C" {
    #[link_name = "SystemCoreClock"]
    pub static SYSTEM_CORE_CLOCK: u32;
}

/// # Core Peripheral Clock
/// Get the peripheral clock used for timing things like I2C and UART for the CPU.
pub fn core_peripheral_clock() -> u32 {
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
