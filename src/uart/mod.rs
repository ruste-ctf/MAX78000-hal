use crate::error::{ErrorKind, Result};
use crate::gcr::{peripheral_reset, system_clock_enable};
use crate::gpio::GpioPin;
use crate::memory_map::mmio;
use core::marker::PhantomData;

pub mod registers;

mod private {
    pub trait UARTPortCompatable {
        const PORT_PTR: usize;
        const PORT_NUM: usize;
    }
}

pub struct NoPort {}
pub struct UART0 {}
pub struct UART1 {}
pub struct UART2 {}

impl private::UARTPortCompatable for UART0 {
    const PORT_PTR: usize = mmio::UART_0;
    const PORT_NUM: usize = 0;
}
impl private::UARTPortCompatable for UART1 {
    const PORT_PTR: usize = mmio::UART_1;
    const PORT_NUM: usize = 1;
}
impl private::UARTPortCompatable for UART2 {
    const PORT_PTR: usize = mmio::UART_2;
    const PORT_NUM: usize = 2;
}

pub struct UART<Port = NoPort> {
    reg: registers::Registers,
    _ph: PhantomData<Port>,
    _gpio: [GpioPin; 2],
}

#[allow(unused)]
impl UART<NoPort> {
    /// # Port 0 Init
    /// Initializes UART 0
    /// # Arguments
    /// * `baud_rate` - The baud rate that the UART will use
    /// * `character_length` - The number of data bits that will be transferred in a frame
    /// * `stop_bits` - The number of stop bits that will be used
    /// * `transmit_parity` - Enables the generation of the parity bit
    /// * `parity` - Specifies whether to use odd, or even parity
    /// * `hfc` - Enables the use of hardware flow control
    /// # Example
    ///
    /// ```no_run
    /// use max78000_hal::uart::{UART, BaudRates, CharacterLength, StopBits, ParityValueSelect, Parity};
    /// let mut uart_test = UART::port_0_init(
    ///     BaudRates::Baud115200,
    ///     CharacterLength::EightBits,
    ///     StopBits::OneBit,
    ///     false,
    ///     Parity::Odd,
    ///     ParityValueSelect::OneBased,
    ///     false,
    /// );
    /// ```
    pub fn port_0_init(
        baud_rate: BaudRates,
        character_length: CharacterLength,
        stop_bits: StopBits,
        transmit_parity: bool,
        parity: Parity,
        parity_value: ParityValueSelect,
        hfc: bool,
    ) -> Result<UART<UART0>> {
        peripheral_reset(crate::gcr::HardwareSource::UART0);
        system_clock_enable(crate::gcr::HardwareSource::UART0, true);
        UART::<UART0>::init(
            baud_rate,
            character_length,
            stop_bits,
            transmit_parity,
            parity,
            parity_value,
            hfc,
        )
    }
    /// # Port 1 Init
    /// Initializes UART 1
    /// # Arguments
    /// * `baud_rate` - The baud rate that the UART will use
    /// * `character_length` - The number of data bits that will be transferred in a frame
    /// * `stop_bits` - The number of stop bits that will be used
    /// * `transmit_parity` - Enables the generation of the parity bit
    /// * `parity` - Specifies whether to use odd, or even parity
    /// * `hfc` - Enables the use of hardware flow control
    /// # Example
    ///
    /// ```no_run
    /// use max78000_hal::uart::{UART, BaudRates, CharacterLength, StopBits, ParityValueSelect, Parity};
    /// let mut uart_test = UART::port_0_init(
    ///     BaudRates::Baud115200,
    ///     CharacterLength::EightBits,
    ///     StopBits::OneBit,
    ///     false,
    ///     Parity::Odd,
    ///     ParityValueSelect::OneBased,
    ///     false,
    /// );
    /// ```
    pub fn port_1_init(
        baud_rate: BaudRates,
        character_length: CharacterLength,
        stop_bits: StopBits,
        transmit_parity: bool,
        parity: Parity,
        parity_value: ParityValueSelect,
        hfc: bool,
    ) -> Result<UART<UART1>> {
        peripheral_reset(crate::gcr::HardwareSource::UART1);
        system_clock_enable(crate::gcr::HardwareSource::UART1, true);
        UART::<UART1>::init(
            baud_rate,
            character_length,
            stop_bits,
            transmit_parity,
            parity,
            parity_value,
            hfc,
        )
    }

    /// # Port 2 Init
    /// Initializes UART 2
    /// # Arguments
    /// * `baud_rate` - The baud rate that the UART will use
    /// * `character_length` - The number of data bits that will be transferred in a frame
    /// * `stop_bits` - The number of stop bits that will be used
    /// * `transmit_parity` - Enables the generation of the parity bit
    /// * `parity` - Specifies whether to use odd, or even parity
    /// * `hfc` - Enables the use of hardware flow control
    /// # Example
    ///
    /// ```no_run
    /// use max78000_hal::uart::{UART, BaudRates, CharacterLength, StopBits, ParityValueSelect, Parity};
    /// let mut uart_test = UART::port_0_init(
    ///     BaudRates::Baud115200,
    ///     CharacterLength::EightBits,
    ///     StopBits::OneBit,
    ///     false,
    ///     Parity::Odd,
    ///     ParityValueSelect::OneBased,
    ///     false,
    /// );
    /// ```
    pub fn port_2_init(
        baud_rate: BaudRates,
        character_length: CharacterLength,
        stop_bits: StopBits,
        transmit_parity: bool,
        parity: Parity,
        parity_value: ParityValueSelect,
        hfc: bool,
    ) -> Result<UART<UART2>> {
        peripheral_reset(crate::gcr::HardwareSource::UART2);
        system_clock_enable(crate::gcr::HardwareSource::UART2, true);
        UART::<UART2>::init(
            baud_rate,
            character_length,
            stop_bits,
            transmit_parity,
            parity,
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

impl Into<bool> for StopBits {
    fn into(self) -> bool {
        match self {
            StopBits::OneBit => false,
            StopBits::TwoBits => true,
        }
    }
}

/// # Hardware Flow Control Deassert Condition
/// When to deassert the hardware flow control
pub enum HFCDeassertCondition {
    EqualsFIFODepth,
    ExceedsRxThreshold,
}

pub enum ParityValueSelect {
    OneBased,
    ZeroBased,
}

impl Into<bool> for ParityValueSelect {
    fn into(self) -> bool {
        match self {
            ParityValueSelect::OneBased => false,
            ParityValueSelect::ZeroBased => true,
        }
    }
}

/// # Parity Odd / Even
/// Which type of parity to use.
pub enum Parity {
    Odd,
    Even,
}

impl Into<bool> for Parity {
    fn into(self) -> bool {
        match self {
            Parity::Odd => false,
            Parity::Even => true,
        }
    }
}

impl<Port: private::UARTPortCompatable> UART<Port> {
    fn init(
        baud_rate: BaudRates,
        character_length: CharacterLength,
        stop_bits: StopBits,
        transmit_parity: bool,
        parity: Parity,
        parity_value: ParityValueSelect,
        hfc: bool,
    ) -> Result<Self> {
        let mut uart = Self {
            reg: registers::Registers::new(Port::PORT_PTR),
            _gpio: crate::gpio::hardware::uart_n(Port::PORT_NUM).ok_or(ErrorKind::Busy)?,
            _ph: PhantomData,
        };

        // Clear the FIFOs
        uart.clear_rx_fifo();
        uart.clear_tx_fifo();

        unsafe {
            // Disable the baud clock
            uart.reg.set_baud_clock_enable(false);
            // Set the number of character bits to 8
            uart.reg.set_character_length(character_length as u8);
            // Set the number of stop bits to 1
            uart.reg.set_number_of_stop_bits(stop_bits.into());
            uart.reg
                .set_transmit_parity_generation_enable(transmit_parity);
            // Set the parity value
            uart.reg.set_parity_value(parity_value.into());
            // Set the parity
            uart.reg.set_parity_odd_even(parity.into());
            // Set the clock source to IBRO
            uart.reg.set_baud_clock_source(ClockSources::IBRO as u8);
            // Set the clock divisor to 7.3728 Mhz / baud rate
            let divisor = 7372800u32 / baud_rate as u32;
            uart.reg.set_baud_rate_divisor(divisor);
            // Set the Hardware Flow Control
            uart.reg.set_hardware_flow_control(hfc);
            // Disable UART auto gating
            uart.reg.set_clock_auto_gating(false);
            // Set RX threshold to 1 byte
            uart.reg.set_recieve_fifo_threshold(1);
            // Set the OSR to 28
            uart.reg.set_lpuart_oversampling_rate(5);
            // Enable the baud clock
            uart.reg.set_baud_clock_enable(true);
            // Wait until the baud clock is ready
            while !uart.reg.get_baud_clock_ready() {}
        }

        Ok(uart)
    }

    /// # Print String
    /// Prints the string passed
    /// Note: Calls ```write_blocking_transmit_fifo(char)```
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
    pub fn read_blocking_receive_fifo(&mut self) -> u8 {
        while self.reg.get_receive_busy() {}
        self.reg.get_fifo_data()
    }

    /// # Write Transmit FIFO
    /// Writes to the FIFO if possible
    pub fn write_transmit_fifo(&mut self, data: u8) -> Result<()> {
        if self.reg.get_transmit_fifo_full() {
            Err(ErrorKind::Busy)
        } else {
            unsafe { self.reg.set_fifo_data(data) }
            Ok(())
        }
    }

    /// # Read Receive FIFO
    /// Reads from the receive FIFO if possible
    pub fn read_receive_fifo(&mut self) -> Result<u8> {
        if self.reg.get_receive_fifo_empty() {
            Err(ErrorKind::NoneAvailable)
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
