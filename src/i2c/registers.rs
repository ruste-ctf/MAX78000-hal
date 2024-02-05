use crate::const_assert;
use crate::memory_map::mmio;
use core::ptr;

/// # Realative Register Offsets
/// These are the offsets for the I2C registers that the
/// Maxim Integrated - spec shows. Found on page 224.
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

// FIXME: Should be a trait that gets impl on a struct. Please remember to refactor this out later.
macro_rules! reg_impl {
    (RW, $t:tt, $v:expr) => {
        impl<const PORT_PTR: usize> $t<PORT_PTR> {
            /// # Register Address Bits
            /// The raw usize address of this register.
            const REGISTER_ADDRESS_BITS: usize = PORT_PTR + $v;

            // We should only I2C_PORT_0, I2C_PORT_1, and I2C_PORT_2 into this struct.
            // It should not be possible to compile with any other port address.
            const_assert!(
                STRUCT,
                (PORT_PTR == mmio::I2C_PORT_0)
                    || (PORT_PTR == mmio::I2C_PORT_1)
                    || (PORT_PTR == mmio::I2C_PORT_2),
                "Should only except I2C_PORT_0, I2C_PORT_1, or I2C_PORT_2!"
            );

            /// # Get Ptr
            /// Get the raw ptr for which this address is stored. Only volatile
            /// accesses should be used to read/write to this ptr.
            pub const fn get_ptr() -> *mut u32 {
                Self::REGISTER_ADDRESS_BITS as *mut u32
            }

            /// # Read
            /// Get the value stored at this register with **1** volatile
            /// memory read.
            #[inline]
            pub fn read() -> u32 {
                unsafe { ptr::read_volatile(Self::get_ptr()) }
            }

            /// # Write
            /// Write to the value stored at this register with **1** volatile
            /// memory read.
            #[inline]
            pub unsafe fn write(value: u32) {
                unsafe { ptr::write_volatile(Self::get_ptr(), value) }
            }
        }
    };
    (RO, $t:tt, $v:expr) => {
        impl<const PORT_PTR: usize> $t<PORT_PTR> {
            /// # Register Address Bits
            /// The raw usize address of this register.
            const REGISTER_ADDRESS_BITS: usize = PORT_PTR + $v;

            // We should only I2C_PORT_0, I2C_PORT_1, and I2C_PORT_2 into this struct.
            // It should not be possible to compile with any other port address.
            const_assert!(
                STRUCT,
                (PORT_PTR == mmio::I2C_PORT_0)
                    || (PORT_PTR == mmio::I2C_PORT_1)
                    || (PORT_PTR == mmio::I2C_PORT_2),
                "Should only except I2C_PORT_0, I2C_PORT_1, or I2C_PORT_2!"
            );

            /// # Get Ptr
            /// Get the raw ptr for which this address is stored. Only volatile
            /// accesses should be used to read/write to this ptr.
            pub const fn get_ptr() -> *mut u32 {
                Self::REGISTER_ADDRESS_BITS as *mut u32
            }

            /// # Read
            /// Get the value stored at this register with **1** volatile
            /// memory read.
            #[inline]
            pub fn read() -> u32 {
                unsafe { ptr::read_volatile(Self::get_ptr()) }
            }
        }
    };
}

/// # I2C Control Register
/// The control register for I2C related tasks, page 224-226 (MAX78000 User Guide)
pub struct ControlRegister<const PORT_PTR: usize> {}
reg_impl!(RW, ControlRegister, rro::I2C_CTRL_OFFSET);

/// # I2C Status Register
/// The status register for I2C related tasks, page 226 (MAX78000 User Guide)
pub struct StatusRegister<const PORT_PTR: usize> {}
reg_impl!(RO, StatusRegister, rro::I2C_STATUS_OFFSET);

/// # I2C Interrupt Flag 0 Register
/// The interrupt flag 0 register for controlling interrupt flags for I2C related tasks, page 226-229 (MAX78000 User Guide)
pub struct InterruptFlag0<const PORT_PTR: usize> {}
reg_impl!(RW, InterruptFlag0, rro::I2C_INTFL0_OFFSET);

/// # I2C Interrupt Enable 0 Register
/// The interrupt enable 0 register for controlling if interrupts are enabled for I2C, page 229-230 (MAX78000 User Guide)
pub struct InterruptEnable0<const PORT_PTR: usize> {}
reg_impl!(RW, InterruptEnable0, rro::I2C_INTEN0_OFFSET);

/// # I2C Interrupt Flag 1 Register
/// The interrupt flag 1 register for controlling interrupt flags for I2C related tasks, page 230-231 (MAX78000 User Guide)
pub struct InterruptFlag1<const PORT_PTR: usize> {}
reg_impl!(RW, InterruptFlag1, rro::I2C_INTFL1_OFFSET);
