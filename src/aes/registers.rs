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
/// The AES control register. See Page 360-361, Table 24-4.
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
