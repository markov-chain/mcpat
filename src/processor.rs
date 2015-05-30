use raw;
use std::marker::PhantomData;

use {Result, System};

/// A processor.
pub struct Processor<'l> {
    raw: *mut raw::Processor,
    phantom: PhantomData<&'l raw::Processor>,
}

impl<'l> Processor<'l> {
    /// Compute the processor corresponding to a system.
    pub fn new(system: &System<'l>) -> Result<Processor<'l>> {
        let raw = unsafe { raw::new_Processor(::system::as_raw(system)) };
        if raw.is_null() {
            raise!(NoMemory, "cannot allocate memory for a processor");
        }
        Ok(Processor { raw: raw, phantom: PhantomData })
    }
}

impl<'l> Drop for Processor<'l> {
    #[inline]
    fn drop(&mut self) {
        debug_assert!(!self.raw.is_null());
        unsafe { raw::delete_Processor(self.raw) };
    }
}
