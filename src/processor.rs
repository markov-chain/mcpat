use libc::c_int;
use raw;
use std::marker::PhantomData;

use {Core, Raw, Phantom};

/// A processor.
pub struct Processor<'l> {
    raw: Raw<raw::Processor>,
    phantom: Phantom<'l, raw::Processor>,
}

struct Items<'l, T> {
    length: usize,
    position: usize,
    raw: Raw<raw::Processor>,
    phantom: PhantomData<(&'l raw::Processor, &'l raw::root_system, T)>,
}

trait Reader {
    fn read(raw: Raw<raw::Processor>, i: usize) -> Self;
}

/// An iterator over cores.
pub type Cores<'l> = Items<'l, Core<'l>>;

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

impl<'l, T> Items<'l, T> {
    /// Return the total number of items regardless of how many have been
    /// traversed.
    #[inline]
    pub fn len(&self) -> usize { self.length }
}

impl<'l, T> Iterator for Items<'l, T> where T: Reader {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.position == self.length {
            None
        } else {
            self.position += 1;
            Some(<T as Reader>::read(self.raw, self.position - 1))
        }
    }
}

impl<'l> Reader for Core<'l> {
    #[inline]
    fn read(raw: Raw<raw::Processor>, i: usize) -> Core<'l> {
        unsafe {
            ::core::from_raw((debug_not_null!(raw::Processor_cores(raw.0, i as c_int)), raw.1))
        }
    }
}

#[inline]
pub fn from_raw<'l>(raw: (*mut raw::Processor, *mut raw::root_system)) -> Processor<'l> {
    Processor { raw: raw, phantom: PhantomData }
}
