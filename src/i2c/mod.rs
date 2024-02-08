use crate::error::Result;
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

pub struct I2CMaster<Port: private::I2CPortCompatable = NoPort> {
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
        type ControlRegister = registers::ControlRegister<{ $port }>;
        type DMAControl = registers::DMAControl<{ $port }>;
        type DataRegister = registers::DataRegister<{ $port }>;
        type FIFOLengthRegister = registers::FIFOLengthRegister<{ $port }>;
        type HighSCLControl = registers::HighSCLControl<{ $port }>;
        type HighSpeedClockControl = registers::HighSpeedClockControl<{ $port }>;
        type InterruptEnable0 = registers::InterruptEnable0<{ $port }>;
        type InterruptEnable1 = registers::InterruptEnable1<{ $port }>;
        type InterruptFlag0 = registers::InterruptFlag0<{ $port }>;
        type InterruptFlag1 = registers::InterruptFlag1<{ $port }>;
        type LowSCLControl = registers::LowSCLControl<{ $port }>;
        type MasterControl = registers::MasterControl<{ $port }>;
        type ReceiveControl0 = registers::ReceiveControl0<{ $port }>;
        type ReceiveControl1 = registers::ReceiveControl1<{ $port }>;
        type SlaveAddress = registers::SlaveAddress<{ $port }>;
        type StatusRegister = registers::StatusRegister<{ $port }>;
        type TimeoutControl = registers::TimeoutControl<{ $port }>;
        type TransmitControl0 = registers::TransmitControl0<{ $port }>;
        type TransmitControl1 = registers::TransmitControl1<{ $port }>;
    };
}

#[allow(unused)]
impl I2CMaster<NoPort> {
    pub fn port_0_init() -> Result<I2CMaster<I2CPort0>> {
        todo!()
    }

    pub fn port_1_init() -> Result<I2CMaster<I2CPort1>> {
        todo!()
    }

    pub fn port_2_init() -> Result<I2CMaster<I2CPort2>> {
        todo!()
    }
}
