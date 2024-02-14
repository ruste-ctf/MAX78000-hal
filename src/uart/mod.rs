use crate::error::{ErrorKind, Result};
use crate::memory_map::mmio;
use crate::uart::registers::{ControlRegister, StatusRegister};
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
        type InterruptFlagRegister = registers::InterrptFlagRegister<{ $port }>;
        #[allow(unused)]
        type ClockDivisorRegister = registers::ClockDivisorRegister<{ $port }>;
        #[allow(unused)]
        type OversamplingControlRegister = registers::OversamplingControlRegister<{ $port }>;
        #[allow(unused)]
        type TransmitFIFORegister = registers::TransmitFIFORegister<{ $port }>;
        #[allow(unused)]
        type PinControlregister = registers::PinControlRegister<{ $port }>;
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
        UART::<UART0>::init()
    }
}

impl UART<UART0> {
    pub fn init() -> Self {
        registers!(mmio::UART_0);
        // Clear the FIFOs
        Self::clear_rx_fifo();
        Self::clear_tx_fifo();

        // Set the character length to 8
        Self::set_character_length(3).unwrap();

        // Set the number of stop bits to 1
        Self::set_number_stop_bits(false);

        // Dissable parity
        Self::parity_enable(false);

        Self { ph: PhantomData }
    }

    pub fn clear_rx_fifo() {
        registers!(mmio::UART_0);

        unsafe {
            ControlRegister::activate_receive_fifo_flush();
        }
    }

    pub fn clear_tx_fifo() {
        registers!(mmio::UART_0);

        unsafe {
            ControlRegister::activiate_transmit_fifo_flush();
        }
    }

    pub fn set_character_length(length: u8) -> Result<()> {
        registers!(mmio::UART_0);

        // If the value is not in the allowed range, Err
        if length > 3 {
            return Err(ErrorKind::BadParam);
        }

        unsafe {
            ControlRegister::set_character_length(length);
        }
        // TODO Check if this will every return
        while ControlRegister::check_character_length() != length {}
        Ok(())
    }

    pub fn set_baud_clock_source(source: u8) -> Result<()> {
        registers!(mmio::UART_0);

        // If the value is not in the allowed range, Err
        if source > 3 {
            return Err(ErrorKind::BadParam);
        }

        unsafe {
            ControlRegister::set_baud_clock_source(source);
        }
        // TODO Check if this will every return
        while ControlRegister::check_baud_clock_source() != source {}
        Ok(())
    }

    pub fn set_baud_clock(enable: bool) {
        registers!(mmio::UART_0);
        unsafe { ControlRegister::set_baud_clock_enable(enable) }
    }

    pub fn set_number_stop_bits(number: bool) {
        registers!(mmio::UART_0);
        unsafe {
            // A "number" of true means use 1 stop bit
            ControlRegister::set_number_of_stop_bits(number);
        }
    }

    pub fn parity_enable(enabled: bool) {
        registers!(mmio::UART_0);
        unsafe {
            ControlRegister::set_transmit_parity_generation_enable(enabled);
        }
    }

    pub fn receive_busy() -> bool {
        registers!(mmio::UART_0);
        StatusRegister::is_receive_busy()
    }

    pub fn transmit_busy() -> bool {
        registers!(mmio::UART_0);
        StatusRegister::is_transmit_busy()
    }

    pub fn peek_transmit_fifo() -> u8 {
        registers!(mmio::UART_0);
        TransmitFIFORegister::get_transmit_fifo_data()
    }
}
