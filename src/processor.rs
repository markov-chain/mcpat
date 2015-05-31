use libc::c_int;
use raw;
use std::marker::PhantomData;

use {Core, Raw, Phantom};

/// A processor.
pub struct Processor<'l> {
    raw: Raw<raw::Processor>,
    phantom: Phantom<'l, raw::Processor>,
}

/// An iterator over cores.
pub struct Cores<'l> {
    length: usize,
    position: usize,
    raw: Raw<raw::Processor>,
    phantom: Phantom<'l, raw::Processor>,
}

impl<'l> Processor<'l> {
    /// Return an iterator over cores.
    #[inline]
    pub fn cores(&self) -> Cores<'l> {
        Cores {
            length: unsafe { raw::Processor_numCore(self.raw.0) } as usize,
            position: 0,
            raw: self.raw,
            phantom: PhantomData,
        }
    }
}

impl<'l> Drop for Processor<'l> {
    #[inline]
    fn drop(&mut self) {
        unsafe { raw::delete_Processor(debug_not_null!(self.raw.0)) };
    }
}

impl<'l> Cores<'l> {
    #[inline]
    pub fn len(&self) -> usize { self.length }
}

impl<'l> Iterator for Cores<'l> {
    type Item = Core<'l>;

    fn next(&mut self) -> Option<Core<'l>> {
        if self.position == self.length {
            None
        } else {
            let raw = unsafe {
                debug_not_null!(raw::Processor_cores(self.raw.0, self.position as c_int))
            };
            self.position += 1;
            Some(::core::from_raw((raw, self.raw.1)))
        }
    }
}

#[inline]
pub fn from_raw<'l>(raw: (*mut raw::Processor, *mut raw::root_system)) -> Processor<'l> {
    Processor { raw: raw, phantom: PhantomData }
}
