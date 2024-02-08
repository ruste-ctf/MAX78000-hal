use crate::bits::BitManipulation;
use crate::const_assert;
use crate::memory_map::mmio;
use crate::{bit_impl, reg_impl};
use core::ptr;

/// # UART Register Offsets
/// See Max 78000 User Guide Page 180, Table 12-7
mod uro {
    /// # UART Control Register
    pub const UART_CTRL: usize = 0x0000;
    /// # UART Status Register
    pub const UART_STATUS: usize = 0x0004;
    /// # UART Interrupt Enable Regiser
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
    /// # UART Wakeup Interrupt Flage Register
    pub const UART_WKFL: usize = 0x0038;
}

/// # UART Control Register
/// The uart control register. See Page 180, Table 12-8.
pub struct ControlRegister<const PORT_PTR: usize> {}
reg_impl!(RW, ControlRegister, uro::UART_CTRL);

impl<const PORT_PTR: usize> ControlRegister<PORT_PTR> {
    bit_impl! {22, RW,
    /// # Receive Dual Edge Sampling
    /// This feature can **only** be used with LPUART
    /// Can choose to sample only on the rising edge, or both the rising and falling edges.
    /// - 0: Only rising edge
    /// - 1: Both edges
    set_rx_dual_edge_sampling,
    /// # Is Receive Dual Edge Sampling Enabled
    /// Check if reeceive dual edge sampling is currently enabled.
    /// - 0: Only rising edge enabled
    /// - 1: Both edges enabled
    is_rx_dual_edge_sampling_enabled}

    bit_impl! {21, RW,
    /// # Fractional Division Mode
    /// This feature can **only** be used with LPUART
    /// Can choose to enable fractional baud rate divisor
    /// - 0: Integer baud rate
    /// - 1: 0.5 division resolution
    set_fractional_divison_mode,
    /// # Is Fractional Division Mode Enabled
    /// Check if fractional divison mode is currently enabled
    /// - 0: Integer baud rate enabled
    /// - 1: 0.5 divison resolution enabled
    is_fractional_division_mode_enabled}

    bit_impl! {20, RW,
    /// # Clock Auto Gating Mode
    /// Choose to use no auto gating, or to pause uart clock during idle states
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
    /// This feature can **only** be used with LPUART
    /// - 0: Error detection disabled
    /// - 1: Error detection enabled
    set_bit_frame_error_detection,
    /// # Is Bit Frame Error Detection Enabled
    /// Check if bit frame error detection is current enabled
    /// - 0: Error detection is disabled
    /// - 1: Error detection is enabled
    is_bit_frame_error_detection_enabled}
}
