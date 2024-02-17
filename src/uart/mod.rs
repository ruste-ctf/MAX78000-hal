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
    PCLK = 0,
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
        uart.set_character_length(CharacterLength::EightBits);

        // Set the number of stop bits to 1
        uart.set_number_stop_bits(StopBits::OneBit);

        // Dissable parity
        uart.transmit_parity_enable(false);
        // Set the Divisor to 64
        uart.set_clock_divisor(64);
        // Set the clock source
        uart.set_baud_clock_source(ClockSources::IBRO);
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

    /// # Enable CTS Sampling
    /// Enables or disables CTS sampling
    pub fn enable_cts_sampling(&mut self, disable: bool) {
        unsafe { self.reg.set_cts_sampling_disable(disable) }
    }

    /// # Get CTS Sampling Value
    /// Gets the value of the CTS pin
    pub fn get_cts_pin_value(&mut self) -> bool {
        self.reg.get_cts_sampling_disable()
    }

    /// # Parity Value Select
    /// Selects the parity value
    pub fn parity_value_select(&mut self, value: ParityValueSelect) {
        let value = match value {
            ParityValueSelect::OneBased => true,
            ParityValueSelect::ZeroBased => false,
        };

        unsafe {
            self.reg.set_parity_value(value);
        }
    }

    /// # Set Character Length
    /// Sets the number of data bits to send in a UART frame
    pub fn set_character_length(&mut self, length: CharacterLength) {
        unsafe {
            self.reg.set_character_length(length as u8);
        }
    }

    /// # Set Baud Clock Source
    /// Sets the clock to derive the baud clock from
    pub fn set_baud_clock_source(&mut self, source: ClockSources) {
        unsafe {
            self.reg.set_baud_clock_source(source as u8);
        }
    }

    /// # Set Baud Clock
    /// Enables or disables the baud clock
    pub fn set_baud_clock(&mut self, enable: bool) {
        unsafe { self.reg.set_baud_clock_enable(enable) }
    }

    /// # Hardware Flow Control RTS Deassert Condition
    /// Controls when RTS is deasserted
    pub fn set_hardware_flow_control_rts_deassert_condition(
        &mut self,
        condition: HFCDeassertCondition,
    ) {
        let condition = match condition {
            HFCDeassertCondition::EqualsFIFODepth => false,
            HFCDeassertCondition::ExceedsRxThreshold => true,
        };

        unsafe {
            self.reg.set_hardware_flow_rts_deassert_condition(condition);
        }
    }

    /// # Hardware Flow Control Enable
    /// Enables or disables hardware flow control
    pub fn enable_hardware_flow_control(&mut self, enable: bool) {
        unsafe { self.reg.set_hardware_flow_control(enable) }
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

    /// # Get Receive FIFO Parity
    /// Gets the

    /// # Set Clock Divisor
    /// Sets the divisor to use in UART clock generation
    pub fn set_clock_divisor(&mut self, divisor: u32) {
        unsafe {
            self.reg.set_baud_rate_divisor(divisor);
        }
    }

    /// # Set Baud Rate
    /// Sets the clock source to , and sets the divisor to generate the specified baud rate
    pub fn set_baud_rate(&mut self, rate: BaudRates) {
        // Match the baud rate with 7.3728 MHz / rate
        let divisor = 7372800 / rate as u32;
        // Set the clock divisor
        self.set_clock_divisor(divisor);
    }

    /// # Enable Receive DMA
    /// Enables the receive dma to be triggered
    pub fn enable_receive_dma(&mut self, enable: bool) {
        unsafe {
            self.reg.set_receive_dma_channel_enable(enable);
        }
    }

    /// # Enable Transmit DMA
    /// Enables the transmit dma to be triggered
    pub fn enable_transmit_dma(&mut self, enable: bool) {
        unsafe {
            self.reg.set_transmit_dma_channel_enable(enable);
        }
    }

    /// # Transmit FIFO Level
    /// Gets the current level of the transmit FIFO
    pub fn get_tx_fifo_level(&mut self) -> u8 {
        self.reg.get_transmit_fifo_level()
    }

    /// # Receive FIFO Level
    /// Gets the current level of the receive FIFO
    pub fn get_rx_fifo_level(&mut self) -> u8 {
        self.reg.get_receive_fifo_level()
    }

    /// # Set Receive FIFO Threshold
    /// Sets the receive FIFO threshold
    pub fn set_rx_fifo_threshold(&mut self, threshold: ThresholdSize) {
        unsafe {
            self.reg.set_recieve_fifo_threshold(threshold as u8);
        }
    }

    /// # Enable Transmit FIFO Half-Empty Interrupt
    /// Enables or disables the FIFO Half-Empty Interrupt
    pub fn enable_tx_half_empty_interrupt(&mut self, enable: bool) {
        unsafe { self.reg.set_transmit_fifo_half_empty_event(enable) }
    }

    /// # Enable Receive FIFO Threshold Event Interrupt
    /// Enables or disables the receive FIFO threshold event interrupt
    pub fn enable_rx_fifo_threshold_interrupt(&mut self, enable: bool) {
        unsafe {
            self.reg.set_receive_fifo_thershold_event(enable);
        }
    }

    /// # Enable Receive FIFO Overrun Event Interrupt Enable
    /// Enables or disables the receive FIFO overrun interrupt
    pub fn enable_rx_fifo_overrun_interrupt(&mut self, enable: bool) {
        unsafe {
            self.reg.set_receive_fifo_overrun_event(enable);
        }
    }

    /// # CTS Signal Change Event Interrupt Enable
    /// Enables or disables the CTS signal change interrupt
    pub fn enable_cts_interrupt(&mut self, enable: bool) {
        unsafe {
            self.reg.set_cts_signal_change_event(enable);
        }
    }

    /// # Receive Parity Event Interrupt Enable
    /// Enables or disables the receive parity interrupt
    pub fn enable_rx_parity_interrupt(&mut self, enable: bool) {
        unsafe {
            self.reg.set_receive_parity_event(enable);
        }
    }

    /// # Receive Frame Error Event Interrupt Enable
    /// Enables or disables the frame error interrupt
    pub fn enable_rx_frame_error_interrupt(&mut self, enable: bool) {
        unsafe {
            self.reg.set_receive_frame_error_event(enable);
        }
    }

    /// # Get Transmit FIFO Half-Empty Interrupt Flag
    /// Gets the state of the transmit FIFO half-empty flag
    pub fn get_tx_half_empty_flag(&mut self) -> bool {
        self.reg.get_transmit_fifo_half_empty_event()
    }

    /// # Get Receive FIFO Threshold Interrupt Flag
    /// Gets the state of the receive FIFO threshold flag
    pub fn get_rx_threshold_flag(&mut self) -> bool {
        self.reg.get_receive_fifo_threshold_wakeup_event()
    }

    /// # Get Receive FIFO Overrun Interrupt Flag
    /// Gets the state of the receive FIFO overrun flag
    pub fn get_rx_overrun_flag(&mut self) -> bool {
        self.reg.get_receive_fifo_overrun_event()
    }

    /// # Get CTS Signal Change Interrupt Flag
    /// Gets the state of the CTS signal change interrupt
    pub fn get_cts_signal_change_interrupt(&mut self) -> bool {
        self.reg.get_cts_signal_change_event()
    }

    /// # Get Receive Parity Error Flag
    /// Gets the state of the receive parity flag
    pub fn get_receive_parity_error_flag(&mut self) -> bool {
        self.reg.get_receive_parity_event()
    }

    /// # Get Receive Frame Error Interrupt Flag
    /// Gets the state of the receive frame error flag
    pub fn get_receive_frame_error_flag(&mut self) -> bool {
        self.reg.get_receive_frame_error_event()
    }

    /// # Set RTS State
    /// Sets the state of the RTS pin
    pub fn set_rts_state(&mut self, enable: bool) {
        unsafe {
            self.reg.set_rts_output_state(enable);
        }
    }

    /// # Get RTS State
    /// Gets the state of the RTS pin
    pub fn get_rts_state(&mut self) -> bool {
        self.reg.get_rts_output_state()
    }

    /// # Get CTS State
    /// Gets the state of the CTS pin
    pub fn get_cts_state(&mut self) -> bool {
        self.reg.get_cts_pin_state()
    }
}

impl<Port: private::UARTPortCompatable> core::fmt::Write for UART<Port> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print_string(s);
        Ok(())
    }
}
