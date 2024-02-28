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
/// Possible hardware devices on the MAX78000 Chip set. Use this enum
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

/// # Controller Reset
/// Preform a complete reset of the controller.
pub fn controller_reset() -> ! {
    ensure_gcr();
    unsafe {
        GLOBAL_CONTROL_REGISTER
            .as_mut()
            .unwrap()
            .activate_system_reset()
    };
    loop {}
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

/// # Peripheral Reset
/// Reset the given device to default settings and configuration.
pub fn peripheral_reset(device: HardwareSource) {
    ensure_gcr();

    let gcr = unsafe { GLOBAL_CONTROL_REGISTER.as_mut().unwrap() };
    unsafe {
        match device {
            HardwareSource::GPIO0 => gcr.activate_gpio0_reset(),
            HardwareSource::GPIO1 => gcr.activate_gpio1_reset(),
            HardwareSource::DMA => gcr.activate_dma_access_block_reset(),
            HardwareSource::SPI1 => gcr.activate_spi1_reset(),
            HardwareSource::UART0 => gcr.activate_uart0_reset(),
            HardwareSource::UART1 => gcr.activate_uart1_reset(),
            HardwareSource::I2C0 => gcr.activate_i2c0_reset(),
            HardwareSource::I2C2 => gcr.activate_i2c2_reset(),
            HardwareSource::TMR0 => gcr.activate_tiemr0_reset(),
            HardwareSource::TMR1 => gcr.activate_timer1_reset(),
            HardwareSource::TMR2 => gcr.activate_timer2_reset(),
            HardwareSource::TMR3 => gcr.activate_timer3_reset(),
            HardwareSource::ADC => gcr.activate_adc_reset(),
            HardwareSource::CNN => gcr.activate_cnn_reset(),
            HardwareSource::I2C1 => gcr.activate_i2c1_reset(),
            HardwareSource::PT => gcr.activate_pulse_train_reset(),
            HardwareSource::UART2 => gcr.activate_uart2_reset(),
            HardwareSource::TRNG => gcr.activate_trng_reset(),
            HardwareSource::SMPHR => gcr.activate_semaphore_block_reset(),
            HardwareSource::OWIRE => gcr.activate_one_wire_reset(),
            HardwareSource::CRC => gcr.activate_crc_reset(),
            HardwareSource::AES => gcr.activate_aes_block_reset(),
            HardwareSource::I2S => gcr.activate_audio_interface_reset(),
            HardwareSource::SPI0 => gcr.activate_spi0_reset(),
            HardwareSource::WDT0 => gcr.activate_watchdog_timer0_reset(),
            HardwareSource::CPU1 => gcr.activate_cpu1_riscv32_reset(),
            HardwareSource::WDT1 => gcr.activate_watchdog_timer0_reset(),
            HardwareSource::LPCOMP => gcr.activate_adc_reset(),
        }
    }

    // Wait until reset is complete
    while gcr.get_reset_status0() | gcr.get_reset_status1() != 0 {}
}
