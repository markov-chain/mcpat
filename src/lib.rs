extern crate mcpat_sys as raw;

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

mod processor;
mod system;

pub use processor::Processor;
pub use system::System;
