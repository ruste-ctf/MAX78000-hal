#[macro_export]
macro_rules! reg_impl {
    (RW, $t:tt, $v:expr) => {
        impl<const PORT_PTR: usize> $t<PORT_PTR> {
            reg_impl!(@gen BLANKET, $v);
            reg_impl!(@gen READ);
            reg_impl!(@gen WRITE);
        }
    };
    (RO, $t:tt, $v:expr) => {
        impl<const PORT_PTR: usize> $t<PORT_PTR> {
            reg_impl!(@gen BLANKET, $v);
            reg_impl!(@gen READ);
        }
    };
    (RW1C, $t:tt, $v:expr, $read:literal) => {
        impl<const PORT_PTR: usize> $t<PORT_PTR> {
            reg_impl!(@gen BLANKET, $v);
            reg_impl!(@gen READ);
            reg_impl!(@gen READ_MASK, $read);
            reg_impl!(@gen WRITE);
        }
    };
    (@gen READ) => {
        /// # Read
        /// Get the value stored at this register with **1** volatile
        /// memory read.
        ///
        /// # Safety
        /// It is ultimately up to the caller to determine that any read
        /// from this register will be safe. Mostly, reading from registers
        /// do not change processor state, but it should still be warned
        /// that reading could be unsafe in some cases.
        ///
        /// # Volatile
        /// This read function will preform **1** volatile `read` from the given
        /// register. Each register's helper functions will call this very function
        /// to read, and thus each register's helper functions conform to the same
        /// safety and volatility of this function.
        #[inline]
        pub fn read() -> u32 {
            unsafe { ptr::read_volatile(Self::get_ptr()) }
        }
    };
    (@gen READ_MASK, $read:literal) => {
        /// # Read Masked
        /// Get the value stored at this register, but mask the value with
        /// all RW1C register locations. This is important because when
        /// writing back the value, we must not change the _'write 1 to
        /// clear'_ based registers.
        #[inline]
        pub fn read_masked() -> u32 {
            unsafe { core::ptr::read_volatile(Self::get_ptr()) & $read}
        }
    };
    (@gen WRITE) => {
        /// # Write
        /// Write to the value stored at this register with **1** volatile
        /// memory read.
        ///
        /// # Safety
        /// It is up to the caller to verify that this register write will not
        /// cause any side effects. There could be an event that setting this
        /// register could cause undefined behavior elsewhere in the program.
        ///
        /// ## Other Register State
        /// In some examples it is true that ones register state depends on another
        /// register's status. In these cases, it is up to the caller to properly
        /// set this register to a valid (and ONLY valid value).
        ///
        /// # Volatile
        /// This read function will preform **1** volatile `write` from the given
        /// register. Each register's helper functions will call this very function
        /// to read, and thus each register's helper functions conform to the same
        /// safety and volatility of this function.
        #[inline]
        pub unsafe fn write(value: u32) {
            unsafe { ptr::write_volatile(Self::get_ptr(), value) }
        }
    };
    (@gen BLANKET, $v:expr) => {
        /// # Register Address Bits
        /// The raw usize address of this register.
        const REGISTER_ADDRESS_BITS: usize = PORT_PTR + $v;

        // We should only I2C_PORT_0, I2C_PORT_1, and I2C_PORT_2 into this struct.
        // It should not be possible to compile with any other port address.
        const_assert!(
            STRUCT,
            (PORT_PTR == mmio::I2C_PORT_0)
                || (PORT_PTR == mmio::I2C_PORT_1)
                || (PORT_PTR == mmio::I2C_PORT_2),
            "Should only except I2C_PORT_0, I2C_PORT_1, or I2C_PORT_2!"
        );

        /// # Get Ptr
        /// Get the raw ptr for which this address is stored. Only volatile
        /// accesses should be used to read/write to this ptr.
        pub const fn get_ptr() -> *mut u32 {
            Self::REGISTER_ADDRESS_BITS as *mut u32
        }
    }
}

/// # Bit Impl
/// A macro to help with implementing large number of single bit operations on registers.
///
/// ## How to use
/// ```text
/// bit_impl!{5, RW, set_my_register, get_my_register}
///           ^
///       Bit to use
/// ```
#[macro_export]
macro_rules! bit_impl {
    ($bit:literal, RW, $(#[$meta_set:meta])* $set:ident, $(#[$meta_get:meta])* $get:ident) => {
        bit_impl!($bit, WO, $(#[$meta_set])* $set);
        bit_impl!($bit, RO, $(#[$meta_get])* $get);
    };
    ($bit:literal, RW1C, $(#[$meta_set:meta])* $set:ident, $(#[$meta_get:meta])* $get:ident) => {
        bit_impl!($bit, RESET, $(#[$meta_set])* $set);
        bit_impl!($bit, RO, $(#[$meta_get])* $get);
    };
    ($bits:expr, RO $type:ty, $(#[$meta_get:meta])* $get:ident) => {
        $(#[$meta_get])*
        ///
        /// # Safety
        /// It is ultimately up to the caller to ensure this function will
        /// never cause any side effects. However, usually reading from
        /// registers does not modify any processor state (just looks at it).
        ///
        /// # Volatile
        /// This function only preforms **1** volatile *read* and immediately copies
        /// the value and extracts the bits to return the result.
        ///
        #[inline]
        pub fn $get() -> $type {
            Self::read().get_bit_range($bits) as $type
        }
    };
    ($bit:literal, RO, $(#[$meta_get:meta])* $get:ident) => {
        $(#[$meta_get])*
        ///
        /// # Safety
        /// It is ultimately up to the caller to ensure this function will
        /// never cause any side effects. However, usually reading from
        /// registers does not modify any processor state (just looks at it).
        ///
        /// # Volatile
        /// This function only preforms **1** volatile *read* and immediately copies
        /// the value to test the flag and return the result.
        ///
        #[inline]
        pub fn $get() -> bool {
            Self::read().get_bit($bit)
        }
    };
    ($bit:literal, WO, $(#[$meta_set:meta])* $set:ident) => {
        $(#[$meta_set])*
        ///
        /// # Safety
        /// It is up to the caller to verify that this register write will not
        /// cause any side effects. There could be an event that setting this
        /// register could cause undefined behavior elsewhere in the program.
        ///
        /// ## Other Register State
        /// In some examples it is true that ones register state depends on another
        /// register's status. In these cases, it is up to the caller to properly
        /// set this register to a valid (and ONLY valid value).
        ///
        /// # Volatile
        /// This function only preforms **1** volatile *read* using `Self::read()`,
        /// immediately modifies the flag and does **1** volatile *write* using
        /// the internal provided function `Self::write(value)`.
        #[inline]
        pub unsafe fn $set(flag: bool) {
            let mut value = Self::read();
            value.set_bit($bit, flag);
            Self::write(value);
        }
    };
    ($bit:literal, RESET, $(#[$meta_set:meta])* $set:ident) => {
        $(#[$meta_set])*
        ///
        /// # Safety
        /// It is up to the caller to verify that this register write will not
        /// cause any side effects. There could be an event that setting this
        /// register could cause undefined behavior elsewhere in the program.
        ///
        /// ## Other Register State
        /// In some examples it is true that ones register state depends on another
        /// register's status. In these cases, it is up to the caller to properly
        /// set this register to a valid (and ONLY valid value).
        ///
        /// # Volatile
        /// This function only preforms **1** volatile *read* using `Self::read()`,
        /// immediately modifies the flag and does **1** volatile *write* using
        /// the internal provided function `Self::write(value)`.
        #[inline]
        pub unsafe fn $set() {
            let mut value = Self::read_masked();
            value.set_bit($bit, true);
            Self::write(value);
        }
    }
}
