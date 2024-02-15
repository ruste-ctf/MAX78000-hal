use crate::const_assert;
use crate::memory_map::mmio;
use crate::{bit_impl, reg_impl};

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

use hal_macros::RW;
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
    #[bit(13, RW1C, rro::TMR_CTRL1)]
    done_flag,

    /// example of RW
    #[bit(12, RW, rro::TMR_NOLCMP)]
    my_read_write_flag,
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

/// # Timer Non-Overlapping Compare Register
/// The Timer Non-Overlapping Compare Register. See Page 319, Table 19-14.
pub struct NonOverlappingCompareRegister<const PORT_PTR: usize> {}
reg_impl!(RW, NonOverlappingCompareRegister, rro::TMR_NOLCMP);

impl<const PORT_PTR: usize> NonOverlappingCompareRegister<PORT_PTR> {
    bit_impl! {24..=31, RW u8,
    /// # Set TimerA Non-Overlapping High Compare 1
    set_timera_non_overlapping_high_compare_1,
    /// # Get TimerA Non-Overlapping High Compare 1
    get_timera_non_overlapping_high_compare_1}

    bit_impl! {16..=23, RW u8,
    /// # Set TimerA Non-Overlapping Low Compare 1
    set_timera_non_overlapping_low_compare_1,
    /// # Get TimerA Non-Overlapping Low Compare 1
    get_timera_non_overlapping_low_compare_1}

    bit_impl! {8..=15, RW u8,
    /// # Set TimerA Non-Overlapping High Compare 0
    set_timera_non_overlapping_high_compare_0,
    /// # Get TimerA Non-Overlapping High Compare 0
    get_timera_non_overlapping_high_compare_0}

    bit_impl! {0..=7, RW u8,
    /// # Set TimerA Non-Overlapping Low Compare 0
    set_timera_non_overlapping_low_compare_0,
    /// # Get TimerA Non-Overlapping Low Compare 0
    get_timera_non_overlapping_low_compare_0}
}

/// # Timer Control 1 Register
/// The Timer Control 1 Register. See Page 319-321, Table 19-15.
pub struct Control1Register<const PORT_PTR: usize> {}
reg_impl!(RW, Control1Register, rro::TMR_CTRL1);

impl<const PORT_PTR: usize> Control1Register<PORT_PTR> {
    bit_impl! {31, RW,
    /// # Set 32-bit Cascade Timer Enable
    set_32bit_cascade_timer_enable,
    /// # Get 32-bit Cascade Timer Enable
    get_32bit_cascade_timer_enable}

    bit_impl! {28, RW,
    /// # Set TimerB Wake-Up Function
    set_timerb_wakeup_function,
    /// # Get TimerB Wake-Up Function
    get_timerb_wakeup_function}

    bit_impl! {27, RW,
    /// # Set TimerB Software Event Capture
    set_timerb_software_event_capture,
    /// # Get TimerB Software Event Capture
    get_timerb_software_event_capture}

    bit_impl! {25..=26, RW u8,
    /// # Set TimerB Event Capture Selection
    set_timerb_event_capture_selection,
    /// # Get TimerB Event Capture Selection
    get_timerb_event_capture_selection}

    bit_impl! {24, RW,
    /// # Set TimerB Interrupt Enable
    set_timerb_interrupt_enable,
    /// # Get TimerB Interrupt Enable
    get_timerb_interrupt_enable}

    bit_impl! {23, RW,
    /// # Set TimerB Negative Edge Trigger for Event
    set_timerb_negative_edge_trigger_for_event,
    /// # Get TimerB Negative Edge Trigger for Event
    get_timerb_negative_edge_trigger_for_event}

    bit_impl! {20..=22, RW u8,
    /// # Set TimerB Event Selection
    set_timerb_event_selection,
    /// # Get TimerB Event Selection
    get_timerb_event_selection}

    bit_impl! {19, RO,
    /// # Get TimerB Clock Ready Status
    get_timerb_clock_ready_status}

    bit_impl! {18, RO,
    /// # Get TimerB Clock Enable Status
    get_timerb_clock_enable_status}

    bit_impl! {16..=17, RW u8,
    /// # Set TimerB Clock Source
    set_timerb_clock_source,
    /// # Get TimerB Clock Source
    get_timerb_clock_source}

    bit_impl! {14, RW,
    /// # Set Output B Enable
    set_output_b_enable,
    /// # Get Output B Enable
    get_output_b_enable}

    bit_impl! {13, RW,
    /// # Set Output Enable
    set_output_enable,
    /// # Get Output Enable
    get_output_enable}

    bit_impl! {12, RW,
    /// # Set TimerA Wake-Up Function
    set_timera_wakeup_function,
    /// # Get TimerA Wake-Up Function
    get_timera_wakeup_function}

    bit_impl! {11, RW,
    /// # Set TimerA Software Event Capture
    set_timera_software_event_capture,
    /// # Get TimerA Software Event Capture
    get_timera_software_event_capture}

    bit_impl! {9..=10, RW u8,
    /// # Set TimerA Event Capture Selection
    set_timera_event_capture_selection,
    /// # Get TimerA Event Capture Selection
    get_timera_event_capture_selection}

    bit_impl! {8, RW,
    /// # Set TimerA Interrupt Enable
    set_timera_interrupt_enable,
    /// # Get TimerA Interrupt Enable
    get_timera_interrupt_enable}

    bit_impl! {7, RW,
    /// # Set TimerA Negative Edge Trigger for Event
    set_timera_negative_edge_trigger_for_event,
    /// # Get TimerA Negative Edge Trigger for Event
    get_timera_negative_edge_trigger_for_event}

    bit_impl! {4..=6, RW u8,
    /// # Set TimerA Event Selection
    set_timera_event_selection,
    /// # Get TimerA Event Selection
    get_timera_event_selection}

    bit_impl! {3, RO,
    /// # Get TimerA Clock Ready
    get_timera_clock_ready}

    bit_impl! {2, RW,
    /// # Set TimerA Clock Enable
    set_timera_clock_enable,
    /// # Get TimerA Clock Enable
    get_timera_clock_enable}

    bit_impl! {0..=1, RW u8,
    /// # Set TimerA Clock Source
    set_timera_clock_source,
    /// # Get TimerA Clock Source
    get_timera_clock_source}
}

/// # Timer Wake-Up Status Register
/// The Timer Wake-Up Status Register. See Page 321-322, Table 19-16.
pub struct WakeupStatusRegister<const PORT_PTR: usize> {}
reg_impl!(
    RW1C,
    WakeupStatusRegister,
    rro::TMR_WKFL,
    0b00000000000000000000000000000000
);

impl<const PORT_PTR: usize> WakeupStatusRegister<PORT_PTR> {
    bit_impl! {16, RW1C,
    /// # Set TimerB Wake-Up Event
    set_timerb_wakeup_event,
    /// # Get TimerB Wake-Up Event
    get_timerb_wakeup_event}

    bit_impl! {0, RW1C,
    /// # Set TimerA Wake-Up Event
    set_timera_wakeup_event,
    /// # Get TimerA Wake-Up Event
    get_timera_wakeup_event}
}
