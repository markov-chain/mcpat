extern crate libc;
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
type Phantom<'l, T> = std::marker::PhantomData<(&'l T, &'l raw::root_system)>;

mod component;
mod core;
mod processor;
mod system;

pub use component::{Component, Power};
pub use core::Core;
pub use processor::{Cores, Processor};
pub use system::System;
