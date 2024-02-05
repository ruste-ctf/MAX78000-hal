/// # Memory Mapped IO
/// The Peripheral space for memory mapped registers.
/// Found at: MAX78000 Pg 31
pub mod mmio {

    /// # Global Control (GCR)
    /// The Global Control Registers ptr.
    pub const GLOBAL_CONTROL: usize = 0x4000_0000;
    /// # System Interface (SIR)
    /// The System Initialization Registers ptr.
    pub const SYSTEM_INTERFACE: usize = 0x4000_0400;
    /// # Function Control (FCR)
    /// The Function Control Registers prt.
    pub const FUNCTION_CONTROL: usize = 0x4000_0800;
    /// # Watchdog Timer 0 (WDT0)
    /// The Watchgog Timer 0 ptr.
    pub const WATCHDOG_TIMER0: usize = 0x4000_3000;
    pub const DYNAMIC_VOLTAGE_SCALING: usize = 0x4000_3C00;
    pub const SIMO: usize = 0x4000_4400;
    pub const TRIM_SYSTEM_INITIALIZATION: usize = 0x4000_5400;
    pub const GENERAL_CONTROL_FUNCTION: usize = 0x4000_5800;
    pub const REAL_TIME_CLOCK: usize = 0x4000_6000;
    pub const WAKEUP_TIMER: usize = 0x4000_6400;
    pub const POWER_SEQUENCER: usize = 0x4000_6800;
    pub const MISCELLANEOUS_CONTROL: usize = 0x4000_6c00;
    pub const AES: usize = 0x4000_7400;
    pub const AES_KEYS: usize = 0x4000_7800;
    pub const GPIO_PORT_0: usize = 0x4000_8000;
    pub const GPIO_PORT_1: usize = 0x4000_9000;
    pub const PARALLEL_CAMERA_INTERFACE: usize = 0x4000_e000;
    pub const CRC: usize = 0x4000_f000;
    pub const TIMER_0: usize = 0x4001_0000;
    pub const TIMER_1: usize = 0x4001_1000;
    pub const TIMER_2: usize = 0x4001_2000;
    pub const TIMER_3: usize = 0x4001_3000;
    /// ... TODO: FINISH THIS
    /// # I2C Port 0 (I2C0)
    /// The I2C port 0 memory space ptr.
    pub const I2C_PORT_0: usize = 0x4001_D000;
    pub const I2C_PORT_1: usize = 0x4001_E000;
    pub const I2C_PORT_2: usize = 0x4001_F000;
    pub const STANDARD_DMA: usize = 0x4002_8000;
    pub const FLASH_CONTROLLER_0: usize = 0x4002_9000;
    pub const ICC_0_CM4: usize = 0x4002_a000;
    pub const ICC_1_RV32: usize = 0x4002_a800;
    pub const ADC: usize = 0x4003_4000;
    pub const PULSE_TRAIN_ENGINE: usize = 0x4003_c000;
    pub const ONE_WIRE_MASTER: usize = 0x4003_d000;
    pub const SEMAPHORE: usize = 0x4003_e000;
    pub const UART_0: usize = 0x4004_2000;
    pub const UART_1: usize = 0x4004_3000;
    pub const UART_2: usize = 0x4004_4000;
    pub const SPI_1: usize = 0x4004_6000;
    pub const TRNG: usize = 0x4004_d000;
    pub const I2S: usize = 0x4006_0000;
    pub const LOW_POWER_CONTROL: usize = 0x4008_0000;
    pub const GPIO_PORT_2: usize = 0x4008_0400;
    pub const LOW_POWER_WATCHDOG_TIMER_0: usize = 0x4008_0800;
    pub const LOW_POWER_TIMER_0: usize = 0x4008_0c00;
    pub const LOW_POWER_TIMER_1: usize = 0x4008_1000;
    pub const LOW_POWER_UART_0: usize = 0x4008_1400;
    pub const LOW_POWER_COMPARATORS: usize = 0x4008_8000;
    pub const SPI_0: usize = 0x400b_e000;
    pub const CNN_GLOBAL_CONTROL: usize = 0x5000_0000;
    pub const CNNX16_QUADRANT_0: usize = 0x5010_0000;
    pub const CNNX16_QUADRANT_1: usize = 0x5050_0000;
    pub const CNNX16_QUADRANT_2: usize = 0x5090_0000;
    pub const CNNX16_QUADRANT_3: usize = 0x50d0_0000;
}
