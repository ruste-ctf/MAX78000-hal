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

/// # I2C Interrupt Enable 1 Register
/// The interrupt enable 1 register for controlling if interrupts are enabled for I2C, page 231 (MAX78000 User Guide)
pub struct InterruptEnable1<const PORT_PTR: usize> {}
reg_impl!(RW, InterruptEnable1, rro::I2C_INTEN1_OFFSET);

/// # I2C FIFO Length Register
/// The FIFO length status register. Used to see the length of the transmit/receive FIFO, page 231 (MAX78000 User Guide)
pub struct FIFOLengthRegister<const PORT_PTR: usize> {}
reg_impl!(RO, FIFOLengthRegister, rro::I2C_FIFOLEN_OFFSET);

/// # I2C Receive Control 0 Register
/// The Receive control register is used to set the receive FIFO threshold level, and set flush receive FIFO, page 231-232 (MAX78000 User Guide)
pub struct ReceiveControl0<const PORT_PTR: usize> {}
reg_impl!(RW, ReceiveControl0, rro::I2C_RXCTRL0_OFFSET);

/// # I2C Receive Control 1 Register
/// The receive control register is used to set receive FIFO byte count configuration, and read byte cound, page 232-233 (MAX78000 User Guide)
pub struct ReceiveControl1<const PORT_PTR: usize> {}
reg_impl!(RW, ReceiveControl1, rro::I2C_RXCTRL1_OFFSET);

/// # I2C Transmit Control 0 Register
/// The transmit control register is used to control transmitting related I2C tasks, page 233-234 (MAX78000 User Guide)
pub struct TransmitControl0<const PORT_PTR: usize> {}
reg_impl!(RW, TransmitControl0, rro::I2C_TXCTRL0_OFFSET);

/// # I2C Transmit Control 1 Register
/// The trasmit control register is used to control transmitting related I2C tasks, page 234-235 (MAX78000 User Guide)
pub struct TransmitControl1<const PORT_PTR: usize> {}
reg_impl!(RW, TransmitControl1, rro::I2C_TXCTRL1_OFFSET);

/// # I2C Data Register
/// The data register is used to send and receive data to the FIFO, page 235 (MAX78000 User Guide)
pub struct DataRegister<const PORT_PTR: usize> {}
reg_impl!(RW, DataRegister, rro::I2C_FIFO_OFFSET);

/// # I2C Master Control Register
/// The master control register is used to control the bus when the device is configured to be the master, page 235-236 (MAX78000 User Guide)
pub struct MasterControl<const PORT_PTR: usize> {}
reg_impl!(RW, MasterControl, rro::I2C_MSTCTRL_OFFSET);

/// # I2C SCL Low Control Register
/// The SCL low control register is used to control the clock low time of the bus, page 236 (MAX78000 User Guide)
pub struct LowSCLControl<const PORT_PTR: usize> {}
reg_impl!(RW, LowSCLControl, rro::I2C_CLKLO_OFFSET);

/// # I2C SCL High Control Register
/// The SCL high control register is used to control the clock high time of the bus, page 236 (MAX78000 User Guide)
pub struct HighSCLControl<const PORT_PTR: usize> {}
reg_impl!(RW, HighSCLControl, rro::I2C_CLKHI_OFFSET);

/// # I2C High Speed Clock Control Register
/// The high speed clock control register is used to control the high speed clock rate, page 236-237 (MAX78000 User Guide)
pub struct HighSpeedClockControl<const PORT_PTR: usize> {}
reg_impl!(RW, HighSpeedClockControl, rro::I2C_HSCLK_OFFSET);

/// # I2C Timeout Register
/// The timeout register is used to control the bus error scl timeout period, page 237 (MAX78000 User Guide)
pub struct TimeoutControl<const PORT_PTR: usize> {}
reg_impl!(RW, TimeoutControl, rro::I2C_TIMEOUT_OFFSET);

/// # I2C Slave Address Register
/// The slave address register is used to control the addressing mode of the bus, page 237-238 (MAX78000 User Guide)
pub struct SlaveAddress<const PORT_PTR: usize> {}
reg_impl!(RW, SlaveAddress, rro::I2C_SLAVE_OFFSET);
