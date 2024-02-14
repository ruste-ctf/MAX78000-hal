use crate::error::{ErrorKind, Result};
use crate::i2c::registers::ControlRegister;
use crate::memory_map::mmio;
use core::marker::PhantomData;
pub mod registers;

mod private {
    pub trait UARTPortCompatable {}
}

pub struct NoPort {}
pub struct UART0 {}
pub struct UART1 {}
pub struct UART2 {}

impl private::UARTPortCompatable for NoPort {}
impl private::UARTPortCompatable for UART0 {}
impl private::UARTPortCompatable for UART1 {}
impl private::UARTPortCompatable for UART2 {}

pub struct UART<Port: private::UARTPortCompatable = NoPort> {
    ph: PhantomData<Port>,
}

macro_rules! registers {
    ($port:expr) => {
        #[allow(unused)]
        type ControlRegister = registers::ControlRegister<{ $port }>;
        #[allow(unused)]
        type StatusRegister = registers::StatusRegister<{ $port }>;
        #[allow(unused)]
        type InterruptEnableRegister = registers::InterruptEnableRegister<{ $port }>;
        #[allow(unused)]
        type InterruptFlagRegister = registers::InterruptFlagRegister<{ $port }>;
        #[allow(unused)]
        type ClockDivisorRegister = registers::ClockDivisorRegister<{ $port }>;
        #[allow(unused)]
        type OversamplingControlRegister = registers::OversamplingControlRegister<{ $port }>;
        #[allow(unused)]
        type TransmitFIFORegister = registers::TransmitFIFORegister<{ $port }>;
        #[allow(unused)]
        type PinControlregister = registers::PinControlregister<{ $port }>;
        #[allow(unused)]
        type DataRegister = registers::DataRegister<{ $port }>;
        #[allow(unused)]
        type DMARegister = registers::DMARegister<{ $port }>;
        #[allow(unused)]
        type WakeupEnableRegister = registers::WakeupEnableRegister<{ $port }>;
        #[allow(unused)]
        type WakeupFlagRegister = registers::WakeupFlagRegister<{ $port }>;
    };
}

#[allow(unused)]
impl UART<NoPort> {
    pub fn port_0_init() -> UART<UART0> {
        TODO!
    }
}

impl UART<UART0> {
    fn init() -> Self {
        registers!(mmio::UART_0);
        // Clear the FIFOs
        Self::clear_rx_fifo();
        Self::clear_tx_fifo();

        Self { ph: PhantomData }
    }

    fn clear_rx_fifo() {
        registers!(mmio::UART_0);

        unsafe {
            ControlRegister::activate_receive_fifo_flush();
        }

        while ControlRegister::is_receive_fifo_full() {}
        
    }

    fn clear_tx_fifo() {
        registers!(mmio::UART_0);

        unsafe {
            ControlRegister::activiate_transmit_fifo_flush();
        }

        while ControlRegister::is_transmit_fifo_full() {}
        
    }

    fn set_character_length(length: u8) -> Result<()> {
        registers!(mmio::UART_0);

        // If the value is not in the allowed range, Err
        if length < 0 || length > 3 {
            return Err(ErrorKind::BadParam);
        }
        

        unsafe {
            ControlRegister::set_character_length(length);
        }
        // TODO Check if this will every return
        while ControlRegister::check_character_length() != length {}
        Ok(())
    }

    fn set_baud_clocks_source(source: u8) -> Result<()> {
        registers!(mmio::UART_0);        
        
        // If the value is not in the allowed range, Err
        if source < 0 || source > 3 {
            return Err(ErrorKind::BadParam);
        }

        unsafe {
            ControlRegister::set_baud_clock_source(source);
        }
        // TODO Check if this will every return
        while ControlRegister::check_baud_clock_source() != source {}
        Ok(())
        
    }
}
