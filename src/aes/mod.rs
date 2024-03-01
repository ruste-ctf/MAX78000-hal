pub mod registers;

use crate::{
    gcr::{peripheral_reset, system_clock_enable, HardwareSource},
    memory_map::mmio,
};
use registers::Registers;

/// The type of a cipher operation. This enum is used to set the `Encryption Type`
/// flag of the AES Control Register. See Page 360-361, Table 24-4.
/// Note: We excluded external key decryption because it would cause decryption to
/// fail. Instead we always use internal key decryption and always set our own key.
/// The user guide suggest that when no key is set that a key will be automatically
/// generated for you using TRNG, but this doesn't make sense because the AES keys
/// register is write only and you would have no way of storing the key in order to
/// decrypt your data later.
#[repr(u8)]
pub enum CipherType {
    Encrypt = 0b_00,
    Decrypt = 0b_10,
}

/// A wrapper for an array containing an AES key. Used to allow multiple key sizes
/// and assure they are the correct length.
pub enum Key<'a> {
    Bits128(&'a [u8; 16]),
    Bits192(&'a [u8; 24]),
    Bits256(&'a [u8; 32]),
}

/// A wrapper around the AES register. Used to allow the borrow checker to keep
/// track of who can mutate the state of AES.
pub struct AES {
    registers: Registers,
}

impl AES {
    /// Initializes a new instance of AES. Should never be called more than once.
    pub fn init() -> Self {
        peripheral_reset(HardwareSource::AES);
        system_clock_enable(HardwareSource::AES, true);
        Self {
            registers: Registers::new(mmio::AES),
        }
    }

    /// Writes the given key to the beginning of the AES keys register. Before setting
    /// the key it will wipe all 1024 bytes of the register and after setting the key
    /// it will run a dummy encryption to assure that the first decryption will always work.
    pub fn set_key(&mut self, key: &Key) {
        let (key_ptr, key_len) = match key {
            Key::Bits128(key) => (key.as_ptr(), 16),
            Key::Bits192(key) => (key.as_ptr(), 24),
            Key::Bits256(key) => (key.as_ptr(), 32),
        };
        #[cfg(not(test))]
        unsafe {
            for i in 0..256 {
                core::ptr::write_volatile((mmio::AES_KEYS + (i * 4)) as *mut u32, 0u32);
            }
            core::ptr::copy_nonoverlapping(key_ptr, mmio::AES_KEYS as *mut u8, key_len);
            [0; 16]
                .into_iter()
                .cipher(self, CipherType::Encrypt)
                .for_each(|_| {});
        }
        #[cfg(test)]
        {
            _ = (key_ptr, key_len);
        }
    }

    /// Loads a block into AES FIFO Register. The hardware will automatically start the
    /// calculation on this block after each of the four words are written.
    fn load_fifo(&mut self, data: [u8; 16]) {
        let block: u128 = u128::from_le_bytes(data);
        for word in unsafe { &*(&block as *const u128 as *const [u32; 4]) } {
            unsafe { self.registers.set_aes_fifo(*word) };
        }
    }

    /// Reads a block from the AES FIFO Register.
    pub fn read_back_fifo(&self) -> [u8; 16] {
        let mut data = [0u32; 4];
        for word in data.iter_mut() {
            *word = self.registers.get_aes_fifo();
        }
        let block = unsafe { &*(data.as_ptr() as *const u32 as *const u128) };
        block.to_le_bytes()
    }
}

/// Holds the state of an AES cipher operation.
pub struct AESIter<'a, I> {
    iter: I,
    aes: &'a mut AES,
    block_buffer: [u8; 16],
    send_index: usize,
}

impl<I: Iterator> AESIterExt for I {}

pub trait AESIterExt: Iterator {
    /// Initializes a new AES cipher operation. Returns an iterator over the ciphered bytes.
    fn cipher<'a>(self, aes: &'a mut AES, cipher_type: CipherType) -> AESIter<Self>
    where
        Self::Item: Into<u8>,
        Self: Sized,
    {
        unsafe {
            aes.registers.set_aes_control_register(0);
            aes.registers.set_encryption_type(cipher_type as u8);
            aes.registers.set_aes_enable(true);
        }

        AESIter {
            iter: self,
            aes,
            block_buffer: [0; 16],
            send_index: 16,
        }
    }
}

impl<'a, I> Iterator for AESIter<'a, I>
where
    I: Iterator,
    I::Item: Into<u8>,
{
    type Item = u8;

    /// Returns the next cipher byte from the operation. `AESIter` buffers AES blocks
    /// such that every 16th call will advance the internal iterator 16 times and
    /// perform the next block cipher using the hardware.
    fn next(&mut self) -> Option<u8> {
        if self.send_index == 16 {
            for (i, byte) in self.block_buffer.iter_mut().enumerate() {
                *byte = match self.iter.next() {
                    Some(next_byte) => next_byte.into(),
                    None if i == 0 => return None,
                    None => 0,
                }
            }
            self.aes.load_fifo(self.block_buffer);
            self.block_buffer
                .copy_from_slice(&self.aes.read_back_fifo());
            self.send_index = 0;
        }
        let result = self.block_buffer[self.send_index];
        self.send_index += 1;
        Some(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_fifo_test() {
        let mut fake_aes_registers: [u32; 6] = [0; 6];
        let mut aes = AES {
            registers: Registers::new(fake_aes_registers.as_mut_ptr() as usize),
        };
        let data = [0b_01110101; 16];
        aes.load_fifo(data);
        assert_eq!(
            fake_aes_registers[4],
            0b_01110101_01110101_01110101_01110101
        );
    }

    #[test]
    fn read_back_fifo_test() {
        let mut fake_aes_registers: [u32; 6] = [0; 6];
        fake_aes_registers[4] = 0b_01110101_01110101_01110101_01110101;
        let aes = AES {
            registers: Registers::new(fake_aes_registers.as_mut_ptr() as usize),
        };
        let data = aes.read_back_fifo();
        assert_eq!(
            data[0..4],
            [0b_01110101, 0b_01110101, 0b_01110101, 0b_01110101]
        );
    }
}
