use hal_macros::{VolatileRead, RW};
use hal_macros_derive::make_device;

const FIRST_4_BYTES: usize = 0;
const SECOND_4_BYTES: usize = 1;
const THREE_4_BYTES: usize = 2;
const FOUR_4_BYTES: usize = 3;

make_device! {
    // No need for device ports when testing, since
    // the macro will disable its checking.
    device_ports();

    #[bit(0, RW, FIRST_4_BYTES)]
    bit_0,

    #[bit(1, RW, FIRST_4_BYTES)]
    bit_1,

    #[bit(2, RW, FIRST_4_BYTES)]
    bit_2,

    #[bit(3, RW, FIRST_4_BYTES)]
    bit_3,

    #[bit(4, RW, FIRST_4_BYTES)]
    bit_4,

    #[bit(5, RW, FIRST_4_BYTES)]
    bit_5,

    #[bit(6, RW, FIRST_4_BYTES)]
    bit_6,

    #[bit(7, RW, FIRST_4_BYTES)]
    bit_7,

    #[bit(8, RW, FIRST_4_BYTES)]
    bit_8,

    #[bit(9, RW, FIRST_4_BYTES)]
    bit_9,

    #[bit(10, RW, FIRST_4_BYTES)]
    bit_10,

    #[bit(11, RW, FIRST_4_BYTES)]
    bit_11,

    #[bit(12, RW, FIRST_4_BYTES)]
    bit_12,

    #[bit(13, RW, FIRST_4_BYTES)]
    bit_13,

    #[bit(14, RW, FIRST_4_BYTES)]
    bit_14,

    #[bit(15, RW, FIRST_4_BYTES)]
    bit_15,

    #[bit(16, RW, FIRST_4_BYTES)]
    bit_16,

    #[bit(17, RW, FIRST_4_BYTES)]
    bit_17,

    #[bit(18, RW, FIRST_4_BYTES)]
    bit_18,

    #[bit(19, RW, FIRST_4_BYTES)]
    bit_19,

    #[bit(20, RW, FIRST_4_BYTES)]
    bit_20,

    #[bit(21, RW, FIRST_4_BYTES)]
    bit_21,

    #[bit(22, RW, FIRST_4_BYTES)]
    bit_22,

    #[bit(23, RW, FIRST_4_BYTES)]
    bit_23,

    #[bit(24, RW, FIRST_4_BYTES)]
    bit_24,

    #[bit(25, RW, FIRST_4_BYTES)]
    bit_25,

    #[bit(26, RW, FIRST_4_BYTES)]
    bit_26,

    #[bit(27, RW, FIRST_4_BYTES)]
    bit_27,

    #[bit(28, RW, FIRST_4_BYTES)]
    bit_28,

    #[bit(29, RW, FIRST_4_BYTES)]
    bit_29,

    #[bit(30, RW, FIRST_4_BYTES)]
    bit_30,

    #[bit(31, RW, FIRST_4_BYTES)]
    bit_31,

    #[bit(0..=31, RW, SECOND_4_BYTES)]
    range_0,

    #[bit(0..=15, RW, THREE_4_BYTES)]
    range_1,

    #[bit(16..=31, RW, THREE_4_BYTES)]
    range_2,

    #[bit(0..=7, RW, FOUR_4_BYTES)]
    range_3,

    #[bit(8..=15, RW, FOUR_4_BYTES)]
    range_4,

    #[bit(16..=23, RW, FOUR_4_BYTES)]
    range_5,

    #[bit(24..=31, RW, FOUR_4_BYTES)]
    range_6,
}

macro_rules! single_bit_test {
    ($bit:literal, $name:ident, $set:ident, $get:ident) => {
        #[test]
        fn $name() {
            #[allow(unused)]
            let mut fake_device_storage = [0u32; 4];
            #[allow(unused)]
            let mut reg =
                Registers::new(fake_device_storage.as_mut() as *mut [u32] as *mut u32 as usize);

            unsafe { reg.$set(false) };
            assert_eq!(reg.first_4_bytes.read(), 0b0);
            assert_eq!(reg.$get(), false);

            unsafe { reg.$set(true) };
            assert_eq!(reg.first_4_bytes.read(), 0b1 << $bit);
            assert_eq!(reg.$get(), true);

            unsafe { reg.$set(false) };
            assert_eq!(reg.first_4_bytes.read(), 0b0);
            assert_eq!(reg.$get(), false);

            unsafe { reg.$set(true) };
            assert_eq!(reg.first_4_bytes.read(), 0b1 << $bit);
            assert_eq!(reg.$get(), true);
        }
    };
}

macro_rules! range_bit_test {
    ($bit_shift:literal, $name:ident) => {
        #[test]
        fn $name() {
            #[allow(unused)]
            let mut fake_device_storage = [0u32; 4];
            #[allow(unused)]
            let mut reg =
                Registers::new(fake_device_storage.as_mut() as *mut [u32] as *mut u32 as usize);
            for i in 0..=(u16::MAX as u32) {
                unsafe { reg.set_range_0(i << $bit_shift) };
                assert_eq!(reg.second_4_bytes.read(), i << $bit_shift, "acct value");
                assert_eq!(reg.get_range_0(), i << $bit_shift, "read value");
            }
        }
    };
}

#[test]
fn test_entire_u32max() {
    #[allow(unused)]
    let mut fake_device_storage = [0u32; 4];
    #[allow(unused)]
    let mut reg = Registers::new(fake_device_storage.as_mut() as *mut [u32] as *mut u32 as usize);
    unsafe { reg.set_range_0(u32::MAX) };
    assert_eq!(reg.second_4_bytes.read(), u32::MAX, "acct value");
    assert_eq!(reg.get_range_0(), u32::MAX, "read value");
    unsafe { reg.set_range_0(0) };
    assert_eq!(reg.second_4_bytes.read(), 0, "acct value");
    assert_eq!(reg.get_range_0(), 0, "read value");
}

// Test the full range, but since its u32 it can take some time
// so we make it multithreaded here.
range_bit_test!(0, range_bit_test_0);
range_bit_test!(1, range_bit_test_1);
range_bit_test!(2, range_bit_test_2);
range_bit_test!(3, range_bit_test_3);
range_bit_test!(4, range_bit_test_4);
range_bit_test!(5, range_bit_test_5);
range_bit_test!(6, range_bit_test_6);
range_bit_test!(7, range_bit_test_7);
range_bit_test!(8, range_bit_test_8);
range_bit_test!(9, range_bit_test_9);
range_bit_test!(10, range_bit_test_10);
range_bit_test!(11, range_bit_test_11);
range_bit_test!(12, range_bit_test_12);
range_bit_test!(13, range_bit_test_13);
range_bit_test!(14, range_bit_test_14);
range_bit_test!(15, range_bit_test_15);
range_bit_test!(16, range_bit_test_16);

#[test]
fn test_range_u16() {
    #[allow(unused)]
    let mut fake_device_storage = [0u32; 4];
    #[allow(unused)]
    let mut reg = Registers::new(fake_device_storage.as_mut() as *mut [u32] as *mut u32 as usize);
    for value in 0..u16::MAX {
        let first_test = value as u32;
        let second_test = (value as u32) << 16;
        let expected_value = first_test | second_test;

        unsafe {
            reg.set_range_1(value);
            assert_eq!(reg.get_range_1(), value, "1");
            reg.set_range_2(value);
            assert_eq!(reg.get_range_2(), value, "2");
            reg.set_range_1(0);
            assert_eq!(reg.get_range_1(), 0, "3");
            reg.set_range_2(0);
            assert_eq!(reg.get_range_2(), 0, "4");
            reg.set_range_1(value);
            assert_eq!(reg.three_4_bytes.read(), first_test, "5: {value}");
            reg.set_range_1(0);
            reg.set_range_2(value);
            assert_eq!(reg.three_4_bytes.read(), second_test, "6: {value}");
            reg.set_range_2(0);
            reg.set_range_1(value);
            reg.set_range_2(value);
            assert_eq!(reg.three_4_bytes.read(), expected_value, "7: {value}");
        }
    }
}

#[test]
fn test_range_u8() {
    #[allow(unused)]
    let mut fake_device_storage = [0u32; 4];
    #[allow(unused)]
    let mut reg = Registers::new(fake_device_storage.as_mut() as *mut [u32] as *mut u32 as usize);
    for value in 0..u8::MAX {
        let first_test = value as u32;
        let second_test = (value as u32) << 8;
        let three_test = (value as u32) << 16;
        let four_test = (value as u32) << 24;

        let expected = first_test | second_test | three_test | four_test;

        unsafe {
            reg.set_range_3(value);
            assert_eq!(reg.get_range_3(), value, "1");
            reg.set_range_4(value);
            assert_eq!(reg.get_range_4(), value, "2");
            reg.set_range_5(value);
            assert_eq!(reg.get_range_5(), value, "3");
            reg.set_range_6(value);
            assert_eq!(reg.get_range_6(), value, "4");

            reg.set_range_3(0);
            assert_eq!(reg.get_range_3(), 0, "5");
            reg.set_range_4(0);
            assert_eq!(reg.get_range_4(), 0, "6");
            reg.set_range_5(0);
            assert_eq!(reg.get_range_5(), 0, "7");
            reg.set_range_6(0);
            assert_eq!(reg.get_range_6(), 0, "8");

            reg.set_range_3(value);
            assert_eq!(reg.four_4_bytes.read(), first_test, "9: {value}");
            reg.set_range_3(0);
            reg.set_range_4(value);
            assert_eq!(reg.four_4_bytes.read(), second_test, "10: {value}");
            reg.set_range_4(0);
            reg.set_range_5(value);
            assert_eq!(reg.four_4_bytes.read(), three_test, "11: {value}");
            reg.set_range_5(0);
            reg.set_range_6(value);
            assert_eq!(reg.four_4_bytes.read(), four_test, "12: {value}");
            reg.set_range_6(0);

            reg.set_range_3(value);
            reg.set_range_4(value);
            reg.set_range_5(value);
            reg.set_range_6(value);
            assert_eq!(reg.four_4_bytes.read(), expected, "13: {value}");
        }
    }
}

#[test]
fn test_making_device() {
    #[allow(unused)]
    let mut fake_device_storage = [0u32; 4];
    #[allow(unused)]
    let mut reg = Registers::new(fake_device_storage.as_mut() as *mut [u32] as *mut u32 as usize);
}

single_bit_test!(0, test_bit_0, set_bit_0, get_bit_0);
single_bit_test!(1, test_bit_1, set_bit_1, get_bit_1);
single_bit_test!(2, test_bit_2, set_bit_2, get_bit_2);
single_bit_test!(3, test_bit_3, set_bit_3, get_bit_3);
single_bit_test!(4, test_bit_4, set_bit_4, get_bit_4);
single_bit_test!(5, test_bit_5, set_bit_5, get_bit_5);
single_bit_test!(6, test_bit_6, set_bit_6, get_bit_6);
single_bit_test!(7, test_bit_7, set_bit_7, get_bit_7);
single_bit_test!(8, test_bit_8, set_bit_8, get_bit_8);
single_bit_test!(9, test_bit_9, set_bit_9, get_bit_9);
single_bit_test!(10, test_bit_10, set_bit_10, get_bit_10);
single_bit_test!(11, test_bit_11, set_bit_11, get_bit_11);
single_bit_test!(12, test_bit_12, set_bit_12, get_bit_12);
single_bit_test!(13, test_bit_13, set_bit_13, get_bit_13);
single_bit_test!(14, test_bit_14, set_bit_14, get_bit_14);
single_bit_test!(15, test_bit_15, set_bit_15, get_bit_15);
single_bit_test!(16, test_bit_16, set_bit_16, get_bit_16);
single_bit_test!(17, test_bit_17, set_bit_17, get_bit_17);
single_bit_test!(18, test_bit_18, set_bit_18, get_bit_18);
single_bit_test!(19, test_bit_19, set_bit_19, get_bit_19);
single_bit_test!(20, test_bit_20, set_bit_20, get_bit_20);
single_bit_test!(21, test_bit_21, set_bit_21, get_bit_21);
single_bit_test!(22, test_bit_22, set_bit_22, get_bit_22);
single_bit_test!(23, test_bit_23, set_bit_23, get_bit_23);
single_bit_test!(24, test_bit_24, set_bit_24, get_bit_24);
single_bit_test!(25, test_bit_25, set_bit_25, get_bit_25);
single_bit_test!(26, test_bit_26, set_bit_26, get_bit_26);
single_bit_test!(27, test_bit_27, set_bit_27, get_bit_27);
single_bit_test!(28, test_bit_28, set_bit_28, get_bit_28);
single_bit_test!(29, test_bit_29, set_bit_29, get_bit_29);
single_bit_test!(30, test_bit_30, set_bit_30, get_bit_30);
single_bit_test!(31, test_bit_31, set_bit_31, get_bit_31);
