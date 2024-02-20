use hal_macros::RW;
use hal_macros_derive::make_device;

use crate::memory_map::mmio;

/// # Relative Register Offsets
/// These are the offsets for the GCR registers that the
/// Maxim Integrated - spec shows. Found on page 80.
mod rro {
    /// # System Control Register
    pub const GCR_SYSCTRL: usize = 0x0000;
    /// # Reset Register 0
    pub const GCR_RST0: usize = 0x0004;
    /// # Clock Control Register
    pub const GCR_CLKCTRL: usize = 0x0008;
    /// # Power Management Register
    pub const GCR_PM: usize = 0x000C;
    /// # Peripheral Clocks Divisor
    pub const GCR_PCLKDIV: usize = 0x0018;
    /// # Peripheral Clocks Disable 0
    pub const GCR_PCLKDIS0: usize = 0x0024;
    /// # Memory Clock Control
    pub const GCR_MEMCTRL: usize = 0x0028;
    /// # Memory `Zeroize` Register
    pub const GCR_MEMZ: usize = 0x002C;
    /// # System Status Flags
    pub const GCR_SYSST: usize = 0x0040;
    /// # Reset Register 1
    pub const GCR_RST1: usize = 0x0044;
    /// # Peripheral Clocks Disable 1
    pub const GCR_PCLKDIS1: usize = 0x0048;
    /// # Event Enable Register
    pub const GCR_EVENTEN: usize = 0x004C;
    /// # Revision Register
    pub const GCR_REVISION: usize = 0x0050;
    /// # System Status Interrupt Enable Register
    pub const GCR_SYSIE: usize = 0x0054;
    /// # Error Correction Coding Error Register
    pub const GCR_ECCERR: usize = 0x0064;
    /// # Error Correction Coding Correctable Error Detected
    pub const GCR_ECCCED: usize = 0x0068;
    /// # Error Correction Coding Interrupt Enable Register
    pub const GCR_ECCIE: usize = 0x006C;
    /// # Error Correction Coding Error Address Register
    pub const GCR_ECCADDR: usize = 0x0070;
    /// # General Purpose Register 0
    pub const GCR_GPR0: usize = 0x0080;
}

make_device! {
    device_ports(mmio::GLOBAL_CONTROL);

    #[bit(0..=31, RO, rro::GCR_RST0)]
    reset_status0,

    #[bit(0..=31, RO, rro::GCR_RST1)]
    reset_status1,

    #[bit(16..=17, RW, rro::GCR_SYSCTRL)]
    operating_voltage_range,

    #[bit(15, RO, rro::GCR_SYSCTRL)]
    rom_checksum_calc_pass,

    #[bit(14, RW, rro::GCR_SYSCTRL)]
    serial_wire_debug_enable,

    #[bit(13, RW1O, rro::GCR_SYSCTRL)]
    calculate_rom_checksum,

    #[bit(6, RW, rro::GCR_SYSCTRL)]
    icc0_cache_flush,

    #[bit(4, RO, rro::GCR_SYSCTRL)]
    flash_page_flip_flag,

    #[bit(31, RW1O, rro::GCR_RST0)]
    system_reset,

    #[bit(30, RW1O, rro::GCR_RST0)]
    peripheral_reset,

    #[bit(29, RW1O, rro::GCR_RST0)]
    soft_reset,

    #[bit(28, RW1O, rro::GCR_RST0)]
    uart2_reset,

    #[bit(26, RW1O, rro::GCR_RST0)]
    adc_reset,

    #[bit(25, RW1O, rro::GCR_RST0)]
    cnn_reset,

    #[bit(24, RW1O, rro::GCR_RST0)]
    trng_reset,

    #[bit(17, RW1O, rro::GCR_RST0)]
    rtc_reset,

    #[bit(16, RW1O, rro::GCR_RST0)]
    i2c0_reset,

    #[bit(13, RW1O, rro::GCR_RST0)]
    spi1_reset,

    #[bit(12, RW1O, rro::GCR_RST0)]
    uart1_reset,

    #[bit(11, RW1O, rro::GCR_RST0)]
    uart0_reset,

    #[bit(8, RW1O, rro::GCR_RST0)]
    timer3_reset,

    #[bit(7, RW1O, rro::GCR_RST0)]
    timer2_reset,

    #[bit(6, RW1O, rro::GCR_RST0)]
    timer1_reset,

    #[bit(5, RW1O, rro::GCR_RST0)]
    tiemr0_reset,

    #[bit(3, RW1O, rro::GCR_RST0)]
    gpio1_reset,

    #[bit(2, RW1O, rro::GCR_RST0)]
    gpio0_reset,

    #[bit(1, RW1O, rro::GCR_RST0)]
    watchdog_timer0_reset,

    #[bit(0, RW1O, rro::GCR_RST0)]
    dma_access_block_reset,

    #[bit(29, RO, rro::GCR_CLKCTRL)]
    internal_nano_ring_oscillator_ready,

    #[bit(28, RO, rro::GCR_CLKCTRL)]
    internal_baud_rate_oscillator_ready,

    #[bit(27, RO, rro::GCR_CLKCTRL)]
    internal_primary_oscillator_ready,

    #[bit(26, RO, rro::GCR_CLKCTRL)]
    internal_secondary_oscillator_ready,

    #[bit(25, RO, rro::GCR_CLKCTRL)]
    external_rtc_oscillator_ready,

    #[bit(21, RW, rro::GCR_CLKCTRL)]
    internal_baud_rate_oscillator_power_supply_select,

    #[bit(20, RO, rro::GCR_CLKCTRL)]
    internal_baud_rate_oscillator_enable,

    #[bit(19, RW, rro::GCR_CLKCTRL)]
    internal_primary_oscillator_enable,

    #[bit(18, RW, rro::GCR_CLKCTRL)]
    internal_secondary_oscillator_enable,

    #[bit(17, RW, rro::GCR_CLKCTRL)]
    external_rtc_oscillator_enable,

    #[bit(13, RO, rro::GCR_CLKCTRL)]
    sys_clock_source_ready,

    #[bit(9..=11, RW, rro::GCR_CLKCTRL)]
    sys_clock_source_select,

    #[bit(6..=8, RW, rro::GCR_CLKCTRL)]
    sys_clock_prescaler,

    #[bit(17, RW, rro::GCR_PM)]
    internal_baud_rate_oscillator_power_down,

    #[bit(16, RW, rro::GCR_PM)]
    internal_primary_oscillator_power_down,

    #[bit(15, RW, rro::GCR_PM)]
    internal_secondary_oscillator_power_down,

    #[bit(9, RW, rro::GCR_PM)]
    analog_input_comparator_wakeup_enable,

    #[bit(7, RW, rro::GCR_PM)]
    wake_up_timer_enable,

    #[bit(5, RW, rro::GCR_PM)]
    rtc_alarm_wakeup_enable,

    #[bit(4, RW, rro::GCR_PM)]
    gpio_wakeup_enable,

    #[bit(0..=3, RW, rro::GCR_PM)]
    operating_mode_select,

    #[bit(17, RW, rro::GCR_PCLKDIV)]
    cnn_peripheral_clock_select,

    #[bit(14..=16, RW, rro::GCR_PCLKDIV)]
    cnn_peripheral_clock_frequency_divider,

    #[bit(10..=13, RW,  rro::GCR_PCLKDIV)]
    adc_peripheral_clock_frequency_select,

    #[bit(29, RW, rro::GCR_PCLKDIS0)]
    pulse_train_clock_disable,

    #[bit(28, RW, rro::GCR_PCLKDIS0)]
    i2c1_clock_disable,

    #[bit(25, RW, rro::GCR_PCLKDIS0)]
    cnn_clock_disable,

    #[bit(23, RW, rro::GCR_PCLKDIS0)]
    adc_clock_disable,

    #[bit(18, RW, rro::GCR_PCLKDIS0)]
    timer3_clock_disable,

    #[bit(17, RW, rro::GCR_PCLKDIS0)]
    timer2_clock_disable,

    #[bit(16, RW, rro::GCR_PCLKDIS0)]
    timer1_clock_disable,

    #[bit(15, RW, rro::GCR_PCLKDIS0)]
    timer0_clock_disable,

    #[bit(13, RW, rro::GCR_PCLKDIS0)]
    i2c0_clock_disable,

    #[bit(10, RW, rro::GCR_PCLKDIS0)]
    uart1_clock_disable,

    #[bit(9, RW, rro::GCR_PCLKDIS0)]
    uart0_clock_disable,

    #[bit(6, RW, rro::GCR_PCLKDIS0)]
    spi1_clock_disable,

    #[bit(5, RW, rro::GCR_PCLKDIS0)]
    dma_clock_disable,

    #[bit(1, RW, rro::GCR_PCLKDIS0)]
    gpio1_port_and_pad_logic_clock_disable,

    #[bit(0, RW, rro::GCR_PCLKDIS0)]
    gpio0_port_and_pad_logic_clock_disable,

    #[bit(16, RW, rro::GCR_MEMCTRL)]
    sysram0_ecc_enable,

    #[bit(0..=2, RW, rro::GCR_MEMCTRL)]
    program_flash_wait_states,

    #[bit(6, RW1O, rro::GCR_MEMZ)]
    icc1_zeroization,

    #[bit(5, RW1O, rro::GCR_MEMZ)]
    icc0_zeroization,

    #[bit(4, RW1O, rro::GCR_MEMZ)]
    sysram0_ecc_zeroization,

    #[bit(3, RW1O, rro::GCR_MEMZ)]
    sysram3_zeroization,

    #[bit(2, RW1O, rro::GCR_MEMZ)]
    sysram2_zeroization,

    #[bit(1, RW1O, rro::GCR_MEMZ)]
    sysram1_zeroization,

    #[bit(0, RW1O, rro::GCR_MEMZ)]
    sysram0_zeroiztion,

    #[bit(0, RO, rro::GCR_SYSST)]
    arm_ice_lock_status,

    #[bit(31, RW1O, rro::GCR_RST1)]
    cpu1_riscv32_reset,

    #[bit(25, RW1O, rro::GCR_RST1)]
    single_inductor_multiple_output_block_reset,

    #[bit(24, RW1O, rro::GCR_RST1)]
    dynamic_voltage_scaling_controller_reset,

    #[bit(20, RW1O, rro::GCR_RST1)]
    i2c2_reset,

    #[bit(19, RW1O, rro::GCR_RST1)]
    audio_interface_reset,

    #[bit(16, RW1O, rro::GCR_RST1)]
    semaphore_block_reset,

    #[bit(11, RW1O, rro::GCR_RST1)]
    spi0_reset,

    #[bit(10, RW1O, rro::GCR_RST1)]
    aes_block_reset,

    #[bit(9, RW1O, rro::GCR_RST1)]
    crc_reset,

    #[bit(7, RW1O, rro::GCR_RST1)]
    one_wire_reset,

    #[bit(1, RW1O, rro::GCR_RST1)]
    pulse_train_reset,

    #[bit(0, RW1O, rro::GCR_RST1)]
    i2c1_reset,

    #[bit(31, RW, rro::GCR_PCLKDIS1)]
    cpu1_risv32_clock_disable,

    #[bit(27, RW, rro::GCR_PCLKDIS1)]
    watchdog_timer0_disable,

    #[bit(24, RW, rro::GCR_PCLKDIS1)]
    i2c2_clock_disable,

    #[bit(23, RW, rro::GCR_PCLKDIS1)]
    i2s_audio_interface_clock_disable,

    #[bit(16, RW, rro::GCR_PCLKDIS1)]
    spi0_clock_disable,

    #[bit(15, RW, rro::GCR_PCLKDIS1)]
    aes_block_clock_disable,

    #[bit(14, RW, rro::GCR_PCLKDIS1)]
    crc_clock_disable,

    #[bit(13, RW, rro::GCR_PCLKDIS1)]
    one_wire_clock_disable,

    #[bit(9, RW, rro::GCR_PCLKDIS1)]
    semaphore_block_clock_disable,

    #[bit(2, RW, rro::GCR_PCLKDIS1)]
    trng_clock_disable,

    #[bit(1, RW, rro::GCR_PCLKDIS1)]
    uart2_clock_disable,

    #[bit(2, RW, rro::GCR_EVENTEN)]
    cpu0_cm4_txev_event_enable,

    #[bit(0, RW, rro::GCR_EVENTEN)]
    cpu0_cm4_dma_ctz_wake_up_enable,

    #[bit(0..=15, RO, rro::GCR_REVISION)]
    device_revision,

    #[bit(0, RW, rro::GCR_SYSIE)]
    arm_ice_unlocked_interrupt_enable,

    #[bit(0, RW1C, rro::GCR_ECCERR)]
    sysram0_ecc_error,

    #[bit(0, RW1C, rro::GCR_ECCCED)]
    sysram0_correctable_ecc_error_detected,

    #[bit(0, RW, rro::GCR_ECCIE)]
    sysram0_ecc_error_interrupt_enable,

    #[bit(31, RO, rro::GCR_ECCADDR)]
    ecc_error_address_tag_ram_error,

    #[bit(30, RO, rro::GCR_ECCADDR)]
    ecc_error_address_tag_ram_error_bank,

    #[bit(16..=29, RO, rro::GCR_ECCADDR)]
    ecc_error_address_tag_ram_address,

    #[bit(15, RO, rro::GCR_ECCADDR)]
    ecc_error_address_cache_data_ram_error,

    #[bit(14, RO, rro::GCR_ECCADDR)]
    ecc_error_address_cache_data_ram_error_bank,

    #[bit(0..=13, RO, rro::GCR_ECCADDR)]
    ecc_error_address_cache_data_ram_error_address,

    #[bit(0..=31, RW, rro::GCR_GPR0)]
    general_purpose_register
}
