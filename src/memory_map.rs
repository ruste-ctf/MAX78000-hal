/// # Memory Mapped IO
/// The Peripheral space for memory mapped registers.
/// Found at: MAX78000 Pg 31 & 37
pub mod mmio {

    /// # Global Control (GCR)
    /// The Global Control Registers ptr.
    pub const GLOBAL_CONTROL: usize = 0x4000_0000;
    /// # System Interface (SIR)
    /// The System Initialization Registers ptr.
    pub const SYSTEM_INTERFACE: usize = 0x4000_0400;
    /// # Function Control (FCR)
    /// The Function Control Registers ptr.
    pub const FUNCTION_CONTROL: usize = 0x4000_0800;
    /// # Watchdog Timer 0 (WDT0)
    /// The Watchdog Timer 0 ptr.
    pub const WATCHDOG_TIMER0: usize = 0x4000_3000;
    /// # Dynamic Voltage Scaling (DVS)
    /// The Dynamic Voltage Scaling Controller ptr.
    pub const DYNAMIC_VOLTAGE_SCALING: usize = 0x4000_3C00;
    /// # SIMO (SIMO)
    /// The Single Input Multiple Output ptr.
    pub const SIMO: usize = 0x4000_4400;
    /// # Trim System Initialization (TRIMS IR)
    /// The Trim System Initialization ptr.
    pub const TRIM_SYSTEM_INITIALIZATION: usize = 0x4000_5400;
    /// # General Control Function (GCFR)
    /// The General Control Function ptr.
    pub const GENERAL_CONTROL_FUNCTION: usize = 0x4000_5800;
    /// # Real-Time Clock (RTC)
    /// The Real-Time Clock ptr.
    pub const REAL_TIME_CLOCK: usize = 0x4000_6000;
    /// # Wakeup Timer (WUT)
    /// The Wakeup Timer ptr.
    pub const WAKEUP_TIMER: usize = 0x4000_6400;
    /// # Power Sequencer (PWRSEQ)
    /// The Power Sequencer ptr.
    pub const POWER_SEQUENCER: usize = 0x4000_6800;
    /// # Miscellaneous Control (MCR)
    /// The Miscellaneous Control Register ptr.
    pub const MISCELLANEOUS_CONTROL: usize = 0x4000_6c00;
    /// # AES (AES)
    /// The AES ptr.
    pub const AES: usize = 0x4000_7400;
    /// # AES KEYS (AESKEY)
    /// The AES Key ptr.
    pub const AES_KEYS: usize = 0x4000_7800;
    /// # GPIO Port 0 (GPIO0)
    /// The GPIO Port 0 ptr.
    pub const GPIO_PORT_0: usize = 0x4000_8000;
    /// # GPIO Port 1 (GPIO1)
    /// The GPIO Port 1 ptr.
    pub const GPIO_PORT_1: usize = 0x4000_9000;
    /// # Parallel Camera Interface (PCIF)
    /// The Parallel Camera Interface ptr.
    pub const PARALLEL_CAMERA_INTERFACE: usize = 0x4000_e000;
    /// # CRC (CRC)
    /// The CRC ptr.
    pub const CRC: usize = 0x4000_f000;
    /// # Timer 0 (TMR0)
    /// The Timer 0 ptr.
    pub const TIMER_0: usize = 0x4001_0000;
    /// # Timer 1 (TMR1)
    /// The Timer 1 ptr.
    pub const TIMER_1: usize = 0x4001_1000;
    /// # Timer 2 (TMR2)
    /// The Timer 2 ptr.
    pub const TIMER_2: usize = 0x4001_2000;
    /// # Timer 3 (TMR3)
    /// The Timer 3 ptr.
    pub const TIMER_3: usize = 0x4001_3000;
    /// # I2C Port 0 (I2C0)
    /// The I2C port 0 memory space ptr.
    pub const I2C_PORT_0: usize = 0x4001_D000;
    /// # I2C Port 1 (I2C1)
    /// The I2C port 1 memory space ptr.
    pub const I2C_PORT_1: usize = 0x4001_E000;
    /// # I2C Port 2 (I2C2)
    /// The I2C port 2 memory space ptr.
    pub const I2C_PORT_2: usize = 0x4001_F000;
    /// # Standard DMA (DMA)
    /// The Standard DMA ptr.
    pub const STANDARD_DMA: usize = 0x4002_8000;
    /// # Flash Controller 0 (FLC0)
    /// The Flash Controller 0 ptr.
    pub const FLASH_CONTROLLER_0: usize = 0x4002_9000;
    /// # ICC 0 CM4 (ICC0)
    /// The Instruction Cache Controller 0 (CM4) ptr.
    pub const ICC_0_CM4: usize = 0x4002_a000;
    /// # ICC 1 RV32 (ICC1)
    /// The Instruction Cache Controller 1 (RV32) ptr.
    pub const ICC_1_RV32: usize = 0x4002_a800;
    /// # ADC (ADC)
    /// The ADC ptr.
    pub const ADC: usize = 0x4003_4000;
    /// # Pulse Train Engine (PT)
    /// The Pulse Train Engine ptr.
    pub const PULSE_TRAIN_ENGINE: usize = 0x4003_c000;
    /// # 1-Wire Master (OWM)
    /// The 1-Wire Master ptr.
    pub const ONE_WIRE_MASTER: usize = 0x4003_d000;
    /// # Semaphore (SEMA)
    /// The Semaphore ptr.
    pub const SEMAPHORE: usize = 0x4003_e000;
    /// # UART 0 (UART0)
    /// The UART 0 ptr.
    pub const UART_0: usize = 0x4004_2000;
    /// # UART 1 (UART1)
    /// The UART 1 ptr.
    pub const UART_1: usize = 0x4004_3000;
    /// # UART 2 (UART2)
    /// The UART 2 ptr.
    pub const UART_2: usize = 0x4004_4000;
    /// # SPI 1 (SPI1)
    /// The SPI 1 ptr.
    pub const SPI_1: usize = 0x4004_6000;
    /// # TRNG (TRNG)
    /// The TRNG Engine ptr.
    pub const TRNG: usize = 0x4004_d000;
    /// # I2S (I2S)
    /// The I2S ptr.
    pub const I2S: usize = 0x4006_0000;
    /// # Low Power Control (LPCGR)
    /// The Low Power General Control Register ptr.
    pub const LOW_POWER_CONTROL: usize = 0x4008_0000;
    /// # GPIO Port 2 (GPIO2)
    /// The GPIO Port 2 ptr.
    pub const GPIO_PORT_2: usize = 0x4008_0400;
    /// # Low Power Watchdog Timer 0 (WDT1)
    /// The Low Power Watchdog Timer 0 ptr.
    pub const LOW_POWER_WATCHDOG_TIMER_0: usize = 0x4008_0800;
    /// # Low Power Timer 0 (TRM4)
    /// The Low Power Timer 4 ptr.
    pub const LOW_POWER_TIMER_0: usize = 0x4008_0c00;
    /// # Low Power Timer 1 (TRM5)
    /// The Low Power Timer 5 ptr.
    pub const LOW_POWER_TIMER_1: usize = 0x4008_1000;
    /// # Low Power UART 0 (UART 3)
    /// The Low Power UART 0 ptr.
    pub const LOW_POWER_UART_0: usize = 0x4008_1400;
    /// # Low Power Comparators (LPCMP)
    /// The Low Power `Comparator` ptr.
    pub const LOW_POWER_COMPARATORS: usize = 0x4008_8000;
    /// # SPI 0 (SPI0)
    /// The SPI 0 ptr.
    pub const SPI_0: usize = 0x400b_e000;
    /// # CNN Global Control (CNN)
    /// The CNN Global Control ptr.
    pub const CNN_GLOBAL_CONTROL: usize = 0x5000_0000;
    /// # CNNx16 Quadrant 0 (CNNx16_0)
    /// The CNNx16 Quadrant 0 ptr.
    pub const CNNX16_QUADRANT_0: usize = 0x5010_0000;
    /// # CNNx16 Quadrant 1 (CNNx16_1)
    /// The CNNx16 Quadrant 11 ptr.
    pub const CNNX16_QUADRANT_1: usize = 0x5050_0000;
    /// # CNNx16 Quadrant 2 (CNNx16_2)
    /// The CNNx16 Quadrant 2 ptr.
    pub const CNNX16_QUADRANT_2: usize = 0x5090_0000;
    /// # CNNx16 Quadrant 3 (CNNx16_3)
    /// The CNNx16 Quadrant 3 ptr.
    pub const CNNX16_QUADRANT_3: usize = 0x50d0_0000;
}
