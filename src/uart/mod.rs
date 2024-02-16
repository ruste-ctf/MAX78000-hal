use crate::error::{ErrorKind, Result};
use crate::memory_map::mmio;
use core::marker::PhantomData;

use self::private::UARTPortCompatable;
use self::registers::Registers;
pub mod registers;

mod private {
    pub trait UARTPortCompatable {
        const PORT_PTR: usize;
    }
}

pub struct NoPort {}
pub struct UART0 {}
pub struct UART1 {}
pub struct UART2 {}

impl private::UARTPortCompatable for UART0 {
    const PORT_PTR: usize = mmio::UART_0;
}
impl private::UARTPortCompatable for UART1 {
    const PORT_PTR: usize = mmio::UART_1;
}
impl private::UARTPortCompatable for UART2 {
    const PORT_PTR: usize = mmio::UART_2;
}

pub struct UART<Port = NoPort> {
    reg: Registers,
    ph: PhantomData<Port>,
}

#[allow(unused)]
impl UART<NoPort> {
    pub fn port_0_init() -> UART<UART0> {
        UART::<UART0>::init()
    }

    pub fn port_1_init() -> UART<UART1> {
        UART::<UART1>::init()
    }

    pub fn port_2_init() -> UART<UART2> {
        UART::<UART2>::init()
    }
}

impl<Port: private::UARTPortCompatable> UART<Port> {
    fn init() -> Self {
        let mut uart = Self {
            reg: Registers::new(Port::PORT_PTR),
            ph: PhantomData,
        };

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

    pub fn print_string(&mut self, string: &str) {
        for char in string.bytes() {
            while self.transmit_busy() {}
            self.write_transmit_fifo(char);
        }
    }

    pub fn clear_rx_fifo(&mut self) {
        unsafe {
            self.reg.activate_receive_fifo_flush();
        }
    }

    pub fn clear_tx_fifo(&mut self) {
        unsafe {
            self.reg.activate_transmit_fifo_flush();
        }
    }

    pub fn set_character_length(&mut self, length: u8) -> Result<()> {
        // If the value is not in the allowed range, Err
        if length > 3 {
            return Err(ErrorKind::BadParam);
        }

        unsafe {
            self.reg.set_character_length(length);
        }
        Ok(())
    }

    pub fn set_baud_clock_source(&mut self, source: u8) -> Result<()> {
        // If the value is not in the allowed range, Err
        if source > 3 {
            return Err(ErrorKind::BadParam);
        }

        unsafe {
            self.reg.set_baud_clock_source(source);
        }
        Ok(())
    }

    pub fn set_baud_clock(&mut self, enable: bool) {
        unsafe { self.reg.set_baud_clock_enable(enable) }
    }

    pub fn set_number_stop_bits(&mut self, number: bool) {
        unsafe {
            // A "number" of true means use 1 stop bit
            self.reg.set_number_of_stop_bits(number);
        }
    }

    pub fn parity_enable(&mut self, enabled: bool) {
        unsafe {
            self.reg.set_transmit_parity_generation_enable(enabled);
        }
    }

    pub fn receive_busy(&self) -> bool {
        self.reg.get_receive_busy()
    }

    pub fn transmit_busy(&self) -> bool {
        self.reg.get_transmit_busy()
    }

    pub fn peek_transmit_fifo(&self) -> u8 {
        self.reg.get_transmit_fifo_data()
    }

    pub fn get_transmit_fifo_level(&self) -> u8 {
        self.reg.get_transmit_fifo_level()
    }

    pub fn get_receive_fifo_level(&self) -> u8 {
        self.reg.get_receive_fifo_level()
    }

    pub fn read_receive_fifo(&self) -> u8 {
        self.reg.get_fifo_data()
    }

    pub fn write_transmit_fifo(&mut self, data: u8) {
        unsafe {
            self.reg.set_fifo_data(data);
        }
    }

    pub fn set_clock_divisor(&mut self, divisor: u32) {
        unsafe {
            self.reg.set_baud_rate_divisor(divisor);
        }
    }
}

impl<Port: private::UARTPortCompatable> core::fmt::Write for UART<Port> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print_string(s);
        Ok(())
    }
}
