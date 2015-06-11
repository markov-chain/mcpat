use raw;
use std::marker::PhantomData;

use {Phantom, Raw};
use component::{self, Component};

/// A central processing unit.
pub struct Core<'l> {
    raw: Raw<raw::Core>,
    phantom: Phantom<'l, raw::Core>,
}

impl<'l> Component for Core<'l> {
    #[inline]
    fn area(&self) -> f64 {
        unsafe { 1e-12 * raw::Area_get_area(raw::Component_area(self.raw.0 as *mut _)) }
    }

    fn dynamic_power(&self) -> f64 {
        unsafe {
            let raw = raw::Component_rt_power(self.raw.0 as *mut _);
            debug_assert!(!raw.is_null());
            let raw = raw::powerDef_readOp(raw);
            debug_assert!(!raw.is_null());
            raw::powerComponents_dynamic(raw) / raw::Core_executionTime(self.raw.0)
        }
    }

    fn leakage_power(&self) -> f64 {
        unsafe {
            let raw = raw::Component_power(self.raw.0 as *mut _);
            debug_assert!(!raw.is_null());
            let raw = raw::powerDef_readOp(raw);
            debug_assert!(!raw.is_null());
            component::leakage(self.raw.1, raw)
        }
    }
}

#[inline]
pub fn from_raw<'l>(raw: (*mut raw::Core, *mut raw::root_system)) -> Core<'l> {
    Core { raw: raw, phantom: PhantomData }
}
