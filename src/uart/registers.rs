use crate::memory_map::mmio;
use hal_macros::RW;
use hal_macros_derive::make_device;

/// # UART Register Offsets
/// See Max 78000 User Guide Page 180, Table 12-7.
mod uro {
    /// # UART Control Register
    pub const UART_CTRL: usize = 0x0000;
    /// # UART Status Register
    pub const UART_STATUS: usize = 0x0004;
    /// # UART Interrupt Enable Register
    pub const UART_INT_EN: usize = 0x0008;
    /// # UART Interrupt Flag Register
    pub const UART_INTERRUPT_FL: usize = 0x000c;
    /// # UART Clock Divisor Register
    pub const UART_CLKDIV: usize = 0x0010;
    /// # UART Oversampling Control Register
    pub const UART_OSR: usize = 0x0014;
    /// # UART Transmit FIFO
    pub const UART_TXPEEK: usize = 0x0018;
    /// # UART Pin Control Register
    pub const UART_PNR: usize = 0x001c;
    /// # UART FIFO Data Register
    pub const UART_FIFO: usize = 0x0020;
    /// # UART DMA Control Register
    pub const UART_DMA: usize = 0x0030;
    /// # UART Wakeup Interrupt Enable Register
    pub const UART_WKEN: usize = 0x0034;
    /// # UART Wakeup Interrupt Flag Register
    pub const UART_WKFL: usize = 0x0038;
}

make_device! {
    device_ports(mmio::UART_0, mmio::UART_1, mmio::UART_2);
    /// Receive Dual Edge Sampling. See Page 180, Table 12-8.
    /// This feature can **only** be used with `LPUART`
    /// Can choose to sample only on the rising edge, or both the rising and falling edges.
    /// - 0: Only rising edge
    /// - 1: Both edges
    #[bit(22, RW, uro::UART_CTRL)]
    rx_dual_edge_sampling,


    /// Fractional Division Mode. See Page 180, Table 12-8.
    /// This feature can **only** be used with `LPUART`
    /// Can choose to enable fractional baud rate divisor
    /// - 0: Integer baud rate
    /// - 1: 0.5 division resolution
    #[bit(21, RW, uro::UART_CTRL)]
    fractional_divison_mode,


    /// Clock Auto Gating Mode. See Page 180, Table 12-8.
    /// Choose to use no auto gating, or to pause UART clock during idle states
    /// *NOTE:* Software should set this to 1
    /// - 0: No gating
    /// - 1: Clock paused during idle states
    #[bit(20, RW, uro::UART_CTRL)]
    clock_auto_gating,


    /// Baud Clock Ready. See Page 180, Table 12-8.
    /// Check if baud clock is ready
    /// - 0: Baud clock is not ready
    /// - 1: Baud clock is ready
    #[bit(19, RO, uro::UART_CTRL)]
    baud_clock_ready,


    /// Bit Frame Error Detection Enable. See Page 180, Table 12-8.
    /// Enable or disable frame error detection
    /// This feature can **only** be used with `LPUART`
    /// - 0: Error detection disabled
    /// - 1: Error detection enabled
    #[bit(18, RW, uro::UART_CTRL)]
    bit_frame_error_detection,


    /// Baud Clock Source. See Page 180, Table 12-8.
    /// Select the source for the baud generator (See Table 12-1)
    /// - 0: Clock 0
    /// - 1: Clock 1
    /// - 2: Clock 2
    /// - 3: Clock 3
    #[bit(16..=17, RW, uro::UART_CTRL)]
    baud_clock_source,


    /// Baud Clock Enable. See Page 180, Table 12-8.
    /// Choose if the baud clock is enabled or not
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(15, RW, uro::UART_CTRL)]
    baud_clock_enable,


    /// Hardware Flow Control RTS `Deassert` Condition.
    /// Describes the conditions when RTS is deasserted
    /// - 0: When FIFO level = C_RX_FIFO_DEPTH, RTS is deasserted
    /// - 1: When FIFO level `>=` UART_CTRL.rx_thd_val, RTS is deasserted
    #[bit(14, WO, uro::UART_CTRL)]
    hardware_flow_rts_deassert_condition,


    /// Hardware Flow Control
    /// Choose if hardware flow control is enabled, or disabled
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(13, RW, uro::UART_CTRL)]
    hardware_flow_control,


    /// Number of stop bits
    /// The number of stop bits
    /// - 0: 1 stop bit
    /// - 1: 1.5 stop bit for 5 bit mode, 2 bit mode otherwise
    #[bit(12, RW, uro::UART_CTRL)]
    number_of_stop_bits,


    /// Character Length
    /// The number of bits in a character in an UART frame.
    /// - 0: 5 bits
    /// - 1: 6 bits
    /// - 2: 7 bits
    /// - 3: 8 bits
    #[bit(10..=11, RW, uro::UART_CTRL)]
    character_length,


    /// Activate Receive FIFO Flush
    /// Write a 1 to flush the receive FIFO
    #[bit(9, RW1O, uro::UART_CTRL)]
    receive_fifo_flush,


    /// Activate Transmit FIFO Flush
    /// Write a 1 to flush the transmit FIFO
    #[bit(8, RW1O, uro::UART_CTRL)]
    transmit_fifo_flush,


    /// `CTS` Sampling Disable
    /// Choose to enable or disable `CTS` (Clear To Send)
    /// - 0: Enabled
    /// - 1: Disabled
    #[bit(7, RW, uro::UART_CTRL)]
    cts_sampling_disable,


    /// Parity Value
    /// The parity calculation uses 1s or 0s in data frame
    /// - 0: Use 1s
    /// - 1: Use 0s
    #[bit(6, RW, uro::UART_CTRL)]
    parity_value,


    /// Parity Odd Even Select
    /// parity to ensure even or odd
    /// - 0: Even parity (default)
    /// - 1: Odd parity
    #[bit(5, RW, uro::UART_CTRL)]
    parity_odd_even,


    /// Transmit Parity Generation Enable
    /// Use parity for outward transmissions
    /// - 0: Disable parity
    /// - 1: Use parity (placed after data frame)
    #[bit(4, RW, uro::UART_CTRL)]
    transmit_parity_generation_enable,


    /// Receive FIFO Threshold
    /// The byte size of FIFO before CPU interrupt is sent
    /// ```text
    /// Note: Setting threshold too low at high speeds can slow CPU
    /// and cause loss of data
    /// ```
    /// - 0: Reserved
    /// - 1: 1 byte
    /// - 2: 2 bytes
    /// - 3: 3 bytes
    /// - 4: 4 bytes
    /// - 5: 5 bytes
    /// - 6: 6 bytes
    /// - 7: 7 bytes
    /// - 8: 8 bytes
    /// - 9-15: Reserved
    #[bit(0..=3, RW, uro::UART_CTRL)]
    recieve_fifo_threshold,

    /// The UART Status Register. See page 182, table 12-9
    /// Transmit FIFO Level
    /// Checks of bytes in outbound FIFO buffer
    /// - 0-8: Current of bytes in buffer
    /// - 9-15: Reserved
    #[bit(12..=15, RO, uro::UART_STATUS)]
    transmit_fifo_level,


    /// Receive FIFO Level
    /// Checks of bytes in inbound FIFO buffer
    /// - 0-8: Current of bytes in buffer
    /// - 9-15: Reserved
    #[bit(8..=11, RO, uro::UART_STATUS)]
    receive_fifo_level,


    /// Transmit FIFO Full
    /// Checks if the outbound data buffer has filled up
    /// - 0: Not full
    /// - 1: Full
    #[bit(7, RO, uro::UART_STATUS)]
    transmit_fifo_full,


    /// Transmit FIFO Empty
    /// Checks if the outbound data buffer is empty
    /// - 0: Not empty
    /// - 1: Empty
    #[bit(6, RO, uro::UART_STATUS)]
    transmit_fifo_empty,


    /// Receive FIFO Full
    /// Checks if the inbound data buffer has filled up
    /// - 0: Not full
    /// - 1: Full
    #[bit(5, RO, uro::UART_STATUS)]
    receive_fifo_full,


    /// Receive FIFO Empty
    /// Checks if the inbound data buffer is empty
    /// - 0: Not empty
    /// - 1: Empty
    #[bit(4, RO, uro::UART_STATUS)]
    receive_fifo_empty,


    /// Receive Busy
    /// Checks if the inbound data line is busy
    /// - 0: Not busy
    /// - 1: Busy
    #[bit(1, RO, uro::UART_STATUS)]
    receive_busy,


    /// Transmit Busy
    /// Checks if the outbound data line is busy
    /// - 0: Not busy
    /// - 1: Busy
    #[bit(0, RO, uro::UART_STATUS)]
    transmit_busy,

    /// # UART Interrupt Enable Register
    /// The UART Interrupt Enable Register. See page 182, table 12-10
    /// Transmit FIFO Half-Empty Event Interrupt Enable
    /// Sets whether the interrupt for half-full outbound FIFO buffer is enabled
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(6, RW, uro::UART_INT_EN)]
    transmit_fifo_half_empty_event,


    /// Receive FIFO Half-Empty Event Interrupt Enable
    /// Sets whether the interrupt for half-full inbound FIFO buffer is enabled
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(4, RW, uro::UART_INT_EN)]
    receive_fifo_half_empty_even,


    /// Receive FIFO Threshold Event Interrupt Enable
    /// Sets whether the interrupt for filled inbound FIFO buffer is enabled
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(3, RW, uro::UART_INT_EN)]
    receive_fifo_thershold_event,


    /// `CTS` Signal Change Event Interrupt Enable
    /// Sets if the interrupt for a change in CTS Signal is enabled
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(2, RW, uro::UART_INT_EN)]
    cts_signal_change_event,


    /// Receive Parity Event Interrupt Enable
    /// Set if parity errors on received data is enabled
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(1, RW, uro::UART_INT_EN)]
    receive_parity_event,


    /// Receive Frame Error Event Interrupt Enable
    /// Set if stop bit not being recognized generates an interrupt
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(0, RW, uro::UART_INT_EN)]
    receive_frame_error_event,

    /// UART Interrupt Flag Register
    /// The UART Interrupt Flag Register. See Page 183, Table 12-11.
    /// # Get Transmit FIFO Half-Empty Interrupt Flag
    /// Get the status of the transmit FIFO half-empty flag
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(6, RW1C, uro::UART_INTERRUPT_FL)]
    transmit_fifo_half_empty_interrupt_flag,


    /// # Get Receive FIFO Threshold Interrupt Flag
    /// Get the status flag for the FIFO-filled flag
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(4, RW1C, uro::UART_INTERRUPT_FL)]
    receive_fifo_threshold_interrupt_flag,


    /// # Get Receive FIFO Overrun Interrupt Flag
    /// Get the status flag for the inbound FIFO buffer overrun flag
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(3, RW1C, uro::UART_INTERRUPT_FL)]
    receive_fifo_overrun_interrupt_flag,


    /// # Get CTS Signal Change Interrupt Flag
    /// The status flag for changes in CTS Signal
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(2, RW1C, uro::UART_INTERRUPT_FL)]
    cts_signal_change_interrupt_flag,


    /// # Get Receive Parity Error Interrupt Flag
    /// The status flag for errors in the received parity bit
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(1, RW1C, uro::UART_INTERRUPT_FL)]
    receive_parity_error_interrupt_flag,


    /// Receive Frame Error Interrupt Flag
    /// The status flag for errors in the received parity bit
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(0, RW1C, uro::UART_INTERRUPT_FL)]
    receive_frame_error_interrupt_flag,

    /// The UART Clock Divisor Register. See Page 183-184, Table 12-12
    /// # Get Baud Rate Divisor
    /// The divisor for generating the baud tick from baud clock
    #[bit(0..=19, RW, uro::UART_CLKDIV)]
    baud_rate_divisor,

    /// The UART Oversampling Control Register. See Page 184, Table 12-13
    /// LPUART Over Sampling Rate
    /// How many times faster LPUART is sampling than the clock speed
    /// FDM Enabled:
    /// - 0: 8x
    /// - 1: 12x
    /// - 2: 16x
    /// - 3: 20x
    /// - 4: 24x
    /// - 5: 28x
    /// - 6: 32x
    /// - 7: 36x
    /// FDM Disabled:
    /// - 0: 128x
    /// - 1: 64x
    /// - 2: 32x
    /// - 3: 16x
    /// - 4: 8x
    /// - 5: 4x
    /// - 6-7: Reserved
    #[bit(0..=2, RW, uro::UART_OSR)]
    lpuart_oversampling_rate,

    /// The UART Transmit FIFO Register. See Page 184, Table 12-14.
    /// Transmit FIFO Data
    /// Reads the data in the outbound FIFO, no data reads as 0
    #[bit(0..=7, RO, uro::UART_TXPEEK)]
    transmit_fifo_data,

    /// # UART Pin Control Register
    /// The UART Pin Control Register. See Page 184-185, Table 12-15.
    /// RTS Output State
    /// The outbound RTS's state
    /// - 0: Push to 0
    /// - 1: Push to 1
    #[bit(1, RW, uro::UART_PNR)]
    rts_output_state,

    /// CTS Pin State
    /// The CTS pin's state
    /// - 0: 0
    /// - 1: 1
    #[bit(0, RO, uro::UART_PNR)]
    cts_pin_state,

    /// The UART Data Register. See Page 185, Table 12-16.
    /// # Get Receive FIFO Byte Parity
    /// Shows if parity error occurred while receiving last byte
    /// - 0: No error
    /// - 1: Error occurred
    #[bit(8, RO, uro::UART_FIFO)]
    receive_fifo_byte_parity,

    /// Get/Set Transmit/Receive FIFO Data
    /// Sets outbound FIFO and Gets inbound FIFO data
    #[bit(0..=7, RW, uro::UART_FIFO)]
    fifo_data,

    /// The UART DMA Register. See Page 185, Table 12-17.
    /// Receive DMA Channel Enable
    /// Enabling Direct Memory Access for inbound UART to allow using multiple buffers
    /// The documentation has a typo for this bit's access.
    /// It says "0" while it should say "R/W".
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(9, RW, uro::UART_DMA)]
    receive_dma_channel_enable,

    /// # Set Receive FIFO Level DMA Threshold
    /// How many bytes in inbound FIFO there must be to tell the DMA there is data to transfer
    /// The documentation has a typo for this bit's access.
    /// It says "0" while it should say "R/W".
    /// - Nothing listed in documentation
    #[bit(5..=8, RW, uro::UART_DMA)]
    receive_fifo_level_dma_threshold,

    /// # Set Transmit DMA Channel Enable
    /// Enabling Direct Memory Access for outbound UART to allow using multiple buffers
    #[bit(4, RW, uro::UART_DMA)]
    transmit_dma_channel_enable,

    /// # Set Transmit FIFO Level DMA Threshold
    /// How many bytes in outbound FIFO there must be to tell the DMA there is spare room
    /// - Nothing listed in documentation
    #[bit(0..=3, RW, uro::UART_DMA)]
    transmit_dma_level_dma_threshold,

    /// The UART Wakeup Enable Register. See Page 185-186, Table 12-18.
    /// # Set Receive FIFO Threshold Wake-up Event Enable
    /// Allow a threshold of bytes in the inbound FIFO to wake up the CPU and resume normal operation
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(2, RW, uro::UART_WKEN)]
    receive_fifo_threshold_wakeup_event_enable,

    /// # Receive FIFO Full Wake-up Event Enable
    /// Allow a full inbound FIFO to wake up the CPU and resume normal operation
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(1, RW, uro::UART_WKEN)]
    receive_fifo_full_wakeup_event_enable,

    /// # Receive FIFO Not Empty Wake-up Event Enable
    /// Allow a non-empty inbound FIFO to wake up the CPU and resume normal operation
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(0, RW, uro::UART_WKEN)]
    receive_fifo_not_empty_wakeup_event_enable,

    /// The UART Wakeup Flag register. See Page 186, Table 12-19.
    /// # Set Receive FIFO Threshold Wake-up Event
    /// Flag to tell the CPU to wake up when the inbound FIFO has more bytes than the threshold
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(2, RW, uro::UART_WKFL)]
    receive_fifo_threshold_wakeup_event,

    /// # Receive FIFO Full Wake-up Event
    /// Flag to tell the CPU to wake up when the inbound FIFO is full
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(1, RW, uro::UART_WKFL)]
    receive_fifo_full_wakeup_event,

    /// # Receive FIFO Not Empty Wake-up Event
    /// Flag to tell the CPU to wake up when the inbound FIFO is not empty
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(0, RW, uro::UART_WKFL)]
    receive_fifo_not_empty_wakeup_event,
}
