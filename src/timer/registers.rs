use hal_macros_derive::make_device;

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

make_device! {
    device_ports(mmio::Timer_0, mmio::Timer_1, mmio::Timer_2);

    /// Timer Count. See Page 315, Table 19-9.
    #[bit(0..=31, RW, rro::TMR_CNT)]
    timer_count,

    /// The Timer Compare Register
    /// Timer Compare Value. See Page 315, Table 19-10.
    #[bit(0..=31, RW, rro::TMR_CMP)]
    timer_compare_value,

    /// PWM. See Page 315, Table 19-11.
    #[bit(0..=31, RW, rro::TMR_PWM)]
    pwm,

    /// TimerB Write Protect in Dual Timer Mode. See Page 315-316, Table 19-12.
    #[bit(24, RW, rro::TMR_INTFL)]
    timerb_write_protect_in_dual_timer_mode,

    /// TimerB Write Done. See Page 315-316, Table 19-12.
    #[bit(25, RO, rro::TMR_INTFL)]
    timerb_write_done,

    /// TimerB Interrupt Event. See Page 315-316, Table 19-12.
    #[bit(16, RW1C, rro::TMR_INTFL)]
    timerb_interrupt_event,

    /// TimerB Dual Timer Mode Write Protect. See Page 315-316, Table 19-12.
    #[bit(9, RW, rro::TMR_INTFL)]
    timerb_dual_timer_mode_write_protect,

    /// TimerA Write Done. See Page 315-316, Table 19-12.
    #[bit(8, RO, rro::TMR_INTFL)]
    timera_write_done,

    /// TimerA Interrupt Event. See Page 315-316, Table 19-12.
    #[bit(0, RW1C, rro::TMR_INTFL)]
    timera_interrupt_event,

    /// TimerB Enable. See Page 316-319, Table 19-13.
    #[bit(31, RW, rro::TMR_CTRL0)]
    timerb_enable,

    /// TimerB Clock Enable. See Page 316-319, Table 19-13.
    #[bit(30, RW, rro::TMR_CTRL0)]
    timerb_clock_enable,

    /// TimerB Reset. See Page 316-319, Table 19-13.
    #[bit(29, RW1O, rro::TMR_CTRL0)]
    timeb_reset,

    /// TimerB Prescaler Select. See Page 316-319, Table 19-13.
    #[bit(20..=23, RW, rro::TMR_CTRL0)]
    timerb_prescaler_select,

    /// TimerB Mode Select. See Page 316-319, Table 19-13.
    #[bit(16..=19, RW, rro::TMR_CTRL0)]
    timerb_mode_select,

    /// TimerA Enable. See Page 316-319, Table 19-13.
    #[bit(15, RW, rro::TMR_CTRL0)]
    timera_enable,

    /// TimerA Clock Enable. See Page 316-319, Table 19-13.
    #[bit(14, RW, rro::TMR_CTRL0)]
    timera_clock_enable,

    /// TimerA Reset. See Page 316-319, Table 19-13.
    #[bit(13, RW1O, rro::TMR_CTRL0)]
    timea_reset,

    /// TimerA PWM Output 𝝓𝑨′ Disable. See Page 316-319, Table 19-13.
    #[bit(12, RW, rro::TMR_CTRL0)]
    timera_pwm_output_phi_alpha_prime_disable,

    /// TimerA PWM Output 𝝓𝑨′ Polarity Bit. See Page 316-319, Table 19-13.
    #[bit(11, RW, rro::TMR_CTRL0)]
    timera_pwm_output_phi_alpha_prime_polarity_bit,

    /// TimerA PWM Output 𝝓𝑨 Polarity Bit. See Page 316-319, Table 19-13.
    #[bit(10, RW, rro::TMR_CTRL0)]
    timera_pwm_output_phi_alpha_polarity_bit,

    /// TimerA/TimerB PWM Synchronization Mode. See Page 316-319, Table 19-13.
    #[bit(9, RW, rro::TMR_CTRL0)]
    timera_timerb_pwm_synchronization_mode,

    /// TimerA Polarity. See Page 316-319, Table 19-13.
    #[bit(8, RW, rro::TMR_CTRL0)]
    timera_polarity,

    /// TimerA Prescaler Select. See Page 316-319, Table 19-13.
    #[bit(4..=7, RW, rro::TMR_CTRL0)]
    timera_prescaler_select,

    /// TimerA Mode Select. See Page 316-319, Table 19-13.
    #[bit(0..=3, RW, rro::TMR_CTRL0)]
    timera_mode_select,

    /// TimerA Non-Overlapping High Compare 1. See Page 319, Table 19-14.
    #[bit(24..=31, RW, rro::TMR_NOLCMP)]
    timera_non_overlapping_high_compare_1,

    /// TimerA Non-Overlapping Low Compare 1. See Page 319, Table 19-14.
    #[bit(16..=23, RW, rro::TMR_NOLCMP)]
    timera_non_overlapping_low_compare_1,

    /// TimerA Non-Overlapping High Compare 0. See Page 319, Table 19-14.
    #[bit(8..=15, RW, rro::TMR_NOLCMP)]
    timera_non_overlapping_high_compare_0,

    /// TimerA Non-Overlapping Low Compare 0. See Page 319, Table 19-14.
    #[bit(0..=7, RW, rro::TMR_NOLCMP)]
    timera_non_overlapping_low_compare_0,

    /// 32-bit Cascade Timer Enable. See Page 319-321, Table 19-15.
    #[bit(31, RW, rro::TMR_CTRL1)]
    bit32_cascade_timer_enable,

    /// TimerB Wake-Up Function. See Page 319-321, Table 19-15.
    #[bit(28, RW, rro::TMR_CTRL1)]
    timerb_wakeup_function,

    /// TimerB Software Event Capture. See Page 319-321, Table 19-15.
    #[bit(27, RW, rro::TMR_CTRL1)]
    timerb_software_event_capture,

    /// TimerB Event Capture Selection. See Page 319-321, Table 19-15.
    #[bit(25..=26, RW, rro::TMR_CTRL1)]
    timerb_event_capture_selection,

    /// TimerB Interrupt Enable. See Page 319-321, Table 19-15.
    #[bit(24, RW, rro::TMR_CTRL1)]
    timerb_interrupt_enable,

    /// TimerB Negative Edge Trigger for Event. See Page 319-321, Table 19-15.
    #[bit(23, RW, rro::TMR_CTRL1)]
    timerb_negative_edge_trigger_for_event,

    /// TimerB Event Selection. See Page 319-321, Table 19-15.
    #[bit(20..=22, RW, rro::TMR_CTRL1)]
    timerb_event_selection,

    /// TimerB Clock Ready Status. See Page 319-321, Table 19-15.
    #[bit(19, RO, rro::TMR_CTRL1)]
    timerb_clock_ready_status,

    /// TimerB Clock Enable Status. See Page 319-321, Table 19-15.
    #[bit(18, RO, rro::TMR_CTRL1)]
    timerb_clock_enable_status,

    /// TimerB Clock Source. See Page 319-321, Table 19-15.
    #[bit(16..=17, RW, rro::TMR_CTRL1)]
    timerb_clock_source,

    /// Output B Enable. See Page 319-321, Table 19-15.
    #[bit(14, RW, rro::TMR_CTRL1)]
    output_b_enable,

    /// Output Enable. See Page 319-321, Table 19-15.
    #[bit(13, RW, rro::TMR_CTRL1)]
    output_enable,

    /// TimerA Wake-Up Function. See Page 319-321, Table 19-15.
    #[bit(12, RW, rro::TMR_CTRL1)]
    timera_wakeup_function,

    /// TimerA Software Event Capture. See Page 319-321, Table 19-15.
    #[bit(11, RW, rro::TMR_CTRL1)]
    timera_software_event_capture,

    /// TimerA Event Capture Selection. See Page 319-321, Table 19-15.
    #[bit(9..=10, RW, rro::TMR_CTRL1)]
    timera_event_capture_selection,

    /// TimerA Interrupt Enable. See Page 319-321, Table 19-15.
    #[bit(8, RW, rro::TMR_CTRL1)]
    timera_interrupt_enable,

    /// TimerA Negative Edge Trigger for Event. See Page 319-321, Table 19-15.
    #[bit(7, RW, rro::TMR_CTRL1)]
    timera_negative_edge_trigger_for_event,

    /// TimerA Event Selection. See Page 319-321, Table 19-15.
    #[bit(4..=6, RW, rro::TMR_CTRL1)]
    timera_event_selection,

    /// TimerA Clock Ready. See Page 319-321, Table 19-15.
    #[bit(3, RO, rro::TMR_CTRL1)]
    timera_clock_ready,

    /// TimerA Clock Enable. See Page 319-321, Table 19-15.
    #[bit(2, RW, rro::TMR_CTRL1)]
    timera_clock_enable2,

    /// TimerA Clock Source. See Page 319-321, Table 19-15.
    #[bit(0..=1, RW, rro::TMR_CTRL1)]
    timera_clock_source,

    /// TimerB Wake-Up Event. See Page 321-322, Table 19-16.
    #[bit(16, RW1C, rro::TMR_WKFL)]
    timerb_wakeup_event,

    /// TimerA Wake-Up Event. See Page 321-322, Table 19-16.
    #[bit(0, RW1C, rro::TMR_WKFL)]
    timera_wakeup_event,
}
