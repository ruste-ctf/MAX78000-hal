use crate::bits::BitManipulation;
use crate::const_assert;
use crate::memory_map::mmio;
use crate::{bit_impl, reg_impl};

/// # Relative Register Offsets
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
    /// - 0: Disabled
    /// - 1: Enabled
    is_high_speed_mode_enabled}

    bit_impl! {13, RW,
    /// # Set One Master Mode
    /// Set if the controller is going to be using one master mode. When set
    /// to true, the device must only be used with slave devices. No other
    /// masters should be attached to the bus. When using one master mode,
    /// it must also be true that no slave devices will hold SCL low for
    /// any given reason (i.e clock stretching).
    ///
    /// - 0: Disabled
    /// - 1: Enabled
    set_one_master_mode,
    /// # Is One Master Mode Enabled
    /// Check to see if the device is in single master mode. When in single
    /// device master mode, there must be only one master on the bus.
    ///
    /// - 0: Disabled
    /// - 1: Enabled
    is_one_master_mode_enabled}

    bit_impl! {12, RW,
    /// # Set Disable Slave Clock Stretching
    /// Sets if slave clock stretching will be disabled. In this mode, it must
    /// also be true that `one_master_mode` must also be set since slave devices
    /// will be pulling SCL low.
    ///
    /// - 0: Enabled
    /// - 1: Disabled
    set_disable_slave_clock_stretching,
    /// # Is Slave Clock Stretching Disabled
    /// Check to see if the device is currently disabling slave devices from using
    /// clock stretching on the bus. If this mode is active, it must also be true
    /// that `one_master_mode` must also be active.
    ///
    /// - 0: Enabled
    /// - 1: Disabled
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
    /// the on-board hardware will generate basic I2C based signals (provided you tell it to).
    ///
    /// - 0: The I2C controller will manage I2C in hardware.
    /// - 1: SDA and SCL will need to be "bit-banged" by software by setting them manually.
    set_software_i2c_mode,
    /// # Is Software I2C Mode Enabled
    /// Checks if the hardware I2C controller will be managing the SCL and SDA pins.
    is_software_i2c_mode_enabled}

    bit_impl! {9, RO,
    /// # Get SDA Pin
    /// Get the `SDA` pin status, whether it be high or low.
    ///
    /// - 0: The `SDA` pin is logic low
    /// - 1: The `SDA` pin is logic high
    get_sda_pin}

    bit_impl! {8, RO,
    /// # Get SCL Pin
    /// Get the `SCL` pin status, whether it be high or low.
    ///
    /// - 0: The `SCL` pin is logic low
    /// - 1: The `SCL` pin is logic high
    get_scl_pin}

    bit_impl! {7, RW,
    /// # Set SDA Hardware Pin Released
    /// Set the state of the SDA hardware pin. (Actively pull the pin low, or leave it floating). This
    /// mode is only active during `software_i2c_mode_enabled`, and other state is to be undefined.
    ///
    /// - 0: Actively Pull SDA Low
    /// - 1: Leave SDA floating
    set_sda_hardware_pin_released,
    /// # Is SDA Hardware Pin Released
    /// Check if the SDA hardware pin is being pulled low, or is being released.
    ///
    /// - 0: Actively Pulled low
    /// - 1: SDA is currently floating
    is_sda_hardware_pin_released}

    bit_impl! {6, RW,
    /// # Set SCL Hardware Pin Released
    /// Set the state of the SCL hardware pin. (Actively pull the pin low, or leave it floating). This
    /// mode is only active during `software_i2c_mode_enabled`, and other state is to be undefined.
    ///
    /// - 0: Actively Pull SCL Low
    /// - 1: Leave SCL floating
    set_scl_hardware_pin_released,
    /// # Is SCL Hardware Pin Released
    /// Check if the SCL hardware pin is being pulled low, or is being released.
    ///
    /// - 0: Actively Pulled low
    /// - 1: SCL is currently floating
    is_scl_hardware_pin_released}

    bit_impl! {4, RW,
    /// # Set IRXM Response `NACK`
    /// If the IRXM is currently enabled, this will set if the IRXM response will be an `ACK`, or
    /// a `NACK`. This also requires that the IRXM be enabled.
    ///
    /// - 0: Respond to IRXM with `ACK`
    /// - 1: Respond to IRXM with `NACK`
    set_irxm_responce_nack,
    /// # Is IRXM Responding with `NACK`
    /// Check to see if the IRXM will respond with `ACK`, or `NACK`.
    ///
    /// - 0: The controller will respond with `ACK`
    /// - 1: The controller will respond with `NACK`
    is_irxm_responding_with_nack}

    bit_impl! {3, RW,
    /// # Set if IRXM will be Enabled
    /// When currently receiving data, the IRXM will allow for interactive receive mode (IRXM)
    /// interrupts for each byte of data received. Configuration of if the hardware will send
    /// an `ACK` to IRXM is with `set_irxm_response_nack`.
    ///
    /// - 0: Disabled
    /// - 1: Enabled
    set_irxm_enable,
    /// # Is IRXM Enabled
    /// Check if IRXM (interactive receive mode) will send interrupts for each byte of data received.
    ///
    /// - 0: Disabled
    /// - 1: Enabled
    is_irxm_enabled}

    bit_impl! {2, RW,
    /// # Set Acknowledge General Call
    /// Set the I2C controller to acknowledge the general call address, and respond with ACK.
    ///
    /// - 0: Ignore General Call Address
    /// - 1: Acknowledge General Call Address
    set_acknowledge_general_call,
    /// # Is Acknowledging General Call
    /// Checks is the controller is currently configured to acknowledge the general call
    /// addressing.
    ///
    /// - 0: Ignore General Call Address
    /// - 1: Acknowledge General Call Address
    is_acknowledging_general_call}

    bit_impl! {1, RW,
    /// # Set Master Mode Enabled
    /// Sets the controller to be the bus master on the I2C bus. Only some operations
    /// are available during master mode. It is important to know which mode the
    /// controller is in to use other functions.
    ///
    /// - 0: Slave Mode Enabled
    /// - 1: Master Mode Enabled
    set_master_mode_enable,
    /// # Is Master Mode Enabled
    /// Checks if the controller is currently configured to use I2C bus master mode.
    ///
    /// - 0: Slave Mode Enabled
    /// - 1: Master Mode Enabled
    is_master_mode_enabled}

    bit_impl! {0, RW,
    /// # Set I2C Peripheral Enable
    /// Enables the I2C bus. Allows communication to be sent over to I2C peripherals.
    ///
    /// - 0: Disabled
    /// - 1: Enabled
    set_i2c_peripheral_enable,
    /// # Is I2C Peripheral Enabled
    /// Checks if the I2C peripheral bus is currently enabled.
    ///
    /// - 0: Disabled
    /// - 1: Enabled
    is_i2c_peripheral_enabled}
}

/// # I2C Status Register
/// The status register for I2C related tasks, page 226 (MAX78000 User Guide)
pub struct StatusRegister<const PORT_PTR: usize> {}
reg_impl!(RO, StatusRegister, rro::I2C_STATUS_OFFSET);

impl<const PORT_PTR: usize> StatusRegister<PORT_PTR> {
    bit_impl! {5, RO,
    /// # Is Master Busy
    /// When the controller is in master mode, and is currently processing a transaction,
    /// then this flag will be set for the duration that `START` until `STOP`.
    ///
    /// - 0: Device is currently not driving the SCL clock
    /// - 1: Device is currently driving the SCL clock
    is_master_busy}

    bit_impl! {4, RO,
    /// # Is Transmit FIFO Full
    /// Checks to see if the transmit FIFO is currently full.
    ///
    /// - 0: Not Full
    /// - 1: Full
    is_transmit_fifo_full}

    bit_impl! {3, RO,
    /// # Is Transmit FIFO Empty
    /// Checks to see if the transmit FIFO is currently empty.
    ///
    /// - 0: Not Empty
    /// - 1: Empty
    is_transmit_fifo_empty}

    bit_impl! {2, RO,
    /// # Is Receive FIFO Full
    /// Checks to see if the current receive FIFO is full.
    ///
    /// - 0: Not Full
    /// - 1: Full
    is_receive_fifo_full}

    bit_impl! {1, RO,
    /// # Is Receive FIFO Empty
    /// Checks to see if the current FIFO is currently empty.
    ///
    /// - 0: Not Empty
    /// - 1: Empty
    is_receive_fifo_empty}

    bit_impl! {0, RO,
    /// # Is Transaction Active
    /// Checks to see (either in master or slave mode) if the I2C controller is currently
    /// processing a transaction.
    ///
    /// - 0: I2C bus is idle
    /// - 1: I2C bus is busy
    is_transaction_active}
}

/// # I2C Interrupt Flag 0 Register
/// The interrupt flag 0 register for controlling interrupt flags for I2C related tasks, page 226-229 (MAX78000 User Guide)
pub struct InterruptFlag0<const PORT_PTR: usize> {}
reg_impl!(
    RW1C,
    InterruptFlag0,
    rro::I2C_INTFL0_OFFSET,
    0b00000000000000000000000000000000
);

impl<const PORT_PTR: usize> InterruptFlag0<PORT_PTR> {
    bit_impl! {23, RW1C,
    /// # Clear Slave Write Address Match Interrupt
    /// If this bit is set, the current device (currently configured for slave mode) has just been accessed for a write (ie. receive)
    /// and the address requested matches our I2C address (the bus is talking to us).
    ///
    /// - 0: No Address Match
    /// - 1: Address Match
    clear_slave_write_addr_match_interrupt,
    /// # Is Slave Write Address Match Interrupt
    /// If this bit is set, the current device (currently configured for slave mode) has just been accessed for a write (ie. receive)
    /// and the address requested matches our I2C address (the bus is talking to us).
    ///
    /// - 0: No Address Match
    /// - 1: Address Match
    is_slave_write_addr_match_interrupt}

    bit_impl! {22, RW1C,
    /// # Clear Slave Write Address Match Interrupt
    /// If this bit is set, the current device (currently configured for slave mode) has just been accessed for a read (ie. write)
    /// and the address requested matches our I2C address (the bus is talking to us).
    ///
    /// - 0: No Address Match
    /// - 1: Address Match
    clear_slave_read_addr_match_interrupt,
    /// # Is Slave Write Address Match Interrupt
    /// If this bit is set, the current device (currently configured for slave mode) has just been accessed for a read (ie. write)
    /// and the address requested matches our I2C address (the bus is talking to us).
    ///
    /// - 0: No Address Match
    /// - 1: Address Match
    is_slave_read_addr_match_interrupt}

    bit_impl! {16, RW1C,
    /// # Clear MAMI Interrupt Flag
    // FIXME: The MAX78000 User Guide Page 227 does not contain info about this register, we should find out what it does.
    clear_mami_interrupt_flag,
    /// # Is MAMI Interrupt Flag
    // FIXME: The MAX78000 User Guide Page 227 does not contain info about this register, we should find out what it does.
    is_mami_interrupt_flag}

    bit_impl! {15, RW1C,
    /// # Clear Transmit FIFO Locked
    /// If this flag is set, the transmit FIFO is currently locked. If any more data is pushed to the transmit FIFO, it will be
    /// ignored. The flag must be cleared for writes to be valid. While this register is set, the transmit FIFO is automatically flushed.
    ///
    /// - 0: Transmit FIFO is not locked
    /// - 1: Transmit FIFO is currently locked
    clear_transmit_fifo_locked,
    /// # Is Transmit FIFO Locked
    /// If this flag is set, the transmit FIFO is currently locked. If any more data is pushed to the transmit FIFO, it will be
    /// ignored. The flag must be cleared for writes to be valid. While this register is set, the transmit FIFO is automatically flushed.
    ///
    /// - 0: Transmit FIFO is not locked
    /// - 1: Transmit FIFO is currently locked
    is_transmit_fifo_locked}

    bit_impl! {14, RW1C,
    /// # Clear Out Of Sequence STOP flag
    /// If this flag is set, a STOP condition occurred out of sequence.
    ///
    /// - 0: Normal Operation
    /// - 1: Out of sequence STOP condition occurred
    clear_out_of_sequence_stop_flag,
    /// # Is Out Of Sequence STOP flag
    /// If this flag is set, a STOP condition occurred out of sequence.
    ///
    /// - 0: Normal Operation
    /// - 1: Out of sequence STOP condition occurred
    is_out_of_sequence_stop_flag}

    bit_impl! {13, RW1C,
    /// # Clear Out Of Sequence START Flag
    /// If this flag is set, a START condition occurred out of sequence.
    ///
    /// - 0: Normal Operation
    /// - 1: Out of sequence START condition occurred
    clear_out_of_sequence_start_flag,
    /// # Is Out Of Sequence START Flag
    /// If this flag is set, a START condition occurred out of sequence.
    ///
    /// - 0: Normal Operation
    /// - 1: Out of sequence START condition occurred
    is_out_of_sequence_start_flag}

    bit_impl! {12, RW1C,
    /// # Clear Slave Mode Do-Not-Respond
    /// If this flag is set the controller received an address match, but the transmit FIFO or receive FIFO are not ready.
    ///
    /// - 0: Normal Operation
    /// - 1: FIFO not configured
    clear_slave_mode_do_not_respond,
    /// # Is Slave Mode Do-Not-Respond
    /// If this flag is set the controller received an address match, but the transmit FIFO or receive FIFO are not ready.
    ///
    /// - 0: Normal Operation
    /// - 1: FIFO not configured
    is_slave_mode_do_not_respond_fla}

    bit_impl! {11, RW1C,
    /// # Clear Master Data NACK from External Slave Error
    /// If this flag is set, the current device has received a NACK from a slave device (only if the current device is
    /// configured to be in master mode).
    ///
    /// - 0: Normal Operation
    /// - 1: Data NACK received from a Slave
    clear_master_data_nack_from_slave_err,
    /// # Is Master Data NACK from External Slave Error
    /// If this flag is set, the current device has received a NACK from a slave device (only if the current device is
    /// configured to be in master mode).
    ///
    /// - 0: Normal Operation
    /// - 1: Data NACK received from a Slave
    is_master_data_nack_from_slave_err_flag}

    bit_impl! {10, RW1C,
    /// # Clear Master Address NACK from Slave Error
    /// If this flag is set, the current device has received a NACK from a slave device (only if the current device is
    /// configured to be in master mode).
    ///
    /// - 0: Normal Operation
    /// - 1: Address NACK received from a Slave
    clear_master_address_nack_from_slave_err,
    /// # Is Master Address NACK from Slave Error
    /// If this flag is set, the current device has received a NACK from a slave device (only if the current device is
    /// configured to be in master mode).
    ///
    /// - 0: Normal Operation
    /// - 1: Address NACK received from a Slave
    is_master_address_nack_from_slave_err_flag}

    bit_impl! {9, RW1C,
    /// # Clear Timeout Error Flag
    /// If this flag is set, the current device has held SCL low for longer than the timeout value. This is valid either
    /// in master or slave operation.
    ///
    /// - 0: Normal Operation
    /// - 1: Timeout occurred
    clear_timeout_error_flag,
    /// # Is Timeout Error Flag
    /// If this flag is set, the current device has held SCL low for longer than the timeout value. This is valid either
    /// in master or slave operation.
    ///
    /// - 0: Normal Operation
    /// - 1: Timeout occurred
    is_timeout_error_flag_set}

    bit_impl! {8, RW1C,
    /// # Clear Master Mode Arbitration Lost
    /// If this flag is set than the device has lost arbitration.
    ///
    /// - 0: Normal Operation
    /// - 1: Condition occurred
    clear_master_mode_arbitration_lost,
    /// # Is Master Mode Arbitration Lost
    /// If this flag is set than the device has lost arbitration.
    ///
    /// - 0: Normal Operation
    /// - 1: Condition occurred
    is_master_mode_arbitration_lost_flag}

    bit_impl! {7, RW1C,
    /// # Clear Master ACK from External Slave
    /// If this flag is set, then this device (currently configured to be bus master) has just received an ACK from
    /// a slave device.
    ///
    /// - 0: Normal Operation
    /// - 1: ACK Received
    clear_master_ack_from_external_slave,
    /// # Is Master ACK from External Slave
    /// If this flag is set, then this device (currently configured to be bus master) has just received an ACK from
    /// a slave device.
    ///
    /// - 0: Normal Operation
    /// - 1: ACK Received
    is_master_ack_from_external_slave_flag}

    bit_impl! {6, RW1C,
    /// # Clear Slave Mode STOP Condition
    /// When this flag is set, the hardware noticed a STOP condition.
    ///
    /// - 0: Normal Operation
    /// - 1: STOP condition occurred
    clear_slave_mode_stop_condition,
    /// # Is Slave Mode STOP Condition
    /// When this flag is set, the hardware noticed a STOP condition.
    ///
    /// - 0: Normal Operation
    /// - 1: STOP condition occurred
    is_slave_mode_stop_condition}

    bit_impl! {5, RW1C,
    /// # Clear Transmit FIFO Threshold Level
    /// (MAYBE ERROR IN DOCUMENTATION PAGE 228 MAX78000 USER GUIDE)
    ///
    /// When this flag is set, the transmit FIFO has less then or equal to the number of threshold bytes set. This
    /// flag is automatically cleared when the transmit FIFO contains (MORE/LESS) bytes then the threshold level.
    ///
    /// - 0: Transmit FIFO contains more bytes than the transmit threshold level.
    /// - 1: Transmit FIFO contains less bytes than the transmit threshold level.
    clear_transmit_fifo_threshold_level,
    /// # Is Transmit FIFO Threshold Level
    /// (MAYBE ERROR IN DOCUMENTATION PAGE 228 MAX78000 USER GUIDE)
    ///
    /// When this flag is set, the transmit FIFO has less then or equal to the number of threshold bytes set. This
    /// flag is automatically cleared when the transmit FIFO contains (MORE/LESS) bytes then the threshold level.
    ///
    /// - 0: Transmit FIFO contains more bytes than the transmit threshold level.
    /// - 1: Transmit FIFO contains less bytes than the transmit threshold level.
    is_transmit_fifo_threshold_level}

    bit_impl! {4, RW1C,
    /// # Clear Receive FIFO Threshold Level
    /// (MAYBE ERROR IN DOCUMENTATION PAGE 228 MAX78000 USER GUIDE)
    ///
    /// When this flag is set, the receive FIFO has less then or equal to the number of threshold bytes set. This
    /// flag is automatically cleared when the receive FIFO contains (MORE/LESS) bytes then the threshold level.
    ///
    /// - 0: Receive FIFO contains more bytes than the transmit threshold level.
    /// - 1: Receive FIFO contains less bytes than the transmit threshold level.
    clear_receive_fifo_threshold_level,
    /// # Is Receive FIFO Threshold Level
    /// (MAYBE ERROR IN DOCUMENTATION PAGE 228 MAX78000 USER GUIDE)
    ///
    /// When this flag is set, the receive FIFO has less then or equal to the number of threshold bytes set. This
    /// flag is automatically cleared when the receive FIFO contains (MORE/LESS) bytes then the threshold level.
    ///
    /// - 0: Receive FIFO contains more bytes than the transmit threshold level.
    /// - 1: Receive FIFO contains less bytes than the transmit threshold level.
    is_receive_fifo_threshold_level}

    bit_impl! {3, RW1C,
    /// # Clear Slave mode Incoming Address Match Status
    /// If the controller is configured for Slave mode, the hardware will set this flag is the incoming address
    /// has been matched to ours. (Depends on this device being configured for Slave Mode)
    ///
    /// - 0: Slave Mode Address Match has not occurred
    /// - 1: Slave Mode Address Match has occurred
    clear_slave_incoming_address_match_status,
    /// # Is Slave mode Incoming Address Match Status
    /// If the controller is configured for Slave mode, the hardware will set this flag is the incoming address
    /// has been matched to ours. (Depends on this device being configured for Slave Mode)
    ///
    /// - 0: Slave Mode Address Match has not occurred
    /// - 1: Slave Mode Address Match has occurred
    is_slave_incoming_address_match_status}

    bit_impl! {2, RW1C,
    /// # Clear Slave General Call Address Match Received
    /// If the controller is configured for Slave mode, the hardware will set this flag if the general call
    /// address match has occurred. (Depends on this device being configured for Slave Mode)
    ///
    /// - 0: Normal Operation
    /// - 1: General Call Address Match occurred
    clear_slave_general_call_address_match_received,
    /// # Is Slave General Call Address Match Received
    /// If the controller is configured for Slave mode, the hardware will set this flag if the general call
    /// address match has occurred. (Depends on this device being configured for Slave Mode)
    ///
    /// - 0: Normal Operation
    /// - 1: General Call Address Match occurred
    is_slave_general_call_address_match_received}

    bit_impl! {1, RW1C,
    /// # Clear IRXM Interrupt Flag
    /// Determines if the IRXM flag is set.
    ///
    /// - 0: Normal Operation
    /// - 1: Interrupt Condition occurred
    clear_irxm_interrupt_flag,
    /// # Is IRXM Interrupt Flag
    /// Determines if the IRXM flag is set.
    ///
    /// - 0: Normal Operation
    /// - 1: Interrupt Condition occurred
    is_irxm_interrupt_flag}

    bit_impl! {0, RW1C,
    /// # Clear Transfer Complete Flag
    /// The controller sets this flag when the current transaction has completed. This flag is both valid for
    /// slave mode transfer and for master mode transfer.
    ///
    /// - 0: Transfer is not complete
    /// - 1: Transfer is complete
    clear_transfer_complete_flag,
    /// # Is Transfer Complete Flag
    /// The controller sets this flag when the current transaction has completed. This flag is both valid for
    /// slave mode transfer and for master mode transfer.
    ///
    /// - 0: Transfer is not complete
    /// - 1: Transfer is complete
    is_transfer_complete}
}

impl<const PORT_PTR: usize> InterruptFlag0<PORT_PTR> {
    /// # Is Error Condition
    /// Checks if any of the error conditions as been reached.
    pub fn is_error_condition() -> bool {
        let value = Self::read();

        // Bits 8 though 14 all contain error flags
        value.get_bit_range(8..=14) != 0
    }
}

/// # I2C Interrupt Enable 0 Register
/// The interrupt enable 0 register for controlling if interrupts are enabled for I2C, page 229-230 (MAX78000 User Guide)
pub struct InterruptEnable0<const PORT_PTR: usize> {}
reg_impl!(RW, InterruptEnable0, rro::I2C_INTEN0_OFFSET);

impl<const PORT_PTR: usize> InterruptEnable0<PORT_PTR> {
    bit_impl! {23, RW,
    /// # Set Slave Write Address Match Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    set_slave_write_address_match_interrupt_enable,
    /// # Is Slave Write Address Match Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    is_slave_write_address_match_interrupt_enabled}

    bit_impl! {22, RW,
    /// # Set Slave Read Address Match Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    set_slave_read_address_match_interrupt_enable,
    /// # Is Slave Read Address Match Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    is_slave_read_address_match_interrupt_enabled}

    bit_impl! {16, RW,
    /// # Set MAMI Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    set_mami_interrupt_enable,
    /// # Is MAMI Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    is_mami_interrupts_enabled}

    bit_impl! {15, RW,
    /// # Set Transmit FIFO lock-out Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    set_transmit_fifo_lock_out_interrupt_enable,
    /// # Is Transmit FIFO lock-out Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    is_transmit_fifo_lock_out_interrupt_enabled}

    bit_impl! {14, RW,
    /// # Set Out Of Sequence STOP condition Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    set_out_of_sequence_stop_condition_interrupt_enable,
    /// # Is Out Of Sequence STOP condition Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    is_out_of_sequence_stop_condition_interrupt_enabled}

    bit_impl! {13, RW,
    /// # Set Out-Of-Sequence START condition Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    set_out_of_sequence_start_condidtion_interrupt_enable,
    /// # Is Out-Of-Sequence START condition Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    is_out_of_sequence_start_condidtion_interrupt_enabled}

    bit_impl! {12, RW,
    /// # Set Slave Mode Do-Not-Repsond Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    set_slave_mode_do_not_respond_interrupt_enable,
    /// # Is Slave Mode Do-Not-Repsond Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    is_slave_mode_do_not_respond_interrupt_enabled}

    bit_impl! {11, RW,
    /// # Set Master Received Data NACK from Slave Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    set_master_received_data_nack_from_slave_interrupt_enable,
    /// # Is Master Received Data NACK from Slave Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    is_master_received_data_nack_from_slave_interrupt_enabled}

    bit_impl! {10, RW,
    /// # Set Master Received Address NACK from Slave Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    set_master_received_address_nack_from_slave_interrupt_enable,
    /// # Is Master Received Address NACK from Slave Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    is_master_received_address_nack_from_slave_interrupt_enabled}

    bit_impl! {9, RW,
    /// # Set Timeout Error Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    set_timeout_error_interrupt_enable,
    /// # Is Timeout Error Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    is_timeout_error_interrupt_enabled}

    bit_impl! {8, RW,
    /// # Set Master Mode Arbitration Lost Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    set_master_mode_arbitration_lost_interrupt_enable,
    /// # Is Master Mode Arbitration Lost Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    is_master_mode_arbitration_lost_interrupt_enabled}

    bit_impl! {7, RW,
    /// # Set Received Address ACK from Slave Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    set_received_address_ack_from_slave_interrupt_enable,
    /// # Is Received Address ACK from Slave Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    is_received_address_ack_from_slave_interrupt_enabled}

    bit_impl! {6, RW,
    /// # Set Stop Condition Detected Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    set_stop_condition_detected_interrupt_enable,
    /// # Is Stop Condition Detected Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    is_stop_condition_detected_interrupt_enabled}

    bit_impl! {5, RW,
    /// # Set Transmit FIFO Threshold Level Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    set_transmit_fifo_threshold_level_interrupt_enable,
    /// # Is Transmit FIFO Threshold Level Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    is_transmit_fifo_threshold_level_interrupt_enabled}

    bit_impl! {4, RW,
    /// # Set Receive FIFO Threshold Level Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    set_receive_fifo_threshold_level_interrupt_enable,
    /// # Is Receive FIFO Threshold Level Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    is_receive_fifo_threshold_level_interrupt_enabled}

    bit_impl! {3, RW,
    /// # Set Slave Mode Incoming Address Match Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    set_slave_mode_incoming_address_match_interrupt_enable,
    /// # Is Slave Mode Incoming Address Match Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    is_slave_mode_incoming_address_match_interrupt_enabled}

    bit_impl! {2, RW,
    /// # Set Slave General Call Address Match Received Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    set_slave_general_call_address_match_received_interrupt_enable,
    /// # Is Slave General Call Address Match Received Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    is_slave_general_call_address_match_received_interrupt_enabled}

    bit_impl! {1, RW,
    /// # Set IRXM Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    set_irxm_interrupt_enable,
    /// # Is IRXM Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    is_irxm_interrupt_enabled}

    bit_impl! {0, RW,
    /// # Set Transfer Complete Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    set_transfer_complete_interrupt_enable,
    /// # Is Transfer Complete Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    is_transfer_complete_interrupt_enabled}
}

/// # I2C Interrupt Flag 1 Register
/// The interrupt flag 1 register for controlling interrupt flags for I2C related tasks, page 230-231 (MAX78000 User Guide)
pub struct InterruptFlag1<const PORT_PTR: usize> {}
reg_impl!(
    RW1C,
    InterruptFlag1,
    rro::I2C_INTFL1_OFFSET,
    0b00000000000000000000000000000000
);

impl<const PORT_PTR: usize> InterruptFlag1<PORT_PTR> {
    bit_impl! {2, RW1C,
    /// # Clear START Condition Flag
    /// The I2C hardware will set this flag if it detects a START condition on the bus.
    ///
    /// - 0: START condition has not been detected
    /// - 1: START condition has been detected
    clear_start_condition_flag,
    /// # Is START Condition Flag
    /// The I2C hardware will set this flag if it detects a START condition on the bus.
    ///
    /// - 0: START condition has not been detected
    /// - 1: START condition has been detected
    is_start_condidtion_flag}

    bit_impl! {1, RW1C,
    /// # Clear Slave Mode Transmit FIFO Underflow Flag
    /// While the device is configured for slave mode operation, the hardware will enable this flag
    /// if the transmit FIFO is currently empty and the bus master requests more data by sending an ACK
    /// directly after the previous byte transfer is complete.
    ///
    /// - 0: Slave Mode FIFO has not had an underflow
    /// - 1: Slave Mode FIFO has underflow
    clear_slave_mode_transmit_fifo_underflow_flag,
    /// # Is Slave Mode Transmit FIFO Underflow Flag
    /// While the device is configured for slave mode operation, the hardware will enable this flag
    /// if the transmit FIFO is currently empty and the bus master requests more data by sending an ACK
    /// directly after the previous byte transfer is complete.
    ///
    /// - 0: Slave Mode FIFO has not had an underflow
    /// - 1: Slave Mode FIFO has underflow
    is_slave_mode_trasmit_fifo_underflow_flag}

    bit_impl! {0, RW1C,
    /// # Clear Slave Mode Receive FIFO Overflow Flag
    /// While the device is configured for slave mode operation, the hardware will enable this flag
    /// if the receive FIFO is currently full and the bus master sent us data. (DATA LOSS)
    ///
    /// - 0: Slave Mode FIFO has not overflowed
    /// - 1: Slave Mode FIFO has overflowed (DATA HAS BEEN LOST)
    clear_slave_mode_receive_fifo_overflow_flag,
    /// # Is Slave Mode Receive FIFO Overflow Flag
    /// While the device is configured for slave mode operation, the hardware will enable this flag
    /// if the receive FIFO is currently full and the bus master sent us data. (DATA LOSS)
    ///
    /// - 0: Slave Mode FIFO has not overflowed
    /// - 1: Slave Mode FIFO has overflowed (DATA HAS BEEN LOST)
    is_slave_mode_receive_fifo_overflow_flag}
}

/// # I2C Interrupt Enable 1 Register
/// The interrupt enable 1 register for controlling if interrupts are enabled for I2C, page 231 (MAX78000 User Guide)
pub struct InterruptEnable1<const PORT_PTR: usize> {}
reg_impl!(RW, InterruptEnable1, rro::I2C_INTEN1_OFFSET);

impl<const PORT_PTR: usize> InterruptEnable1<PORT_PTR> {
    bit_impl! {2, RW,
    /// # Set Start Condition Interrupt Enable
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    set_start_condition_interrupt_enable,
    /// # Is Start Condition Interrupt Enable
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    is_start_condition_interrupt_enabled}

    bit_impl! {1, RW,
    /// # Set Slave Mode Transmit FIFO Underflow Interrupt Enable
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    set_slave_mode_transmit_fifo_underflow_interrupt_enable,
    /// # Is Slave Mode Transmit FIFO Underflow Interrupt Enable
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    is_slave_mode_transmit_fifo_underflow_interrupt_enabled}

    bit_impl! {0, RW,
    /// # Set Slave Mode Receive FIFO Overflow Interrupt Enable
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    set_slave_mode_receive_fifo_overflow_interrupt_enable,
    /// # Is Slave Mode Receive FIFO Overflow Interrupt Enable
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    is_slave_mode_receive_fifo_overflow_interrupt_enabled}
}

/// # I2C FIFO Length Register
/// The FIFO length status register. Used to see the length of the transmit/receive FIFO, page 231 (MAX78000 User Guide)
pub struct FIFOLengthRegister<const PORT_PTR: usize> {}
reg_impl!(RO, FIFOLengthRegister, rro::I2C_FIFOLEN_OFFSET);

impl<const PORT_PTR: usize> FIFOLengthRegister<PORT_PTR> {
    bit_impl!(8..=15, RO u8, 
    /// # Get Transmit FIFO Length
    /// Get the current transmit FIFO depth in bytes.
    get_transmit_fifo_len);

    bit_impl! {0..=7, RO u8,
    /// # Get Receive FIFO Length
    /// Get the current receive FIFO depth in bytes.
    get_receive_fifo_len}
}

impl<const PORT_PTR: usize> FIFOLengthRegister<PORT_PTR> {
    /// # Max FIFO Receive Length
    /// The maximum amount of bytes the Receive FIFO can store.
    pub const MAX_FIFO_RECEIVE_LEN: usize = 8;
    /// # Max FIFO Transmit Length
    /// The maximum amount of bytes the Transmit FIFO can store.
    pub const MAX_FIFO_TRANSMIT_LEN: usize = 8;
}

/// # I2C Receive Control 0 Register
/// The Receive control register is used to set the receive FIFO threshold level, and set flush receive FIFO, page 231-232 (MAX78000 User Guide)
pub struct ReceiveControl0<const PORT_PTR: usize> {}
reg_impl!(RW1O, ReceiveControl0, rro::I2C_RXCTRL0_OFFSET, 0b00000000000000000001111111100000001);

impl<const PORT_PTR: usize> ReceiveControl0<PORT_PTR> {
    bit_impl! {8..=11, RW u8,
    /// # Set Receive FIFO Threshold Level
    /// This is the number of bytes to trigger a receive FIFO threshold event. If the bytes in the FIFO are greater than or equal to
    /// this value, the hardware will generate an interrupt (if enabled) and set `InterruptFlag0::is_receive_fifo_threshold_level_interrupt_enabled` to
    /// true.
    /// 
    /// - 0: 0 bytes or more causes an event
    /// - 1: 1 bytes or more causes an event
    /// -  ...
    /// - 8: 8 bytes (only when the FIFO is full)
    set_receive_fifo_threshold_level,
    /// # Is Receive FIFO Threshold Level
    /// This is the number of bytes to trigger a receive FIFO threshold event. If the bytes in the FIFO are greater than or equal to
    /// this value, the hardware will generate an interrupt (if enabled) and set `InterruptFlag0::is_receive_fifo_threshold_level_interrupt_enabled` to
    /// true.
    /// 
    /// - 0: 0 bytes or more causes an event
    /// - 1: 1 bytes or more causes an event
    /// -  ...
    /// - 8: 8 bytes (only when the FIFO is full)
    get_receive_fifo_threshold_level}
    
    bit_impl! {7, RW1O,
    /// # Activate Flush Receive FIFO
    /// When activated, this will initiate a receive FIFO flush. The hardware will then clear all the data in the receive FIFO. Among finishing
    /// the hardware will set this flag back to `0`.
    ///
    /// - 0: Receive FIFO flush complete (or not started)
    /// - 1: Flushing the Receive FIFO
    activate_flush_receive_fifo,
    /// # Is Flush Receive FIFO
    /// When activated, this will initiate a receive FIFO flush. The hardware will then clear all the data in the receive FIFO. Among finishing
    /// the hardware will set this flag back to `0`.
    ///
    /// - 0: Receive FIFO flush complete (or not started)
    /// - 1: Flushing the Receive FIFO
    is_flush_receive_fifo}

    bit_impl! {0, RW,
    /// # Set Slave Do-Not-Respond
    /// If this device (configured only in slave mode operation) has just been addressed for a write operation, and the receive FIFO is not
    /// empty the device will respond with a NACK.
    ///
    /// - 0: ACK the address, but NACK the data
    /// - 1: NACK the address
    set_slave_do_not_respond,
    /// # Is Slave Do-Not-Respond
    /// If this device (configured only in slave mode operation) has just been addressed for a write operation, and the receive FIFO is not
    /// empty the device will respond with a NACK.
    ///
    /// - 0: ACK the address, but NACK the data
    /// - 1: NACK the address
    is_slave_do_not_respond}
}
/// # I2C Receive Control 1 Register
/// The receive control register is used to set receive FIFO byte count configuration, and read byte count, page 232-233 (MAX78000 User Guide)
pub struct ReceiveControl1<const PORT_PTR: usize> {}
reg_impl!(RW, ReceiveControl1, rro::I2C_RXCTRL1_OFFSET);

impl<const PORT_PTR: usize> ReceiveControl1<PORT_PTR> {
    bit_impl! {8..=11, RO u8,
    /// # Get Current Receive FIFO Bytes
    /// Get the current number of bytes in the receive FIFO.
    ///
    /// - 0: 0 bytes (No data)
    /// - 1: 1 byte
    ///  ...
    /// - 8: 8 bytes
    get_current_receive_fifo_bytes}

    bit_impl! {0..=7, RW u8,
    /// # Set Receive FIFO Transaction Size
    /// Write the number of bytes to be received in a transaction (when device is configured to be in master mode).
    ///
    /// - 0: 256 byte receive transaction
    /// - 1: 1 byte receive transaction
    /// ...
    /// - 255: 255 byte receive transaction 
    set_receive_fifo_transaction_size,
    /// # Get Receive FIFO Transaction Size
    /// Write the number of bytes to be received in a transaction (when device is configured to be in master mode).
    ///
    /// - 0: 256 byte receive transaction
    /// - 1: 1 byte receive transaction
    /// ...
    /// - 255: 255 byte receive transaction 
    get_receive_fifo_transaction_size}
}

/// # I2C Transmit Control 0 Register
/// The transmit control register is used to control transmitting related I2C tasks, page 233-234 (MAX78000 User Guide)
pub struct TransmitControl0<const PORT_PTR: usize> {}
reg_impl!(RW1O, TransmitControl0, rro::I2C_TXCTRL0_OFFSET, 0b0000000000000000000100111111);

impl<const PORT_PTR: usize> TransmitControl0<PORT_PTR> {
    bit_impl! {8..=11, RW u8,
    /// # Set Transmit FIFO Threshold Level
    /// Sets the number of bytes that are required to trigger an interrupt (if that interrupt is enabled) and
    /// set the flag. The number of bytes must be smaller or equal to this value for such an interrupt to occur.
    ///
    /// - 0: 0 or fewer bytes triggers event
    /// - 1: 1 or fewer bytes triggers event
    /// ...
    /// - 7: 7 or fewer bytes triggers event
    set_transmit_fifo_threshold_level,
    /// # Get Transmit FIFO Threshold Level
    /// Sets the number of bytes that are required to trigger an interrupt (if that interrupt is enabled) and
    /// set the flag. The number of bytes must be smaller or equal to this value for such an interrupt to occur.
    ///
    /// - 0: 0 or fewer bytes triggers event
    /// - 1: 1 or fewer bytes triggers event
    /// ...
    /// - 7: 7 or fewer bytes triggers event
    get_transmit_fifo_threshold_level}

    bit_impl! {7, RW1O, 
    /// # Activate Transmit FIFO Flush
    /// A transmit FIFO flush will clear all data from the transmit FIFO.
    ///
    /// - 0: Transmit FIFO flush is complete (or is not active).
    /// - 1: The Transmit FIFO Flush is currently being serviced
    activate_transmit_fifo_flush,
    /// # Is Transmit FIFO Flush
    /// A transmit FIFO flush will clear all data from the transmit FIFO.
    ///
    /// - 0: Transmit FIFO flush is complete (or is not active).
    /// - 1: The Transmit FIFO Flush is currently being serviced
    is_transmit_fifo_flush}

    bit_impl! {5, RW,
    /// # Set Transmit FIFO Received NACK Auto Flush Disable
    /// There are some cases in which other registers will cause a transmit FIFO flush. In such a case, one of the following
    /// values will be true.
    ///
    /// - 0: Received NACK at the end of a slave transmit operation enabled
    /// - 1: Received NACK at the end of a slave transmit operation disabled
    set_transmit_fifo_received_nack_auto_flush_disable,
    /// # Is Transmit FIFO Received NACK Auto Flush Disable
    /// There are some cases in which other registers will cause a transmit FIFO flush. In such a case, one of the following
    /// values will be true.
    ///
    /// - 0: Received NACK at the end of a slave transmit operation enabled
    /// - 1: Received NACK at the end of a slave transmit operation disabled
    is_transmit_fifo_received_nack_auto_flush_disable}

    bit_impl! {4, RW,
    /// # Set Transmit FIFO Slave Address Match Read Auto Flush Disable
    /// There are some cases in which other registers will cause a transmit FIFO flush. In such a case, one of the following
    /// values will be true.
    ///
    /// - 0: Enabled
    /// - 1: Disabled
    set_transmit_fifo_slave_address_match_read_auto_flush_disable,
    /// # Is Transmit FIFO Slave Address Match Read Auto Flush Disable
    /// There are some cases in which other registers will cause a transmit FIFO flush. In such a case, one of the following
    /// values will be true.
    ///
    /// - 0: Enabled
    /// - 1: Disabled
    is_transmit_fifo_slave_address_match_read_auto_flush_disable}

    bit_impl! {3, RW,
    /// # Set Transmit FIFO Slave Address Match Write Auto Flush Disable
    /// There are some cases in which other registers will cause a transmit FIFO flush. In such a case, one of the following
    /// values will be true.
    ///
    /// - 0: Enabled
    /// - 1: Disabled
    set_transmit_fifo_slave_address_match_write_auto_flush_disable,
    /// # Is Transmit FIFO Slave Address Match Write Auto Flush Disable
    /// There are some cases in which other registers will cause a transmit FIFO flush. In such a case, one of the following
    /// values will be true.
    ///
    /// - 0: Enabled
    /// - 1: Disabled
    is_transmit_fifo_slave_address_match_write_auto_flush_disable}

    bit_impl! {2, RW,
    /// # Set Transmit FIFO General Call Address Match Auto Flush Disable
    /// There are some cases in which other registers will cause a transmit FIFO flush. In such a case, one of the following
    /// values will be true.
    ///
    /// - 0: Enabled
    /// - 1: Disabled
    set_transmit_fifo_general_call_address_match_auto_flush_disable,
    /// # Is Transmit FIFO General Call Address Match Auto Flush Disable
    /// There are some cases in which other registers will cause a transmit FIFO flush. In such a case, one of the following
    /// values will be true.
    ///
    /// - 0: Enabled
    /// - 1: Disabled
    is_transmit_fifo_general_call_address_match_auto_flush_disable}

    bit_impl! {1, RW,
    /// # Set Transmit FIFO Read Manual Mode
    /// Disable or enable the hardware from controlling the `preload ready` flag. When enabled, it allows software to only
    /// control this flag.
    ///
    /// - 0: Hardware Controls Preload Ready
    /// - 1: Software Controls Preload Ready
    set_transmit_fifo_read_manual_mode,
    /// # Is Transmit FIFO Read Manual Mode
    /// Disable or enable the hardware from controlling the `preload ready` flag. When enabled, it allows software to only
    /// control this flag.
    ///
    /// - 0: Hardware Controls Preload Ready
    /// - 1: Software Controls Preload Ready
    is_transmit_fifo_read_manual_mode}

    bit_impl! {0, RW,
    /// # Set Transmit FIFO Preload Mode Enable
    /// The following conditions are held with this flag.
    ///
    /// - 0: An Address match in slave mode, or a general call address does lock the transmit FIFO.
    /// - 1: Transmit FIFO preload mode. An address match in slave mode does not lock the transmit FIFO.
    set_transmit_fifo_preload_mode_enable,
    /// # Is Transmit FIFO Preload Mode Enable
    /// The following conditions are held with this flag.
    ///
    /// - 0: An Address match in slave mode, or a general call address does lock the transmit FIFO.
    /// - 1: Transmit FIFO preload mode. An address match in slave mode does not lock the transmit FIFO.
    is_transmit_fifo_preload_mode_enable}
}

/// # I2C Transmit Control 1 Register
/// The transmit control register is used to control transmitting related I2C tasks, page 234-235 (MAX78000 User Guide)
pub struct TransmitControl1<const PORT_PTR: usize> {}
reg_impl!(RW1O, TransmitControl1, rro::I2C_TXCTRL1_OFFSET, 0b00000000000000001111111100000000);

impl<const PORT_PTR: usize> TransmitControl1<PORT_PTR> {
    bit_impl! {8..=11, RO u8,
    /// # Get Transmit FIFO Byte Count
    /// Get the current number of bytes that reside in the transmit FIFO.
    ///
    /// - 0: 0 bytes (no data)
    /// - 1: 1 byte
    /// ...
    /// - 8: 8 bytes (FIFO full)
    get_transmit_fifo_byte_count}

    bit_impl! {0, RW1O,
    /// # Activate Transmit FIFO Preload Ready (page 235)
    // TODO: Finish this documentation
    activate_transmit_fifo_preload_ready,
    /// # Is Transmit FIFO Preload Ready (page 235)
    // TODO: Finish this documentation
    is_transmit_fifo_preload_ready}
}

/// # I2C Data Register
/// The data register is used to send and receive data to the FIFO, page 235 (MAX78000 User Guide)
pub struct DataRegister<const PORT_PTR: usize> {}
reg_impl!(RW, DataRegister, rro::I2C_FIFO_OFFSET);

impl<const PORT_PTR: usize> DataRegister<PORT_PTR> {
    bit_impl! {0..=7, RW u8,
    /// # Write FIFO Data
    /// Write to the transmit FIFO (pushes the data onto the transmit FIFO).
    ///
    /// If the FIFO is full, this operation is ignored (the data will be lost).
    write_fifo_data,
    /// # Read FIFO Data
    /// Read from the receive FIFO (pops the data off the received FIFO).
    ///
    /// If the FIFO is empty, this operation will return 0xFF (error).
    read_fifo_data}
}

/// # I2C Master Control Register
/// The master control register is used to control the bus when the device is configured to be the master, page 235-236 (MAX78000 User Guide)
pub struct MasterControl<const PORT_PTR: usize> {}
reg_impl!(RW1O, MasterControl, rro::I2C_MSTCTRL_OFFSET, 0b00000000000000000000000111000000);

impl<const PORT_PTR: usize> MasterControl<PORT_PTR> {
    bit_impl! {8..=10, RW u8,
    /// # Set MCODE
    /// This property sets the master code used in HS-Mode operation.
    set_mcode,
    /// # Get MCODE
    /// This property gets the master code used in HS-Mode operation.
    get_mcode}

    bit_impl! {7, RW,
    /// # Set Slave Extended Addressing
    /// Sets the master to enable slave extended bit addressing, this allows up to 10-bit addresses for slave devices.
    ///
    /// - 0: 7-bit Addressing (The most used and common)
    /// - 1: 10-bit Addressing
    set_slave_extended_addressing,
    /// # Is Slave Extended Addressing
    /// Sets the master to enable slave extended bit addressing, this allows up to 10-bit addresses for slave devices.
    ///
    /// - 0: 7-bit Addressing (The most used and common)
    /// - 1: 10-bit Addressing
    is_slave_extended_addressing_enabled}

    bit_impl! {2, RW1O,
    /// # Activate Send STOP Condition
    /// Tell the master to send a STOP condition at the end of the current transaction.
    activate_send_stop_condition,
    /// # Is Send STOP Condition (might do nothing, please use `activate_send_stop_condition`)
    /// Tell the master to send a STOP condition at the end of the current transaction.
    is_send_stop_condition}

    bit_impl! {1, RW1O,
    /// # Activate Send Repeated START Condition
    /// After sending data to a slave device, the master will send another START to retain control over the bus.
    activate_send_repeated_start_condition,
    /// # Is Send Repeated START Condition (might do nothing, please use `activate_send_repeated_start_condition`)
    /// After sending data to a slave device, the master will send another START to retain control over the bus.
    is_send_repeated_start_condition}

    bit_impl! {0, RW1O,
    /// # Activate Start Master Mode Transfer
    /// Start a master mode transfer over the I2C bus. 
    activate_start_master_mode_transfer,
    /// # Is Start Master Mode Transfer (might do nothing, please use `activate_start_master_mode_transfer`)
    /// Start a master mode transfer over the I2C bus. 
    is_start_master_mode_transfer}
}

/// # I2C SCL Low Control Register
/// The SCL low control register is used to control the clock low time of the bus, page 236 (MAX78000 User Guide)
pub struct LowSCLControl<const PORT_PTR: usize> {}
reg_impl!(RW, LowSCLControl, rro::I2C_CLKLO_OFFSET);

impl<const PORT_PTR: usize> LowSCLControl<PORT_PTR> {
    bit_impl! {0..=8, RW u16,
    /// # Set Clock Low Time
    /// Sets the current clock low time for `SCL`. Please use page 236 of the MAX78000 User Guide to determine
    /// the math in setting this value.
    set_clock_low_time,
    /// # Get Clock Low Time
    /// Gets the current clock low time for `SCL`. Please use page 236 of the MAX78000 User Guide to determine
    /// the math in getting this value.
    get_clock_low_time}
}

/// # I2C SCL High Control Register
/// The SCL high control register is used to control the clock high time of the bus, page 236 (MAX78000 User Guide)
pub struct HighSCLControl<const PORT_PTR: usize> {}
reg_impl!(RW, HighSCLControl, rro::I2C_CLKHI_OFFSET);

impl<const PORT_PTR: usize> HighSCLControl<PORT_PTR> {
    bit_impl! {0..=8, RW u16,
    /// # Set Clock High Time
    /// Sets the current clock High time for `SCL`. Please use page 236 of the MAX78000 User Guide to determine
    /// the math in setting this value.
    set_clock_high_time,
    /// # Get Clock High Time
    /// Gets the current clock High time for `SCL`. Please use page 236 of the MAX78000 User Guide to determine
    /// the math in setting this value.
    get_clock_high_time}
}

/// # I2C High Speed Clock Control Register
/// The high speed clock control register is used to control the high speed clock rate, page 236-237 (MAX78000 User Guide)
pub struct HighSpeedClockControl<const PORT_PTR: usize> {}
reg_impl!(RW, HighSpeedClockControl, rro::I2C_HSCLK_OFFSET);

impl<const PORT_PTR: usize> HighSpeedClockControl<PORT_PTR> {
    bit_impl! {8..=15, RW u8,
    /// # Set High Speed Mode Clock High Time
    /// Sets the high time duration for high speed mode on the I2C bus.
    set_high_speed_mode_clock_high_time,
    /// # Get High Speed Mode Clock High Time
    /// Gets the high time duration for high speed mode on the I2C bus.
    get_high_speed_mode_clock_high_time}

    bit_impl! {0..=7, RW u8,
    /// # Set High Speed Mode Clock Low Time
    /// Sets the low time duration for high speed mode on the I2C bus.
    set_high_speed_mode_clock_low_time,
    /// # Get High Speed Mode Clock Low Time
    /// Gets the low time duration for high speed mode on the I2C bus.
    get_high_speed_mode_clock_low_time}
}

/// # I2C Timeout Register
/// The timeout register is used to control the bus error scl timeout period, page 237 (MAX78000 User Guide)
pub struct TimeoutControl<const PORT_PTR: usize> {}
reg_impl!(RW, TimeoutControl, rro::I2C_TIMEOUT_OFFSET);

impl<const PORT_PTR: usize> TimeoutControl<PORT_PTR> {
    bit_impl! {0..=15, RW u16,
    /// # Set Bus Error SCL Timeout Period
    /// Sets the time that the SCL will be inactive after an error as occurred. Please use page 237
    /// on the MAX78000 User Guide to determine the calculation.
    set_bus_error_scl_timeout_period,
    /// # Get Bus Error SCL Timeout Period
    /// Sets the time that the SCL will be inactive after an error as occurred. Please use page 237
    /// on the MAX78000 User Guide to determine the calculation.
    get_bus_error_scl_timeout_period}
}

/// # I2C DMA Enable Register
/// The DMA control register used to control direct memory accessing for the I2C bus, page 237 (MAX78000 User Guide)
pub struct DMAControl<const PORT_PTR: usize> {}
reg_impl!(RW, DMAControl, rro::I2C_DMA_OFFSET);

impl<const PORT_PTR: usize> DMAControl<PORT_PTR> {
    bit_impl! {1, RW,
    /// # Set Receive DMA Channel Enable
    /// Enable the DMA Receive channel.
    /// 
    /// - 0: Disabled
    /// - 1: Enabled
    set_receive_dma_channel_enable,
    /// # Is Receive DMA Channel Enable
    /// Enable the DMA Receive channel.
    /// 
    /// - 0: Disabled
    /// - 1: Enabled
    is_receive_dma_channel_enabled}

    bit_impl! {0, RW,
    /// # Set Transmit DMA Channel Enable
    /// Enable the DMA Transmit channel.
    /// 
    /// - 0: Disabled
    /// - 1: Enabled
    set_transmit_dma_channel_enable,
    /// # Is Transmit DMA Channel Enable
    /// Enable the DMA Transmit channel.
    /// 
    /// - 0: Disabled
    /// - 1: Enabled
    is_transmit_dma_channel_enabled}
}

/// # I2C Slave Address Register
/// The slave address register is used to control the addressing mode of the bus, page 237-238 (MAX78000 User Guide)
pub struct SlaveAddress<const PORT_PTR: usize> {}
reg_impl!(RW, SlaveAddress, rro::I2C_SLAVE_OFFSET);

impl<const PORT_PTR: usize> SlaveAddress<PORT_PTR> {
    bit_impl! {15, RW,
    /// # Set Slave Mode Extended Address Length Select
    /// Set if (while in slave mode) to use the address extension.
    ///
    /// - 0: 7-bit addressing (the most used and normal one)
    /// - 1: 10-bit addressing 
    set_slave_mode_extended_address_length_select,
    /// # Is Slave Mode Extended Address Length Select
    /// Set if (while in slave mode) to use the address extension.
    ///
    /// - 0: 7-bit addressing (the most used and normal one)
    /// - 1: 10-bit addressing 
    is_slave_mode_extended_address_length_select}

    bit_impl! {0..=9, RW u16,
    /// # Set Slave Mode Address
    /// Sets the address of this device (must be configured to be in slave mode).
    ///
    /// Take note: There are a few reserved addresses!
    set_slave_mode_address,
    /// # get Slave Mode Address
    /// Sets the address of this device (must be configured to be in slave mode).
    ///
    /// Take note: There are a few reserved addresses!
    get_slave_mode_address}
}
