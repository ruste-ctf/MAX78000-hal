pub mod registers;

/// # GPIO Select
/// Select a GPIO port.
#[repr(u8)]
pub enum GpioSelect {
    Gpio0 = 0,
    Gpio1 = 1,
    Gpio2 = 2,
}

impl Into<registers::PortOffset> for GpioSelect {
    fn into(self) -> usize {
        match self {
            GpioSelect::Gpio0 => registers::GPIO_0,
            GpioSelect::Gpio1 => registers::GPIO_1,
            GpioSelect::Gpio2 => registers::GPIO_2,
        }
    }
}

pub enum ResistorStrength {
    HighImpedance,
    WeakPullup,
    StrongPullup,
    WeakPulldown,
    StrongPulldown,
}

pub enum Mode {
    Input,
    Output,
}

pub enum VoltageSelect {
    VddIO,
    VddIOH,
}

static mut GLOBAL_OWNED_MASKS: [u32; 3] = [0_u32; 3];

fn is_owned(masks: &[u32; 3]) -> bool {
    for (i, port) in unsafe { GLOBAL_OWNED_MASKS }.iter().enumerate() {
        let mask_port = masks[i];

        if mask_port & port != 0 {
            return true;
        }
    }

    false
}

fn set_owned(masks: &[u32; 3]) {
    for (i, port) in unsafe { GLOBAL_OWNED_MASKS }.iter_mut().enumerate() {
        let mask_port = masks[i];

        *port &= mask_port;
    }
}

pub struct GpioPins([u32; 3]);

impl GpioPins {
    pub fn new_single_pin(port: GpioSelect, pin: u32) -> Option<Self> {
        let index = port as u8 as usize;
        let pin_mask = 1 << pin;

        let mut mask = [0u32; 3];
        mask[index] |= pin_mask;

        if is_owned(&mask) {
            None
        } else {
            set_owned(&mask);
            Some(GpioPins(mask))
        }
    }

    pub fn new_multi_pin_raw(mask: [u32; 3]) -> Option<Self> {
        if is_owned(&mask) {
            None
        } else {
            set_owned(&mask);
            Some(GpioPins(mask))
        }
    }

    pub fn configure_mode(&self, mode: Mode) {}
}
