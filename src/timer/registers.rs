use crate::bits::BitManipulation;
use crate::const_assert;
use crate::memory_map::mmio;
use crate::{bit_impl, reg_impl};
use core::ptr;

/// # Timer Register Offsets
/// See Max 78000 User Guide Page 314, Table 19-8.
mod rro {
    /// # Timer Counter Register
    pub const TMR_CNT: usize = 0x0000;
    /// # Timer Compare Register
    pub const TMR_CMP: usize = 0x0004;
    /// # Timer PWM Register
    pub const TMR_PWM: usize = 0x0008;
    /// # Timer Interrupt Register
    pub const TMR_INTFL: usize = 0x000C;
    /// # Timer Control Register
    pub const TMR_CTRL0: usize = 0x0010;
    /// # Timer Non-Overlapping Compare Register
    pub const TMR_NOLCMP: usize = 0x0014;
    /// # Timer Configuration Register
    pub const TMR_CTRL1: usize = 0x0018;
    /// # Timer Wake-up Status Register
    pub const TMR_WKFL: usize = 0x001C;
}

/// # Timer Count Register
/// The Timer Count Register. See Page 315, Table 19-9.
pub struct CountRegister<const PORT_PTR: usize> {}
reg_impl!(RW, CountRegister, rro::TMR_CNT);

impl<const PORT_PTR: usize> CountRegister<PORT_PTR> {
    bit_impl! {0..=31, RW u32,
    /// # Set Timer Count
    set_timer_count,
    /// # Get Timer Count
    get_timer_count}
}

/// # Timer Compare Register
/// The Timer Compare Register. See Page 315, Table 19-10.
pub struct CompareRegister<const PORT_PTR: usize> {}
reg_impl!(RW, CompareRegister, rro::TMR_CMP);

impl<const PORT_PTR: usize> CompareRegister<PORT_PTR> {
    bit_impl! {0..=31, RW u32,
    /// # Set Timer Compare Value
    set_timer_compare_value,
    /// # Get Timer Compare Value
    get_timer_compare_value}
}

/// # Timer PWM Register
/// The Timer PWM Register. See Page 315, Table 19-11.
pub struct PWMRegister<const PORT_PTR: usize> {}
reg_impl!(RW, PWMRegister, rro::TMR_PWM);

impl<const PORT_PTR: usize> PWMRegister<PORT_PTR> {
    bit_impl! {0..=31, RW u32,
    /// # Set PWM
    set_pwm,
    /// # Get PWM
    get_pwm}
}
