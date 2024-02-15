use crate::core_peripheral_clock;
use crate::error::{ErrorKind, Result};
use crate::memory_map::mmio;
use core::marker::PhantomData;

pub mod registers;

mod private {
    pub trait I2CPortCompatable {}
}

pub struct NoPort {}
pub struct I2CPort0 {}
pub struct I2CPort1 {}
pub struct I2CPort2 {}

impl private::I2CPortCompatable for NoPort {}
impl private::I2CPortCompatable for I2CPort0 {}
impl private::I2CPortCompatable for I2CPort1 {}
impl private::I2CPortCompatable for I2CPort2 {}

#[allow(dead_code)]
pub struct I2C<Port: private::I2CPortCompatable = NoPort> {
    master_enabled: bool,
    slave_address: usize,
    ph: PhantomData<Port>,
}

/// # Registers
/// This is a "420 what you smoking?" kinda issue with Rust.
///
/// Waiting for this unstable feature to become stable to avoid
/// doing this trash. (See issues 8995, 16240, 108491).
///
/// # What we cannot do
/// ```no_compile
/// struct MyStruct {}
///
/// impl MyStruct {
///    // error[E0658]: inherent associated types are unstable
///    type MyInnerType = usize;
/// }
/// ```
/// We are also unable to put them into a struct, macro, or anything else.
/// They **MUST** be done in each and every function. This macro just copy
/// and pastes each and every type into each function to avoid this problem.
///
/// # What does this macro _(\*cough\* trash)_ do?
/// It will paste each of the registers into the function. This allows one
/// to not have to write `registers::ControlRegister<{ mmio::I2C_PORT_0}>` each
/// time they would like to use the register types.
macro_rules! registers {
    ($port:expr) => {
        #[allow(unused)]
        type ControlRegister = registers::ControlRegister<{ $port }>;
        #[allow(unused)]
        type DMAControl = registers::DMAControl<{ $port }>;
        #[allow(unused)]
        type DataRegister = registers::DataRegister<{ $port }>;
        #[allow(unused)]
        type FIFOLengthRegister = registers::FIFOLengthRegister<{ $port }>;
        #[allow(unused)]
        type HighSCLControl = registers::HighSCLControl<{ $port }>;
        #[allow(unused)]
        type HighSpeedClockControl = registers::HighSpeedClockControl<{ $port }>;
        #[allow(unused)]
        type InterruptEnable0 = registers::InterruptEnable0<{ $port }>;
        #[allow(unused)]
        type InterruptEnable1 = registers::InterruptEnable1<{ $port }>;
        #[allow(unused)]
        type InterruptFlag0 = registers::InterruptFlag0<{ $port }>;
        #[allow(unused)]
        type InterruptFlag1 = registers::InterruptFlag1<{ $port }>;
        #[allow(unused)]
        type LowSCLControl = registers::LowSCLControl<{ $port }>;
        #[allow(unused)]
        type MasterControl = registers::MasterControl<{ $port }>;
        #[allow(unused)]
        type ReceiveControl0 = registers::ReceiveControl0<{ $port }>;
        #[allow(unused)]
        type ReceiveControl1 = registers::ReceiveControl1<{ $port }>;
        #[allow(unused)]
        type SlaveAddress = registers::SlaveAddress<{ $port }>;
        #[allow(unused)]
        type StatusRegister = registers::StatusRegister<{ $port }>;
        #[allow(unused)]
        type TimeoutControl = registers::TimeoutControl<{ $port }>;
        #[allow(unused)]
        type TransmitControl0 = registers::TransmitControl0<{ $port }>;
        #[allow(unused)]
        type TransmitControl1 = registers::TransmitControl1<{ $port }>;
    };
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

#[allow(unused)]
impl I2C<NoPort> {
    /// # Port 0 Init Master
    /// Init and Quarry port 0 on the I2C bus.  
    pub fn port_0_init_master() -> Result<I2C<I2CPort0>> {
        I2C::<I2CPort0>::init(true, 0x00)
    }

    /// # Port 0 Init Slave
    /// Init and Quarry port 0 on the I2C bus.
    pub fn port_0_init_slave(slave_address: usize) -> Result<I2C<I2CPort0>> {
        I2C::<I2CPort0>::init(false, slave_address)
    }

    pub fn port_1_init() -> Result<I2C<I2CPort1>> {
        todo!()
    }

    pub fn port_2_init() -> Result<I2C<I2CPort2>> {
        todo!()
    }
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

#[cfg(not(test))]
extern "C" {
    fn MXC_Delay(us: u32);
}

/// # We need this for I2C, but uh I have not gotten to it yet :)
#[cfg(not(test))]
fn microcontroller_delay(us: usize) {
    unsafe { MXC_Delay(us as u32) }
}

#[cfg(test)]
fn microcontroller_delay(_us: usize) {}

#[allow(unused)]
impl I2C<I2CPort0> {
    fn init(enable_master: bool, slave_address: usize) -> Result<Self> {
        registers!(mmio::I2C_PORT_1);

        // Attempt to take control of the bus
        Self::bus_recover(16)?;

        // Enable the I2C peripheral
        unsafe {
            ControlRegister::set_i2c_peripheral_enable(true);
        }

        Self::clear_rx_fifo();
        Self::clear_tx_fifo();

        Self::set_rx_fifo_threshold(2);
        Self::set_tx_fifo_threshold(6);

        Self::enable_master(enable_master)?;

        if !enable_master {
            Self::set_hardware_slave_address(slave_address)?;
        }

        Ok(Self {
            master_enabled: enable_master,
            slave_address,
            ph: PhantomData,
        })
    }

    fn set_hardware_slave_address(address: usize) -> Result<()> {
        registers!(mmio::I2C_PORT_1);

        if address > MAX_I2C_SLAVE_ADDRESS_10_BIT {
            return Err(ErrorKind::BadParam);
        }

        if address > MAX_I2C_SLAVE_ADDRESS_7_BIT {
            unsafe {
                SlaveAddress::set_slave_mode_extended_address_length_select(true);
            }
        }

        unsafe {
            SlaveAddress::set_slave_mode_address(address as u16);
        }

        Ok(())
    }
  
    pub fn master_transaction(
        &self,
        address: usize,
        rx: Option<&mut [u8]>,
        tx: Option<&[u8]>,
    ) -> Result<()> {
        let reading = rx.is_some();
        let writing = tx.is_some();

        if !reading || writing {}
        todo!()
    }

    fn send_address_with_rw(&self, address: usize, is_writting: bool) {}

    pub fn send_bus_event(&self, event: I2CBusControlEvent) -> Result<()> {
        registers!(mmio::I2C_PORT_0);
        if !self.master_enabled {
            return Err(ErrorKind::BadState);
        }
      
        registers!(mmio::I2C_PORT_1);
        let reading = rx.is_some();
        let writing = tx.is_some();

        if !reading || writing {
            Self::send_address_with_rw(address, true);
            Self::send_bus_event(I2CBusControlEvent::Start)?;
        }

        // Not the best, but I think its fine for now
        let mut tx_iter = tx.unwrap_or(&[0_u8; 0]).iter().copied();

        loop {
            if InterruptFlag0::is_transmit_fifo_threshold_level() {
                if Self::write_fifo(&mut tx_iter) == 0 {
                    break;
                }

                unsafe { InterruptFlag0::clear_transmit_fifo_threshold_level() };
            }

            if InterruptFlag0::is_error_condition() {
                Self::send_bus_event(I2CBusControlEvent::Stop)?;
                return Err(ErrorKind::ComError);
            }
        }

        unsafe {
            InterruptFlag0::clear_transfer_complete_flag();
            InterruptFlag0::clear_receive_fifo_threshold_level();
        }

        let mut bytes_written = 0;
        if let Some(rx) = rx {
            let transaction_size = if rx.len() >= MAX_I2C_FIFO_TRANSACTION {
                0
            } else {
                rx.len() as u8
            };
            unsafe { ReceiveControl1::set_receive_fifo_transaction_size(transaction_size) };

            Self::send_bus_event(I2CBusControlEvent::Start);
            while MasterControl::is_send_repeated_start_condition() {}

            Self::send_address_with_rw(address, false);

            while bytes_written <= rx.len() {
                if InterruptFlag0::is_receive_fifo_threshold_level()
                    || InterruptFlag0::is_transfer_complete()
                {
                    bytes_written += Self::read_fifo(&mut rx[bytes_written..]);
                    unsafe { InterruptFlag0::clear_receive_fifo_threshold_level() };
                }

                if InterruptFlag0::is_error_condition() {
                    Self::send_bus_event(I2CBusControlEvent::Stop)?;
                    return Err(ErrorKind::ComError);
                }

                if InterruptFlag0::is_transfer_complete()
                    && bytes_written <= rx.len()
                    && FIFOLengthRegister::get_receive_fifo_len() == 0
                {
                    let bytes_diff = rx.len() - bytes_written;
                    let transaction_size = if bytes_diff > MAX_I2C_FIFO_TRANSACTION {
                        0
                    } else {
                        bytes_diff as u8
                    };

                    unsafe {
                        ReceiveControl1::set_receive_fifo_transaction_size(transaction_size);
                        Self::send_bus_event(I2CBusControlEvent::Restart)?;
                        InterruptFlag0::clear_transfer_complete_flag();
                        Self::send_address_with_rw(address, false);
                    }
                }
            }
        }

        Self::send_bus_event(I2CBusControlEvent::Stop)?;
        while !InterruptFlag0::is_slave_mode_stop_condition() {}
        while !InterruptFlag0::is_transfer_complete() {}

        unsafe {
            InterruptFlag0::clear_transfer_complete_flag();
            InterruptFlag0::clear_slave_mode_stop_condition();
        }

        if InterruptFlag0::is_error_condition() {
            Err(ErrorKind::ComError)
        } else {
            Ok(())
        }
    }

    fn set_freq(hz: usize) -> Result<usize> {
        registers!(mmio::I2C_PORT_1);

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
            HighSCLControl::set_clock_high_time((high_clock_time + high_clock_roundover) as u16);
            LowSCLControl::set_clock_low_time(low_clock_time as u16);
        }

        Ok(Self::get_freq())
    }

    fn get_freq() -> usize {
        registers!(mmio::I2C_PORT_1);

        if ControlRegister::is_high_speed_mode_enabled() {
            todo!("Highspeed I2C Mode is currently not supported");
        }

        let cycles_low = LowSCLControl::get_clock_low_time();
        let cycles_high = HighSCLControl::get_clock_high_time();

        debug_assert_ne!(cycles_low, 0, "Cycles low should be larger then 0!");
        debug_assert_ne!(cycles_high, 0, "Cycles High should be larger then 0!");

        let cycles_total = cycles_low + cycles_high;

        (core_peripheral_clock() as usize) / (cycles_total as usize)
    }

    fn write_fifo<Bytes>(tx: &mut Bytes) -> usize
    where
        Bytes: Iterator<Item = u8>,
    {
        registers!(mmio::I2C_PORT_1);

        let current_fifo_level = FIFOLengthRegister::get_transmit_fifo_len() as usize;
        let max_fifo_level = FIFOLengthRegister::MAX_FIFO_TRANSMIT_LEN;
        let fifo_free = max_fifo_level - current_fifo_level;
        let mut bytes_written = 0;

        for i in 0..fifo_free {
            let Some(data) = tx.next() else {
                return bytes_written;
            };

            unsafe {
                DataRegister::write_fifo_data(data);
            }

            bytes_written += 1;
        }

        bytes_written
    }

    fn read_fifo(rx: &mut [u8]) -> usize {
        registers!(mmio::I2C_PORT_1);

        let current_fifo_level = FIFOLengthRegister::get_receive_fifo_len() as usize;
        let max_fifo_level = FIFOLengthRegister::MAX_FIFO_RECEIVE_LEN;
        let fifo_free = max_fifo_level - current_fifo_level;
        let max_receive = fifo_free.min(rx.len());

        for data in rx.iter_mut().take(max_receive) {
            *data = DataRegister::read() as u8;
        }

        max_receive
    }

    fn send_address_with_rw(address: usize, is_writting: bool) {
        registers!(mmio::I2C_PORT_1);
        let writting_value = if is_writting { 0 } else { 1 };
        // TODO: We should check the state of the FIFO before adding data to it!
        //       What if the FIFO is full, we do not want to loose data here.
        unsafe {
            DataRegister::write_fifo_data((address << 1 | writting_value) as u8);
        }
    }

    fn send_bus_event(event: I2CBusControlEvent) -> Result<()> {
        registers!(mmio::I2C_PORT_1);
        match event {
            I2CBusControlEvent::Start => unsafe {
                MasterControl::activate_start_master_mode_transfer();
            },
            I2CBusControlEvent::Restart => unsafe {
                MasterControl::activate_send_repeated_start_condition();
            },
            I2CBusControlEvent::Stop => unsafe {
                MasterControl::activate_send_stop_condition();
            },
        }

        Ok(())
    }

    pub fn clear_rx_fifo() {
        registers!(mmio::I2C_PORT_1);
        unsafe {
            ReceiveControl0::activate_flush_receive_fifo();
        }

        while ReceiveControl0::is_flush_receive_fifo() {}
    }

    pub fn clear_tx_fifo() {
        registers!(mmio::I2C_PORT_1);
        unsafe {
            TransmitControl0::activate_transmit_fifo_flush();
        }

        while TransmitControl0::is_transmit_fifo_flush() {}
    }

    pub fn set_rx_fifo_threshold(threshold: usize) {
        registers!(mmio::I2C_PORT_1);
        debug_assert!(
            threshold <= 8,
            "Cannot set the bytes threshold {threshold} over the max register threshold of 8!"
        );

        unsafe {
            ReceiveControl0::set_receive_fifo_threshold_level(threshold as u8);
        }
    }

    pub fn set_tx_fifo_threshold(threshold: usize) {
        registers!(mmio::I2C_PORT_1);
        debug_assert!(
            threshold <= 7,
            "Cannot set the bytes threshold {threshold} over the max register threshold of 8!"
        );

        unsafe {
            TransmitControl0::set_transmit_fifo_threshold_level(threshold as u8);
        }
    }

    pub fn enable_master(flag: bool) -> Result<()> {
        registers!(mmio::I2C_PORT_1);

        if flag {
            // Another Master is currently controlling the bus,
            // we should not enable master mode!
            if StatusRegister::is_transaction_active() {
                return Err(ErrorKind::BadState);
            }

            unsafe {
                ControlRegister::set_master_mode_enable(true);
            }
        } else {
            unsafe {
                ControlRegister::set_master_mode_enable(false);
            }
        }

        Ok(())
    }

    pub fn bus_recover(retry_count: usize) -> Result<()> {
        registers!(mmio::I2C_PORT_1);

        // Save the state so we can restore it
        let state_prior = ControlRegister::read();

        // Switch to Software Mode, and enable the I2C bus
        unsafe {
            ControlRegister::set_software_i2c_mode(true);
            ControlRegister::set_i2c_peripheral_enable(true);
        }

        let release_scl_and_sda = || unsafe {
            ControlRegister::set_scl_hardware_pin_released(true);
            ControlRegister::set_sda_hardware_pin_released(true);
        };

        let mut success = false;
        // Lets try and recover the bus
        for _ in 0..retry_count {
            microcontroller_delay(10);

            // Pull SCL low
            unsafe {
                ControlRegister::set_scl_hardware_pin_released(false);
            }

            microcontroller_delay(10);

            // If SCL is high we were unable to pull the bus low
            if ControlRegister::get_scl_pin() {
                release_scl_and_sda();
                continue;
            }

            microcontroller_delay(10);

            unsafe {
                ControlRegister::set_scl_hardware_pin_released(true);
            }

            microcontroller_delay(10);

            // If SCL is low we were unable to release the bus
            if !ControlRegister::get_scl_pin() {
                release_scl_and_sda();
                continue;
            }

            microcontroller_delay(10);

            unsafe {
                ControlRegister::set_sda_hardware_pin_released(false);
            }

            microcontroller_delay(10);

            // If SDA is high we were unable to pull the bus low
            if ControlRegister::get_sda_pin() {
                release_scl_and_sda();
                continue;
            }

            microcontroller_delay(10);

            unsafe {
                ControlRegister::set_sda_hardware_pin_released(true);
            }

            microcontroller_delay(10);

            // If SDA is low we were unable to pull release the bus
            if !ControlRegister::get_sda_pin() {
                release_scl_and_sda();
                continue;
            }

            // We where able to take control over the bus!
            success = true;
            break;
        }

        // We could not take control over the bus
        if !success {
            return Err(ErrorKind::ComError);
        }

        unsafe {
            ControlRegister::write(state_prior);
        }

        Ok(())
    }
}
