/// # Memory Mapped IO
/// The Peripheral space for memory mapped registers.
pub mod mmio {
    const GLOBAL_CONTROL: usize = 0x4000_0000;
    const SYSTEM_INTERFACE: usize = 0x4000_0400;
    const FUNCTION_CONTROL: usize = 0x4000_0800;
    const WATCHDOG_TIMER0: usize = 0x4000_3000;
    const DYNAMIC_VOLTAGE_SCALING: usize = 0x4000_3C00;
    const SIMO: usize = 0x4000_4400;
    const TRIM_SYSTEM_INITIALIZATION: usize = 0x4000_5400;
    const GENERAL_CONTROL_FUNCTION: usize = 0x4000_5800;
    const REAL_TIME_CLOCK: usize = 0x4000_6000;
    const WAKEUP_TIMER: usize = 0x4000_6400;
    /// ... TODO: FINISH THIS
    /// # I2C Port 0 (I2C0)
    /// The I2C port 0 memory space ptr.
    pub const I2C_PORT_0: usize = 0x4001_D000;
    pub const I2C_PORT_1: usize = 0x4001_E000;
    pub const I2C_PORT_2: usize = 0x4001_F000;
}
