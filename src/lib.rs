extern crate libc;
extern crate mcpat_sys as raw;

#[cfg(feature = "caching")]
extern crate hiredis;

use std::fmt::{self, Display, Formatter};
use std::marker::PhantomData;
use std::path::Path;

/// An error.
#[derive(Clone, Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: Option<String>,
}

/// The class that an error belongs to.
#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
    OutOfMemory,
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
            message: Some($message.to_string()),
        })
    );
);

macro_rules! str_to_c_str(
    ($string:expr) => (match ::std::ffi::CString::new($string) {
        Ok(string) => string.as_ptr(),
        Err(_) => raise!("failed to process a string"),
    });
);

macro_rules! path_to_c_str(
    ($path:expr) => (match $path.to_str() {
        Some(path) => str_to_c_str!(path),
        None => raise!("failed to process a path"),
    });
);

macro_rules! not_null(
    ($result:expr) => ({
        let pointer = $result;
        if pointer.is_null() {
            raise!(OutOfMemory, "cannot allocate memory");
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
type Phantom<'l, T> = PhantomData<(T, &'l raw::root_system)>;

mod cache;
mod component;
mod core;
mod processor;
mod system;

#[cfg(feature = "caching")]
pub mod caching;

pub use cache::{Cache, L3};
pub use component::Component;
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
            OutOfMemory => write!(formatter, "out of memory"),
            NotFound => write!(formatter, "file not found"),
            Other => write!(formatter, "other"),
        }
    }
}

/// Load a system from a file.
#[inline]
pub fn open(path: &Path) -> Result<System> {
    System::open(path)
}

/// Set a flag controlling the optimization procedure.
///
/// If the flag is set to true, apart from other optimization goals, the
/// optimization is performed for the target clock rate. The switch is turned
/// off by default.
#[inline]
pub fn optimze_for_clock_rate(value: bool) {
    unsafe { raw::opt_for_clk_set(if value { 1 } else { 0 }) };
}
