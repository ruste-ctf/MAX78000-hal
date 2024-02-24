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

const MAX_I2C_SLAVE_ADDRESS_7_BIT: usize = 0b1111111;
const MAX_I2C_SLAVE_ADDRESS_10_BIT: usize = 0b1111111111;

#[allow(unused)]
const MAX_I2C_NORMAL_CLOCK_HZ: usize = 100000;
#[allow(unused)]
const MAX_I2C_FAST_CLOCK_HZ: usize = 400000;
const MAX_I2C_FASTPLUS_CLOCK_TIME: usize = 1000000;
const MAX_I2C_HIGHSPEED_CLOCK_TIME: usize = 3400000;

const MAX_I2C_FIFO_TRANSACTION: usize = 256;
const MAX_TRANSMIT_FIFO_LEN: usize = 8;
const MAX_RECEIVE_FIFO_LEN: usize = 8;

fn microcontroller_delay(us: usize) {
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
            _ph: PhantomData,
        };

        // Attempt to take control of the bus
        i2c.bus_recover(16)?;

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

    pub fn slave_transaction<RXFun, TXFun>(&mut self, mut rx: RXFun, mut tx: TXFun) -> Result<()>
    where
        RXFun: FnMut(u8) -> Result<()>,
        TXFun: FnMut() -> Result<u8>,
    {
        if self.master_enabled {
            return Err(ErrorKind::BadState);
        }

        unsafe {
            self.clear_rx_fifo();
            self.clear_tx_fifo();
            self.debug_dump_int_status();
            self.reg
                .set_interrupt_flags_0(self.reg.get_interrupt_flags_0());
            self.reg
                .set_interrupt_flags_1(self.reg.get_interrupt_flags_1());
        }

        self.set_rx_fifo_threshold(1);
        self.set_tx_fifo_threshold(1);

        // TODO: Refacter this to be async later
        loop {
            if self.reg.is_slave_mode_stop_condition_active()
                && !self.reg.is_transfer_complete_flag_active()
            {
                debug_println!("STOP");
                unsafe { self.reg.clear_slave_mode_stop_condition() };
            }

            if self.reg.is_slave_read_addr_match_interrupt_active() {
                debug_println!("READ!!");
            }

            if self.reg.is_slave_write_addr_match_interrupt_active() {
                debug_println!("WRITE!!");
                unsafe {
                    self.reg.clear_slave_write_addr_match_interrupt();
                    self.reg
                        .is_slave_general_call_address_match_received_active();
                    self.reg.is_slave_incoming_address_match_status_active();
                }

                let bytes_to_tx = self.reg.get_transmit_fifo_threshold_level();

                for i in 0..(8 - bytes_to_tx) {
                    let byte = tx()?;

                    unsafe { self.reg.set_fifo_data(byte) };
                }
                unsafe { self.reg.clear_transmit_fifo_threshold_level() };
                unsafe { self.reg.clear_transmit_fifo_locked() };
                unsafe { self.reg.activate_transmit_fifo_flush() };
            }

            if self.reg.is_slave_mode_receive_fifo_overflow_flag_active() {
                todo!("Buffer Overrun")
            }

            if self.reg.is_slave_mode_transmit_fifo_underflow_flag_active() {
                debug_println!("Buffer Underrun");
                unsafe { self.reg.clear_slave_mode_transmit_fifo_underflow_flag() };
            }

            if self.reg.is_receive_fifo_threshold_level_active() {
                unsafe { self.reg.clear_receive_fifo_threshold_level() };

                while !self.reg.get_receive_fifo_empty() {
                    rx(self.reg.get_fifo_data())?;
                }
            }

            if self.reg.is_transmit_fifo_threshold_level_active() {
                let bytes_to_tx = self.reg.get_transmit_fifo_threshold_level();

                for i in 0..(8 - bytes_to_tx) {
                    let byte = tx()?;

                    unsafe { self.reg.set_fifo_data(byte) };
                }
                unsafe { self.reg.clear_transmit_fifo_threshold_level() };
                unsafe { self.reg.clear_transmit_fifo_locked() };
                unsafe { self.reg.activate_transmit_fifo_flush() };
                while self.reg.is_transmit_fifo_flush_pending() {
                    debug_println!("Wating for TX Flush");
                }
            }

            if self.reg.is_slave_mode_stop_condition_active()
                || self.reg.is_transfer_complete_flag_active()
            {
                unsafe {
                    self.reg.clear_slave_mode_stop_condition();
                    self.reg.clear_transfer_complete_flag();
                }

                return Ok(());
            }
        }
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

    pub fn master_transaction(
        &mut self,
        address: usize,
        rx: Option<&mut [u8]>,
        tx: Option<&[u8]>,
    ) -> Result<()> {
        if !self.master_enabled {
            return Err(ErrorKind::BadState);
        }

        unsafe {
            self.reg.clear_transmit_fifo_locked();
            self.clear_tx_fifo();
            self.reg.set_interrupt_flags_0(u32::MAX);
            self.reg.set_interrupt_flags_1(u32::MAX);
        }

        let reading = rx.is_some();
        let writing = tx.is_some();

        if !reading || writing {
            self.send_address_with_rw(address, true);
            self.send_bus_event(I2CBusControlEvent::Start);

            // Not the best, but I think its fine for now
            let mut tx_iter = tx.unwrap_or(&[0_u8; 0]).iter().copied();

            loop {
                if self.reg.is_transmit_fifo_threshold_level_active() {
                    if self.write_fifo(&mut tx_iter).is_err() {
                        break;
                    }

                    unsafe { self.reg.clear_transmit_fifo_threshold_level() };
                }

                let error = self.reg.get_error_condition();
                if error != 0 {
                    self.debug_dump_int_status();
                    // Clear the error
                    unsafe { self.reg.set_error_condition(error) };
                    self.send_bus_event(I2CBusControlEvent::Stop);

                    return Err(ErrorKind::ComError);
                }
            }
        }

        unsafe {
            self.reg.clear_transfer_complete_flag();
        }

        let mut bytes_written = 0;
        if let Some(rx) = rx {
            let transaction_size = if rx.len() >= MAX_I2C_FIFO_TRANSACTION {
                0
            } else {
                rx.len() as u8
            };
            unsafe { self.reg.set_receive_fifo_transaction_size(transaction_size) };

            self.send_bus_event(I2CBusControlEvent::Start);
            while self.reg.is_send_repeated_start_condition_pending() {}

            self.send_address_with_rw(address, false);

            while bytes_written <= rx.len() {
                if self.reg.is_receive_fifo_threshold_level_active()
                    || self.reg.is_transfer_complete_flag_active()
                {
                    bytes_written += self.read_fifo(&mut rx[bytes_written..]);
                    unsafe { self.reg.clear_receive_fifo_threshold_level() };
                }

                if self.reg.get_error_condition() != 0 {
                    self.send_bus_event(I2CBusControlEvent::Stop);
                    return Err(ErrorKind::ComError);
                }

                if self.reg.is_transfer_complete_flag_active()
                    && bytes_written <= rx.len()
                    && self.reg.get_receive_fifo_len() == 0
                {
                    debug_println!("Transfer Complete");
                    let bytes_diff = rx.len() - bytes_written;
                    let transaction_size = if bytes_diff > MAX_I2C_FIFO_TRANSACTION {
                        0
                    } else {
                        bytes_diff as u8
                    };

                    unsafe {
                        self.reg.set_receive_fifo_transaction_size(transaction_size);
                        self.send_bus_event(I2CBusControlEvent::Restart);
                        self.reg.clear_transfer_complete_flag();
                        self.send_address_with_rw(address, false);
                    }
                }
            }
        }

        debug_println!("Cum Zone");

        self.send_bus_event(I2CBusControlEvent::Stop);
        while !self.reg.is_slave_mode_stop_condition_active() {}
        while !self.reg.is_transfer_complete_flag_active() {}

        unsafe {
            self.reg.clear_transfer_complete_flag();
            self.reg.clear_slave_mode_stop_condition();
        }

        let error = self.reg.get_error_condition();
        if error != 0 {
            unsafe { self.reg.set_error_condition(error) };
            Err(ErrorKind::ComError)
        } else {
            Ok(())
        }
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
        let current_fifo_level = self.reg.get_transmit_fifo_len() as usize;
        let max_fifo_level = MAX_TRANSMIT_FIFO_LEN;
        let fifo_free = max_fifo_level - current_fifo_level;
        let mut bytes_written = 0;

        for i in 0..fifo_free {
            let Some(data) = tx.next() else {
                return Err(ErrorKind::NoneAvailable);
            };

            unsafe {
                self.reg.set_fifo_data(data);
            }

            bytes_written += 1;
        }

        Ok(bytes_written)
    }

    fn read_fifo(&self, rx: &mut [u8]) -> usize {
        let current_fifo_level = self.reg.get_current_receive_fifo_bytes() as usize;
        let max_receive = current_fifo_level.min(rx.len());

        for data in rx.iter_mut().take(max_receive) {
            *data = self.reg.get_fifo_data();
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
        match event {
            I2CBusControlEvent::Start => unsafe {
                self.reg.activate_start_master_mode_transfer();
            },
            I2CBusControlEvent::Restart => unsafe {
                self.reg.activate_send_repeated_start_condition();
            },
            I2CBusControlEvent::Stop => unsafe {
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
