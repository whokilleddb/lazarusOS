/// This code defines the `print!()` and `println()` functions so as to
/// allow printing information using UEFI stdout
use core::fmt::{Result, Write};

/// A dummy screen writing structure we can implement `Write` on
pub struct ScreenOutWriter;

impl Write for ScreenOutWriter{
    fn write_str(&mut self, string: &str) -> Result {
        crate::efi::output_string(string);
        Ok(())
    }
}


/// A dummy screen writing structure we can implement `Write` on for stderr
pub struct ScreenErrWriter;

impl Write for ScreenErrWriter{
    fn write_str(&mut self, string: &str) -> Result {
        crate::efi::stderr_string(string);
        Ok(())
    }
}



/// Standard Rust `print!()`
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        // We use a hardcoded full path because we are using this in a macro
        // Hence it will be called from a lot of different paths
    let _ = <$crate::print::ScreenOutWriter as core::fmt::Write>::write_fmt(
            &mut $crate::print::ScreenOutWriter,
            format_args!($($arg)*)
        );
    }
}


/// `eprint!()` implementation
#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {
        // We use a hardcoded full path because we are using this in a macro
        // Hence it will be called from a lot of different paths
    let _ = <$crate::print::ScreenErrWriter as core::fmt::Write>::write_fmt(
            &mut $crate::print::ScreenErrWriter,
            format_args!($($arg)*)
        );
    }
}


