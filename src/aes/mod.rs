pub mod registers;

use crate::error::{ErrorKind, Result};
use crate::memory_map::mmio;
use registers::Registers;

pub struct AES {
    registers: Registers,
}

pub enum Key<'a> {
    Bits128(&'a [u8; 16]),
    Bits192(&'a [u8; 24]),
    Bits256(&'a [u8; 32]),
}

const ENCRYPT_EXTERNAL_KEY: u8 = 0b_00;
const DECRYPT_EXTERNAL_KEY: u8 = 0b_01;
// FIXME User Guide is vague and local generation doesn't make sense
//const DECRYPT_LOCAL_KEY: u8 = 0b_10;

impl AES {
    pub fn init() -> Self {
        Self {
            registers: Registers::new(mmio::AES),
        }
    }

    pub fn encrypt(&mut self, data: &mut [u8], key_size: Key) -> Result<()> {
        self.external_key(data, key_size, false)
    }

    pub fn decrypt(&mut self, data: &mut [u8], key_size: Key) -> Result<()> {
        self.external_key(data, key_size, true)
    }

    fn external_key(&mut self, data: &mut [u8], key_size: Key, decrypt: bool) -> Result<()> {
        if data.len() % 16 != 0 {
            return Err(ErrorKind::BadParam);
        }

        // See MAX78000 User Guide 24.2 Encryption of 128-Bit Blocks of Data Using FIFO

        let (key_ptr, key_len) = match key_size {
            Key::Bits128(key) => (key.as_ref() as *const [u8] as *const u8, 16),
            Key::Bits192(key) => (key.as_ref() as *const [u8] as *const u8, 24),
            Key::Bits256(key) => (key.as_ref() as *const [u8] as *const u8, 32),
        };
        unsafe {
            core::ptr::copy_nonoverlapping(key_ptr, mmio::AES_KEYS as *mut u8, key_len);
        }

        for block_byte_index in (0..data.len()).step_by(16) {
            while self.registers.get_aes_busy() {}
            unsafe {
                self.registers.set_aes_enable(false);
            }

            if !self.registers.get_input_fifo_empty() {
                unsafe {
                    self.registers.activate_flush_data_input_fifo();
                }
            }
            if !self.registers.get_output_fifo_empty() {
                unsafe {
                    self.registers.activate_flush_data_output_fifo();
                }
            }

            unsafe {
                self.registers.set_encryption_key_size(match key_size {
                    Key::Bits128(_) => 0b_00,
                    Key::Bits192(_) => 0b_01,
                    Key::Bits256(_) => 0b_10,
                });
            }

            let encryption_type = match decrypt {
                false => ENCRYPT_EXTERNAL_KEY,
                true => DECRYPT_EXTERNAL_KEY,
            };
            unsafe {
                self.registers.set_encryption_type(encryption_type);
            }

            // FIXME User Guide says W1C for these enable interrupts which doesn't make sense
            /*
            unsafe {
                self.registers.clear_calculation_done_event_interrupt();
            }
            */

            unsafe {
                self.registers.set_aes_enable(true);
            }

            for word_byte_index in (0..16).step_by(4) {
                let word_start = block_byte_index + word_byte_index;
                unsafe {
                    self.registers.set_aes_fifo(u32::from_ne_bytes(
                        data[word_start..(word_start + 4)].try_into().unwrap(),
                    ));
                }
            }

            while self.registers.get_aes_busy() {}

            for word_byte_index in (0..16).step_by(4) {
                let word_start = block_byte_index + word_byte_index;
                let fifo_bytes = self.registers.get_aes_fifo().to_ne_bytes();
                data[word_start..(4 + word_start)].copy_from_slice(&fifo_bytes);
            }
        }
        Ok(())
    }
}
