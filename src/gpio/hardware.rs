use super::GpioPin;

/// # Led 1 RED
/// Pre-configured pin for led-1.
pub fn led_red() -> Option<GpioPin> {
    let pin = GpioPin::new(super::GpioSelect::Gpio2, 0)?;

    pin.configure_output(
        super::OutputDriveStrength::Strength0(super::VoltageSelect::VddIOH),
        super::PinFunction::IO,
    );
    Some(pin)
}

/// # Led 1 GREEN
/// Pre-configured pin for led-1.
pub fn led_green() -> Option<GpioPin> {
    let pin = GpioPin::new(super::GpioSelect::Gpio2, 1)?;

    pin.configure_output(
        super::OutputDriveStrength::Strength0(super::VoltageSelect::VddIOH),
        super::PinFunction::IO,
    );
    Some(pin)
}

/// # Led 1 BLUE
/// Pre-configured pin for led-1.
pub fn led_blue() -> Option<GpioPin> {
    let pin = GpioPin::new(super::GpioSelect::Gpio2, 2)?;

    pin.configure_output(
        super::OutputDriveStrength::Strength0(super::VoltageSelect::VddIOH),
        super::PinFunction::IO,
    );
    Some(pin)
}
