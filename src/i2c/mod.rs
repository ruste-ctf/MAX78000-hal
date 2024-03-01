use crate::error::{ErrorKind, Result};
use crate::gcr::{peripheral_reset, system_clock_enable};
use crate::gpio::GpioPin;
use crate::memory_map::mmio;
use crate::{core_peripheral_clock, debug_print, debug_println};
use core::marker::PhantomData;

use self::registers::Registers;

pub mod registers;

mod private {
    pub trait I2CPortCompatable {
        const PORT_PTR: usize;
        const PORT_NUM: usize;
    }
}

pub struct NoPort {}
pub struct I2CPort0 {}
pub struct I2CPort1 {}
pub struct I2CPort2 {}

impl private::I2CPortCompatable for I2CPort0 {
    const PORT_PTR: usize = mmio::I2C_PORT_0;
    const PORT_NUM: usize = 0;
}
impl private::I2CPortCompatable for I2CPort1 {
    const PORT_PTR: usize = mmio::I2C_PORT_1;
    const PORT_NUM: usize = 1;
}
impl private::I2CPortCompatable for I2CPort2 {
    const PORT_PTR: usize = mmio::I2C_PORT_2;
    const PORT_NUM: usize = 2;
}

#[allow(dead_code)]
pub struct I2C<Port = NoPort> {
    reg: Registers,
    master_enabled: bool,
    slave_address: usize,
    gpio: [GpioPin; 2],
    slave_underflow: bool,
    _ph: PhantomData<Port>,
}

/// # I2C Bus Control Event
/// Send a event marker to the I2C bus.
///
/// Event markers are messages (really just state changes) that significant
/// a device that some operation is occurring. For example, you can send
/// either `START`, `RESTART` (kinda), or `STOP` over the I2C bus.
///
/// # `START`
/// When sending a start event, devices on the I2C bus are ready and waiting
/// to receive their address. If a device is addressed (with RW bit) then it
/// will be the one that is going to handle this transmission.
///
/// # `RESTART`
/// When sending a restart event, devices on the I2C bus will not first be prompted
/// to go into idle (if the device supports it). This is usually faster for slave devices
/// to respond, so when sending a command immediately after another its a good idea
/// to send `RESTART`
///
/// # `STOP`
/// This signifies to the devices on the bus that this communication frame is over
/// and the bus can go back to idle.
///
pub enum I2CBusControlEvent {
    StartOrRestart,
    /// # `START`
    /// When sending a start event, devices on the I2C bus are ready and waiting
    /// to receive their address. If a device is addressed (with RW bit) then it
    /// will be the one that is going to handle this transmission.
    Start,
    /// # `RESTART`
    /// When sending a restart event, devices on the I2C bus will not first be prompted
    /// to go into idle (if the device supports it). This is usually faster for slave devices
    /// to respond, so when sending a command immediately after another its a good idea
    /// to send `RESTART`
    Restart,
    /// # `STOP`
    /// This signifies to the devices on the bus that this communication frame is over
    /// and the bus can go back to idle.
    Stop,
}

pub enum SlaveStatus {
    None,
    IncomingRequest { is_write: bool },
    TransmitFIFOLocked,
    WriteRequested,
    ReadRequested,
    Stop,
    TransferDone,
}

#[derive(Debug)]
pub enum MasterStatus {
    None,
    SlaveAck,
    SlaveNack,
    WriteRequested,
    ReadRequested,
    TransferDone,
    NextReadChunkRequested,
}

pub enum MasterCommand {
    StartWrite { address: usize },
    StartRead { address: usize, read_amount: usize },
    Stop,
}

const MAX_I2C_SLAVE_ADDRESS_7_BIT: usize = 0b1111111;
const MAX_I2C_SLAVE_ADDRESS_10_BIT: usize = 0b1111111111;

#[allow(unused)]
const MAX_I2C_NORMAL_CLOCK_HZ: usize = 100000;
#[allow(unused)]
const MAX_I2C_FAST_CLOCK_HZ: usize = 400000;
const MAX_I2C_FASTPLUS_CLOCK_TIME: usize = 1000000;
const MAX_I2C_HIGHSPEED_CLOCK_TIME: usize = 3400000;

const MAX_TRANSMIT_FIFO_LEN: usize = 8;

fn microcontroller_delay(_us: usize) {
    for _ in 0..100000 {
        unsafe { core::arch::asm!("nop") }
    }
}

impl I2C<NoPort> {
    pub fn init_port_0_master() -> Result<I2C<I2CPort0>> {
        peripheral_reset(crate::gcr::HardwareSource::I2C0);
        system_clock_enable(crate::gcr::HardwareSource::I2C0, true);
        I2C::<I2CPort0>::init(true, 0x00)
    }

    pub fn init_port_1_master() -> Result<I2C<I2CPort1>> {
        peripheral_reset(crate::gcr::HardwareSource::I2C1);
        system_clock_enable(crate::gcr::HardwareSource::I2C1, true);
        I2C::<I2CPort1>::init(true, 0x00)
    }

    pub fn init_port_2_master() -> Result<I2C<I2CPort2>> {
        peripheral_reset(crate::gcr::HardwareSource::I2C2);
        system_clock_enable(crate::gcr::HardwareSource::I2C2, true);
        I2C::<I2CPort2>::init(true, 0x00)
    }

    pub fn init_port_0_slave(address: usize) -> Result<I2C<I2CPort0>> {
        peripheral_reset(crate::gcr::HardwareSource::I2C0);
        system_clock_enable(crate::gcr::HardwareSource::I2C0, true);
        I2C::<I2CPort0>::init(false, address)
    }

    pub fn init_port_1_slave(address: usize) -> Result<I2C<I2CPort1>> {
        peripheral_reset(crate::gcr::HardwareSource::I2C1);
        system_clock_enable(crate::gcr::HardwareSource::I2C1, true);
        I2C::<I2CPort1>::init(false, address)
    }

    pub fn init_port_2_slave(address: usize) -> Result<I2C<I2CPort2>> {
        peripheral_reset(crate::gcr::HardwareSource::I2C2);
        system_clock_enable(crate::gcr::HardwareSource::I2C2, true);
        I2C::<I2CPort2>::init(false, address)
    }
}

#[allow(unused)]
impl<Port: private::I2CPortCompatable> I2C<Port> {
    fn init(master_enabled: bool, slave_address: usize) -> Result<Self> {
        let mut i2c = Self {
            reg: Registers::new(Port::PORT_PTR),
            slave_address,
            gpio: crate::gpio::hardware::i2c_n(Port::PORT_NUM).ok_or(ErrorKind::Busy)?,
            master_enabled,
            slave_underflow: false,
            _ph: PhantomData,
        };

        // Attempt to take control of the bus
        if master_enabled {
            i2c.bus_recover(16)?;
        }

        // Enable the I2C peripheral
        unsafe {
            i2c.reg.set_i2c_peripheral_enable(true);
        }

        i2c.clear_rx_fifo();
        i2c.clear_tx_fifo();

        i2c.set_rx_fifo_threshold(2);
        i2c.set_tx_fifo_threshold(6);

        i2c.enable_master(master_enabled)?;

        if !master_enabled {
            i2c.set_hardware_slave_address(slave_address)?;
            unsafe {
                i2c.reg.set_i2c_peripheral_enable(false);

                // Pulling Mode Enabled
                i2c.reg.set_disable_slave_clock_stretching(false);
                i2c.reg
                    .set_transmit_fifo_received_nack_auto_flush_disable(true);
                i2c.reg
                    .set_transmit_fifo_slave_address_match_read_auto_flush_disable(false);
                i2c.reg
                    .set_transmit_fifo_slave_address_match_write_auto_flush_disable(false);
                i2c.reg
                    .set_transmit_fifo_general_call_address_match_auto_flush_disable(false);
                i2c.reg.set_i2c_peripheral_enable(true);
                i2c.reg.set_disable_slave_clock_stretching(false);
                i2c.reg.set_transmit_fifo_preload_mode_enable(false);
            }
        } else {
            unsafe {
                i2c.reg.set_one_master_mode(false);
            }
        }

        Ok(i2c)
    }

    fn set_hardware_slave_address(&mut self, address: usize) -> Result<()> {
        if address > MAX_I2C_SLAVE_ADDRESS_10_BIT {
            return Err(ErrorKind::BadParam);
        }

        if address > MAX_I2C_SLAVE_ADDRESS_7_BIT {
            unsafe {
                self.reg.set_slave_mode_extended_address_length_select(true);
            }
        }

        unsafe {
            self.reg.set_slave_mode_address(address as u16);
        }

        Ok(())
    }

    pub fn slave_status(&mut self) -> Result<SlaveStatus> {
        if self.master_enabled {
            return Err(ErrorKind::BadState);
        }

        if self.reg.get_error_condition() != 0 {
            return Err(ErrorKind::ComError);
        }

        if self.reg.is_receive_fifo_threshold_level_active() {
            return Ok(SlaveStatus::ReadRequested);
        }

        if self.reg.is_slave_mode_stop_condition_active() {
            return Ok(SlaveStatus::Stop);
        }

        if self.reg.is_transfer_complete_flag_active() {
            return Ok(SlaveStatus::TransferDone);
        }

        if self.reg.is_slave_incoming_address_match_status_active()
            || self.reg.is_slave_write_addr_match_interrupt_active()
            || self.reg.is_slave_read_addr_match_interrupt_active()
        {
            let is_write = self.reg.get_read_write_bit_status()
                && !self.reg.is_slave_read_addr_match_interrupt_active();
            return Ok(SlaveStatus::IncomingRequest { is_write });
        }

        if self.reg.is_transmit_fifo_locked_active() {
            return Ok(SlaveStatus::TransmitFIFOLocked);
        }

        if self.reg.is_transmit_fifo_threshold_level_active() {
            return Ok(SlaveStatus::WriteRequested);
        }

        Ok(SlaveStatus::None)
    }

    pub fn slave_manual_pulling<Iter>(
        &mut self,
        iter: &mut Iter,
    ) -> Result<impl IntoIterator<Item = u8>>
    where
        Iter: Iterator<Item = u8>,
    {
        let mut recv_buffer = [0; 256];
        let mut recv_buffer_index = 0;

        if self.master_enabled {
            return Err(ErrorKind::BadState);
        }

        unsafe {
            self.reg.clear_slave_mode_do_not_respond();
        }

        self.set_rx_fifo_threshold(1);
        self.set_tx_fifo_threshold(1);

        // If we got an error in the middle of a tx_state, we want to
        // restore it.
        let mut tx_state = self.slave_underflow;
        self.slave_underflow = false;

        // RX does not have this problem, because its always ok to read from
        // a non-empty fifo
        let mut rx_state = false;

        loop {
            match self.slave_status() {
                Err(cond) => {
                    debug_println!("Error Condition");
                    self.debug_dump_int_status();
                    unsafe {
                        self.reg.set_interrupt_flags_0(u32::MAX);
                        self.reg.set_interrupt_flags_1(u32::MAX);
                    }

                    return Err(cond);
                }
                Ok(SlaveStatus::IncomingRequest { is_write: false }) => {
                    debug_println!("Incoming Read");
                    rx_state = true;
                    unsafe { self.reg.clear_slave_incoming_address_match_status() };
                    unsafe { self.reg.clear_slave_read_addr_match_interrupt() };
                }
                Ok(SlaveStatus::IncomingRequest { is_write: true }) => {
                    debug_println!("Incoming Write");
                    tx_state = true;
                    unsafe { self.reg.clear_slave_incoming_address_match_status() };
                    unsafe { self.reg.clear_slave_write_addr_match_interrupt() };
                    unsafe { self.reg.clear_transmit_fifo_locked() };
                }
                Ok(SlaveStatus::Stop) => {
                    debug_println!("Stop");
                    tx_state = false;
                    rx_state = false;
                    unsafe { self.reg.clear_slave_mode_stop_condition() };
                    break;
                }
                Ok(SlaveStatus::ReadRequested) => {
                    while !self.reg.get_receive_fifo_empty() {
                        unsafe { self.reg.clear_slave_mode_receive_fifo_overflow_flag() };
                        if recv_buffer_index >= recv_buffer.len() {
                            unsafe { self.reg.activate_transmit_fifo_flush() };
                            while !self.reg.is_transmit_fifo_flush_pending()
                                && !self.reg.is_transmit_fifo_locked_active()
                            {
                            }
                            return Err(ErrorKind::Overflow);
                        }

                        let data = self.reg.get_fifo_data();
                        recv_buffer[recv_buffer_index] = data;
                        recv_buffer_index += 1;

                        unsafe { self.reg.clear_receive_fifo_threshold_level() };
                    }
                }
                Ok(SlaveStatus::WriteRequested) if tx_state => {
                    let data = iter
                        .next()
                        .ok_or(ErrorKind::Underflow)
                        .inspect_err(|_err| self.slave_underflow = true)?;
                    unsafe { self.reg.clear_slave_mode_transmit_fifo_underflow_flag() };
                    unsafe { self.reg.set_fifo_data(data) };
                    unsafe { self.reg.clear_transmit_fifo_threshold_level() };
                }
                Ok(SlaveStatus::TransferDone) => {
                    unsafe { self.reg.clear_transfer_complete_flag() };
                    tx_state = false;
                }
                Ok(_) => {
                    if !rx_state && !tx_state {
                        return Err(ErrorKind::NoneAvailable);
                    }
                }
            }
        }

        Ok(recv_buffer.into_iter().take(recv_buffer_index))
    }

    // Maybe this should use slave_manual_pulling instead?
    pub fn slave_transaction<RXFun, TXFun>(&mut self, mut rx: RXFun, mut tx: TXFun) -> Result<()>
    where
        RXFun: FnMut(u8) -> Result<()>,
        TXFun: FnMut() -> Result<u8>,
    {
        if self.master_enabled {
            return Err(ErrorKind::BadState);
        }

        unsafe {
            self.reg.clear_slave_mode_do_not_respond();
        }

        self.set_rx_fifo_threshold(1);
        self.set_tx_fifo_threshold(1);

        debug_println!("Start");

        let mut tx_state = false;

        // TODO: Refacter this to be async later
        loop {
            match self.slave_status() {
                Err(cond) => {
                    debug_println!("Error Condition");
                    self.debug_dump_int_status();
                    unsafe {
                        self.reg.set_interrupt_flags_0(u32::MAX);
                        self.reg.set_interrupt_flags_1(u32::MAX);
                    }

                    return Err(cond);
                }
                Ok(SlaveStatus::IncomingRequest { is_write: false }) => {
                    debug_println!("Incoming Read");
                    // self.debug_dump_int_status();
                    unsafe { self.reg.clear_slave_incoming_address_match_status() };
                    unsafe { self.reg.clear_slave_read_addr_match_interrupt() };
                }
                Ok(SlaveStatus::IncomingRequest { is_write: true }) => {
                    debug_println!("Incoming Write");
                    tx_state = true;
                    // self.debug_dump_int_status();
                    unsafe { self.reg.clear_slave_incoming_address_match_status() };
                    unsafe { self.reg.clear_slave_write_addr_match_interrupt() };
                    unsafe { self.reg.clear_transmit_fifo_locked() };
                }
                Ok(SlaveStatus::Stop) => {
                    tx_state = false;
                    unsafe { self.reg.clear_slave_mode_stop_condition() };
                    break;
                }
                Ok(SlaveStatus::ReadRequested) => {
                    while !self.reg.get_receive_fifo_empty() {
                        rx(self.reg.get_fifo_data())?;
                    }
                    // unsafe { self.reg.clear_receive_fifo_threshold_level() };
                }
                Ok(SlaveStatus::WriteRequested) if tx_state => {
                    unsafe { self.reg.clear_slave_mode_transmit_fifo_underflow_flag() };
                    let data = tx()?;
                    unsafe { self.reg.set_fifo_data(data) };
                    unsafe { self.reg.clear_transmit_fifo_threshold_level() };
                }
                Ok(SlaveStatus::TransferDone) => {
                    unsafe { self.reg.clear_transfer_complete_flag() };
                    // self.purge_flags();
                    // while !self.reg.get_receive_fifo_empty() {
                    // rx(self.reg.get_fifo_data())?;
                    // }
                    // unsafe { self.reg.clear_receive_fifo_threshold_level() };
                    debug_println!("Transfer Done");
                    tx_state = false;
                }
                Ok(_) => {
                    // debug_println!("What?");
                    // self.debug_dump_int_status();
                    // microcontroller_delay(10);
                }
            }
        }

        Ok(())
    }

    fn debug_dump_int_status(&self) {
        debug_println!(
            r#"I2C Status: {:b} {:b}
    done: {},
    irxm: {},
    gc_addr_match: {},
    addr_match: {},
    rx_thd: {},
    tx_thd: {},
    stop: {},
    addr_ack: {},
    arb_err: {},
    to_error: {},
    addr_nack_error: {},
    data_err: {},
    dnr_err: {},
    start_err: {},
    stop_err: {},
    tx_lockout: {},
    rd_addr_match: {},
    wr_addr_match: {},
    start: {},
    tx_un: {},
    rx_ov: {}
"#,
            self.reg.get_interrupt_flags_0(),
            self.reg.get_interrupt_flags_1(),
            self.reg.is_transfer_complete_flag_active(),
            self.reg.is_irxm_interrupt_flag_active(),
            self.reg
                .is_slave_general_call_address_match_received_active(),
            self.reg.is_slave_incoming_address_match_status_active(),
            self.reg.is_receive_fifo_threshold_level_active(),
            self.reg.is_transmit_fifo_threshold_level_active(),
            self.reg.is_slave_mode_stop_condition_active(),
            self.reg.is_master_ack_from_external_slave_active(),
            self.reg.is_master_mode_arbitration_lost_active(),
            self.reg.is_timeout_error_flag_active(),
            self.reg.is_master_address_nack_from_slave_err_active(),
            self.reg.is_master_data_nack_from_slave_err_active(),
            self.reg.is_slave_mode_do_not_respond_active(),
            self.reg.is_out_of_sequence_start_flag_active(),
            self.reg.is_out_of_sequence_stop_flag_active(),
            self.reg.is_transmit_fifo_locked_active(),
            self.reg.is_slave_read_addr_match_interrupt_active(),
            self.reg.is_slave_write_addr_match_interrupt_active(),
            self.reg.is_start_condition_flag_active(),
            self.reg.is_slave_mode_transmit_fifo_underflow_flag_active(),
            self.reg.is_slave_mode_receive_fifo_overflow_flag_active(),
        );
    }

    pub fn master_status(&self) -> Result<MasterStatus> {
        if self.reg.is_master_ack_from_external_slave_active() {
            return Ok(MasterStatus::SlaveAck);
        }

        if self.reg.is_receive_fifo_threshold_level_active() {
            return Ok(MasterStatus::ReadRequested);
        }

        if self.reg.get_error_condition() != 0 {
            return Err(ErrorKind::ComError);
        }

        if self.reg.is_master_ack_from_external_slave_active() {
            return Ok(MasterStatus::SlaveAck);
        }

        if self.reg.is_master_data_nack_from_slave_err_active() {
            return Ok(MasterStatus::SlaveNack);
        }

        if self.reg.is_transfer_complete_flag_active() {
            return Ok(MasterStatus::TransferDone);
        }

        if self.reg.is_transmit_fifo_threshold_level_active() {
            return Ok(MasterStatus::WriteRequested);
        }

        Ok(MasterStatus::None)
    }

    fn purge_flags(&mut self) {
        unsafe {
            self.reg.set_interrupt_flags_0(u32::MAX);
            self.reg.set_interrupt_flags_1(u32::MAX);
        }
    }

    pub fn master_command(&mut self, cmd: MasterCommand) {
        let active = !self.reg.get_transaction_active();

        match cmd {
            MasterCommand::StartWrite { address } => {
                self.send_address_with_rw(address, true);
                self.send_bus_event(I2CBusControlEvent::StartOrRestart);
                while self.reg.is_send_repeated_start_condition_pending() {}
            }
            MasterCommand::StartRead {
                address,
                read_amount,
            } => {
                self.send_bus_event(I2CBusControlEvent::StartOrRestart);
                while self.reg.is_send_repeated_start_condition_pending() {}
                self.send_address_with_rw(address, false);

                let new_read_amount = if read_amount >= 256 {
                    0
                } else {
                    read_amount as u8
                };

                unsafe { self.reg.set_receive_fifo_transaction_size(new_read_amount) };
                while self.reg.is_send_repeated_start_condition_pending() {}
            }
            MasterCommand::Stop => {
                self.send_bus_event(I2CBusControlEvent::Stop);
                while self.reg.is_send_stop_condition_pending() {}
            }
        }
    }

    fn handle_i2c_master_error(&mut self, error: ErrorKind, msg: &str) -> Result<()> {
        debug_println!("Error Condition: {}", msg);
        self.debug_dump_int_status();
        self.purge_flags();
        self.master_command(MasterCommand::Stop);
        while !self.reg.is_slave_mode_stop_condition_active() {}
        unsafe { self.reg.clear_slave_mode_stop_condition() };

        Err(error)
    }

    pub fn master_transaction(
        &mut self,
        address: usize,
        mut rx: Option<&mut [u8]>,
        tx: Option<&[u8]>,
    ) -> Result<()> {
        if !self.master_enabled {
            return Err(ErrorKind::BadState);
        }

        self.purge_flags();

        if let Some(tx) = tx {
            let mut tx_iter = tx.iter().copied();
            self.master_command(MasterCommand::StartWrite { address });

            let mut got_ack = false;

            loop {
                match self.master_status() {
                    Ok(MasterStatus::SlaveAck) => {
                        debug_println!("Slave ACK");
                        got_ack = true;
                        unsafe { self.reg.clear_master_ack_from_external_slave() };
                    }
                    Ok(MasterStatus::SlaveNack) => {
                        self.handle_i2c_master_error(ErrorKind::NoResponse, "Slave NACK")?
                    }
                    Ok(MasterStatus::WriteRequested) if got_ack => {
                        if self.write_fifo(&mut tx_iter).is_err() {
                            break;
                        }
                        unsafe { self.reg.clear_transmit_fifo_threshold_level() };
                    }
                    Ok(MasterStatus::TransferDone) => self.handle_i2c_master_error(
                        ErrorKind::Abort,
                        "Got Transfer done flag at wrong time",
                    )?,
                    Ok(_) => {
                        // debug_println!("Nothing...");
                    }
                    Err(err) => self.handle_i2c_master_error(err, "COMM ERROR")?,
                }
            }
        }

        unsafe { self.reg.clear_transmit_fifo_locked() };

        if let Some(rx) = rx {
            let mut bytes_written = 0;
            let read_amount = rx.len() - bytes_written;

            self.master_command(MasterCommand::StartRead {
                address,
                read_amount,
            });

            if tx.is_some() {
                while !self.reg.is_transfer_complete_flag_active() {}
                unsafe { self.reg.clear_transfer_complete_flag() };
            }

            let mut got_ack = false;

            while bytes_written < rx.len() {
                match self.master_status() {
                    Ok(MasterStatus::SlaveAck) => {
                        debug_println!("Slave ACK");
                        got_ack = true;
                        unsafe { self.reg.clear_master_ack_from_external_slave() };
                    }
                    Ok(MasterStatus::SlaveNack) => {
                        self.handle_i2c_master_error(ErrorKind::NoResponse, "Slave NACK")?
                    }
                    Ok(MasterStatus::TransferDone) => {
                        got_ack = false;
                        unsafe { self.reg.clear_transfer_complete_flag() };
                        while !self.reg.get_receive_fifo_empty() {
                            bytes_written += self.read_fifo(&mut rx[bytes_written..]);
                        }
                        unsafe { self.reg.clear_receive_fifo_threshold_level() };

                        if bytes_written < rx.len() {
                            let read_amount = rx.len() - bytes_written;
                            self.master_command(MasterCommand::StartRead {
                                address,
                                read_amount,
                            });
                        } else if bytes_written == rx.len() {
                            break;
                        } else {
                            self.handle_i2c_master_error(
                                ErrorKind::Abort,
                                "Transfer Done at unexpected time",
                            )?;
                        }
                    }
                    Ok(MasterStatus::ReadRequested) if got_ack => {
                        while !self.reg.get_receive_fifo_empty() {
                            bytes_written += self.read_fifo(&mut rx[bytes_written..]);
                        }
                        unsafe { self.reg.clear_receive_fifo_threshold_level() };
                    }
                    Ok(_) => (),
                    Err(err) => self.handle_i2c_master_error(err, "COMM ERROR")?,
                }
            }
        }

        self.master_command(MasterCommand::Stop);
        while !self.reg.is_slave_mode_stop_condition_active() {}
        // while !self.reg.is_transfer_complete_flag_active() {}

        unsafe {
            // self.reg.clear_transfer_complete_flag();
            self.reg.clear_slave_mode_stop_condition();
        }

        Ok(())
    }

    fn set_freq(&mut self, hz: usize) -> Result<usize> {
        if hz > MAX_I2C_HIGHSPEED_CLOCK_TIME {
            return Err(ErrorKind::BadParam);
        }

        if hz <= MAX_I2C_HIGHSPEED_CLOCK_TIME && hz > MAX_I2C_FASTPLUS_CLOCK_TIME {
            todo!("Highspeed I2C Mode is currently not supported");
        }

        let peripheral_clock = core_peripheral_clock() as usize;
        let ticks_total = peripheral_clock / hz;
        let high_clock_time = (ticks_total >> 1) - 1;
        let low_clock_time = (ticks_total >> 1) - 1;

        let high_clock_roundover = ticks_total % 2;

        // The clock time should always be a valid value
        if low_clock_time == 0 || high_clock_time == 0 {
            return Err(ErrorKind::BadParam);
        }

        unsafe {
            self.reg
                .set_clock_high_time((high_clock_time + high_clock_roundover) as u16);
            self.reg.set_clock_low_time(low_clock_time as u16);
        }

        Ok(self.get_freq())
    }

    fn get_freq(&self) -> usize {
        if self.reg.get_high_speed_mode() {
            todo!("Highspeed I2C Mode is currently not supported");
        }

        let cycles_low = self.reg.get_clock_low_time();
        let cycles_high = self.reg.get_clock_high_time();

        debug_assert_ne!(cycles_low, 0, "Cycles low should be larger then 0!");
        debug_assert_ne!(cycles_high, 0, "Cycles High should be larger then 0!");

        let cycles_total = cycles_low + cycles_high;

        (core_peripheral_clock() as usize) / (cycles_total as usize)
    }

    fn write_fifo<Bytes>(&mut self, tx: &mut Bytes) -> Result<usize>
    where
        Bytes: Iterator<Item = u8>,
    {
        let current_fifo_level = self.reg.get_transmit_fifo_byte_count() as usize;
        let fifo_free = MAX_TRANSMIT_FIFO_LEN - current_fifo_level;
        let mut bytes_written = 0;

        for i in 0..fifo_free {
            let Some(data) = tx.next() else {
                return Err(ErrorKind::NoneAvailable);
            };

            unsafe {
                self.reg.set_fifo_data(data);
            }

            debug_println!("TX Byte {}", data);

            bytes_written += 1;
        }

        Ok(bytes_written)
    }

    fn read_fifo(&self, rx: &mut [u8]) -> usize {
        let current_fifo_level = self.reg.get_current_receive_fifo_bytes() as usize;
        let max_receive = current_fifo_level.min(rx.len());

        for data in rx.iter_mut().take(max_receive) {
            *data = self.reg.get_fifo_data();
            debug_println!("RX Byte: {}", data);
        }

        max_receive
    }

    fn send_address_with_rw(&mut self, address: usize, is_writting: bool) {
        let writting_value = if is_writting { 0 } else { 1 };
        // TODO: We should check the state of the FIFO before adding data to it!
        //       What if the FIFO is full, we do not want to loose data here.
        unsafe {
            self.reg
                .set_fifo_data((address << 1 | writting_value) as u8);
        }
    }

    fn send_bus_event(&mut self, event: I2CBusControlEvent) {
        while self.reg.is_transmit_fifo_locked_active() {}
        match event {
            I2CBusControlEvent::StartOrRestart => unsafe {
                if self.reg.get_transaction_active() {
                    debug_println!("Sent RESTART");
                    self.reg.activate_send_repeated_start_condition();
                } else {
                    debug_println!("Sent START");
                    self.reg.activate_start_master_mode_transfer();
                }
            },
            I2CBusControlEvent::Start => unsafe {
                debug_println!("Sent START");
                self.reg.activate_start_master_mode_transfer();
            },
            I2CBusControlEvent::Restart => unsafe {
                debug_println!("Sent RESTART");
                self.reg.activate_send_repeated_start_condition();
            },
            I2CBusControlEvent::Stop => unsafe {
                debug_println!("Sent STOP");
                self.reg.activate_send_stop_condition();
            },
        }
    }

    pub fn clear_rx_fifo(&mut self) {
        unsafe {
            self.reg.activate_receive_fifo_flush();
        }

        while self.reg.is_receive_fifo_flush_pending() {}
    }

    pub fn clear_tx_fifo(&mut self) {
        unsafe {
            self.reg.activate_transmit_fifo_flush();
        }

        //while self.reg.is_transmit_fifo_flush_pending() {}
    }

    pub fn set_rx_fifo_threshold(&mut self, threshold: usize) {
        debug_assert!(
            threshold <= 8,
            "Cannot set the bytes threshold {threshold} over the max register threshold of 8!"
        );

        unsafe {
            self.reg.set_receive_fifo_threshold_level(threshold as u8);
        }
    }

    pub fn set_tx_fifo_threshold(&mut self, threshold: usize) {
        debug_assert!(
            threshold <= 7,
            "Cannot set the bytes threshold {threshold} over the max register threshold of 8!"
        );

        unsafe {
            self.reg.set_transmit_fifo_threshold_level(threshold as u8);
        }
    }

    pub fn enable_master(&mut self, flag: bool) -> Result<()> {
        if flag {
            // Another Master is currently controlling the bus,
            // we should not enable master mode!
            if self.reg.get_transaction_active() {
                return Err(ErrorKind::BadState);
            }

            unsafe {
                self.reg.set_master_mode_enable(true);
            }
        } else {
            unsafe {
                self.reg.set_master_mode_enable(false);
            }
        }

        Ok(())
    }

    pub fn bus_recover(&mut self, retry_count: usize) -> Result<()> {
        microcontroller_delay(10);
        // Save the state so we can restore it
        let state_prior = self.reg.get_control_register();

        // Switch to Software Mode, and enable the I2C bus
        unsafe {
            self.reg.set_i2c_peripheral_enable(true);
            self.reg.set_software_i2c_mode(true);
        }

        let mut success = false;
        // Lets try and recover the bus
        for _ in 0..retry_count {
            debug_print!("Testing I2C Bus... ");
            microcontroller_delay(10);

            // Pull SCL low
            unsafe {
                self.reg.set_scl_hardware_pin_released(false);
            }

            microcontroller_delay(10);

            // If SCL is high we were unable to pull the bus low
            if self.reg.get_scl_pin() {
                debug_println!("SCL-LOW-FAIL");
                unsafe { self.reg.set_scl_hardware_pin_released(true) };
                unsafe { self.reg.set_sda_hardware_pin_released(true) };
                continue;
            }
            debug_print!("SCL-LOW ");

            microcontroller_delay(10);

            // Release SCL (pull high)
            unsafe {
                self.reg.set_scl_hardware_pin_released(true);
            }

            microcontroller_delay(10);

            // If SCL is low we were unable to release the bus
            if !self.reg.get_scl_pin() {
                debug_println!("SCL-HIGH-FAIL");
                unsafe { self.reg.set_scl_hardware_pin_released(true) };
                unsafe { self.reg.set_sda_hardware_pin_released(true) };
                continue;
            }
            debug_print!("SCL-HIGH ");

            microcontroller_delay(10);

            // Pull SDA Low
            unsafe {
                self.reg.set_sda_hardware_pin_released(false);
            }

            microcontroller_delay(10);

            // If SDA is high we were unable to pull the bus low
            if self.reg.get_sda_pin() {
                debug_println!("SDA-LOW-FAIL");
                unsafe { self.reg.set_scl_hardware_pin_released(true) };
                unsafe { self.reg.set_sda_hardware_pin_released(true) };
                continue;
            }
            debug_print!("SDA-LOW ");

            microcontroller_delay(10);

            // Release SDA (pull high)
            unsafe {
                self.reg.set_sda_hardware_pin_released(true);
            }

            microcontroller_delay(10);

            // If SDA is low we were unable to pull release the bus
            if !self.reg.get_sda_pin() {
                debug_println!("SDA-HIGH-FAIL");
                unsafe { self.reg.set_scl_hardware_pin_released(true) };
                unsafe { self.reg.set_sda_hardware_pin_released(true) };
                continue;
            }

            debug_print!("SDA-HIGH ");

            // We where able to take control over the bus!
            success = true;
            break;
        }

        // We could not take control over the bus
        if !success {
            return Err(ErrorKind::ComError);
        }

        unsafe {
            self.reg.set_control_register(state_prior);
        }

        debug_println!("  -- OK");

        Ok(())
    }
}
