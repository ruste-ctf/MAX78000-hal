/// # Realative Register Offsets
/// These are the offsets for the I2C registers that the
/// Maxim Integrated - spec shows. Found on page 224.
#[allow(unused)]
mod rro {
    /// # I2C Control Register
    pub const I2C_CTRL_OFFSET: usize = 0x0000;
    /// # I2C Status Register
    pub const I2C_STATUS_OFFSET: usize = 0x0004;
    /// # I2C Interrupt Flags 0 Register
    pub const I2C_INTFL0_OFFSET: usize = 0x0008;
    /// # I2C Interrupt Enable 0 Register
    pub const I2C_INTEN0_OFFSET: usize = 0x000C;
    /// # I2C Interrupt Flags 1 Register
    pub const I2C_INTFL1_OFFSET: usize = 0x0010;
    /// # I2C Interrupt Enable 1 Register
    pub const I2C_INTEN1_OFFSET: usize = 0x0014;
    /// # I2C FIFO Length Register
    pub const I2C_FIFOLEN_OFFSET: usize = 0x0018;
    /// # I2C Receive Control 0 Register
    pub const I2C_RXCTRL0_OFFSET: usize = 0x001C;
    /// # I2C Receive Control 1 Register
    pub const I2C_RXCTRL1_OFFSET: usize = 0x0020;
    /// # I2C Transmit Control 0 Register
    pub const I2C_TXCTRL0_OFFSET: usize = 0x0024;
    /// # I2C Transmit Control 1 Register
    pub const I2C_TXCTRL1_OFFSET: usize = 0x0028;
    /// # I2C Transmit and Receive FIFO Register
    pub const I2C_FIFO_OFFSET: usize = 0x002C;
    /// # I2C Master Control Register
    pub const I2C_MSTCTRL_OFFSET: usize = 0x0030;
    /// # I2C Clock Low Time Register
    pub const I2C_CLKLO_OFFSET: usize = 0x0034;
    /// # I2C Clock High Time Register
    pub const I2C_CLKHI_OFFSET: usize = 0x0038;
    /// # I2C High Speed Clock Control Register
    pub const I2C_HSCLK_OFFSET: usize = 0x003C;
    /// # I2C Timeout Register
    pub const I2C_TIMEOUT_OFFSET: usize = 0x0040;
    /// # I2C DMA Register
    pub const I2C_DMA_OFFSET: usize = 0x0048;
    /// # I2C Slave Register
    pub const I2C_SLAVE_OFFSET: usize = 0x004C;
}
