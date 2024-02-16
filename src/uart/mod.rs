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

#[allow(unused)]
impl UART<NoPort> {
    pub fn port_0_init() -> UART<UART0> {
        UART::<UART0>::init()
    }
}

impl UART<UART0> {
    fn init() -> Self {
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

    pub fn print_string(&self, string: &str) {
        for char in string.bytes() {
            while self.transmit_busy() {}
            self.write_transmit_fifo(char);
        }
    }

    pub fn clear_rx_fifo(&self) {
        unsafe {
            ControlRegister::activate_receive_fifo_flush();
        }
    }

    pub fn clear_tx_fifo(&self) {
        unsafe {
            ControlRegister::activiate_transmit_fifo_flush();
        }
    }

    pub fn set_character_length(&self, length: u8) -> Result<()> {
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
        unsafe { ControlRegister::set_baud_clock_enable(enable) }
    }

    pub fn set_number_stop_bits(&self, number: bool) {
        unsafe {
            // A "number" of true means use 1 stop bit
            ControlRegister::set_number_of_stop_bits(number);
        }
    }

    pub fn parity_enable(&self, enabled: bool) {
        unsafe {
            ControlRegister::set_transmit_parity_generation_enable(enabled);
        }
    }

    pub fn receive_busy(&self) -> bool {
        StatusRegister::is_receive_busy()
    }

    pub fn transmit_busy(&self) -> bool {
        StatusRegister::is_transmit_busy()
    }

    pub fn peek_transmit_fifo(&self) -> u8 {
        TransmitFIFORegister::get_transmit_fifo_data()
    }

    pub fn get_transmit_fifo_level(&self) -> u8 {
        StatusRegister::get_transmit_fifo_level()
    }

    pub fn get_receive_fifo_level(&self) -> u8 {
        StatusRegister::get_receive_fifo_level()
    }

    pub fn read_receive_fifo(&self) -> u8 {
        DataRegister::get_receive_fifo_data()
    }

    pub fn write_transmit_fifo(&self, data: u8) {
        unsafe {
            DataRegister::set_transmit_fifo_data(data);
        }
    }

    pub fn set_clock_divisor(&self, divisor: u32) {
        unsafe {
            // FIXME the functions need to be renamed
            ClockDivisorRegister::get_baud_rate_divisor(divisor);
        }
    }
}
