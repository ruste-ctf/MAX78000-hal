use super::GpioPin;

/// # Led 0
/// Pre-configured pin for led-0.
pub fn led0() -> Option<GpioPin> {
    let pin = GpioPin::new(super::GpioSelect::Gpio2, 0)?;

    pin.configure_output(
        super::OutputDriveStrength::Strength0(super::VoltageSelect::VddIOH),
        super::PinFunction::IO,
    );
    Some(pin)
}

/// # Led 1
/// Pre-configured pin for led-1.
pub fn led1() -> Option<GpioPin> {
    let pin = GpioPin::new(super::GpioSelect::Gpio2, 1)?;

    pin.configure_output(
        super::OutputDriveStrength::Strength0(super::VoltageSelect::VddIOH),
        super::PinFunction::IO,
    );
    Some(pin)
}
