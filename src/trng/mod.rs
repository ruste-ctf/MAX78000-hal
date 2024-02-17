pub mod registers;
use crate::memory_map::mmio;
use registers::Registers;

pub struct TRNG {
    registers: Registers,
    has_key: bool,
}

impl TRNG {
    pub fn init() -> Self {
        let mut registers = Self {
            registers: Registers::new(mmio::TRNG),
            has_key: false,
        };
        registers.wipe_key();
        registers.generate_new_key();
        registers
    }

    pub fn has_key(&self) -> bool {
        self.has_key
    }

    pub fn wipe_key(&mut self) {
        unsafe {
            self.registers.set_wipe_key(true);
        }
        self.has_key = false;
    }

    pub fn generate_new_key(&mut self) {
        unsafe {
            self.registers.set_generate_key(true);
        }
        self.has_key = true;
    }

    pub fn get_trng_data(&mut self) -> u32 {
        // FIXME use interrupts
        assert!(self.has_key);
        while !self.registers.get_random_number_ready() {}
        self.registers.get_trng_data()
    }

    /// # Safety
    /// This function is for educational purposes only. It should only be used to
    /// learn about what happens when you generate a number without having a key.
    pub unsafe fn unchecked_get_trng_data(&self) -> u32 {
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
