use crate::error::Result;
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
    PLCK = 0,
    IBRO = 2,
}

/// # Stop Bits
/// The number of stop bits to use.
/// Note: When using a character length of five bits, passing the variant
/// `TwoBits` uses 1.5 bits.
#[repr(u8)]
pub enum StopBits {
    OneBit = 0,
    TwoBits = 1,
}

impl<Port: private::UARTPortCompatable> UART<Port> {
    /// Creates an UART instance configured to use 8 bits, 1 stop bit, and no parity
    /// TODO make this more generic
    fn init() -> Self {
        let mut uart = Self {
            reg: registers::Registers::new(Port::PORT_PTR),
            ph: PhantomData,
        };

        // Clear the FIFOs
        uart.clear_rx_fifo();
        uart.clear_tx_fifo();

        // Set the character length to 8
        uart.set_character_length(CharacterLength::EightBits)
            .unwrap();

        // Set the number of stop bits to 1
        uart.set_number_stop_bits(StopBits::OneBit);

        // Dissable parity
        uart.transmit_parity_enable(false);
        uart
    }

    /// # Print String
    /// Prints the string passed
    pub fn print_string(&mut self, string: &str) {
        for char in string.bytes() {
            while self.transmit_busy() {}
            self.write_transmit_fifo(char);
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

    /// # Set Character Length
    /// Sets the number of data bits to send in a UART frame
    pub fn set_character_length(&mut self, length: CharacterLength) -> Result<()> {
        unsafe {
            self.reg.set_character_length(length as u8);
        }
        Ok(())
    }

    /// # Set Baud Clock Source
    /// Sets the clock to derive the baud clock from
    pub fn set_baud_clock_source(&mut self, source: ClockSources) -> Result<()> {
        unsafe {
            self.reg.set_baud_clock_source(source as u8);
        }
        Ok(())
    }

    /// # Set Baud Clock
    /// Enables or disables the baud clock
    pub fn set_baud_clock(&mut self, enable: bool) {
        unsafe { self.reg.set_baud_clock_enable(enable) }
    }

    /// # Set Number Stop Bits
    /// Sets the number of stop bits to use
    pub fn set_number_stop_bits(&mut self, number: StopBits) {
        let number = match number {
            StopBits::OneBit => false,
            StopBits::TwoBits => true,
        };
        unsafe {
            // A "number" of true means use 1 stop bit
            self.reg.set_number_of_stop_bits(number);
        }
    }

    /// # Parity Enable
    /// Enables or disables the generation and transmission of the parity bit
    pub fn transmit_parity_enable(&mut self, enabled: bool) {
        unsafe {
            self.reg.set_transmit_parity_generation_enable(enabled);
        }
    }

    /// # Receive Busy
    /// Returns true if there is a character being received
    pub fn receive_busy(&self) -> bool {
        self.reg.get_receive_busy()
    }

    /// # Transmit Busy
    /// Returns true if there is a character being transmitted
    pub fn transmit_busy(&self) -> bool {
        self.reg.get_transmit_busy()
    }

    /// # Peek Transmit FIFO
    /// Reads the next character to be transmitted, without changing it
    pub fn peek_transmit_fifo(&self) -> u8 {
        self.reg.get_transmit_fifo_data()
    }

    /// # Get Transmit FIFO Level
    /// Gets the current level of the transmit FIFO
    pub fn get_transmit_fifo_level(&self) -> u8 {
        self.reg.get_transmit_fifo_level()
    }

    /// # Get Receive FIFO Level
    /// Gets the current level of the receive FIFO
    pub fn get_receive_fifo_level(&self) -> u8 {
        self.reg.get_receive_fifo_level()
    }

    /// # Read Receive FIFO
    /// Reads data from the receive FIFO
    pub fn read_receive_fifo(&self) -> u8 {
        self.reg.get_fifo_data()
    }

    /// # Write Transmit FIFO
    /// Writes data to the transmit FIFO
    pub fn write_transmit_fifo(&mut self, data: u8) {
        unsafe {
            self.reg.set_fifo_data(data);
        }
    }

    /// # Set Clock Divisor
    /// Sets the divisor to use in UART clock generation
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
