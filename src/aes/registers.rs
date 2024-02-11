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
