/// # Bit Manipulation
/// A Simple trait to help with setting and un-setting bits in types.
pub trait BitManipulation {
    /// # Set Bit
    /// Set a single bit in the given type.
    fn set_bit<B>(&mut self, bit: B, set: bool) -> &mut Self
    where
        B: Into<u8>;

    /// # Get Bit
    /// Get a single bit in the given type.
    fn get_bit<B>(&self, bit: B) -> bool
    where
        B: Into<u8>;
}

/// # Bit Manipulation Impl
/// Implement this trait for many types.
/// FIXME: We should use something like PrimInt from the num-traits create
///        to provide a `impl<T: PrimInt> BitManipulation for T {}`.
macro_rules! bit_manipulation_impl {
    ($($t:ty)*) => ($(
     impl BitManipulation for $t {
        /// # Set Bit
        /// Set a single bit in the given type.
        fn set_bit<B>(&mut self, bit: B, set: bool) -> &mut Self
        where
            B: Into<u8>,
        {
            let bit: u8 = bit.into();
            let self_bits = (core::mem::size_of::<Self>() * 8) as u8;

            debug_assert!(
                bit <= self_bits,
                "Bit '{bit}' is larger then type's total bits of '{self_bits}'!"
            );

            if set {
                *self |= 1 << bit;
            } else {
                *self &= !(1 << bit);
            }

            self
        }

        /// # Get Bit
        /// Get a single bit in the given type.
        fn get_bit<B>(&self, bit: B) -> bool
        where
            B: Into<u8> {
            let bit: u8 = bit.into();
            let self_bits = (core::mem::size_of::<Self>() * 8) as u8;

            debug_assert!(
                bit <= self_bits,
                "Bit '{bit}' is larger then type's total bits of '{self_bits}'!"
            );

            *self & (1 << bit) != 0
        }
    }
    )*)
}

bit_manipulation_impl! { u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 usize isize }
