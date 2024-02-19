/// # Relative Register Offsets
/// These are the offsets for the GPIO registers that the
/// Maxim Integrated - spec shows. Found on page 116.
mod rro {
    /// # Configuration Enable Bit 0 Register
    pub const GPIO_EN0GPIO: usize = 0x0000;
    /// # Configuration Enable Atomic Set Bit 0 Register
    pub const GPIO_EN0_SETGPIO: usize = 0x0004;
    /// # Configuration Enable Atomic Clear Bit 0 Register
    pub const GPIO_EN0_CLRGPIO: usize = 0x0008;
    /// # Output Enable Register
    pub const GPIO_OUTENGPIO: usize = 0x000C;
    /// # Output Enable Atomic Set Register
    pub const GPIO_OUTEN_SETGPIO: usize = 0x0010;
    /// # Output Enable Atomic Clear Register
    pub const GPIO_OUTEN_CLRGPIO: usize = 0x0014;
    /// # Output Register
    pub const GPIO_OUTGPIO: usize = 0x0018;
    /// # Output Atomic Set Register
    pub const GPIO_OUT_SETGPIO: usize = 0x001C;
    /// # Output Atomic Clear Register
    pub const GPIO_OUT_CLRGPIO: usize = 0x0020;
    /// # Input Register
    pub const GPIO_INGPIO: usize = 0x0024;
    /// # Interrupt Mode Register
    pub const GPIO_INTMODEGPIO: usize = 0x0028;
    /// # Interrupt Polarity Register
    pub const GPIO_INTPOLGPIO: usize = 0x002C;
    /// # Input Enable Register
    pub const GPIO_INENGPIO: usize = 0x0030;
    /// # Interrupt Enable Register
    pub const GPIO_INTENGPIO: usize = 0x0034;
    /// # Interrupt Enable Atomic Set Register
    pub const GPIO_INTEN_SETGPIO: usize = 0x0038;
    /// # Interrupt Enable Atomic Clear Register
    pub const GPIO_INTEN_CLRGPIO: usize = 0x003C;
    /// # Interrupt Status Register
    pub const GPIO_INTFLGPIO: usize = 0x0040;
    /// # Interrupt Clear Register
    pub const GPIO_INTFL_CLRGPIO: usize = 0x0048;
    /// # Wakeup Enable Register
    pub const GPIO_WKENGPIO: usize = 0x004C;
    /// # Wakeup Enable Atomic Set Register
    pub const GPIO_WKEN_SETGPIO: usize = 0x0050;
    /// # Wakeup Enable Atomic Clear Register
    pub const GPIO_WKEN_CLRGPIO: usize = 0x0054;
    /// # Interrupt Dual Edge Mode Register
    pub const GPIO_DUALEDGEGPIO: usize = 0x005C;
    /// # Pad Configuration 1 Register
    pub const GPIO_PADCTRL0GPIO: usize = 0x0060;
    /// # Pad Configuration 2 Register
    pub const GPIO_PADCTRL1GPIO: usize = 0x0064;
    /// # Configuration Enable Bit 1 Register
    pub const GPIO_EN1GPIO: usize = 0x0068;
    /// # Configuration Enable Atomic Set Bit 1 Register
    pub const GPIO_EN1_SETGPIO: usize = 0x006C;
    /// # Configuration Enable Atomic Clear Bit 1 Register
    pub const GPIO_EN1_CLRGPIO: usize = 0x0070;
    /// # Configuration Enable Bit 2 Register
    pub const GPIO_EN2GPIO: usize = 0x0074;
    /// # Configuration Enable Atomic Set Bit 2 Register
    pub const GPIO_EN2_SETGPIO: usize = 0x0078;
    /// # Configuration Enable Atomic Clear Bit 2 Register
    pub const GPIO_EN2_CLRGPIO: usize = 0x007C;
    /// # Hysteresis Enable Register
    pub const GPIO_HYSENGPIO: usize = 0x00A8;
    /// # Slew Rate Select Register
    pub const GPIO_SRSELGPIO: usize = 0x00AC;
    /// # Output Drive Strength Bit 0 Register
    pub const GPIO_DS0GPIO: usize = 0x00B0;
    /// # Output Drive Strength Bit 1 Register
    pub const GPIO_DS11GPIO: usize = 0x00B4;
}
