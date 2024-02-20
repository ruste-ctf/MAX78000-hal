use crate::bits::BitManipulation;

use super::GpioPin;

static mut PINS_OWNED: [u32; 4] = [0_u32; 4];

fn pin_mode(pin: &GpioPin) -> (usize, usize) {
    (pin.get_port() as u8 as usize, pin.get_pin())
}

pub fn is_owned(pin: &GpioPin) -> bool {
    let (port, pin) = pin_mode(pin);
    unsafe { PINS_OWNED[port].get_bit(pin as u8) }
}

pub fn set_owned(pin: &GpioPin) {
    let (port, pin) = pin_mode(pin);
    unsafe { PINS_OWNED[port].set_bit(pin as u8, true) };
}

pub fn disown_pin(pin: &GpioPin) {
    let (port, pin) = pin_mode(pin);
    unsafe { PINS_OWNED[port].set_bit(pin as u8, false) };
}
