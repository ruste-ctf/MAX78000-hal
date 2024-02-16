pub mod registers;
use crate::memory_map::mmio;
use registers::Registers;

pub struct TRNG {
    registers: Registers,
    pub has_key: bool,
}

impl TRNG {
    pub fn init() -> Self {
        let mut registers = Self {
            registers: Registers::new(mmio::TRNG),
            has_key: false,
        };
        unsafe {
            registers.registers.set_generate_key(true);
        }
        registers.has_key = true;
        return registers;
    }

    pub fn wipe_key(&mut self) {
        unsafe {
            self.registers.set_wipe_key(true);
        }
    }

    pub fn generate_new_key(&mut self) {
        unsafe {
            self.registers.set_generate_key(true);
        }
    }

    pub fn get_trng_data(&self) -> u32 {
        // FIXME use interrupts
        while self.registers.get_random_number_ready() == false {}
        self.registers.get_trng_data()
    }
}
