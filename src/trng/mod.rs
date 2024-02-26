pub mod registers;

use crate::gcr::HardwareSource;
use crate::gcr::{peripheral_reset, system_clock_enable};
use crate::memory_map::mmio;
use registers::Registers;

/// A wrapper around the TRNG register. Used to allow the borrow checker to keep
/// track of who can mutate the state of TRNG.
pub struct TRNG {
    registers: Registers,
}

impl TRNG {
    /// Initializes TRNG by resetting the TRNG peripheral, enabling TRNG's system
    /// clock, enabling AES's system clock, and clearing the TRNG control register.
    /// Should never be initialized more than once.
    pub fn init() -> Self {
        system_clock_enable(HardwareSource::AES, true);
        peripheral_reset(HardwareSource::TRNG);
        system_clock_enable(HardwareSource::TRNG, true);

        let mut registers = Registers::new(mmio::TRNG);
        unsafe { registers.set_trng_control_register(0) };
        Self { registers }
    }

    /// Get a random number from TRNG.
    pub fn get_trng_data(&mut self) -> u32 {
        while !self.registers.get_random_number_ready() {}
        self.registers.get_trng_data()
    }

    /// Check if `get_trng_data` is ready.
    pub fn ready(&self) -> bool {
        self.registers.get_random_number_ready()
    }
}
