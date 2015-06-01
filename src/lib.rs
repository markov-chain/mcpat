extern crate libc;
extern crate mcpat_sys as raw;

use std::fmt::{self, Display, Formatter};
use std::marker::PhantomData;
use std::path::Path;

/// An error.
#[derive(Clone, Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: Option<String>,
}

/// An error kind.
#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
    NoMemory,
    NotFound,
    Other,
}

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

macro_rules! raise(
    ($message:expr) => (raise!(Other, $message));
    ($kind:ident, $message:expr) => (
        return Err(::Error {
            kind: ::ErrorKind::$kind,
            message: Some(String::from($message)),
        })
    );
);

macro_rules! path_to_c_str(
    ($path:expr) => (match $path.to_str() {
        Some(path) => match ::std::ffi::CString::new(path) {
            Ok(string) => string.as_ptr(),
            Err(_) => raise!("failed to process a path"),
        },
        None => raise!("failed to process a path"),
    });
);

macro_rules! not_null(
    ($result:expr) => ({
        let pointer = $result;
        if pointer.is_null() {
            raise!(NoMemory, "cannot allocate memory");
        }
        pointer
    });
);

macro_rules! debug_not_null(
    ($result:expr) => ({
        let pointer = $result;
        debug_assert!(!pointer.is_null());
        pointer
    });
);

type Raw<T> = (*mut T, *mut raw::root_system);
type Phantom<'l, T> = PhantomData<(&'l T, &'l raw::root_system)>;

mod cache;
mod component;
mod core;
mod processor;
mod system;

pub use cache::{Cache, L3};
pub use component::{Component, Power};
pub use core::Core;
pub use processor::{Cores, L3s, Processor};
pub use system::System;

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self.message {
            Some(ref message) => Display::fmt(message, formatter),
            None => Display::fmt(&self.kind, formatter),
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        use ErrorKind::*;
        match *self {
            NoMemory => write!(formatter, "cannot allocate memory"),
            NotFound => write!(formatter, "a file does not exist"),
            _ => write!(formatter, "an unknown error occurred"),
        }
    }
}

/// Load a system from an XML file.
#[inline]
pub fn open<'l>(path: &Path) -> Result<System<'l>> {
    System::open(path)
}

/// Set a *global* flag controlling the optimization procedure. If true, apart
/// from other optimization goals, the optimization is performed for the target
/// clock rate. The switch is turned off by default.
pub fn set_optimzed_for_clock_rate(value: bool) {
    unsafe { raw::set_opt_for_clk(if value { 1 } else { 0 }) };
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn open() {
        let _system = {
            let path = PathBuf::from("foo");
            ::open(&path)
        };
    }
}
