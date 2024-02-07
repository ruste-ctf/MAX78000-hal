/// # Error Kind
/// Errors for states internal to the MAX78000 microcontroller.
#[non_exhaustive]
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum ErrorKind {
    /// # Null Ptr
    /// Some critical ptr was found to be null, and could not continue.
    NullPtr,
    /// # No Device
    /// The device that was attempted to be reached is not valid
    /// or has not been found.
    NoDevice,
    /// # Bad Pram
    /// The provided argument was invalid, and operation could not continue.
    BadParam,
    /// # Invalid
    /// The operation or value is invalid and will not be accepted.
    Invalid,
    /// # Uninitialized
    /// Value or module is not initialized.
    Uninitialized,
    /// # Busy
    /// The requested operation could no be serviced due to hardware
    /// currently being busy.
    Busy,
    /// # Bad State
    /// Current state is not allowed in the current state.
    BadState,
    /// # Unknown
    /// The error was unknown.
    Unknown,
    /// # Com Error
    /// Communication had an error, and could not service request.
    ComError,
    /// # Time Out
    /// The requested operation took too long to complete, and was canceled.
    TimeOut,
    /// # No Response
    /// The controller was expecting a response, but didn't receive anything.
    NoResponse,
    /// # Overflow
    /// The buffer could not fit anymore data, and resulted in an unexpected
    /// overflow of data.
    Overflow,
    /// # Underflow
    /// The operation had an unexpected underflow.
    Underflow,
    /// # None Available
    /// Data or resource was not available at this time.
    NoneAvailable,
    /// # Shutdown
    /// The operation was shutdown, and could no longer be serviced.
    Shutdown,
    /// # Abort
    /// The operation was aborted.
    Abort,
    /// # Not Supported
    /// The requested operation is not supported at this time.
    NotSupported,
    /// # Fail
    /// The requested operation failed unexpectedly.
    Fail,
}

#[cfg(debug_assertions)]
impl core::fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Since Flash size is so limited, we will only include a few chars
        // to indecate the error message.
        f.write_str(match self {
            Self::NullPtr => "NP",
            Self::NoDevice => "ND",
            Self::BadParam => "BP",
            Self::Invalid => "I",
            Self::Uninitialized => "UI",
            Self::Busy => "B",
            Self::BadState => "BS",
            Self::Unknown => "UK",
            Self::ComError => "CE",
            Self::TimeOut => "TO",
            Self::NoResponse => "NR",
            Self::Overflow => "O",
            Self::Underflow => "U",
            Self::NoneAvailable => "NA",
            Self::Shutdown => "SH",
            Self::Abort => "AB",
            Self::NotSupported => "NS",
            Self::Fail => "F",
        })
    }
}

#[cfg(not(debug_assertions))]
impl core::fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Since we do not want to show debug msg, we will just say error.
        f.write_str("ERR")
    }
}

/// # Result
/// Result type that includes the `ErrorKind` enum as error.
pub type Result<T> = core::result::Result<T, ErrorKind>;
