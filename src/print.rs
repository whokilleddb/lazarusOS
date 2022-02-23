/// This code defines the `print!()` and `println()` functions so as to
/// allow printing information using UEFI stdout
use core::fmt::{Result, Write};

/// A dummy screen writing structure we can implement `Write` on
pub struct ScreenWriter;

impl Write for ScreenWriter{
    fn write_str(&mut self, string: &str) -> Result {
        crate::efi::output_string(string);
        Ok(())
    }
}

/// Standard Rust `print!()`
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        <$crate::print::ScreenWriter as core::fmt::Write>::write_fmt(
            &mut $crate::print::ScreenWriter,
            format_args!($($arg)*)
        );
    }
}


// Standard Rust `println!()`
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => {
        <$crate::print::ScreenWriter as core::fmt::Write>::write_fmt(
            &mut $crate::print::ScreenWriter,
            format_args!($($arg)*)
        );
    }
}


