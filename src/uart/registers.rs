use crate::bits::BitManipulation;
use crate::const_assert;
use crate::memory_map::mmio;
use crate::{bit_impl, reg_impl};
use core::ptr;

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

/// # UART Control Register
/// The UART control register. See Page 180, Table 12-8.
pub struct ControlRegister<const PORT_PTR: usize> {}
reg_impl!(RW, ControlRegister, uro::UART_CTRL);

impl<const PORT_PTR: usize> ControlRegister<PORT_PTR> {
    bit_impl! {22, RW,
    /// # Receive Dual Edge Sampling
    /// This feature can **only** be used with `LPUART`
    /// Can choose to sample only on the rising edge, or both the rising and falling edges.
    /// - 0: Only rising edge
    /// - 1: Both edges
    set_rx_dual_edge_sampling,
    /// # Is Receive Dual Edge Sampling Enabled
    /// Check if receive dual edge sampling is currently enabled.
    /// - 0: Only rising edge enabled
    /// - 1: Both edges enabled
    is_rx_dual_edge_sampling_enabled}

    bit_impl! {21, RW,
    /// # Fractional Division Mode
    /// This feature can **only** be used with `LPUART`
    /// Can choose to enable fractional baud rate divisor
    /// - 0: Integer baud rate
    /// - 1: 0.5 division resolution
    set_fractional_divison_mode,
    /// # Is Fractional Division Mode Enabled
    /// Check if fractional division mode is currently enabled
    /// - 0: Integer baud rate enabled
    /// - 1: 0.5 division resolution enabled
    is_fractional_division_mode_enabled}

    bit_impl! {20, RW,
    /// # Clock Auto Gating Mode
    /// Choose to use no auto gating, or to pause UART clock during idle states
    /// *NOTE:* Software should set this to 1
    /// - 0: No gating
    /// - 1: Clock paused during idle states
    set_clock_auto_gating,
    /// # Is Clock Auto Gating Enabled
    /// Check if auto gating is currently enabled
    /// - 0: Gating is not enabled
    /// - 1: Clock is paused during idle states
    is_clock_auto_gating_enabled}

    bit_impl! {19, RO,
    /// # Is Baud Clock Ready
    /// Check if the baud clock is ready
    /// - 0: Baud clock is not ready
    /// - 1: Baud clock is ready
    is_baud_clock_ready}

    bit_impl! {18, RW,
    /// # Bit Frame Error Detection Enable
    /// Choose to enable or disable frame error detection
    /// This feature can **only** be used with `LPUART`
    /// - 0: Error detection disabled
    /// - 1: Error detection enabled
    set_bit_frame_error_detection,
    /// # Is Bit Frame Error Detection Enabled
    /// Check if bit frame error detection is current enabled
    /// - 0: Error detection is disabled
    /// - 1: Error detection is enabled
    is_bit_frame_error_detection_enabled}

    bit_impl! {16..=17, RW u16,
    /// # Baud Clock Source
    /// Select the source for the baud generator (See Table 12-1)
    /// - 0: Clock 0
    /// - 1: Clock 1
    /// - 2: Clock 2
    /// - 3: Clock 3
    set_baud_clock_source,
    /// # Check Baud Clock Source
    /// Check the current source of the baud generator
    /// - 0: Clock 0
    /// - 1: Clock 1
    /// - 2: Clock 2
    /// - 3: Clock 3
    check_baud_clock_source}

    bit_impl! {15, RW,
    /// # Baud Clock Enable
    /// Choose if the baud clock is enabled or not
    /// - 0: Disabled
    /// - 1: Enabled
    set_baud_clock_enable,
    /// # Is Baud Clock Enabled
    /// Check if the baud clock is enabled
    /// - 0: Disabled
    /// - 1: Enabled
    is_baud_clock_enable}

    bit_impl! {14, WO, // <- FIXME What should this be? Datasheet says RO, but that does not seem right
    /// # Hardware Flow Control RTS `Deassert` Condition.
    /// Describes the conditions when RTS is deasserted
    /// - 0: When FIFO level = C_RX_FIFO_DEPTH, RTS is deasserted
    /// - 1: When FIFO level `>=` UART_CTRL.rx_thd_val, RTS is deasserted
    set_hardware_flow_rts_deassert_condition}

    bit_impl! {13, RW,
    /// # Hardware Flow Control
    /// Choose if hardware flow control is enabled, or disabled
    /// - 0: Disabled
    /// - 1: Enabled
    set_hardware_flow_control,
    /// # Check Hardware Flow Control
    /// Checks if hardware flow control is enabled
    /// - 0: Disabled
    /// - 1: Enabled
    check_hardware_flow_control}

    bit_impl! {12, RW,
    /// # Number of stop bits
    /// Set the number of stop bits
    /// - 0: 1 stop bit
    /// - 1: 1.5 stop bit for 5 bit mode, 2 bit mode otherwise
    set_number_of_stop_bits,
    /// # Check Number Of Stop Bits
    /// Check the current number of stop bits
    /// - 0: 1 stop bit
    /// - 1: 1.5 stop bit for 5 bit mode, 2 bit mode otherwise
    check_number_of_stop_bits}

    bit_impl! {10..=11, RW u16,
    /// # Set Character Length
    /// Set the number of bits in a character in an UART frame.
    /// - 0: 5 bits
    /// - 1: 6 bits
    /// - 2: 7 bits
    /// - 3: 8 bits
    set_character_length,
    /// # Check Character Length
    /// Checks the current number of bits in a character in a UART frame
    /// - 0: 5 bits
    /// - 1: 6 bits
    /// - 2: 7 bits
    /// - 3: 8 bits
    check_character_length}

    bit_impl! {9, RESET, // FIXME This needs to be renamed / changed
    /// # Activate Receive FIFO Flush
    /// Write a 1 to flush the receive FIFO
    activate_receive_fifo_flush}
}

/// # UART Oversampling Control Register
/// The UART Oversampling Control Register. See Page 184, Table 12-13
pub struct OversamplingControlRegister<const PORT_PTR: usize> {}
reg_impl!(RO, OversamplingControlRegister, uro::UART_OSR);

impl<const PORT_PTR: usize> OversamplingControlRegister<PORT_PTR> {
    bit_impl! {0..=2, RW u8,
    /// Get LPUART Over Sampling Rate
    get_lpuart_oversampling_rate,
    /// Set LPUART Over Sampling Rate
    set_lpuart_oversampling_rate}
}

/// # UART Transmit FIFO Register
/// The UART Transmit FIFO Register. See Page 184, Table 12-14.
pub struct TransmitFIFORegister<const PORT_PTR: usize> {}
reg_impl!(RO, TransmitFIFORegister, uro::UART_OSR);

impl<const PORT_PTR: usize> TransmitFIFORegister<PORT_PTR> {
    bit_impl! {0..=7, RO u8,
    /// Get Transmit FIFO Data
    get_transmit_fifo_data}
}

/// # UART Pin Control Register
/// The UART Pin Control Register. See Page 184-185, Table 12-15.
pub struct PinControlRegister<const PORT_PTR: usize> {}
reg_impl!(RW, PinControlRegister, uro::UART_PNR);

impl<const PORT_PTR: usize> PinControlRegister<PORT_PTR> {
    bit_impl! {1, RW,
    /// # Get RTS Output State
    get_rts_output_state,
    /// # Set RTS Output State
    set_rts_output_state}

    bit_impl! {0, RO,
    /// # Get CTS Pin State
    get_cts_pin_state}
}

/// # UART Data Register
/// The UART Data Register. See Page 185, Table 12-16.
pub struct DataRegister<const PORT_PTR: usize> {}
reg_impl!(RW, DataRegister, uro::UART_FIFO);

impl<const PORT_PTR: usize> DataRegister<PORT_PTR> {
    bit_impl! {8, RO,
    /// # Get Receive FIFO Byte Parity
    get_receive_fifo_byte_parity}

    bit_impl! {0..=7, RW u8,
    /// # Get Transmit/Receive FIFO Data
    get_transmit_receive_fifo_data,
    /// # Set Transmit/Receive FIFO Data
    set_transmit_receive_fifo_data}
}

/// # UART DMA Register
/// The UART DMA Register. See Page 185, Table 12-17.
pub struct DMARegister<const PORT_PTR: usize> {}
reg_impl!(RW, DMARegister, uro::UART_DMA);

impl<const PORT_PTR: usize> DMARegister<PORT_PTR> {
    bit_impl! {9, RW,
    /// # Set Receive DMA Channel Enable
    /// The documentation has a typo for this bit's access.
    /// It says "0" while it should say "R/W".
    set_receive_dma_channel_enable,
    /// # Get Receive DMA Channel Enable
    is_receive_dma_channel_enable}

    bit_impl! {5..=8, RW u8,
    /// # Set Receive FIFO Level DMA Threshold
    /// The documentation has a typo for this bit's access.
    /// It says "0" while it should say "R/W".
    set_receive_fifo_level_dma_threshold,
    /// # Get Receive FIFO Level DMA Threshold
    get_receive_fifo_level_dma_threshold}

    bit_impl! {4, RW,
    /// # Set Transmit DMA Channel Enable
    set_transmit_dma_channel_enable,
    /// # Get Transmit DMA Channel Enable
    get_transmit_dma_channel_enable}

    bit_impl! {0..=3, RW u8,
    /// # Set Transmit FIFO Level DMA Threshold
    set_transmit_dma_level_dma_threshold,
    /// # Get Transmit FIFO Level DMA Threshold
    get_transmit_dma_level_dma_threshold}
}

/// # UART Wakeup Enable
/// The UART Wakeup Enable Register. See Page 185-186, Table 12-18.
pub struct WakeupEnableRegister<const PORT_PTR: usize> {}
reg_impl!(RW, WakeupEnableRegister, uro::UART_WKEN);

impl<const PORT_PTR: usize> WakeupEnableRegister<PORT_PTR> {
    bit_impl! {2, RW,
    /// # Set Receive FIFO Threshold Wake-up Event Enable
    set_receive_fifo_threshold_wakeup_event_enable,
    /// # Get Receive FIFO Threshold Wake-up Event Enable
    is_receive_fifo_threshold_wakeup_event_enable}

    bit_impl! {1, RW,
    /// # Receive FIFO Full Wake-up Event Enable
    set_receive_fifo_full_wakeup_event_enable,
    /// # Get Receive FIFO Full Wake-up Event Enable
    get_receive_fifo_full_wakeup_event_enable}

    bit_impl! {0, RW,
    /// # Receive FIFO Not Empty Wake-up Event Enable
    set_receive_fifo_not_empty_wakeup_event_enable,
    /// # Get Receive FIFO Not Empty Wake-up Event Enable
    get_receive_fifo_not_empty_wakeup_event_enable}
}

/// # UART Wakeup Flag Register
/// The UART Wakeup Flag register. See Page 186, Table 12-19.
pub struct WakeupFlagRegister<const PORT_PTR: usize> {}
reg_impl!(RW, WakeupFlagRegister, uro::UART_WKFL);

impl<const PORT_PTR: usize> WakeupFlagRegister<PORT_PTR> {
    bit_impl! {2, RW,
    /// # Set Receive FIFO Threshold Wake-up Event
    set_receive_fifo_threshold_wakeup_event,
    /// # Get Receive FIFO Threshold Wake-up Event
    is_receive_fifo_threshold_wakeup_event}

    bit_impl! {1, RW,
    /// # Receive FIFO Full Wake-up Event
    set_receive_fifo_full_wakeup_event,
    /// # Get Receive FIFO Full Wake-up Event
    get_receive_fifo_full_wakeup_event}

    bit_impl! {0, RW,
    /// # Receive FIFO Not Empty Wake-up Event
    set_receive_fifo_not_empty_wakeup_event,
    /// # Get Receive FIFO Not Empty Wake-up Event
    get_receive_fifo_not_empty_wakeup_event}
}
