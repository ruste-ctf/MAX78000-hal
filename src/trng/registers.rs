use crate::memory_map::mmio;
use hal_macros::RW;
use hal_macros_derive::make_device;

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

make_device! {
    device_ports(mmio::TRNG);

    /// Wipe Key. See Page 363, Table 25-2.
    #[bit(15, RW, rro::TRNG_CTRL)]
    wipe_key,

    /// Generate Key. See Page 363, Table 25-2.
    #[bit(3, RW, rro::TRNG_CTRL)]
    generate_key,

    /// Random Number Interrupt Enable. See Page 363, Table 25-2.
    #[bit(1, RW, rro::TRNG_CTRL)]
    random_number_interrupt_enable,

    /// TRNG Control Register. See Page 363, Table 25-2.
    #[bit(0..=31, RW, rro::TRNG_CTRL)]
    trng_control_register,

    /// Random Number Ready. See Page 363-364, Table 25-3.
    #[bit(0, RO, rro::TRNG_STATUS)]
    random_number_ready,

    /// TRNG Data. See Page 364, Table 25-4.
    #[bit(0..=31, RO, rro::TRNG_DATA)]
    trng_data,
}
