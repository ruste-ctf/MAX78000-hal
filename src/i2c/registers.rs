use crate::bits::BitManipulation;
use crate::const_assert;
use crate::memory_map::mmio;
use crate::{bit_impl, reg_impl};
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
    /// any given reason (i.e clock streaching).
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
    /// the onboard hardware will generate basic I2C based signals (provided you tell it to).
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
    /// Set the state of the SDA hardware pin. (Activly pull the pin low, or leave it floating). This
    /// mode is only active during `software_i2c_mode_enabled`, and other state is to be undefined.
    ///
    /// - 0: Activly Pull SDA Low
    /// - 1: Leave SDA floating
    set_sda_hardware_pin_released,
    /// # Is SDA Hardware Pin Released
    /// Check if the SDA hardware pin is being pulled low, or is being released.
    ///
    /// - 0: Activly Pulled low
    /// - 1: SDA is currently floating
    is_sda_hardware_pin_released}

    bit_impl! {6, RW,
    /// # Set SCL Hardware Pin Released
    /// Set the state of the SCL hardware pin. (Activly pull the pin low, or leave it floating). This
    /// mode is only active during `software_i2c_mode_enabled`, and other state is to be undefined.
    ///
    /// - 0: Activly Pull SCL Low
    /// - 1: Leave SCL floating
    set_scl_hardware_pin_released,
    /// # Is SCL Hardware Pin Released
    /// Check if the SCL hardware pin is being pulled low, or is being released.
    ///
    /// - 0: Activly Pulled low
    /// - 1: SCL is currently floating
    is_scl_hardware_pin_released}

    bit_impl! {4, RW,
    /// # Set IRXM Responce `NACK`
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
    /// - 1: The controller will repsond with `NACK`
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
    /// # Set Acknowledge Gneral Call
    /// Set the I2C controller to acknowledge the general call address, and repsond with ACK.
    ///
    /// - 0: Ignore Gneral Call Address
    /// - 1: Acknowledge General Call Address
    set_acknowledge_general_call,
    /// # Is Acknowledging General Call
    /// Checks is the controller is currently configured to acknowledge the general call
    /// addressing.
    ///
    /// - 0: Ignore Gneral Call Address
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
    /// # Is Trasmit FIFO Empty
    /// Checks to see if the transmit FIFO is currently empty.
    ///
    /// - 0: Not Empty
    /// - 1: Empty
    is_transmit_fifo_empty}

    bit_impl! {2, RO,
    /// # Is Recieve FIFO Full
    /// Checks to see if the current recieve FIFO is full.
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
    /// ignored. The flag must be cleared for writes to be valid. While this register is set, the transmit FIFO is automaticlly flushed.
    ///
    /// - 0: Transmit FIFO is not locked
    /// - 1: Transmit FIFO is currently locked
    clear_transmit_fifo_locked,
    /// # Is Transmit FIFO Locked
    /// If this flag is set, the transmit FIFO is currently locked. If any more data is pushed to the transmit FIFO, it will be
    /// ignored. The flag must be cleared for writes to be valid. While this register is set, the transmit FIFO is automaticlly flushed.
    ///
    /// - 0: Transmit FIFO is not locked
    /// - 1: Transmit FIFO is currently locked
    is_transmit_fifo_locked}

    bit_impl! {14, RW1C,
    /// # Clear Out Of Sequence STOP flag
    /// If this flag is set, a STOP condition occured out of sequence.
    ///
    /// - 0: Normal Operation
    /// - 1: Out of sequence STOP condition occurred
    clear_out_of_sequence_stop_flag,
    /// # Is Out Of Sequence STOP flag
    /// If this flag is set, a STOP condition occured out of sequence.
    ///
    /// - 0: Normal Operation
    /// - 1: Out of sequence STOP condition occurred
    is_out_of_sequence_stop_flag}

    bit_impl! {13, RW1C,
    /// # Clear Out Of Sequence START Flag
    /// If this flag is set, a START condition occured out of sequence.
    ///
    /// - 0: Normal Operation
    /// - 1: Out of sequence START condition occurred
    clear_out_of_sequence_start_flag,
    /// # Is Out Of Sequence START Flag
    /// If this flag is set, a START condition occured out of sequence.
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
    /// - 1: Ack Recveived
    clear_master_ack_from_external_slave,
    /// # Is Master ACK from External Slave
    /// If this flag is set, then this device (currently configured to be bus master) has just received an ACK from
    /// a slave device.
    ///
    /// - 0: Normal Operation
    /// - 1: Ack Recveived
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

    bit_impl! {5, RO,
    /// # Is Transmit FIFO Threshold Level
    /// (MAYBE ERROR IN DOCUMENTATION PAGE 228 MAX78000 USER GUIDE)
    ///
    /// When this flag is set, the transmit FIFO has less then or equal to the number of threshold bytes set. This
    /// flag is automatically cleared when the transmit FIFO contains (MORE/LESS) bytes then the threshold level.
    ///
    /// - 0: Transmit FIFO contains more bytes than the transmit threshold level.
    /// - 1: Transmit FIFO contains less bytes than the transmit threshold level.
    is_transmit_fifo_threshold_level}

    bit_impl! {4, RO,
    /// # Is Receive FIFO Threshold Level
    /// (MAYBE ERROR IN DOCUMENTATION PAGE 228 MAX78000 USER GUIDE)
    ///
    /// When this flag is set, the receive FIFO has less then or equal to the number of threshold bytes set. This
    /// flag is automatically cleared when the receive FIFO contains (MORE/LESS) bytes then the threshold level.
    ///
    /// - 0: Receive FIFO contains more bytes than the transmit threshold level.
    /// - 1: Receive FIFO contains less bytes than the transmit threshold level.
    is_receive_fifo_threshold_leve}

    bit_impl! {3, RW1C,
    /// # Clear Slave mode Incoming Addresss Match Status
    /// If the controller is configured for Slave mode, the hardware will set this flag is the incoming address
    /// has been matched to ours. (Depends on this device being configured for Slave Mode)
    ///
    /// - 0: Slave Mode Address Match has not occurred
    /// - 1: Slave Mode Address Match has occurred
    clear_slave_incoming_address_match_status,
    /// # Is Slave mode Incoming Addresss Match Status
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
    /// - 1: General Call Address Match occured
    clear_slave_general_call_address_match_received,
    /// # Is Slave General Call Address Match Received
    /// If the controller is configured for Slave mode, the hardware will set this flag if the general call
    /// address match has occurred. (Depends on this device being configured for Slave Mode)
    ///
    /// - 0: Normal Operation
    /// - 1: General Call Address Match occured
    is_slave_general_call_address_match_received}

    bit_impl! {1, RW1C,
    /// # Clear IRXM Interrupt Flag
    /// Determains if the IRXM flag is set.
    ///
    /// - 0: Normal Operation
    /// - 1: Interrupt Condition occurred
    clear_irxm_interrupt_flag,
    /// # Is IRXM Interrupt Flag
    /// Determains if the IRXM flag is set.
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
    /// # Set Out Of Sequence STOP condidtion Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    set_out_of_sequence_stop_condition_interrupt_enable,
    /// # Is Out Of Sequence STOP condidtion Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    is_out_of_sequence_stop_condition_interrupt_enabled}

    bit_impl! {13, RW,
    /// # Set Out-Of-Sequence START condidtion Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    set_out_of_sequence_start_condidtion_interrupt_enable,
    /// # Is Out-Of-Sequence START condidtion Interrupt Enable
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
    /// # Set Reveive FIFO Threshold Level Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    set_receive_fifo_threshold_level_interrupt_enable,
    /// # Is Reveive FIFO Threshold Level Interrupt Enable
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
    /// - 1: START condidtion has been detected
    clear_start_condition_flag,
    /// # Is START Condition Flag
    /// The I2C hardware will set this flag if it detects a START condition on the bus.
    ///
    /// - 0: START condition has not been detected
    /// - 1: START condidtion has been detected
    is_start_condidtion_flag}

    bit_impl! {1, RW1C,
    /// # Clear Slave Mode Transmit FIFO Underflow Flag
    /// While the device is configured for slave mode operation, the hardware will enable this flag
    /// if the transmit FIFO is currently empty and the bus master requests more data by sending an ACK
    /// directly after the previous byte transfer is complete.
    ///
    /// - 0: Slave Mode FIFO has not had an underflow
    /// - 1: Slave Mode FIFO has underflowed
    clear_slave_mode_transmit_fifo_underflow_flag,
    /// # Is Slave Mode Transmit FIFO Underflow Flag
    /// While the device is configured for slave mode operation, the hardware will enable this flag
    /// if the transmit FIFO is currently empty and the bus master requests more data by sending an ACK
    /// directly after the previous byte transfer is complete.
    ///
    /// - 0: Slave Mode FIFO has not had an underflow
    /// - 1: Slave Mode FIFO has underflowed
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

/// # i2C DMA Enable Register
/// The DMA control register used to control direct memory accessing for the I2C bus, page 237 (MAX78000 User Guide)
pub struct DMAControl<const PORT_PTR: usize> {}
reg_impl!(RW, DMAControl, rro::I2C_DMA_OFFSET);

/// # I2C Slave Address Register
/// The slave address register is used to control the addressing mode of the bus, page 237-238 (MAX78000 User Guide)
pub struct SlaveAddress<const PORT_PTR: usize> {}
reg_impl!(RW, SlaveAddress, rro::I2C_SLAVE_OFFSET);
