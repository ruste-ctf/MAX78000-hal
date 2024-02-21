use crate::memory_map::mmio;

pub(super) type BaseOffset = usize;
pub(super) type PortOffset = usize;

pub(super) const GPIO_0: PortOffset = mmio::GPIO_PORT_0;
pub(super) const GPIO_1: PortOffset = mmio::GPIO_PORT_1;
pub(super) const GPIO_2: PortOffset = mmio::GPIO_PORT_2;

/// # Relative Register Offsets
/// These are the offsets for the GPIO registers that the
/// Maxim Integrated - spec shows. Found on page 116.
#[allow(unused)]
pub(super) mod rro {
    use super::BaseOffset;

    /// # Configuration Enable Bit 0 Register
    pub const GPIO_EN0: BaseOffset = 0x0000;
    /// # Configuration Enable Atomic Set Bit 0 Register
    pub const GPIO_EN0_SET: BaseOffset = 0x0004;
    /// # Configuration Enable Atomic Clear Bit 0 Register
    pub const GPIO_EN0_CLR: BaseOffset = 0x0008;
    /// # Output Enable Register
    pub const GPIO_OUTEN: BaseOffset = 0x000C;
    /// # Output Enable Atomic Set Register
    pub const GPIO_OUTEN_SET: BaseOffset = 0x0010;
    /// # Output Enable Atomic Clear Register
    pub const GPIO_OUTEN_CLR: BaseOffset = 0x0014;
    /// # Output Register
    pub const GPIO_OUT: BaseOffset = 0x0018;
    /// # Output Atomic Set Register
    pub const GPIO_OUT_SET: BaseOffset = 0x001C;
    /// # Output Atomic Clear Register
    pub const GPIO_OUT_CLR: BaseOffset = 0x0020;
    /// # Input Register
    pub const GPIO_IN: BaseOffset = 0x0024;
    /// # Interrupt Mode Register
    pub const GPIO_INTMODE: BaseOffset = 0x0028;
    /// # Interrupt Polarity Register
    pub const GPIO_INTPOL: BaseOffset = 0x002C;
    /// # Input Enable Register
    pub const GPIO_INEN: BaseOffset = 0x0030;
    /// # Interrupt Enable Register
    pub const GPIO_INTEN: BaseOffset = 0x0034;
    /// # Interrupt Enable Atomic Set Register
    pub const GPIO_INTEN_SET: BaseOffset = 0x0038;
    /// # Interrupt Enable Atomic Clear Register
    pub const GPIO_INTEN_CLR: BaseOffset = 0x003C;
    /// # Interrupt Status Register
    pub const GPIO_INTFL: BaseOffset = 0x0040;
    /// # Interrupt Clear Register
    pub const GPIO_INTFL_CLR: BaseOffset = 0x0048;
    /// # Wakeup Enable Register
    pub const GPIO_WKEN: BaseOffset = 0x004C;
    /// # Wakeup Enable Atomic Set Register
    pub const GPIO_WKEN_SET: BaseOffset = 0x0050;
    /// # Wakeup Enable Atomic Clear Register
    pub const GPIO_WKEN_CLR: BaseOffset = 0x0054;
    /// # Interrupt Dual Edge Mode Register
    pub const GPIO_DUALEDGE: BaseOffset = 0x005C;
    /// # Pad Configuration 1 Register
    pub const GPIO_PADCTRL0: BaseOffset = 0x0060;
    /// # Pad Configuration 2 Register
    pub const GPIO_PADCTRL1: BaseOffset = 0x0064;
    /// # Configuration Enable Bit 1 Register
    pub const GPIO_EN1: BaseOffset = 0x0068;
    /// # Configuration Enable Atomic Set Bit 1 Register
    pub const GPIO_EN1_SET: BaseOffset = 0x006C;
    /// # Configuration Enable Atomic Clear Bit 1 Register
    pub const GPIO_EN1_CLR: BaseOffset = 0x0070;
    /// # Configuration Enable Bit 2 Register
    pub const GPIO_EN2: BaseOffset = 0x0074;
    /// # Configuration Enable Atomic Set Bit 2 Register
    pub const GPIO_EN2_SET: BaseOffset = 0x0078;
    /// # Configuration Enable Atomic Clear Bit 2 Register
    pub const GPIO_EN2_CLR: BaseOffset = 0x007C;
    /// # Hysteresis Enable Register
    pub const GPIO_HYSEN: BaseOffset = 0x00A8;
    /// # Slew Rate Select Register
    pub const GPIO_SRSEL: BaseOffset = 0x00AC;
    /// # Output Drive Strength Bit 0 Register
    pub const GPIO_DS0: BaseOffset = 0x00B0;
    /// # Output Drive Strength Bit 1 Register
    pub const GPIO_DS1: BaseOffset = 0x00B4;
    /// # Pull down/Pull up Strength Select Register
    pub const GPIO_PS: BaseOffset = 0x00B8;
    /// # Voltage Select Register
    pub const GPIO_VSSEL: BaseOffset = 0x00C0;
}

/// # Write GPIO
/// Write to a GPIO register and port.
pub(crate) unsafe fn write_gpio(base: BaseOffset, port: PortOffset, value: u32) {
    let ptr = (base + port) as *mut u32;

    core::ptr::write_volatile(ptr, value);
}

/// # Read GPIO
/// Read from the GPIO register and port.
pub(crate) unsafe fn read_gpio(base: BaseOffset, port: PortOffset) -> u32 {
    let ptr = (base + port) as *const u32;

    core::ptr::read_volatile(ptr)
}

/// # Enable Bit
/// Enable the bit for the given GPIO port and register.
pub(crate) unsafe fn enable_bit(base: BaseOffset, port: PortOffset, bit: usize) {
    let read = read_gpio(base, port);
    let bit = 1 << bit;
    write_gpio(base, port, bit | read);
}

/// # Disable Bit
/// Disable the bit for the given GPIO port and register.
pub(crate) unsafe fn disable_bit(base: BaseOffset, port: PortOffset, bit: usize) {
    let read = read_gpio(base, port);
    let bit = 1 << bit;
    write_gpio(base, port, read & (!bit));
}
