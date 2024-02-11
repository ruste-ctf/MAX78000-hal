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

/// # Timer Interrupt Register
/// The Timer Interrupt Register. See Page 315-316, Table 19-12.
pub struct InterruptRegister<const PORT_PTR: usize> {}
reg_impl!(
    RW1C,
    InterruptRegister,
    rro::TMR_INTFL,
    0b00000000000000000000000000000000
);

impl<const PORT_PTR: usize> InterruptRegister<PORT_PTR> {
    bit_impl! {24, RW,
    /// # Set TimerB Write Protect in Dual Timer Mode
    set_timerb_write_protect_in_dual_timer_mode,
    /// # Get TimerB Write Protect in Dual Timer Mode
    get_timerb_write_protect_in_dual_timer_mode}

    bit_impl! {25, RO,
    /// # Get TimerB Write Done
    get_timerb_write_done}

    bit_impl! {16, RW1C,
    /// # Set TimerB Interrupt Event
    set_timerb_interrupt_event,
    /// # Get TimerB Interrupt Event
    get_timerb_interrupt_event}

    bit_impl! {9, RW,
    /// # Get TimerB Dual Timer Mode Write Protect
    get_timerb_dual_timer_mode_write_protect,
    /// # Set TimerB Dual Timer Mode Write Protect
    set_timerb_dual_timer_mode_write_protect}

    bit_impl! {8, RO,
    /// # Get TimerA Write Done
    get_timera_write_done}

    bit_impl! {0, RW1C,
    /// # Set TimerA Interrupt Event
    set_timera_interrupt_event,
    /// # Get TimerA Interrupt Event
    get_timera_interrupt_event}
}

/// # Timer Control 0 Register
/// The Timer Control 0 Register. See Page 316-319, Table 19-13.
pub struct TimerControl0Register<const PORT_PTR: usize> {}
reg_impl!(RW, TimerControl0Register, rro::TMR_CTRL0);

impl<const PORT_PTR: usize> TimerControl0Register<PORT_PTR> {
    bit_impl! {31, RW,
    /// # Set TimerB Enable
    set_timerb_enable,
    /// # Get TimerB Enable
    get_timerb_enable}

    bit_impl! {30, RW,
    /// # Set TimerB Clock Enable
    set_timerb_clock_enable,
    /// # Get TimerB Clock Enable
    get_timerb_clock_enable}

    bit_impl! {29, RESET, // FIXME
    /// # Activate TimerB Reset
    activate_timeb_reset}

    bit_impl! {20..=23, RW u8,
    /// # Set TimerB Prescaler Select
    set_timerb_prescaler_select,
    /// # Get TimerB Prescaler Select
    get_timerb_prescaler_select}

    bit_impl! {16..=19, RW u8,
    /// # Set TimerB Mode Select
    set_timerb_mode_select,
    /// # Get TimerB Mode Select
    get_timerb_mode_select}

    bit_impl! {15, RW,
    /// # Set TimerA Enable
    set_timera_enable,
    /// # Get TimerA Enable
    get_timera_enable}

    bit_impl! {14, RW,
    /// # Set TimerA Clock Enable
    set_timera_clock_enable,
    /// # Get TimerA Clock Enable
    get_timera_clock_enable}

    bit_impl! {13, RESET, // FIXME
    /// # Activate TimerA Reset
    activate_timea_reset}

    bit_impl! {12, RW,
    /// # Set TimerA PWM Output 𝝓𝑨′ Disable
    set_timera_pwm_output_phi_alpha_prime_disable,
    /// # Get TimerA PWM Output 𝝓𝑨′ Disable
    get_timera_pwm_output_phi_alpha_prime_disable}

    bit_impl! {11, RW,
    /// # Set TimerA PWM Output 𝝓𝑨′ Polarity Bit
    set_timera_pwm_output_phi_alpha_prime_polarity_bit,
    /// # Get TimerA PWM Output 𝝓𝑨′ Polarity Bit
    get_timera_pwm_output_phi_alpha_prime_polarity_bit}

    bit_impl! {10, RW,
    /// # Set TimerA PWM Output 𝝓𝑨 Polarity Bit
    set_timera_pwm_output_phi_alpha_polarity_bit,
    /// # Get TimerA PWM Output 𝝓𝑨 Polarity Bit
    get_timera_pwm_output_phi_alpha_polarity_bit}

    bit_impl! {9, RW,
    /// # Set TimerA/TimerB PWM Synchronization Mode
    set_timera_timerb_pwm_synchronization_mode,
    /// # Get TimerA/TimerB PWM Synchronization Mode
    get_timera_timerb_pwm_synchronization_mode}

    bit_impl! {8, RW,
    /// # Set TimerA Polarity
    set_timera_polarity,
    /// # Get TimerA Polarity
    get_timera_polarity}

    bit_impl! {4..=7, RW u8,
    /// # Set TimerA Prescaler Select
    set_timera_prescaler_select,
    /// # Get TimerA Prescaler Select
    get_timera_prescaler_select}

    bit_impl! {0..=3, RW u8,
    /// # Set TimerA Mode Select
    set_timera_mode_select,
    /// # Get TimerA Mode Select
    get_timera_mode_select}
}
