//! Caching of optimization results.

use libc::c_int;
use raw;
use hiredis;

use {ErrorKind, Result};

/// Turn on caching.
///
/// The caching is facilitated by a Redis server.
pub fn activate(host: &str, port: usize) -> Result<()> {
    match unsafe { raw::cache_activate(str_to_c_str!(host), port as c_int) } {
        result if result != 0 => raise!(hiredis::ErrorKind::from(result as isize)),
        _ => Ok(()),
    }
}

/// Turn off caching.
#[inline]
pub fn deactivate() {
    unsafe { raw::cache_deactivate() };
}
