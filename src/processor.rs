use libc::c_int;
use raw;
use std::marker::PhantomData;

use {Core, L3, Phantom, Raw};

/// A processor.
pub struct Processor<'l> {
    raw: Raw<raw::Processor>,
    phantom: Phantom<'l, raw::Processor>,
}

pub struct Iterator<'l, T> {
    length: usize,
    position: usize,
    raw: Raw<raw::Processor>,
    phantom: PhantomData<(&'l raw::Processor, &'l raw::root_system, T)>,
}

pub trait Reader {
    fn read(raw: Raw<raw::Processor>, i: usize) -> Self;
}

/// An iterator over cores.
pub type Cores<'l> = Iterator<'l, Core<'l>>;

/// An iterator over L3 caches.
pub type L3s<'l> = Iterator<'l, L3<'l>>;

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

    /// Return an iterator over L3 caches.
    #[inline]
    pub fn l3s(&self) -> L3s<'l> {
        L3s {
            length: unsafe { raw::Processor_numL3(self.raw.0) } as usize,
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

impl<'l, T> Iterator<'l, T> {
    /// Return the total number of items regardless of how many have been
    /// traversed.
    #[inline]
    pub fn len(&self) -> usize { self.length }
}

impl<'l, T> ::std::iter::Iterator for Iterator<'l, T> where T: Reader {
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

impl<'l> Reader for L3<'l> {
    #[inline]
    fn read(raw: Raw<raw::Processor>, i: usize) -> L3<'l> {
        use std::mem::transmute;
        unsafe {
            let raw = (debug_not_null!(raw::Processor_l3array(raw.0, i as c_int)), raw.1);
            transmute(::cache::from_raw(raw))
        }
    }
}

#[inline]
pub fn from_raw<'l>(raw: (*mut raw::Processor, *mut raw::root_system)) -> Processor<'l> {
    Processor { raw: raw, phantom: PhantomData }
}
