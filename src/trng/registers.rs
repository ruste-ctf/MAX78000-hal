use crate::const_assert;
use crate::memory_map::mmio;
use crate::{bit_impl, reg_impl};

/// # TRNG Register Offsets
/// See Max 78000 User Guide Page 363, Table 25-1.
mod rro {
    /// # TRNG Control Register
    pub const TRNG_CTRL: usize = 0x0000;
    /// # TRNG Status Register
    pub const TRNG_STATUS: usize = 0x0004;
    /// # TRNG Data Register
    pub const TRNG_DATA: usize = 0x0008;
}

/// # TRNG Control Register
/// The TRNG Control Register. See Page 363, Table 25-2.
pub struct ControlRegister<const PORT_PTR: usize> {}
reg_impl!(RW, ControlRegister, rro::TRNG_CTRL);

impl<const PORT_PTR: usize> ControlRegister<PORT_PTR> {
    bit_impl! {15, RW,
    /// # Set Wipe Key
    set_wipe_key,
    /// # Get Wipe Key
    get_wipe_key}

    bit_impl! {3, RW,
    /// # Set Generate Key
    set_generate_key,
    /// # Get Generate Key
    get_generate_key}

    bit_impl! {1, RW,
    /// # Set Random Number Interrupt Enable
    set_random_number_interrupt_enable,
    /// # Get Random Number Interrupt Enable
    get_random_number_interrupt_enable}
}

/// # TRNG Status Register
/// The TRNG Status Register. See Page 363-364, Table 25-3.
pub struct StatusRegister<const PORT_PTR: usize> {}
reg_impl!(RO, StatusRegister, rro::TRNG_STATUS);

impl<const PORT_PTR: usize> StatusRegister<PORT_PTR> {
    bit_impl! {0, RO,
    /// # Get Random Number Ready
    get_random_number_ready}
}

/// # TRNG Data Register
/// The TRNG Data Register. See Page 364, Table 25-4.
pub struct DataRegister<const PORT_PTR: usize> {}
reg_impl!(RO, DataRegister, rro::TRNG_DATA);

impl<const PORT_PTR: usize> DataRegister<PORT_PTR> {
    bit_impl! {0..=31, RO u32,
    /// # Get TRNG Data
    get_trng_data}
}
