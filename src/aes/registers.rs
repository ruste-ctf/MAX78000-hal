use crate::bits::BitManipulation;
use crate::const_assert;
use crate::memory_map::mmio;
use crate::{bit_impl, reg_impl};
use core::ptr;

/// # AES Register Offsets
/// See Max 78000 User Guide Page 360, Table 24-3.
mod rro {
    /// # AES Control Register
    pub const AES_CTRL: usize = 0x0000;
    /// # AES Status Register
    pub const AES_STATUS: usize = 0x0004;
    /// # AES Interrupt Flag Register
    pub const AES_INTFL: usize = 0x0008;
    /// # AES Interrupt Enable Flag Register
    pub const AES_INTEN: usize = 0x000C;
    /// # AES Data FIFO Register
    pub const AES_FIFO: usize = 0x0010;
}

/// # AES Control Register
/// The AES Control Register. See Page 360-361, Table 24-4.
pub struct ControlRegister<const PORT_PTR: usize> {}
reg_impl!(RW, ControlRegister, rro::AES_CTRL);

impl<const PORT_PTR: usize> ControlRegister<PORT_PTR> {
    bit_impl! {8..=9, RW u8,
    /// # Set Encryption Type
    set_encryption_type,
    /// # Get Encryption Type
    get_encryption_type}

    bit_impl! {6..=7, RW u8,
    /// # Set Encryption Key Size
    set_encryption_key_size,
    /// # Get Encryption Key Size
    get_encryption_key_size}

    bit_impl! {5, RESET, // FIXME This needs to be renamed / changed
    /// # Activate Flush Data Output FIFO
    activate_flush_data_output_fifo}

    bit_impl! {4, RESET, // FIXME This needs to be renamed / changed
    /// # Activate Flush Data Input FIFO
    activate_flush_data_input_fifo}

    bit_impl! {3, RESET, // FIXME This needs to be renamed / changed
    /// # Activate Start AES Calculation
    activate_start_aes_calculation}

    bit_impl! {2, RW,
    /// # Set DMA Request To Write Data Input FIFO
    set_dma_request_to_write_data_input_fifo,
    /// # Get DMA Request To Write Data Input FIFO
    get_dma_request_to_write_data_input_fifo}

    bit_impl! {1, RW,
    /// # Set DMA Request To Read Data Output FIFO
    set_dma_request_to_read_data_output_fifo,
    /// # Get DMA Request To Read Data Output FIFO
    get_dma_request_to_read_data_output_fifo}

    bit_impl! {0, RW,
    /// # Set AES Enable
    set_aes_enable,
    /// # Get AES Enable
    get_aes_enable}
}

/// # AES Status Register
/// The AES Status register. See Page 361, Table 24-5.
pub struct StatusRegister<const PORT_PTR: usize> {}
reg_impl!(RO, StatusRegister, rro::AES_STATUS);

impl<const PORT_PTR: usize> StatusRegister<PORT_PTR> {
    bit_impl! {4, RO,
    /// # Get Output FIFO Full
    get_output_fifo_full}

    bit_impl! {3, RO,
    /// # Get Output FIFO Empty
    get_output_fifo_empty}

    bit_impl! {2, RO,
    /// # Get Input FIFO Full
    get_input_fifo_full}

    bit_impl! {1, RO,
    /// # Get Input FIFO Empty
    get_input_fifo_empty}

    bit_impl! {0, RO,
    /// # Get AES Busy
    get_aes_busy}
}

/// # AES Interrupt Flag Register
/// The AES Interrupt Flag register. See Page 361-362, Table 24-6.
pub struct InterruptFlagRegister<const PORT_PTR: usize> {}
reg_impl!(
    RW1C, // FIXME
    InterruptFlagRegister,
    rro::AES_INTFL,
    0b_00000000000000000000000000000000
);

impl<const PORT_PTR: usize> InterruptFlagRegister<PORT_PTR> {
    bit_impl! {3, RESET, // FIXME
    /// # Activate Data Output FIFO Overrun Event Interrupt
    activate_data_output_fifo_overrun_event_interrupt}

    bit_impl! {2, RESET, // FIXME
    /// # Activate Key Zero Event Interrupt
    activate_key_zero_event_interrupt}

    bit_impl! {1, RESET, // FIXME
    /// # Activate Key Change Event Interrupt
    activate_key_change_event_interrupt}

    bit_impl! {0, RESET, // FIXME
    /// # Activate Calculation Done Event Interrupt
    activate_calculation_done_event_interrupt}
}

/// # AES Interrupt Enable Register
/// The AES Interrupt Enable Register. See Page 362, Table 24-7.
pub struct InterruptEnableRegister<const PORT_PTR: usize> {}
reg_impl!(
    RW1C, // FIXME
    InterruptEnableRegister,
    rro::AES_INTEN,
    0b_00000000000000000000000000000000
);

impl<const PORT_PTR: usize> InterruptEnableRegister<PORT_PTR> {
    bit_impl! {3, RESET,
    /// # Activate Data Output FIFO Overrun Event Interrupt Enable
    activate_date_output_fifo_overrun_event_interrupt_enable}

    bit_impl! {2, RESET,
    /// # Activate Key Zero Event Interrupt Enable
    activate_key_zero_event_interrupt_enable}

    bit_impl! {1, RESET,
    /// # Activate Key Change Event Interrupt Enable
    activate_key_change_event_interrupt_enable}

    bit_impl! {0, RESET,
    /// # Activate Calculation Done Event Interrupt Enable
    activate_calculation_done_event_interrupt_enable}
}

/// # AES FIFO Register
/// The AES FIFO Register. See Page 362, Table 24-8.
pub struct FIFORegister<const PORT_PTR: usize> {}
reg_impl!(RW, FIFORegister, rro::AES_CTRL);

impl<const PORT_PTR: usize> FIFORegister<PORT_PTR> {
    bit_impl! {0..=31, RW u32,
    /// # Set AES FIFO
    set_aes_fifo,
    /// # Get AES FIFO
    get_aes_fifo}
}
