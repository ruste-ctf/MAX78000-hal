pub mod registers;

use crate::error::{ErrorKind, Result};
#[cfg(not(test))]
use crate::memory_map::mmio;
use registers::Registers;

pub struct AES {
    registers: Registers,
    key_status: KeyStatus,
}

pub enum Key<'a> {
    Bits128(&'a [u8; 16]),
    Bits192(&'a [u8; 24]),
    Bits256(&'a [u8; 32]),
}

#[derive(PartialEq)]
enum KeyStatus {
    NoKey,
    Bits128,
    Bits192,
    Bits256,
}

const ENCRYPT_EXTERNAL_KEY: u8 = 0b_00;
const DECRYPT_EXTERNAL_KEY: u8 = 0b_01;
// FIXME Local Key Generation Done VIA TRNG
//const DECRYPT_LOCAL_KEY: u8 = 0b_10;

impl AES {
    #[cfg(not(test))]
    pub fn init() -> Self {
        Self {
            registers: Registers::new(mmio::AES),
            key_status: KeyStatus::NoKey,
        }
    }

    #[cfg(test)]
    pub fn init(port: usize) -> Self {
        Self {
            registers: Registers::new(port),
            key_status: KeyStatus::NoKey,
        }
    }

    pub fn encrypt(&mut self, data: &mut [u8]) -> Result<()> {
        self.polling_external_key(data, false)
    }

    pub fn decrypt(&mut self, data: &mut [u8]) -> Result<()> {
        self.polling_external_key(data, true)
    }

    pub fn set_key(&mut self, key: &Key) {
        let (key_ptr, key_len, key_status) = match key {
            Key::Bits128(key) => (key.as_ptr(), 16, KeyStatus::Bits128),
            Key::Bits192(key) => (key.as_ptr(), 24, KeyStatus::Bits192),
            Key::Bits256(key) => (key.as_ptr(), 32, KeyStatus::Bits256),
        };
        #[cfg(not(test))]
        unsafe {
            core::ptr::copy_nonoverlapping(key_ptr, mmio::AES_KEYS as *mut u8, key_len);
        }
        #[cfg(test)]
        {
            _ = (key_ptr, key_len);
        }
        self.key_status = key_status;
    }

    fn polling_external_key(&mut self, data: &mut [u8], decrypt: bool) -> Result<()> {
        if self.key_status == KeyStatus::NoKey {
            return Err(ErrorKind::Uninitialized);
        }
        if data.len() % 16 != 0 {
            return Err(ErrorKind::BadParam);
        }

        // See (MAX78000 User Guide 24.2 Encryption of 128-Bit Blocks of Data Using FIFO
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
            self.registers
                .set_encryption_key_size(match self.key_status {
                    KeyStatus::Bits128 => 0b_00,
                    KeyStatus::Bits192 => 0b_01,
                    KeyStatus::Bits256 => 0b_10,
                    KeyStatus::NoKey => unreachable!(),
                });
        }

        let encryption_type = match decrypt {
            false => ENCRYPT_EXTERNAL_KEY,
            true => DECRYPT_EXTERNAL_KEY,
        };
        unsafe {
            self.registers.set_encryption_type(encryption_type);
        }

        unsafe {
            self.registers.set_aes_enable(true);
        }

        for block_byte_index in (0..data.len()).step_by(16) {
            self.load_fifo(&data, block_byte_index);

            while self.registers.get_aes_busy() {}

            self.read_back_fifo(data, block_byte_index);
        }
        Ok(())
    }

    fn load_fifo(&mut self, data: &[u8], block_byte_index: usize) {
        for word_byte_index in (0..16).step_by(4) {
            let word_start = block_byte_index + word_byte_index;
            unsafe {
                self.registers.set_aes_fifo(u32::from_ne_bytes(
                    data[word_start..(word_start + 4)].try_into().unwrap(),
                ));
            }
        }
    }

    fn read_back_fifo(&self, data: &mut [u8], block_byte_index: usize) {
        for word_byte_index in (0..16).step_by(4) {
            let word_start = block_byte_index + word_byte_index;
            let fifo_bytes = self.registers.get_aes_fifo().to_ne_bytes();
            data[word_start..(4 + word_start)].copy_from_slice(&fifo_bytes);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_fifo_test() {
        let mut fake_aes_registers: [u32; 6] = [0; 6];
        let mut aes = AES::init(fake_aes_registers.as_mut_ptr() as usize);
        let data = [0b_01110101; 16];
        aes.load_fifo(&data, 0usize);
        assert_eq!(
            fake_aes_registers[4],
            0b_01110101_01110101_01110101_01110101
        );
    }

    #[test]
    fn read_back_fifo_test() {
        let mut fake_aes_registers: [u32; 6] = [0; 6];
        fake_aes_registers[4] = 0b_01110101_01110101_01110101_01110101;
        let aes = AES::init(fake_aes_registers.as_mut_ptr() as usize);
        let mut data = [0b_00000000; 16];
        aes.read_back_fifo(&mut data, 0usize);
        assert_eq!(
            data[0..4],
            [0b_01110101, 0b_01110101, 0b_01110101, 0b_01110101]
        );
    }

    #[test]
    fn test_encrypt() {
        let mut fake_aes_registers: [u32; 6] = [0; 6];
        let mut aes = AES::init(fake_aes_registers.as_mut_ptr() as usize);
        let mut data = [0b_01110101; 16];
        aes.set_key(&Key::Bits128(&[0b_11110101; 16]));
        aes.encrypt(&mut data).unwrap();
        assert_eq!(fake_aes_registers[0], 0b_00000001);
        assert_eq!(
            fake_aes_registers[4],
            0b_01110101_01110101_01110101_01110101
        );
    }

    #[test]
    fn test_decrypt() {
        let mut fake_aes_registers: [u32; 6] = [0; 6];
        let mut aes = AES::init(fake_aes_registers.as_mut_ptr() as usize);
        let mut data = [0b_01110101; 16];
        aes.set_key(&Key::Bits128(&[0b_11110101; 16]));
        aes.decrypt(&mut data).unwrap();
        assert_eq!(fake_aes_registers[0], 0b_1_00000001);
        assert_eq!(
            fake_aes_registers[4],
            0b_01110101_01110101_01110101_01110101
        );
    }
}
