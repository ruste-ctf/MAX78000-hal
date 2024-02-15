#[macro_export]
macro_rules! reg_impl {
    (RW, $t:tt, $v:expr) => {
        impl<const PORT_PTR: usize> $t<PORT_PTR> {
            reg_impl!(@gen BLANKET, $v);
            reg_impl!(@gen READ);
            reg_impl!(@gen READ_MASK_READ);
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
    (RW1O, $t:tt, $v:expr, $read:literal) => {
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
            unsafe { core::ptr::read_volatile(Self::get_ptr()) }
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
    (@gen READ_MASK_READ) => {
        /// # Read Masked (COPY OF READ FOR LOCAL USE ONLY)
        /// This is only implemented so we can use RW1C and RW1O without
        /// bits getting set in write-1-to-xxxx registers.
        #[inline]
        #[allow(unused)]
        fn read_masked() -> u32 {
            unsafe { core::ptr::read_volatile(Self::get_ptr())}
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
            unsafe { core::ptr::write_volatile(Self::get_ptr(), value) }
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
    ($bit:literal, RW1O, $(#[$meta_set:meta])* $set:ident, $(#[$meta_get:meta])* $get:ident) => {
        bit_impl!($bit, RESET, $(#[$meta_set])* $set);
        bit_impl!($bit, RO, $(#[$meta_get])* $get);
    };

    ($bits:expr, RW $type:ty, $(#[$meta_set:meta])* $set:ident, $(#[$meta_get:meta])* $get:ident) => {
        bit_impl!($bits, WO $type, $(#[$meta_set])* $set);
        bit_impl!($bits, RO $type, $(#[$meta_get])* $get);
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
            use $crate::bits::BitManipulation;
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
            use $crate::bits::BitManipulation;
            Self::read().get_bit($bit)
        }
    };
    ($bits:expr, WO $type:ty, $(#[$meta_set:meta])* $set:ident) => {
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
        pub unsafe fn $set(flag: $type) {
            use $crate::bits::BitManipulation;
            let mut value = Self::read_masked();
            value.set_bit_range($bits, flag);
            Self::write(value);
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
            use $crate::bits::BitManipulation;
            let mut value = Self::read_masked();
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
            use $crate::bits::BitManipulation;
            let mut value = Self::read_masked();
            value.set_bit($bit, true);
            Self::write(value);
        }
    }
}

#[cfg(test)]
mod test {
    static mut TEST_PORT_DATA: u32 = 0;

    struct MyTestRegister {}
    impl MyTestRegister {
        pub fn get_ptr() -> *mut u32 {
            unsafe { &mut TEST_PORT_DATA as *mut u32 }
        }

        reg_impl!(@gen READ);
        reg_impl!(@gen READ_MASK_READ);
        reg_impl!(@gen WRITE);

        bit_impl! {0, RW,
        set_test_0_bit,
        get_test_0_bit}

        bit_impl! {1, RW,
        set_test_1_bit,
        get_test_1_bit}

        bit_impl! {2, RW,
        set_test_2_bit,
        get_test_2_bit}

        bit_impl! {3, RW,
        set_test_3_bit,
        get_test_3_bit}

        bit_impl! {4, RW,
        set_test_4_bit,
        get_test_4_bit}

        bit_impl! {5, RW,
        set_test_5_bit,
        get_test_5_bit}

        bit_impl! {6, RW,
        set_test_6_bit,
        get_test_6_bit}

        bit_impl! {7, RW,
        set_test_7_bit,
        get_test_7_bit}

        bit_impl! {8, RW,
        set_test_8_bit,
        get_test_8_bit}

        bit_impl! {9, RW,
        set_test_9_bit,
        get_test_9_bit}

        bit_impl! {10, RW,
        set_test_10_bit,
        get_test_10_bit}

        bit_impl! {11, RW,
        set_test_11_bit,
        get_test_11_bit}

        bit_impl! {12, RW,
        set_test_12_bit,
        get_test_12_bit}

        bit_impl! {13, RW,
        set_test_13_bit,
        get_test_13_bit}

        bit_impl! {14, RW,
        set_test_14_bit,
        get_test_14_bit}

        bit_impl! {15, RW,
        set_test_15_bit,
        get_test_15_bit}

        bit_impl! {16..=20, RW u8,
        set_test_5_bits_register,
        get_test_5_bits_register}
    }

    #[test]
    fn test_bit_impl_0_bit() {
        assert!(!MyTestRegister::get_test_0_bit(), "Register should be zero");
        unsafe { MyTestRegister::set_test_0_bit(false) };
        assert!(!MyTestRegister::get_test_0_bit(), "Register should be zero");
        unsafe { MyTestRegister::set_test_0_bit(true) };
        assert!(MyTestRegister::get_test_0_bit(), "Register should be one");
        assert_eq!(
            unsafe { TEST_PORT_DATA } & (1 << 0),
            1 << 0,
            "Data should be one"
        );
    }

    #[test]
    fn test_bit_impl_1_bit() {
        assert!(!MyTestRegister::get_test_1_bit(), "Register should be zero");
        unsafe { MyTestRegister::set_test_1_bit(false) };
        assert!(!MyTestRegister::get_test_1_bit(), "Register should be zero");
        unsafe { MyTestRegister::set_test_1_bit(true) };
        assert!(MyTestRegister::get_test_1_bit(), "Register should be one");
        assert_eq!(
            unsafe { TEST_PORT_DATA } & (1 << 1),
            1 << 1,
            "Data should be one"
        );
    }

    #[test]
    fn test_bit_impl_2_bit() {
        assert!(!MyTestRegister::get_test_2_bit(), "Register should be zero");
        unsafe { MyTestRegister::set_test_2_bit(false) };
        assert!(!MyTestRegister::get_test_2_bit(), "Register should be zero");
        unsafe { MyTestRegister::set_test_2_bit(true) };
        assert!(MyTestRegister::get_test_2_bit(), "Register should be one");
        assert_eq!(
            unsafe { TEST_PORT_DATA } & (1 << 2),
            1 << 2,
            "Data should be one"
        );
    }

    #[test]
    fn test_bit_impl_3_bit() {
        assert!(!MyTestRegister::get_test_3_bit(), "Register should be zero");
        unsafe { MyTestRegister::set_test_3_bit(false) };
        assert!(!MyTestRegister::get_test_3_bit(), "Register should be zero");
        unsafe { MyTestRegister::set_test_3_bit(true) };
        assert!(MyTestRegister::get_test_3_bit(), "Register should be one");
        assert_eq!(
            unsafe { TEST_PORT_DATA } & (1 << 3),
            1 << 3,
            "Data should be one"
        );
    }

    #[test]
    fn test_bit_impl_4_bit() {
        assert!(!MyTestRegister::get_test_4_bit(), "Register should be zero");
        unsafe { MyTestRegister::set_test_4_bit(false) };
        assert!(!MyTestRegister::get_test_4_bit(), "Register should be zero");
        unsafe { MyTestRegister::set_test_4_bit(true) };
        assert!(MyTestRegister::get_test_4_bit(), "Register should be one");
        assert_eq!(
            unsafe { TEST_PORT_DATA } & (1 << 4),
            1 << 4,
            "Data should be one"
        );
    }

    #[test]
    fn test_bit_impl_5_bit() {
        assert!(!MyTestRegister::get_test_5_bit(), "Register should be zero");
        unsafe { MyTestRegister::set_test_5_bit(false) };
        assert!(!MyTestRegister::get_test_5_bit(), "Register should be zero");
        unsafe { MyTestRegister::set_test_5_bit(true) };
        assert!(MyTestRegister::get_test_5_bit(), "Register should be one");
        assert_eq!(
            unsafe { TEST_PORT_DATA } & (1 << 5),
            1 << 5,
            "Data should be one"
        );
    }

    #[test]
    fn test_bit_impl_6_bit() {
        assert!(!MyTestRegister::get_test_6_bit(), "Register should be zero");
        unsafe { MyTestRegister::set_test_6_bit(false) };
        assert!(!MyTestRegister::get_test_6_bit(), "Register should be zero");
        unsafe { MyTestRegister::set_test_6_bit(true) };
        assert!(MyTestRegister::get_test_6_bit(), "Register should be one");
        assert_eq!(
            unsafe { TEST_PORT_DATA } & (1 << 6),
            1 << 6,
            "Data should be one"
        );
    }

    #[test]
    fn test_bit_impl_7_bit() {
        assert!(!MyTestRegister::get_test_7_bit(), "Register should be zero");
        unsafe { MyTestRegister::set_test_7_bit(false) };
        assert!(!MyTestRegister::get_test_7_bit(), "Register should be zero");
        unsafe { MyTestRegister::set_test_7_bit(true) };
        assert!(MyTestRegister::get_test_7_bit(), "Register should be one");
        assert_eq!(
            unsafe { TEST_PORT_DATA } & (1 << 7),
            1 << 7,
            "Data should be one"
        );
    }

    #[test]
    fn test_bit_impl_8_bit() {
        assert!(!MyTestRegister::get_test_8_bit(), "Register should be zero");
        unsafe { MyTestRegister::set_test_8_bit(false) };
        assert!(!MyTestRegister::get_test_8_bit(), "Register should be zero");
        unsafe { MyTestRegister::set_test_8_bit(true) };
        assert!(MyTestRegister::get_test_8_bit(), "Register should be one");
        assert_eq!(
            unsafe { TEST_PORT_DATA } & (1 << 8),
            1 << 8,
            "Data should be one"
        );
    }

    #[test]
    fn test_bit_impl_9_bit() {
        assert!(!MyTestRegister::get_test_9_bit(), "Register should be zero");
        unsafe { MyTestRegister::set_test_9_bit(false) };
        assert!(!MyTestRegister::get_test_9_bit(), "Register should be zero");
        unsafe { MyTestRegister::set_test_9_bit(true) };
        assert!(MyTestRegister::get_test_9_bit(), "Register should be one");
        assert_eq!(
            unsafe { TEST_PORT_DATA } & (1 << 9),
            1 << 9,
            "Data should be one"
        );
    }

    #[test]
    fn test_bit_impl_10_bit() {
        assert!(
            !MyTestRegister::get_test_10_bit(),
            "Register should be zero"
        );
        unsafe { MyTestRegister::set_test_10_bit(false) };
        assert!(
            !MyTestRegister::get_test_10_bit(),
            "Register should be zero"
        );
        unsafe { MyTestRegister::set_test_10_bit(true) };
        assert!(MyTestRegister::get_test_10_bit(), "Register should be one");
        assert_eq!(
            unsafe { TEST_PORT_DATA } & (1 << 10),
            1 << 10,
            "Data should be one"
        );
    }

    #[test]
    fn test_bit_impl_11_bit() {
        assert!(
            !MyTestRegister::get_test_11_bit(),
            "Register should be zero"
        );
        unsafe { MyTestRegister::set_test_11_bit(false) };
        assert!(
            !MyTestRegister::get_test_11_bit(),
            "Register should be zero"
        );
        unsafe { MyTestRegister::set_test_11_bit(true) };
        assert!(MyTestRegister::get_test_11_bit(), "Register should be one");
        assert_eq!(
            unsafe { TEST_PORT_DATA } & (1 << 11),
            1 << 11,
            "Data should be one"
        );
    }

    #[test]
    fn test_bit_impl_12_bit() {
        assert!(
            !MyTestRegister::get_test_12_bit(),
            "Register should be zero"
        );
        unsafe { MyTestRegister::set_test_12_bit(false) };
        assert!(
            !MyTestRegister::get_test_12_bit(),
            "Register should be zero"
        );
        unsafe { MyTestRegister::set_test_12_bit(true) };
        assert!(MyTestRegister::get_test_12_bit(), "Register should be one");
        assert_eq!(
            unsafe { TEST_PORT_DATA } & (1 << 12),
            1 << 12,
            "Data should be one"
        );
    }

    #[test]
    fn test_bit_impl_13_bit() {
        assert!(
            !MyTestRegister::get_test_13_bit(),
            "Register should be zero"
        );
        unsafe { MyTestRegister::set_test_13_bit(false) };
        assert!(
            !MyTestRegister::get_test_13_bit(),
            "Register should be zero"
        );
        unsafe { MyTestRegister::set_test_13_bit(true) };
        assert!(MyTestRegister::get_test_13_bit(), "Register should be one");
        assert_eq!(
            unsafe { TEST_PORT_DATA } & (1 << 13),
            1 << 13,
            "Data should be one"
        );
    }

    #[test]
    fn test_bit_impl_14_bit() {
        assert!(
            !MyTestRegister::get_test_14_bit(),
            "Register should be zero"
        );
        unsafe { MyTestRegister::set_test_14_bit(false) };
        assert!(
            !MyTestRegister::get_test_14_bit(),
            "Register should be zero"
        );
        unsafe { MyTestRegister::set_test_14_bit(true) };
        assert!(MyTestRegister::get_test_14_bit(), "Register should be one");
        assert_eq!(
            unsafe { TEST_PORT_DATA } & (1 << 14),
            1 << 14,
            "Data should be one"
        );
    }

    #[test]
    fn test_bit_impl_15_bit() {
        assert!(
            !MyTestRegister::get_test_15_bit(),
            "Register should be zero"
        );
        unsafe { MyTestRegister::set_test_15_bit(false) };
        assert!(
            !MyTestRegister::get_test_15_bit(),
            "Register should be zero"
        );
        unsafe { MyTestRegister::set_test_15_bit(true) };
        assert!(MyTestRegister::get_test_15_bit(), "Register should be one");
        assert_eq!(
            unsafe { TEST_PORT_DATA } & (1 << 15),
            1 << 15,
            "Data should be one"
        );
    }

    #[test]
    fn test_bit_impl_5_bit_register() {
        assert_eq!(
            MyTestRegister::get_test_5_bits_register(),
            0,
            "Register should be zero"
        );

        for i in 0..=0b11111 {
            unsafe { MyTestRegister::set_test_5_bits_register(i) };
            assert_eq!(MyTestRegister::get_test_5_bits_register(), i);
            assert_eq!(unsafe { TEST_PORT_DATA & (0b11111 << 16) } >> 16, i as u32);
        }

        unsafe { MyTestRegister::set_test_5_bits_register(0) };
        assert_eq!(MyTestRegister::get_test_5_bits_register(), 0);
        assert_eq!(unsafe { TEST_PORT_DATA & (0b11111 << 16) } >> 16, 0);
    }

    static mut TEST_PORT_RW1C_DATA: u32 = 0;

    struct MyTestRW1C {}
    impl MyTestRW1C {
        pub fn get_ptr() -> *mut u32 {
            unsafe { &mut TEST_PORT_RW1C_DATA as *mut u32 }
        }

        reg_impl!(@gen READ);
        reg_impl!(@gen READ_MASK, 0b1010);
        reg_impl!(@gen WRITE);

        bit_impl! {0, RW1C,
        clear_test_bit_0,
        is_test_bit_0}

        bit_impl! {1, RW,
        set_test_bit_1,
        is_test_bit_1}

        bit_impl! {2, RW1C,
        clear_test_bit_2,
        is_test_bit_2}

        bit_impl! {3, RW,
        set_test_bit_3,
        is_test_bit_3}
    }

    #[test]
    fn test_mask_bits() {
        unsafe { TEST_PORT_RW1C_DATA = 0 };
        assert_eq!(unsafe { TEST_PORT_RW1C_DATA }, 0);
        assert!(!MyTestRW1C::is_test_bit_0());
        assert!(!MyTestRW1C::is_test_bit_1());
        assert!(!MyTestRW1C::is_test_bit_2());
        assert!(!MyTestRW1C::is_test_bit_3());

        unsafe { TEST_PORT_RW1C_DATA |= 1 << 0 };
        assert!(MyTestRW1C::is_test_bit_0());
        assert!(!MyTestRW1C::is_test_bit_1());
        assert!(!MyTestRW1C::is_test_bit_2());
        assert!(!MyTestRW1C::is_test_bit_3());

        unsafe { TEST_PORT_RW1C_DATA |= 1 << 1 };
        assert!(MyTestRW1C::is_test_bit_0());
        assert!(MyTestRW1C::is_test_bit_1());
        assert!(!MyTestRW1C::is_test_bit_2());
        assert!(!MyTestRW1C::is_test_bit_3());

        unsafe { TEST_PORT_RW1C_DATA |= 1 << 2 };
        assert!(MyTestRW1C::is_test_bit_0());
        assert!(MyTestRW1C::is_test_bit_1());
        assert!(MyTestRW1C::is_test_bit_2());
        assert!(!MyTestRW1C::is_test_bit_3());

        unsafe { TEST_PORT_RW1C_DATA |= 1 << 3 };
        assert!(MyTestRW1C::is_test_bit_0());
        assert!(MyTestRW1C::is_test_bit_1());
        assert!(MyTestRW1C::is_test_bit_2());
        assert!(MyTestRW1C::is_test_bit_3());

        unsafe { TEST_PORT_RW1C_DATA = 1 };
        assert_eq!(unsafe { TEST_PORT_RW1C_DATA }, 1);
        assert!(MyTestRW1C::is_test_bit_0());

        unsafe { MyTestRW1C::set_test_bit_1(true) };
        assert_eq!(unsafe { TEST_PORT_RW1C_DATA }, 2);
        assert!(!MyTestRW1C::is_test_bit_0());

        unsafe { TEST_PORT_RW1C_DATA = 1 | (1 << 2) };
        assert_eq!(unsafe { TEST_PORT_RW1C_DATA }, 1 | (1 << 2));
        assert!(MyTestRW1C::is_test_bit_0());
        assert!(MyTestRW1C::is_test_bit_2());

        unsafe { MyTestRW1C::set_test_bit_1(true) };
        assert_eq!(unsafe { TEST_PORT_RW1C_DATA }, 2);
        assert!(!MyTestRW1C::is_test_bit_0());
        assert!(!MyTestRW1C::is_test_bit_2());

        unsafe { TEST_PORT_RW1C_DATA = 0 };
        assert_eq!(unsafe { TEST_PORT_RW1C_DATA }, 0);
        unsafe { MyTestRW1C::clear_test_bit_0() };
        assert!(MyTestRW1C::is_test_bit_0());

        unsafe { TEST_PORT_RW1C_DATA = 0 };
        assert_eq!(unsafe { TEST_PORT_RW1C_DATA }, 0);
        unsafe { MyTestRW1C::clear_test_bit_2() };
        assert!(MyTestRW1C::is_test_bit_2());

        unsafe { TEST_PORT_RW1C_DATA = 0 };
        assert_eq!(unsafe { TEST_PORT_RW1C_DATA }, 0);
        unsafe { MyTestRW1C::set_test_bit_3(true) };
        unsafe { MyTestRW1C::clear_test_bit_0() };
        assert!(MyTestRW1C::is_test_bit_3());
        assert!(MyTestRW1C::is_test_bit_0());
    }
}
