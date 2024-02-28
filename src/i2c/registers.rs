use hal_macros::RW;
use hal_macros_derive::make_device;

/// # Relative Register Offsets
/// These are the offsets for the I2C registers that the
/// Maxim Integrated - spec shows. Found on page 224.
mod rro {
    /// # I2C Control Register
    pub const I2C_CTRL: usize = 0x0000;
    /// # I2C Status Register
    pub const I2C_STATUS: usize = 0x0004;
    /// # I2C Interrupt Flags 0 Register
    pub const I2C_INTFL0: usize = 0x0008;
    /// # I2C Interrupt Enable 0 Register
    pub const I2C_INTEN0: usize = 0x000C;
    /// # I2C Interrupt Flags 1 Register
    pub const I2C_INTFL1: usize = 0x0010;
    /// # I2C Interrupt Enable 1 Register
    pub const I2C_INTEN1: usize = 0x0014;
    /// # I2C FIFO Length Register
    pub const I2C_FIFOLEN: usize = 0x0018;
    /// # I2C Receive Control 0 Register
    pub const I2C_RXCTRL0: usize = 0x001C;
    /// # I2C Receive Control 1 Register
    pub const I2C_RXCTRL1: usize = 0x0020;
    /// # I2C Transmit Control 0 Register
    pub const I2C_TXCTRL0: usize = 0x0024;
    /// # I2C Transmit Control 1 Register
    pub const I2C_TXCTRL1: usize = 0x0028;
    /// # I2C Transmit and Receive FIFO Register
    pub const I2C_FIFO: usize = 0x002C;
    /// # I2C Master Control Register
    pub const I2C_MSTCTRL: usize = 0x0030;
    /// # I2C Clock Low Time Register
    pub const I2C_CLKLO: usize = 0x0034;
    /// # I2C Clock High Time Register
    pub const I2C_CLKHI: usize = 0x0038;
    /// # I2C High Speed Clock Control Register
    pub const I2C_HSCLK: usize = 0x003C;
    /// # I2C Timeout Register
    pub const I2C_TIMEOUT: usize = 0x0040;
    /// # I2C DMA Register
    pub const I2C_DMA: usize = 0x0048;
    /// # I2C Slave Register
    pub const I2C_SLAVE: usize = 0x004C;
}

make_device! {
    device_ports(crate::memory_map::mmio::I2C_PORT_0, crate::memory_map::mmio::I2C_PORT_1, crate::memory_map::mmio::I2C_PORT_2);

    /// The I2C entire control register field.
    #[bit(0..=15, RW, rro::I2C_CTRL)]
    control_register,

    /// The entire I2C interrupt flags 0 register.
    #[bit(0..=31, RW, rro::I2C_INTFL0)]
    interrupt_flags_0,

    /// The entire I2C interrupt flags 1 register.
    #[bit(0..=31, RW, rro::I2C_INTFL1)]
    interrupt_flags_1,

    /// Set I2C to high speed mode, or set it to low speed mode.
    /// 0: Disabled
    /// 1: Enabled
    #[bit(15, RW, rro::I2C_CTRL)]
    high_speed_mode,

    /// One Master Mode
    /// Set if the controller is going to be using one master mode. When set
    /// to true, the device must only be used with slave devices. No other
    /// masters should be attached to the bus. When using one master mode,
    /// it must also be true that no slave devices will hold SCL low for
    /// any given reason (i.e clock stretching).
    ///
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(13, RW, rro::I2C_CTRL)]
    one_master_mode,

    /// Disable Slave Clock Stretching
    /// Sets if slave clock stretching will be disabled. In this mode, it must
    /// also be true that `one_master_mode` must also be set since slave devices
    /// will be pulling SCL low.
    ///
    /// - 0: Enabled
    /// - 1: Disabled
    #[bit(12, RW, rro::I2C_CTRL)]
    disable_slave_clock_stretching,

    /// Write Bit Status
    /// Get the logic level of the R/W bit on a received address match.
    #[bit(11, RO, rro::I2C_CTRL)]
    read_write_bit_status,

    /// Software I2C Mode
    /// Tell the controller to either use software mode (i.e the SCL and SDA are managed
    /// by the software) or to use the on-board I2C controller hardware. This does not mean
    /// that the on-board I2C controller will do all communication by itself, more, it means
    /// the on-board hardware will generate basic I2C based signals (provided you tell it to).
    ///
    /// - 0: The I2C controller will manage I2C in hardware.
    /// - 1: SDA and SCL will need to be "bit-banged" by software by setting them manually.
    #[bit(10, RW, rro::I2C_CTRL)]
    software_i2c_mode,

    /// SDA Pin
    /// Get the `SDA` pin status, whether it be high or low.
    ///
    /// - 0: The `SDA` pin is logic low
    /// - 1: The `SDA` pin is logic high
    #[bit(9, RO, rro::I2C_CTRL)]
    sda_pin,

    /// SCL Pin
    /// Get the `SCL` pin status, whether it be high or low.
    ///
    /// - 0: The `SCL` pin is logic low
    /// - 1: The `SCL` pin is logic high
    #[bit(8, RO, rro::I2C_CTRL)]
    scl_pin,

    /// SDA Hardware Pin Released
    /// Set the state of the SDA hardware pin. (Actively pull the pin low, or leave it floating). This
    /// mode is only active during `software_i2c_mode_enabled`, and other state is to be undefined.
    ///
    /// - 0: Actively Pull SDA Low
    /// - 1: Leave SDA floating
    #[bit(7, RW, rro::I2C_CTRL)]
    sda_hardware_pin_released,

    /// SCL Hardware Pin Released
    /// Set the state of the SCL hardware pin. (Actively pull the pin low, or leave it floating). This
    /// mode is only active during `software_i2c_mode_enabled`, and other state is to be undefined.
    ///
    /// - 0: Actively Pull SCL Low
    /// - 1: Leave SCL floating
    #[bit(6, RW, rro::I2C_CTRL)]
    scl_hardware_pin_released,

    /// IRXM Response `NACK`
    /// If the IRXM is currently enabled, this will set if the IRXM response will be an `ACK`, or
    /// a `NACK`. This also requires that the IRXM be enabled.
    ///
    /// - 0: Respond to IRXM with `ACK`
    /// - 1: Respond to IRXM with `NACK`
    #[bit(4, RW, rro::I2C_CTRL)]
    irxm_responce_nack,

    /// if IRXM will be Enabled
    /// When currently receiving data, the IRXM will allow for interactive receive mode (IRXM)
    /// interrupts for each byte of data received. Configuration of if the hardware will send
    /// an `ACK` to IRXM is with `irxm_response_nack`.
    ///
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(3, RW, rro::I2C_CTRL)]
    irxm_enable,

    /// Acknowledge General Call
    /// Set the I2C controller to acknowledge the general call address, and respond with ACK.
    ///
    /// - 0: Ignore General Call Address
    /// - 1: Acknowledge General Call Address
    #[bit(2, RW, rro::I2C_CTRL)]
    acknowledge_general_call,

    /// Master Mode Enabled
    /// Sets the controller to be the bus master on the I2C bus. Only some operations
    /// are available during master mode. It is important to know which mode the
    /// controller is in to use other functions.
    ///
    /// - 0: Slave Mode Enabled
    /// - 1: Master Mode Enabled
    #[bit(1, RW, rro::I2C_CTRL)]
    master_mode_enable,

    /// I2C Peripheral Enable
    /// Enables the I2C bus. Allows communication to be sent over to I2C peripherals.
    ///
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(0, RW, rro::I2C_CTRL)]
    i2c_peripheral_enable,

    /// Master Busy
    /// When the controller is in master mode, and is currently processing a transaction,
    /// then this flag will be set for the duration that `START` until `STOP`.
    ///
    /// - 0: Device is currently not driving the SCL clock
    /// - 1: Device is currently driving the SCL clock
    #[bit(5, RO, rro::I2C_STATUS)]
    master_busy,

    /// Transmit FIFO Full
    /// Checks to see if the transmit FIFO is currently full.
    ///
    /// - 0: Not Full
    /// - 1: Full
    #[bit(4, RO, rro::I2C_STATUS)]
    transmit_fifo_full,

    /// Transmit FIFO Empty
    /// Checks to see if the transmit FIFO is currently empty.
    ///
    /// - 0: Not Empty
    /// - 1: Empty
    #[bit(3, RO, rro::I2C_STATUS)]
    transmit_fifo_empty,

    /// Receive FIFO Full
    /// Checks to see if the current receive FIFO is full.
    ///
    /// - 0: Not Full
    /// - 1: Full
    #[bit(2, RO, rro::I2C_STATUS)]
    receive_fifo_full,

    /// Receive FIFO Empty
    /// Checks to see if the current FIFO is currently empty.
    ///
    /// - 0: Not Empty
    /// - 1: Empty
    #[bit(1, RO, rro::I2C_STATUS)]
    receive_fifo_empty,

    /// Transaction Active
    /// Checks to see (either in master or slave mode) if the I2C controller is currently
    /// processing a transaction.
    ///
    /// - 0: I2C bus is idle
    /// - 1: I2C bus is busy
    #[bit(0, RO, rro::I2C_STATUS)]
    transaction_active,

    /// I2C has had an error on the Interrupt Flag 0 Register
    #[bit(8..=14, RW, rro::I2C_INTFL0)]
    error_condition,

    /// Slave Write Address Match Interrupt
    /// If this bit is set, the current device (currently configured for slave mode) has just been accessed for a write (ie. receive)
    /// and the address requested matches our I2C address (the bus is talking to us).
    ///
    /// - 0: No Address Match
    /// - 1: Address Match
    #[bit(23, RW1C, rro::I2C_INTFL0)]
    slave_write_addr_match_interrupt,

    /// Slave Write Address Match Interrupt
    /// If this bit is set, the current device (currently configured for slave mode) has just been accessed for a read (ie. write)
    /// and the address requested matches our I2C address (the bus is talking to us).
    ///
    /// - 0: No Address Match
    /// - 1: Address Match
    #[bit(22, RW1C, rro::I2C_INTFL0)]
    slave_read_addr_match_interrupt,

    /// MAMI Interrupt Flag
    #[bit(16, RW1C, rro::I2C_INTFL0)]
    mami_interrupt_flag,

    /// Transmit FIFO Locked
    /// If this flag is set, the transmit FIFO is currently locked. If any more data is pushed to the transmit FIFO, it will be
    /// ignored. The flag must be cleared for writes to be valid. While this register is set, the transmit FIFO is automatically flushed.
    ///
    /// - 0: Transmit FIFO is not locked
    /// - 1: Transmit FIFO is currently locked
    #[bit(15, RW1C, rro::I2C_INTFL0)]
    transmit_fifo_locked,

    /// Out Of Sequence STOP flag
    /// If this flag is set, a STOP condition occurred out of sequence.
    ///
    /// - 0: Normal Operation
    /// - 1: Out of sequence STOP condition occurred
    #[bit(14, RW1C, rro::I2C_INTFL0)]
    out_of_sequence_stop_flag,

    /// Out Of Sequence START Flag
    /// If this flag is set, a START condition occurred out of sequence.
    ///
    /// - 0: Normal Operation
    /// - 1: Out of sequence START condition occurred
    #[bit(13, RW1C, rro::I2C_INTFL0)]
    out_of_sequence_start_flag,

    /// Slave Mode Do-Not-Respond
    /// If this flag is set the controller received an address match, but the transmit FIFO or receive FIFO are not ready.
    ///
    /// - 0: Normal Operation
    /// - 1: FIFO not configured
    #[bit(12, RW1C, rro::I2C_INTFL0)]
    slave_mode_do_not_respond,

    /// Master Data NACK from External Slave Error
    /// If this flag is set, the current device has received a NACK from a slave device (only if the current device is
    /// configured to be in master mode).
    ///
    /// - 0: Normal Operation
    /// - 1: Data NACK received from a Slave
    #[bit(11, RW1C, rro::I2C_INTFL0)]
    master_data_nack_from_slave_err,

    /// Master Address NACK from Slave Error
    /// If this flag is set, the current device has received a NACK from a slave device (only if the current device is
    /// configured to be in master mode).
    ///
    /// - 0: Normal Operation
    /// - 1: Address NACK received from a Slave
    #[bit(10, RW1C, rro::I2C_INTFL0)]
    master_address_nack_from_slave_err,

    /// Timeout Error Flag
    /// If this flag is set, the current device has held SCL low for longer than the timeout value. This is valid either
    /// in master or slave operation.
    ///
    /// - 0: Normal Operation
    /// - 1: Timeout occurred
    #[bit(9, RW1C, rro::I2C_INTFL0)]
    timeout_error_flag,

    /// Master Mode Arbitration Lost
    /// If this flag is set than the device has lost arbitration.
    ///
    /// - 0: Normal Operation
    /// - 1: Condition occurred
    #[bit(8, RW1C, rro::I2C_INTFL0)]
    master_mode_arbitration_lost,

    /// Master ACK from External Slave
    /// If this flag is set, then this device (currently configured to be bus master) has just received an ACK from
    /// a slave device.
    ///
    /// - 0: Normal Operation
    /// - 1: ACK Received
    #[bit(7, RW1C, rro::I2C_INTFL0)]
    master_ack_from_external_slave,

    /// Slave Mode STOP Condition
    /// When this flag is set, the hardware noticed a STOP condition.
    ///
    /// - 0: Normal Operation
    /// - 1: STOP condition occurred
    #[bit(6, RW1C, rro::I2C_INTFL0)]
    slave_mode_stop_condition,

    /// Transmit FIFO Threshold Level
    /// (MAYBE ERROR IN DOCUMENTATION PAGE 228 MAX78000 USER GUIDE)
    ///
    /// When this flag is set, the transmit FIFO has less then or equal to the number of threshold bytes set. This
    /// flag is automatically cleared when the transmit FIFO contains (MORE/LESS) bytes then the threshold level.
    ///
    /// - 0: Transmit FIFO contains more bytes than the transmit threshold level.
    /// - 1: Transmit FIFO contains less bytes than the transmit threshold level.
    #[bit(5, RW1C, rro::I2C_INTFL0)]
    transmit_fifo_threshold_level,

    /// Receive FIFO Threshold Level
    /// (MAYBE ERROR IN DOCUMENTATION PAGE 228 MAX78000 USER GUIDE)
    ///
    /// When this flag is set, the receive FIFO has less then or equal to the number of threshold bytes set. This
    /// flag is automatically cleared when the receive FIFO contains (MORE/LESS) bytes then the threshold level.
    ///
    /// - 0: Receive FIFO contains more bytes than the transmit threshold level.
    /// - 1: Receive FIFO contains less bytes than the transmit threshold level.
    #[bit(4, RW1C, rro::I2C_INTFL0)]
    receive_fifo_threshold_level,

    /// Slave mode Incoming Address Match Status
    /// If the controller is configured for Slave mode, the hardware will set this flag is the incoming address
    /// has been matched to ours. (Depends on this device being configured for Slave Mode)
    ///
    /// - 0: Slave Mode Address Match has not occurred
    /// - 1: Slave Mode Address Match has occurred
    #[bit(3, RW1C, rro::I2C_INTFL0)]
    slave_incoming_address_match_status,

    /// Slave General Call Address Match Received
    /// If the controller is configured for Slave mode, the hardware will set this flag if the general call
    /// address match has occurred. (Depends on this device being configured for Slave Mode)
    ///
    /// - 0: Normal Operation
    /// - 1: General Call Address Match occurred
    #[bit(2, RW1C, rro::I2C_INTFL0)]
    slave_general_call_address_match_received,

    /// IRXM Interrupt Flag
    /// Determines if the IRXM flag is set.
    ///
    /// - 0: Normal Operation
    /// - 1: Interrupt Condition occurred
    #[bit(1, RW1C, rro::I2C_INTFL0)]
    irxm_interrupt_flag,

    /// Transfer Complete Flag
    /// The controller sets this flag when the current transaction has completed. This flag is both valid for
    /// slave mode transfer and for master mode transfer.
    ///
    /// - 0: Transfer is not complete
    /// - 1: Transfer is complete
    #[bit(0, RW1C, rro::I2C_INTFL0)]
    transfer_complete_flag,

    /// Slave Write Address Match Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    #[bit(23, RW, rro::I2C_INTEN0)]
    slave_write_address_match_interrupt_enable,

    /// Slave Read Address Match Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    #[bit(22, RW, rro::I2C_INTEN0)]
    slave_read_address_match_interrupt_enable,

    /// MAMI Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    #[bit(16, RW, rro::I2C_INTEN0)]
    mami_interrupt_enable,

    /// Transmit FIFO lock-out Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    #[bit(15, RW, rro::I2C_INTEN0)]
    transmit_fifo_lock_out_interrupt_enable,

    /// Out Of Sequence STOP condition Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    #[bit(14, RW, rro::I2C_INTEN0)]
    out_of_sequence_stop_condition_interrupt_enable,

    /// Out-Of-Sequence START condition Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    #[bit(13, RW, rro::I2C_INTEN0)]
    out_of_sequence_start_condidtion_interrupt_enable,

    /// Slave Mode Do-Not-Repsond Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    #[bit(12, RW, rro::I2C_INTEN0)]
    slave_mode_do_not_respond_interrupt_enable,

    /// Master Received Data NACK from Slave Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    #[bit(11, RW, rro::I2C_INTEN0)]
    master_received_data_nack_from_slave_interrupt_enable,

    /// Master Received Address NACK from Slave Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    #[bit(10, RW, rro::I2C_INTEN0)]
    master_received_address_nack_from_slave_interrupt_enable,

    /// Timeout Error Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    #[bit(9, RW, rro::I2C_INTEN0)]
    timeout_error_interrupt_enable,

    /// Master Mode Arbitration Lost Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    #[bit(8, RW, rro::I2C_INTEN0)]
    master_mode_arbitration_lost_interrupt_enable,

    /// Received Address ACK from Slave Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    #[bit(7, RW, rro::I2C_INTEN0)]
    received_address_ack_from_slave_interrupt_enable,

    /// Stop Condition Detected Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    #[bit(6, RW, rro::I2C_INTEN0)]
    stop_condition_detected_interrupt_enable,

    /// Transmit FIFO Threshold Level Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    #[bit(5, RW, rro::I2C_INTEN0)]
    transmit_fifo_threshold_level_interrupt_enable,

    /// Receive FIFO Threshold Level Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    #[bit(4, RW, rro::I2C_INTEN0)]
    receive_fifo_threshold_level_interrupt_enable,

    /// Slave Mode Incoming Address Match Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    #[bit(3, RW, rro::I2C_INTEN0)]
    slave_mode_incoming_address_match_interrupt_enable,

    /// Slave General Call Address Match Received Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    #[bit(2, RW, rro::I2C_INTEN0)]
    slave_general_call_address_match_received_interrupt_enable,

    /// IRXM Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    #[bit(1, RW, rro::I2C_INTEN0)]
    irxm_interrupt_enable,

    /// Transfer Complete Interrupt Enable
    /// Set this interrupt to fire if the selected condition is met.
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    #[bit(0, RW, rro::I2C_INTEN0)]
    transfer_complete_interrupt_enable,

    /// START Condition Flag
    /// The I2C hardware will set this flag if it detects a START condition on the bus.
    ///
    /// - 0: START condition has not been detected
    /// - 1: START condition has been detected
    #[bit(2, RW1C, rro::I2C_INTFL1)]
    start_condition_flag,

    /// Slave Mode Transmit FIFO Underflow Flag
    /// While the device is configured for slave mode operation, the hardware will enable this flag
    /// if the transmit FIFO is currently empty and the bus master requests more data by sending an ACK
    /// directly after the previous byte transfer is complete.
    ///
    /// - 0: Slave Mode FIFO has not had an underflow
    /// - 1: Slave Mode FIFO has underflow
    #[bit(1, RW1C, rro::I2C_INTFL1)]
    slave_mode_transmit_fifo_underflow_flag,

    /// Slave Mode Receive FIFO Overflow Flag
    /// While the device is configured for slave mode operation, the hardware will enable this flag
    /// if the receive FIFO is currently full and the bus master sent us data. (DATA LOSS)
    ///
    /// - 0: Slave Mode FIFO has not overflowed
    /// - 1: Slave Mode FIFO has overflowed (DATA HAS BEEN LOST)
    #[bit(0, RW1C, rro::I2C_INTFL1)]
    slave_mode_receive_fifo_overflow_flag,

    /// Start Condition Interrupt Enable
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    #[bit(2, RW, rro::I2C_INTEN1)]
    start_condition_interrupt_enable,

    /// Slave Mode Transmit FIFO Underflow Interrupt Enable
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    #[bit(1, RW, rro::I2C_INTEN1)]
    slave_mode_transmit_fifo_underflow_interrupt_enable,

    /// Slave Mode Receive FIFO Overflow Interrupt Enable
    ///
    /// - 0: Interrupts Disabled
    /// - 1: Interrupts Enabled
    #[bit(0, RW, rro::I2C_INTEN1)]
    slave_mode_receive_fifo_overflow_interrupt_enable,

    /// Transmit FIFO Length
    /// Get the current transmit FIFO depth in bytes.
    #[bit(8..=15, RO, rro::I2C_FIFOLEN)]
    transmit_fifo_len,

    /// Receive FIFO Length
    /// Get the current receive FIFO depth in bytes.
    #[bit(0..=7, RO, rro::I2C_FIFOLEN)]
    receive_fifo_len,

    /// Receive FIFO Threshold Level
    /// This is the number of bytes to trigger a receive FIFO threshold event. If the bytes in the FIFO are greater than or equal to
    /// this value, the hardware will generate an interrupt (if enabled) and set `InterruptFlag0::is_receive_fifo_threshold_level_interrupt_enabled` to
    /// true.
    ///
    /// - 0: 0 bytes or more causes an event
    /// - 1: 1 bytes or more causes an event
    /// -  ...
    /// - 8: 8 bytes (only when the FIFO is full)
    #[bit(8..=11, RW, rro::I2C_RXCTRL0)]
    receive_fifo_threshold_level,

    /// Flush Receive FIFO
    /// When activated, this will initiate a receive FIFO flush. The hardware will then clear all the data in the receive FIFO. Among finishing
    /// the hardware will set this flag back to `0`.
    ///
    /// - 0: Receive FIFO flush complete (or not started)
    /// - 1: Flushing the Receive FIFO
    #[bit(7, RW1O, rro::I2C_RXCTRL0)]
    receive_fifo_flush,

    /// Slave Do-Not-Respond
    /// If this device (configured only in slave mode operation) has just been addressed for a write operation, and the receive FIFO is not
    /// empty the device will respond with a NACK.
    ///
    /// - 0: ACK the address, but NACK the data
    /// - 1: NACK the address
    #[bit(0, RW, rro::I2C_RXCTRL0)]
    slave_do_not_respond,

    /// Current Receive FIFO Bytes
    /// Get the current number of bytes in the receive FIFO.
    ///
    /// - 0: 0 bytes (No data)
    /// - 1: 1 byte
    ///  ...
    /// - 8: 8 bytes
    #[bit(8..=11, RO, rro::I2C_RXCTRL1)]
    current_receive_fifo_bytes,

    /// Receive FIFO Transaction Size
    /// Write the number of bytes to be received in a transaction (when device is configured to be in master mode).
    ///
    /// - 0: 256 byte receive transaction
    /// - 1: 1 byte receive transaction
    /// ...
    /// - 255: 255 byte receive transaction
    #[bit(0..=7, RW, rro::I2C_RXCTRL1)]
    receive_fifo_transaction_size,

    /// Transmit FIFO Threshold Level
    /// Sets the number of bytes that are required to trigger an interrupt (if that interrupt is enabled) and
    /// set the flag. The number of bytes must be smaller or equal to this value for such an interrupt to occur.
    ///
    /// - 0: 0 or fewer bytes triggers event
    /// - 1: 1 or fewer bytes triggers event
    /// ...
    /// - 7: 7 or fewer bytes triggers event
    #[bit(8..=11, RW, rro::I2C_TXCTRL0)]
    transmit_fifo_threshold_level,

    /// Transmit FIFO Flush
    /// A transmit FIFO flush will clear all data from the transmit FIFO.
    ///
    /// - 0: Transmit FIFO flush is complete (or is not active).
    /// - 1: The Transmit FIFO Flush is currently being serviced
    #[bit(7, RW1O, rro::I2C_TXCTRL0)]
    transmit_fifo_flush,

    /// Transmit FIFO Received NACK Auto Flush Disable
    /// There are some cases in which other registers will cause a transmit FIFO flush. In such a case, one of the following
    /// values will be true.
    ///
    /// - 0: Received NACK at the end of a slave transmit operation enabled
    /// - 1: Received NACK at the end of a slave transmit operation disabled
    #[bit(5, RW, rro::I2C_TXCTRL0)]
    transmit_fifo_received_nack_auto_flush_disable,

    /// Transmit FIFO Slave Address Match Read Auto Flush Disable
    /// There are some cases in which other registers will cause a transmit FIFO flush. In such a case, one of the following
    /// values will be true.
    ///
    /// - 0: Enabled
    /// - 1: Disabled
    #[bit(4, RW, rro::I2C_TXCTRL0)]
    transmit_fifo_slave_address_match_read_auto_flush_disable,

    /// Transmit FIFO Slave Address Match Write Auto Flush Disable
    /// There are some cases in which other registers will cause a transmit FIFO flush. In such a case, one of the following
    /// values will be true.
    ///
    /// - 0: Enabled
    /// - 1: Disabled
    #[bit(3, RW, rro::I2C_TXCTRL0)]
    transmit_fifo_slave_address_match_write_auto_flush_disable,

    /// Transmit FIFO General Call Address Match Auto Flush Disable
    /// There are some cases in which other registers will cause a transmit FIFO flush. In such a case, one of the following
    /// values will be true.
    ///
    /// - 0: Enabled
    /// - 1: Disabled
    #[bit(2, RW, rro::I2C_TXCTRL0)]
    transmit_fifo_general_call_address_match_auto_flush_disable,

    /// Transmit FIFO Read Manual Mode
    /// Disable or enable the hardware from controlling the `preload ready` flag. When enabled, it allows software to only
    /// control this flag.
    ///
    /// - 0: Hardware Controls Preload Ready
    /// - 1: Software Controls Preload Ready
    #[bit(1, RW, rro::I2C_TXCTRL0)]
    transmit_fifo_read_manual_mode,

    /// Transmit FIFO Preload Mode Enable
    /// The following conditions are held with this flag.
    ///
    /// - 0: An Address match in slave mode, or a general call address does lock the transmit FIFO.
    /// - 1: Transmit FIFO preload mode. An address match in slave mode does not lock the transmit FIFO.
    #[bit(0, RW, rro::I2C_TXCTRL0)]
    transmit_fifo_preload_mode_enable,

    /// Transmit FIFO Byte Count
    /// Get the current number of bytes that reside in the transmit FIFO.
    ///
    /// - 0: 0 bytes (no data)
    /// - 1: 1 byte
    /// ...
    /// - 8: 8 bytes (FIFO full)
    #[bit(8..=11, RO, rro::I2C_TXCTRL1)]
    transmit_fifo_byte_count,

    /// Transmit FIFO Preload Ready (page 235)
    #[bit(0, RW1O, rro::I2C_TXCTRL1)]
    // TODO: Finish this documentation
    transmit_fifo_preload_ready,

    /// Write FIFO Data
    /// Write to the transmit FIFO (pushes the data onto the transmit FIFO).
    ///
    /// If the FIFO is full, this operation is ignored (the data will be lost).
    #[bit(0..=7, RW, rro::I2C_FIFO)]
    fifo_data,

    /// I2C Master Control Register
    /// The master control register is used to control the bus when the device is configured to be the master, page 235-236 (MAX78000 User Guide)
    /// MCODE
    /// This property sets the master code used in HS-Mode operation.
    #[bit(8..=10, RW, rro::I2C_MSTCTRL)]
    mcode,

    /// Slave Extended Addressing
    /// Sets the master to enable slave extended bit addressing, this allows up to 10-bit addresses for slave devices.
    ///
    /// - 0: 7-bit Addressing (The most used and common)
    /// - 1: 10-bit Addressing
    #[bit(7, RW, rro::I2C_MSTCTRL)]
    slave_extended_addressing,

    /// Send STOP Condition
    /// Tell the master to send a STOP condition at the end of the current transaction.
    #[bit(2, RW1O, rro::I2C_MSTCTRL)]
    send_stop_condition,

    /// Send Repeated START Condition
    /// After sending data to a slave device, the master will send another START to retain control over the bus.
    #[bit(1, RW1O, rro::I2C_MSTCTRL)]
    send_repeated_start_condition,

    /// Start Master Mode Transfer
    /// Start a master mode transfer over the I2C bus.
    #[bit(0, RW1O, rro::I2C_MSTCTRL)]
    start_master_mode_transfer,

      /// Clock Low Time
    /// Sets the current clock low time for `SCL`. Please use page 236 of the MAX78000 User Guide to determine
    /// the math in setting this value.
    #[bit(0..=8, RW, rro::I2C_CLKLO)]
    clock_low_time,

    /// Clock High Time
    /// Sets the current clock High time for `SCL`. Please use page 236 of the MAX78000 User Guide to determine
    /// the math in setting this value.
    #[bit(0..=8, RW, rro::I2C_CLKHI)]
    clock_high_time,

    /// High Speed Mode Clock High Time
    /// Sets the high time duration for high speed mode on the I2C bus.
    #[bit(8..=15, RW, rro::I2C_HSCLK)]
    high_speed_mode_clock_high_time,

    /// High Speed Mode Clock Low Time
    /// Sets the low time duration for high speed mode on the I2C bus.
    #[bit(0..=7, RW, rro::I2C_HSCLK)]
    high_speed_mode_clock_low_time,

    /// Bus Error SCL Timeout Period
    /// Sets the time that the SCL will be inactive after an error as occurred. Please use page 237
    /// on the MAX78000 User Guide to determine the calculation.
    #[bit(0..=15, RW, rro::I2C_TIMEOUT)]
    bus_error_scl_timeout_period,

     /// Receive DMA Channel Enable
    /// Enable the DMA Receive channel.
    ///
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(1, RW, rro::I2C_DMA)]
    receive_dma_channel_enable,

    /// Transmit DMA Channel Enable
    /// Enable the DMA Transmit channel.
    ///
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(0, RW, rro::I2C_DMA)]
    transmit_dma_channel_enable,

    /// Slave Mode Extended Address Length Select
    /// Set if (while in slave mode) to use the address extension.
    ///
    /// - 0: 7-bit addressing (the most used and normal one)
    /// - 1: 10-bit addressing
    #[bit(15, RW, rro::I2C_SLAVE)]
    slave_mode_extended_address_length_select,

    /// Slave Mode Address
    /// Sets the address of this device (must be configured to be in slave mode).
    ///
    /// Take note: There are a few reserved addresses!
    #[bit(0..=9, RW, rro::I2C_SLAVE)]
    slave_mode_address,
}
