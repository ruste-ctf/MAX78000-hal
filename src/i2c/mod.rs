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

/// # We need this for I2C, but uh I have not gotten to it yet :)
fn microcontroller_delay(_us: usize) {
    todo!("Make the timers")
}

impl I2C<I2CPort0> {
    fn init(enable_master: bool, slave_address: usize) -> Result<Self> {
        registers!(mmio::I2C_PORT_0);

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
            Self::set_hardware_slave_address(slave_address);
        }

        Ok(Self {
            master_enabled: enable_master,
            slave_address,
            ph: PhantomData,
        })
    }

    fn set_hardware_slave_address(address: usize) -> Result<()> {
        registers!(mmio::I2C_PORT_0);

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

    pub fn clear_rx_fifo() {
        registers!(mmio::I2C_PORT_0);
        unsafe {
            ReceiveControl0::activate_flush_receive_fifo();
        }

        while ReceiveControl0::is_flush_receive_fifo() {}
    }

    pub fn clear_tx_fifo() {
        registers!(mmio::I2C_PORT_0);
        unsafe {
            TransmitControl0::activate_transmit_fifo_flush();
        }

        while TransmitControl0::is_transmit_fifo_flush() {}
    }

    pub fn set_rx_fifo_threshold(threshold: usize) {
        registers!(mmio::I2C_PORT_0);
        debug_assert!(
            threshold <= 8,
            "Cannot set the bytes threshold {threshold} over the max register threshold of 8!"
        );

        unsafe {
            ReceiveControl0::set_receive_fifo_threshold_level(threshold as u8);
        }
    }

    pub fn set_tx_fifo_threshold(threshold: usize) {
        registers!(mmio::I2C_PORT_0);
        debug_assert!(
            threshold <= 7,
            "Cannot set the bytes threshold {threshold} over the max register threshold of 8!"
        );

        unsafe {
            TransmitControl0::set_transmit_fifo_threshold_level(threshold as u8);
        }
    }

    pub fn enable_master(flag: enable) -> Result<()> {
        registers!(mmio::I2C_PORT_0);

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
        registers!(mmio::I2C_PORT_0);

        // Save the state so we can restore it
        let state_prior = ControlRegister::read();

        // Switch to Software Mode, and enable the I2C bus
        unsafe {
            ControlRegister::set_software_i2c_mode(true);
            ControlRegister::set_i2c_peripheral_enable(true);
        }

        // Both the SCL and SDA pins should be high
        if !ControlRegister::get_scl_pin() || !ControlRegister::get_sda_pin() {
            return Err(ErrorKind::ComError);
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
