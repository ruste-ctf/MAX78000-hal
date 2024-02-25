pub mod registers;
use crate::memory_map::mmio;
use registers::Registers;

pub struct TRNG {
    registers: Registers,
}

impl TRNG {
    pub fn init() -> Self {
        Self {
            registers: Registers::new(mmio::TRNG),
        }
    }

    pub fn wipe_aes_local_key(&mut self) {
        unsafe {
            self.registers.set_wipe_key(true);
        }
    }

    pub fn generate_aes_local_key(&mut self) {
        unsafe {
            self.registers.set_generate_key(true);
        }
    }

    pub fn get_trng_data(&mut self) -> u32 {
        // FIXME use interrupt
        while !self.registers.get_random_number_ready() {}
        self.registers.get_trng_data()
    }

    pub fn ready(&self) -> bool {
        self.registers.get_random_number_ready()
    }

    pub fn get(&self) -> u32 {
        self.registers.get_trng_data()
    }
}
