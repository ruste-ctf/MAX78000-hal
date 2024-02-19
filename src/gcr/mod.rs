use crate::memory_map::mmio;

pub mod registers;

static mut GLOBAL_CONTROL_REGISTER: Option<registers::Registers> = None;

/// # Setup Global Control Register
/// Init the global control register, allows you to call other functions on the global
/// control register.
#[inline(always)]
pub fn init_global_control_register() {
    if unsafe { GLOBAL_CONTROL_REGISTER.is_none() } {
        unsafe { GLOBAL_CONTROL_REGISTER = Some(registers::Registers::new(mmio::GLOBAL_CONTROL)) };
    }
}

#[inline(always)]
fn ensure_gcr() {
    init_global_control_register();
}

/// # Hardware Source
/// Possible hardware devices on the MAX78000 Chipset. Use this enum
/// to select which hardware device to use when enabling/disabling clock
/// or other hardware features.
pub enum HardwareSource {
    GPIO0,
    GPIO1,
    DMA,
    SPI1,
    UART0,
    UART1,
    I2C0,
    TMR0,
    TMR1,
    TMR2,
    TMR3,
    ADC,
    CNN,
    I2C1,
    PT,
    UART2,
    TRNG,
    SMPHR,
    OWIRE,
    CRC,
    AES,
    I2S,
    SPI0,
    I2C2,
    WDT0,
    CPU1,
    WDT1,
    LPCOMP,
}

/// # System Clock Enable
/// Enable/Disable a `HardwareSource`'s clock.
pub fn system_clock_enable(clock: HardwareSource, enable: bool) {
    ensure_gcr();

    let gcr = unsafe { GLOBAL_CONTROL_REGISTER.as_mut().unwrap() };
    unsafe {
        match clock {
            HardwareSource::GPIO0 => gcr.set_gpio0_port_and_pad_logic_clock_disable(!enable),
            HardwareSource::GPIO1 => gcr.set_gpio1_port_and_pad_logic_clock_disable(!enable),
            HardwareSource::DMA => gcr.set_dma_clock_disable(!enable),
            HardwareSource::SPI1 => gcr.set_spi1_clock_disable(!enable),
            HardwareSource::UART0 => gcr.set_uart0_clock_disable(!enable),
            HardwareSource::UART1 => gcr.set_uart1_clock_disable(!enable),
            HardwareSource::I2C0 => gcr.set_i2c0_clock_disable(!enable),
            HardwareSource::I2C2 => gcr.set_i2c2_clock_disable(!enable),
            HardwareSource::TMR0 => gcr.set_timer0_clock_disable(!enable),
            HardwareSource::TMR1 => gcr.set_timer1_clock_disable(!enable),
            HardwareSource::TMR2 => gcr.set_timer2_clock_disable(!enable),
            HardwareSource::TMR3 => gcr.set_timer3_clock_disable(!enable),
            HardwareSource::ADC => gcr.set_adc_clock_disable(!enable),
            HardwareSource::CNN => gcr.set_cnn_clock_disable(!enable),
            HardwareSource::I2C1 => gcr.set_i2c1_clock_disable(!enable),
            HardwareSource::PT => gcr.set_pulse_train_clock_disable(!enable),
            HardwareSource::UART2 => gcr.set_uart2_clock_disable(!enable),
            HardwareSource::TRNG => gcr.set_trng_clock_disable(!enable),
            HardwareSource::SMPHR => gcr.set_semaphore_block_clock_disable(!enable),
            HardwareSource::OWIRE => gcr.set_one_wire_clock_disable(!enable),
            HardwareSource::CRC => gcr.set_crc_clock_disable(!enable),
            HardwareSource::AES => gcr.set_aes_block_clock_disable(!enable),
            HardwareSource::I2S => gcr.set_i2s_audio_interface_clock_disable(!enable),
            HardwareSource::SPI0 => gcr.set_spi0_clock_disable(!enable),
            HardwareSource::WDT0 => gcr.set_watchdog_timer0_disable(!enable),
            HardwareSource::CPU1 => gcr.set_cpu1_risv32_clock_disable(!enable),
            HardwareSource::WDT1 => gcr.set_watchdog_timer0_disable(!enable),
            HardwareSource::LPCOMP => gcr.set_adc_clock_disable(!enable),
        }
    }
}
