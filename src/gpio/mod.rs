pub mod hardware;
mod ownership;
pub mod registers;

/// # GPIO Select
/// Select a GPIO port.
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum GpioSelect {
    Gpio0 = 0,
    Gpio1 = 1,
    Gpio2 = 2,
}

#[allow(clippy::from_over_into)]
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
    None,
    WeakPullup,
    StrongPullup,
    WeakPulldown,
    StrongPulldown,
}

pub enum VoltageSelect {
    VddIO,
    VddIOH,
}

pub enum OutputDriveStrength {
    Strength0(VoltageSelect),
    Strength1(VoltageSelect),
    Strength2(VoltageSelect),
}

pub enum PinFunction {
    AF1,
    AF2,
    IO,
}

pub struct GpioPin(u8);

impl GpioPin {
    pub fn new(port: GpioSelect, pin: usize) -> Option<Self> {
        let port_number = port as u8;
        let pin_number = pin as u8;
        let combined_number = (port_number << 6) | (pin_number & 0x3F);

        let gpio = Self(combined_number);

        if ownership::is_owned(&gpio) {
            None
        } else {
            ownership::set_owned(&gpio);
            unsafe { gpio.set_bit(registers::rro::GPIO_INEN, true) };
            Some(gpio)
        }
    }

    #[inline]
    pub fn get_port(&self) -> GpioSelect {
        let number = self.0 >> 6;

        match number {
            0 => GpioSelect::Gpio0,
            1 => GpioSelect::Gpio1,
            2 => GpioSelect::Gpio2,
            _ => unreachable!("Should not be possible to set a value higher then 2."),
        }
    }

    #[inline]
    pub fn get_pin(&self) -> usize {
        (self.0 & 0x3F) as usize
    }

    unsafe fn set_bit(&self, reg_offset: registers::BaseOffset, flag: bool) {
        if flag {
            registers::enable_bit(reg_offset, self.get_port().into(), self.get_pin())
        } else {
            registers::disable_bit(reg_offset, self.get_port().into(), self.get_pin())
        }
    }

    fn switch_function<Func>(&self, function: PinFunction, func: Func)
    where
        Func: FnOnce(),
    {
        unsafe {
            self.set_bit(registers::rro::GPIO_EN0_SET, true);

            func();

            match function {
                PinFunction::AF1 => {
                    // Alt Functions need EN0 set before ALT1 can be entered
                    self.set_bit(registers::rro::GPIO_EN0_CLR, true);
                    self.set_bit(registers::rro::GPIO_EN1_CLR, true);
                    self.set_bit(registers::rro::GPIO_EN2_CLR, true);
                    true
                }
                PinFunction::AF2 => {
                    // Alt Functions need EN0 set before ALT2 can be entered
                    self.set_bit(registers::rro::GPIO_EN0_CLR, true);
                    self.set_bit(registers::rro::GPIO_EN1_SET, true);
                    self.set_bit(registers::rro::GPIO_EN2_CLR, true);
                    true
                }
                PinFunction::IO => {
                    // The different IO modes do not change pin behavior
                    self.set_bit(registers::rro::GPIO_EN0_SET, true);
                    self.set_bit(registers::rro::GPIO_EN1_CLR, true);
                    self.set_bit(registers::rro::GPIO_EN2_CLR, true);
                    false
                }
            };
        }
    }

    pub fn set_output(&self, output_enable: bool) {
        unsafe { self.set_bit(registers::rro::GPIO_OUT, output_enable) };
    }

    pub fn get_input(&self) -> bool {
        unsafe {
            (registers::read_gpio(registers::rro::GPIO_IN, self.get_port().into())
                & (1 << self.get_pin()))
                != 0
        }
    }

    pub fn configure_input(&self, res: ResistorStrength, function: PinFunction) {
        let (pad_ctrl1, pad_ctrl0, pull_ctrl, power_ctrl) = match res {
            ResistorStrength::None => (false, false, false, false),
            ResistorStrength::WeakPullup => (false, true, false, false),
            ResistorStrength::StrongPullup => (false, true, true, false),
            ResistorStrength::WeakPulldown => (true, false, false, true),
            ResistorStrength::StrongPulldown => (true, false, true, true),
        };

        self.switch_function(function, || unsafe {
            self.set_bit(registers::rro::GPIO_PADCTRL0, pad_ctrl0);
            self.set_bit(registers::rro::GPIO_PADCTRL1, pad_ctrl1);
            self.set_bit(registers::rro::GPIO_PS, pull_ctrl);
            self.set_bit(registers::rro::GPIO_VSSEL, power_ctrl);
            self.set_bit(registers::rro::GPIO_OUTEN_CLR, true);
            self.set_bit(registers::rro::GPIO_INEN, true);
        });
    }

    pub fn configure_output(&self, strength: OutputDriveStrength, function: PinFunction) {
        let (ds_ctrl1, ds_ctrl0, v_sel) = match strength {
            OutputDriveStrength::Strength0(setting) => (false, false, setting),
            OutputDriveStrength::Strength1(setting) => (false, true, setting),
            OutputDriveStrength::Strength2(setting) => (true, true, setting),
        };

        let v_sel = match v_sel {
            VoltageSelect::VddIO => false,
            VoltageSelect::VddIOH => true,
        };

        self.switch_function(function, || unsafe {
            self.set_bit(registers::rro::GPIO_DS1, ds_ctrl1);
            self.set_bit(registers::rro::GPIO_DS0, ds_ctrl0);
            self.set_bit(registers::rro::GPIO_VSSEL, v_sel);
            self.set_bit(registers::rro::GPIO_INEN, false);
            self.set_bit(registers::rro::GPIO_OUTEN_SET, true);
        });
    }

    pub unsafe fn raw_output_enable(&self) {
        self.set_bit(registers::rro::GPIO_OUTEN_SET, true);
    }

    pub unsafe fn raw_input_enable(&self) {
        self.set_bit(registers::rro::GPIO_INEN, true);
    }
}

impl Drop for GpioPin {
    fn drop(&mut self) {
        ownership::disown_pin(self);
    }
}
