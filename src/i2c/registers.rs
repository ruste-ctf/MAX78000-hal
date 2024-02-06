use crate::bits::BitManipulation;
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

/// # Bit Impl
/// A macro to help with implementing large number of single bit operations on registers.
///
/// ## How to use
/// ```text
/// bit_impl!{5, RW, set_my_register, get_my_register}
///           ^
///       Bit to use
/// ```
macro_rules! bit_impl {
    ($bit:literal, RW, $(#[$meta_set:meta])* $set:ident, $(#[$meta_get:meta])* $get:ident) => {
        bit_impl!($bit, WO, $(#[$meta_set])* $set);
        bit_impl!($bit, RO, $(#[$meta_get])* $get);
    };
    ($bit:literal, RO, $(#[$meta_get:meta])* $get:ident) => {
        $(#[$meta_get])*
        ///
        /// # Saftey
        /// It is ultimately up to the caller to ensure this function will
        /// never cause any side effects. However, useally reading from
        /// registers does not modify any processor state (just looks at it).
        ///
        /// # Volitle
        /// This function only preforms **1** volitle *read* and immediatly copies
        /// the value to test the flag and return the result.
        ///
        #[inline]
        pub fn $get() -> bool {
            Self::read().get_bit($bit)
        }
    };
    ($bit:literal, WO, $(#[$meta_set:meta])* $set:ident) => {
        $(#[$meta_set])*
        ///
        /// # Saftey
        /// It is up to the caller to verify that this register write will not
        /// cause any side effects. There could be an event that setting this
        /// register could cause undefined behavior elsewhere in the program.
        ///
        /// ## Other Register State
        /// In some examples it is true that ones register state depends on another
        /// register's status. In these cases, it is up to the caller to properly
        /// set this register to a valid (and ONLY valid value).
        ///
        /// # Volitle
        /// This function only preforms **1** volitle *read* using `Self::read()`,
        /// immediately modifies the flag and does **1** volitle *write* using
        /// the interal provided function `Self::write(value)`.
        #[inline]
        pub unsafe fn $set(flag: bool) {
            let mut value = Self::read();
            value.set_bit($bit, flag);
            Self::write(value);
        }
    }

}

/// # I2C Control Register
/// The control register for I2C related tasks, page 224-226 (MAX78000 User Guide)
pub struct ControlRegister<const PORT_PTR: usize> {}
reg_impl!(RW, ControlRegister, rro::I2C_CTRL_OFFSET);

impl<const PORT_PTR: usize> ControlRegister<PORT_PTR> {
    bit_impl! {15, RW,
    /// # Set High Speed Mode
    /// Set I2C to high speed mode, or set it to low speed mode.
    /// 0: Disabled
    /// 1: Enabled
    set_high_speed_mode,
    /// # Is High Speed Mode Enabled
    /// Check if I2C is set to high speed mode, or if its set to low speed mode.
    /// 0: Disabled
    /// 1: Enabled
    is_high_speed_mode_enabled}

    bit_impl! {13, RW,
    /// # Set One Master Mode
    /// Set if the controller is going to be using one master mode. When set
    /// to true, the device must only be used with slave devices. No other
    /// masters should be attached to the bus. When using one master mode,
    /// it must also be true that no slave devices will hold SCL low for
    /// any given reason (i.e clock streaching).
    ///
    /// 0: Disabled
    /// 1: Enabled
    set_one_master_mode,
    /// # Is One Master Mode Enabled
    /// Check to see if the device is in single master mode. When in single
    /// device master mode, there must be only one master on the bus.
    ///
    /// 0: Disabled
    /// 1: Enabled
    is_one_master_mode_enabled}

    bit_impl! {12, RW,
    /// # Set Disable Slave Clock Stretching
    /// Sets if slave clock stretching will be disabled. In this mode, it must
    /// also be true that `one_master_mode` must also be set since slave devices
    /// will be pulling SCL low.
    ///
    /// 0: Enabled
    /// 1: Disabled
    set_disable_slave_clock_stretching,
    /// # Is Slave Clock Stretching Disabled
    /// Check to see if the device is currently disabling slave devices from using
    /// clock stretching on the bus. If this mode is active, it must also be true
    /// that `one_master_mode` must also be active.
    ///
    /// 0: Enabled
    /// 1: Disabled
    is_slave_clock_stretching_disabled}

    bit_impl! {11, RO,
    /// # Read Write Bit Status
    /// Get the logic level of the R/W bit on a received address match.
    ///
    // ## Extra Flags
    // FIXME: Include Extra flags (e.g `I2Cn_INTFL0.addr_match`) and what they do
    read_write_bit_status}

    bit_impl! {10, RW,
    /// # Set Software I2C Mode
    /// Tell the controller to either use software mode (i.e the SCL and SDA are managed
    /// by the software) or to use the on-board I2C controller hardware. This does not mean
    /// that the on-board I2C controller will do all communication by itself, more, it means
    /// the onboard hardware will generate basic I2C based signals (provided you tell it to).
    ///
    /// 0: The I2C controller will manage I2C in hardware.
    /// 1: SDA and SCL will need to be "bit-banged" by software by setting them manually.
    set_software_i2c_mode,
    /// # Is Software I2C Mode Enabled
    /// Checks if the hardware I2C controller will be managing the SCL and SDA pins.
    is_software_i2c_mode_enabled}

    bit_impl! {9, RO,
    /// # Get SDA Pin
    /// Get the `SDA` pin status, whether it be high or low.
    ///
    /// 0: The `SDA` pin is logic low
    /// 1: The `SDA` pin is logic high
    get_sda_pin}

    bit_impl! {8, RO,
    /// # Get SCL Pin
    /// Get the `SCL` pin status, whether it be high or low.
    ///
    /// 0: The `SCL` pin is logic low
    /// 1: The `SCL` pin is logic high
    get_scl_pin}

    bit_impl! {7, RW,
    /// # Set SDA Hardware Pin Released
    /// Set the state of the SDA hardware pin. (Activly pull the pin low, or leave it floating). This
    /// mode is only active during `software_i2c_mode_enabled`, and other state is to be undefined.
    ///
    /// 0: Activly Pull SDA Low
    /// 1: Leave SDA floating
    set_sda_hardware_pin_released,
    /// # Is SDA Hardware Pin Released
    /// Check if the SDA hardware pin is being pulled low, or is being released.
    ///
    /// 0: Activly Pulled low
    /// 1: SDA is currently floating
    is_sda_hardware_pin_released}

    bit_impl! {6, RW,
    /// # Set SCL Hardware Pin Released
    /// Set the state of the SCL hardware pin. (Activly pull the pin low, or leave it floating). This
    /// mode is only active during `software_i2c_mode_enabled`, and other state is to be undefined.
    ///
    /// 0: Activly Pull SCL Low
    /// 1: Leave SCL floating
    set_scl_hardware_pin_released,
    /// # Is SCL Hardware Pin Released
    /// Check if the SCL hardware pin is being pulled low, or is being released.
    ///
    /// 0: Activly Pulled low
    /// 1: SCL is currently floating
    is_scl_hardware_pin_released}

    bit_impl! {4, RW,
    /// # Set IRXM Responce `NACK`
    /// If the IRXM is currently enabled, this will set if the IRXM response will be an `ACK`, or
    /// a `NACK`. This also requires that the IRXM be enabled.
    ///
    /// 0: Respond to IRXM with `ACK`
    /// 1: Respond to IRXM with `NACK`
    set_irxm_responce_nack,
    /// # Is IRXM Responding with `NACK`
    /// Check to see if the IRXM will respond with `ACK`, or `NACK`.
    ///
    /// 0: The controller will respond with `ACK`
    /// 1: The controller will repsond with `NACK`
    is_irxm_responding_with_nack}

    bit_impl! {3, RW,
    /// # Set if IRXM will be Enabled
    /// When currently receiving data, the IRXM will allow for interactive receive mode (IRXM)
    /// interrupts for each byte of data received. Configuration of if the hardware will send
    /// an `ACK` to IRXM is with `set_irxm_response_nack`.
    ///
    /// 0: Disabled
    /// 1: Enabled
    set_irxm_enable,
    /// # Is IRXM Enabled
    /// Check if IRXM (interactive receive mode) will send interrupts for each byte of data received.
    ///
    /// 0: Disabled
    /// 1: Enabled
    is_irxm_enabled}
}

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
