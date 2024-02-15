use crate::error::{ErrorKind, Result};
use crate::memory_map::mmio;
use crate::uart::registers::{ClockDivisorRegister, ControlRegister, StatusRegister};
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
    fn init() -> Self {
        registers!(mmio::UART_0);
        let uart = Self { ph: PhantomData };

        // Clear the FIFOs
        uart.clear_rx_fifo();
        uart.clear_tx_fifo();

        // Set the character length to 8
        uart.set_character_length(3).unwrap();

        // Set the number of stop bits to 1
        uart.set_number_stop_bits(false);

        // Dissable parity
        uart.parity_enable(false);
        uart
    }

    pub fn clear_rx_fifo(&self) {
        registers!(mmio::UART_0);

        unsafe {
            ControlRegister::activate_receive_fifo_flush();
        }
    }

    pub fn clear_tx_fifo(&self) {
        registers!(mmio::UART_0);

        unsafe {
            ControlRegister::activiate_transmit_fifo_flush();
        }
    }

    pub fn set_character_length(&self, length: u8) -> Result<()> {
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

    pub fn set_baud_clock_source(&self, source: u8) -> Result<()> {
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

    pub fn set_baud_clock(&self, enable: bool) {
        registers!(mmio::UART_0);
        unsafe { ControlRegister::set_baud_clock_enable(enable) }
    }

    pub fn set_number_stop_bits(&self, number: bool) {
        registers!(mmio::UART_0);
        unsafe {
            // A "number" of true means use 1 stop bit
            ControlRegister::set_number_of_stop_bits(number);
        }
    }

    pub fn parity_enable(&self, enabled: bool) {
        registers!(mmio::UART_0);
        unsafe {
            ControlRegister::set_transmit_parity_generation_enable(enabled);
        }
    }

    pub fn receive_busy(&self) -> bool {
        registers!(mmio::UART_0);
        StatusRegister::is_receive_busy()
    }

    pub fn transmit_busy(&self) -> bool {
        registers!(mmio::UART_0);
        StatusRegister::is_transmit_busy()
    }

    pub fn peek_transmit_fifo(&self) -> u8 {
        registers!(mmio::UART_0);
        TransmitFIFORegister::get_transmit_fifo_data()
    }

    pub fn get_transmit_fifo_level(&self) -> u8 {
        registers!(mmio::UART_0);
        StatusRegister::get_transmit_fifo_level()
    }

    pub fn get_receive_fifo_level(&self) -> u8 {
        registers!(mmio::UART_0);
        StatusRegister::get_receive_fifo_level()
    }

    pub fn read_receive_fifo(&self) -> u8 {
        registers!(mmio::UART_0);
        DataRegister::get_receive_fifo_data()
    }

    pub fn write_transmit_fifo(&self, data: u8) {
        registers!(mmio::UART_0);
        unsafe {
            DataRegister::set_transmit_fifo_data(data);
        }
    }

    pub fn set_clock_divisor(&self, divisor: u32) {
        registers!(mmio::UART_0);
        unsafe {
            // FIXME the functions need to be renamed
            ClockDivisorRegister::get_baud_rate_divisor(divisor);
        }
    }
}
