use libc::c_int;
use raw;
use hiredis;

use {ErrorKind, Result};

/// Activate caching of optimization results using a Redis server.
pub fn activate(host: &str, port: usize) -> Result<()> {
    match unsafe { raw::cache_activate(str_to_c_str!(host), port as c_int) } {
        result if result != 0 => raise!(hiredis::ErrorKind::from(result as isize)),
        _ => Ok(()),
    }
}

/// Deactivate caching of optimization results.
#[inline]
pub fn deactivate() {
    unsafe { raw::cache_deactivate() };
}
