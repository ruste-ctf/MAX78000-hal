static mut DEBUG_OUTPUT_STREAM: DebugStream = DebugStream(None);

pub struct DebugStream(Option<&'static mut (dyn core::fmt::Write)>);

pub fn attach_debug(stream: &'static mut (dyn core::fmt::Write)) {
    unsafe {
        DEBUG_OUTPUT_STREAM.0 = Some(stream);
    }
}

impl core::fmt::Write for DebugStream {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        match self.0.as_mut() {
            Some(inner) => inner.write_str(s),
            None => Ok(()),
        }
    }

    fn write_char(&mut self, c: char) -> core::fmt::Result {
        match self.0.as_mut() {
            Some(inner) => inner.write_char(c),
            None => Ok(()),
        }
    }

    fn write_fmt(&mut self, args: core::fmt::Arguments<'_>) -> core::fmt::Result {
        match self.0.as_mut() {
            Some(inner) => inner.write_fmt(args),
            None => Ok(()),
        }
    }
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    unsafe { DEBUG_OUTPUT_STREAM.write_fmt(args).unwrap() };
}

#[macro_export]
macro_rules! debug_print {
    ($($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        $crate::debug::_print(format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! debug_println {
    () => {$crate::debug_print!("\n")};
    ($($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        $crate::debug::_print(format_args!($($arg)*));
        $crate::debug_print!("\n");
    }};
}
