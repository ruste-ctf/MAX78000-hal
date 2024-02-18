use crate::error::{ErrorKind, Result};
use crate::memory_map::mmio;
use core::marker::PhantomData;

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
    reg: registers::Registers,
    ph: PhantomData<Port>,
}

#[allow(unused)]
impl UART<NoPort> {
    pub fn port_0_init(
        baud_rate: BaudRates,
        character_length: CharacterLength,
        stop_bits: StopBits,
        transmit_parity: bool,
        parity_value: ParityValueSelect,
        hfc: bool,
    ) -> UART<UART0> {
        UART::<UART0>::init(
            baud_rate,
            character_length,
            stop_bits,
            transmit_parity,
            parity_value,
            hfc,
        )
    }

    pub fn port_1_init(
        baud_rate: BaudRates,
        character_length: CharacterLength,
        stop_bits: StopBits,
        transmit_parity: bool,
        parity_value: ParityValueSelect,
        hfc: bool,
    ) -> UART<UART1> {
        UART::<UART1>::init(
            baud_rate,
            character_length,
            stop_bits,
            transmit_parity,
            parity_value,
            hfc,
        )
    }

    pub fn port_2_init(
        baud_rate: BaudRates,
        character_length: CharacterLength,
        stop_bits: StopBits,
        transmit_parity: bool,
        parity_value: ParityValueSelect,
        hfc: bool,
    ) -> UART<UART2> {
        UART::<UART2>::init(
            baud_rate,
            character_length,
            stop_bits,
            transmit_parity,
            parity_value,
            hfc,
        )
    }
}
#[repr(u32)]
pub enum BaudRates {
    Baud1200 = 1200,
    Baud2400 = 2400,
    Baud4800 = 4800,
    Baud9600 = 9600,
    Baud19200 = 19200,
    Baud38400 = 38400,
    Baud57600 = 57600,
    Baud115200 = 115200,
}

/// # Character Length
/// The number of data bits in a UART frame.
#[repr(u8)]
pub enum CharacterLength {
    FiveBits = 0,
    SixBits = 1,
    SevenBits = 2,
    EightBits = 3,
}

/// # Clock Sources
/// The clock source to use for UART
#[repr(u8)]
pub enum ClockSources {
    PCLK = 0,
    IBRO = 2,
}

/// # Stop Bits
/// The number of stop bits to use.
/// Note: When using a character length of five bits, passing the variant
/// `TwoBits` uses 1.5 bits.
pub enum StopBits {
    OneBit,
    TwoBits,
}

#[repr(u8)]
pub enum ThresholdSize {
    Threshold1 = 1,
    Threshold2 = 2,
    Threshold3 = 3,
    Threshold4 = 4,
    Threshold5 = 5,
    Threshold6 = 6,
    Threshold7 = 7,
    Threshold8 = 8,
}

pub enum HFCDeassertCondition {
    EqualsFIFODepth,
    ExceedsRxThreshold,
}

pub enum ParityValueSelect {
    OneBased,
    ZeroBased,
}

impl<Port: private::UARTPortCompatable> UART<Port> {
    /// TODO make this more generic
    fn init(
        baud_rate: BaudRates,
        character_length: CharacterLength,
        stop_bits: StopBits,
        transmit_parity: bool,
        parity_value: ParityValueSelect,
        hfc: bool,
    ) -> Self {
        let mut uart = Self {
            reg: registers::Registers::new(Port::PORT_PTR),
            ph: PhantomData,
        };

        // Clear the FIFOs
        uart.clear_rx_fifo();
        uart.clear_tx_fifo();

        unsafe {
            uart.reg.set_character_length(character_length as u8);
            // Set the number of stop bits to 1
            let stop_bits = match stop_bits {
                StopBits::OneBit => true,
                StopBits::TwoBits => false,
            };
            uart.reg.set_number_of_stop_bits(stop_bits);
            uart.reg
                .set_transmit_parity_generation_enable(transmit_parity);
            // Set the parity value
            let parity_value = match parity_value {
                ParityValueSelect::OneBased => false,
                ParityValueSelect::ZeroBased => true,
            };
            uart.reg.set_parity_value(parity_value);
            // Set the clock source to IBRO
            uart.reg.set_baud_clock_source(ClockSources::IBRO as u8);
            // Set the clock divisor to 7.3728 Mhz / baud rate
            let divisor = 7372800 / baud_rate as u32;
            uart.reg.set_baud_rate_divisor(divisor);
            // Set the Hardware Flow Control
            uart.reg.set_hardware_flow_control(hfc);
        }

        uart
    }

    /// # Print String
    /// Prints the string passed
    pub fn print_string(&mut self, string: &str) {
        for char in string.bytes() {
            self.write_blocking_transmit_fifo(char);
        }
    }

    /// # Clear RX FIFO
    /// Clears all data from the receiving FIFO
    pub fn clear_rx_fifo(&mut self) {
        unsafe {
            self.reg.activate_receive_fifo_flush();
        }
    }

    /// # Clear TX FIFO
    /// Clears all data from the transmit FIFO
    pub fn clear_tx_fifo(&mut self) {
        unsafe {
            self.reg.activate_transmit_fifo_flush();
        }
    }

    /// # Write Blocking Transmit FIFO
    /// Writes to the FIFO, waiting until it is empty
    pub fn write_blocking_transmit_fifo(&mut self, data: u8) {
        while self.reg.get_transmit_busy() {}
        unsafe {
            self.reg.set_fifo_data(data);
        }
    }

    /// # Read Blocking Receive FIFO
    /// Reads from the receive FIFO, but only after it is done receiving
    pub fn read_delay_receive_fifo(&mut self) -> Result<u8> {
        if self.reg.get_receive_fifo_level() == 0 {
            Err(ErrorKind::Busy)
        } else {
            Ok(self.reg.get_fifo_data())
        }
    }
}

impl<Port: private::UARTPortCompatable> core::fmt::Write for UART<Port> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print_string(s);
        Ok(())
    }
}
