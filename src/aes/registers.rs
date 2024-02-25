use crate::memory_map::mmio;
use hal_macros::RW;
use hal_macros_derive::make_device;

/// # AES Register Offsets
/// See Max 78000 User Guide Page 360, Table 24-3.
mod rro {
    /// # AES Control Register
    pub const AES_CTRL: usize = 0x0000;
    /// # AES Status Register
    pub const AES_STATUS: usize = 0x0004;
    /// # AES Interrupt Flag Register
    pub const AES_INTFL: usize = 0x0008;
    /// # AES Interrupt Enable Flag Register
    pub const AES_INTEN: usize = 0x000C;
    /// # AES Data FIFO Register
    pub const AES_FIFO: usize = 0x0010;
}

make_device! {
    device_ports(mmio::AES);

    /// Encryption Type. See Page 360-361, Table 24-4.
    #[bit(8..=9, RW, rro::AES_CTRL)]
    encryption_type,

    /// Encryption Key Size. See Page 360-361, Table 24-4.
    #[bit(6..=7, RW, rro::AES_CTRL)]
    encryption_key_size,

    /// Flush Data Output FIFO. See Page 360-361, Table 24-4.
    #[bit(5, RW1O, rro::AES_CTRL)]
    flush_data_output_fifo,

    /// Flush Data Input FIFO. See Page 360-361, Table 24-4.
    #[bit(4, RW1O, rro::AES_CTRL)]
    flush_data_input_fifo,

    /// Start AES Calculation. See Page 360-361, Table 24-4.
    #[bit(3, RW1O, rro::AES_CTRL)]
    start_aes_calculation,

    /// DMA Request To Write Data Input FIFO. See Page 360-361, Table 24-4.
    #[bit(2, RW, rro::AES_CTRL)]
    dma_request_to_write_data_input_fifo,

    /// DMA Request To Read Data Output FIFO. See Page 360-361, Table 24-4.
    #[bit(1, RW, rro::AES_CTRL)]
    dma_request_to_read_data_output_fifo,

    /// AES Enable. See Page 360-361, Table 24-4.
    #[bit(0, RW, rro::AES_CTRL)]
    aes_enable,

    /// AES Control Register. See Page 360-361, Table 24-4.
    #[bit(0..=31, RW, rro::AES_CTRL)]
    aes_control_register,

    /// Output FIFO Full. See Page 361, Table 24-5.
    #[bit(4, RO, rro::AES_STATUS)]
    output_fifo_full,

    /// Output FIFO Empty. See Page 361, Table 24-5.
    #[bit(3, RO, rro::AES_STATUS)]
    output_fifo_empty,

    /// Input FIFO Full. See Page 361, Table 24-5.
    #[bit(2, RO, rro::AES_STATUS)]
    input_fifo_full,

    /// Input FIFO Empty. See Page 361, Table 24-5.
    #[bit(1, RO, rro::AES_STATUS)]
    input_fifo_empty,

    /// AES Busy. See Page 361, Table 24-5.
    #[bit(0, RO, rro::AES_STATUS)]
    aes_busy,

    /// Data Output FIFO Overrun Event Interrupt. See Page 361-362, Table 24-6.
    #[bit(3, RW1C, rro::AES_INTFL)]
    data_output_fifo_overrun_event_interrupt,

    /// Key Zero Event Interrupt. See Page 361-362, Table 24-6.
    #[bit(2, RW1C, rro::AES_INTFL)]
    key_zero_event_interrupt,

    /// Key Change Event Interrupt. See Page 361-362, Table 24-6.
    #[bit(1, RW1C, rro::AES_INTFL)]
    key_change_event_interrupt,

    /// Calculation Done Event Interrupt. See Page 361-362, Table 24-6.
    #[bit(0, RW1C, rro::AES_INTFL)]
    calculation_done_event_interrupt,

    /// Data Output FIFO Overrun Event Interrupt Enable. See Page 362, Table 24-7.
    #[bit(3, RW1C, rro::AES_INTEN)]
    date_output_fifo_overrun_event_interrupt_enable,

    /// Key Zero Event Interrupt Enable. See Page 362, Table 24-7.
    #[bit(2, RW1C, rro::AES_INTEN)]
    key_zero_event_interrupt_enable,

    /// Key Change Event Interrupt Enable. See Page 362, Table 24-7.
    #[bit(1, RW1C, rro::AES_INTEN)]
    key_change_event_interrupt_enable,

    /// Calculation Done Event Interrupt Enable. See Page 362, Table 24-7.
    #[bit(0, RW1C, rro::AES_INTEN)]
    calculation_done_event_interrupt_enable,

    /// AES FIFO. See Page 362, Table 24-8.
    #[bit(0..=31, RW, rro::AES_FIFO)]
    aes_fifo,
}
