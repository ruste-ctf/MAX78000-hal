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

/*

# MSDK Pin Layout
{ MXC_GPIO0, (MXC_GPIO_PIN_10 | MXC_GPIO_PIN_11), MXC_GPIO_FUNC_ALT1,
XC_GPIO_PAD_NONE, MXC_GPIO_VSSEL_VDDIO, MXC_GPIO_DRVSTR_0 };
{ MXC_GPIO0, (MXC_GPIO_PIN_16 | MXC_GPIO_PIN_17), MXC_GPIO_FUNC_ALT1,
XC_GPIO_PAD_NONE, MXC_GPIO_VSSEL_VDDIO, MXC_GPIO_DRVSTR_0 };
{ MXC_GPIO0, (MXC_GPIO_PIN_30 | MXC_GPIO_PIN_31), MXC_GPIO_FUNC_ALT1,
MXC_GPIO_PAD_NONE, MXC_GPIO_VSSEL_VDDIO, MXC_GPIO_DRVSTR_0 };
*/

/// # I2C (n)
/// Get the i2c GPIO pins for port n.
pub fn i2c_n(port: usize) -> Option<[GpioPin; 2]> {
    let pins = match port {
        0 => (10, 11),
        1 => (16, 17),
        2 => (30, 31),

        _ => panic!("Cannot have a port higher then 2"),
    };

    let gpio_0 = GpioPin::new(super::GpioSelect::Gpio0, pins.0)?;
    let gpio_1 = GpioPin::new(super::GpioSelect::Gpio0, pins.1)?;

    gpio_0.configure_input(super::ResistorStrength::None, super::PinFunction::AF1);
    gpio_1.configure_input(super::ResistorStrength::None, super::PinFunction::AF1);

    Some([gpio_0, gpio_1])
}

// UART 0 P0_0 Rx P0_1 Tx
// UART 1 P0_12 Rx P0_13 Tx
// UART 2 P1_0 Rx P1_1 Tx
// LPUART P2_6 Rx P2_7 Tx

/// # UART (n)
/// Get the UART GPIO pins for port n.
pub fn uart_n(port: usize) -> Option<[GpioPin; 2]> {
    // (Rx, Tx, GPIO_port)
    let pins = match port {
        0 => (0, 1, super::GpioSelect::Gpio0),
        1 => (12, 13, super::GpioSelect::Gpio0),
        2 => (0, 1, super::GpioSelect::Gpio1),
        3 => (6, 7, super::GpioSelect::Gpio2),

        _ => panic!("Cannot have a port higher than 3"),
    };

    let gpio_rx = GpioPin::new(pins.2, pins.0)?;
    let gpio_tx = GpioPin::new(pins.2, pins.1)?;

    gpio_rx.configure_input(super::ResistorStrength::None, super::PinFunction::AF1);
    gpio_tx.configure_input(super::ResistorStrength::None, super::PinFunction::AF1);

    Some([gpio_rx, gpio_tx])
}
