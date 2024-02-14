#![no_std]
pub mod bits;
pub mod error;
pub mod i2c;
pub mod memory_map;
pub mod registers;
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

use hal_macros_derive::make_device;

make_device! {
    device_ports(mmio::TIMER_0, mmio::TIMER_1, mmio::TIMER_2);

    /// Set the count of the timer.
    #[bit(0..=31, RW, rro::TMR_CNT)]
    time_count,

    /// The timer compare value.
    #[bit(0..=31, RW, rro::TMR_CMP)]
    timer_compare_value,

    /// The timer PWM register.
    #[bit(0..=31, RW, rro::TMR_PWM)]
    pwm,

    /// The timer Interrupt register.
    #[bit(25, RO, rro::TMR_INTFL)]
    timerb_write_done,

    ///example of some RW1C
    #[bit(13, RW1C, rro::DINGUS)]
    done_flag,

    /// example of RW
    #[bit(12, RW, rro::DINGUS)]
    my_read_write_flag,
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
