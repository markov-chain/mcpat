use raw;
use std::marker::PhantomData;

use {Raw, Phantom};
use component::{self, Component, Power};

/// A cache of a central processing unit.
pub struct Cache<'l> {
    raw: Raw<raw::SharedCache>,
    phantom: Phantom<'l, raw::SharedCache>,
}

/// An L3 cache.
pub type L3<'l> = Cache<'l>;

impl<'l> Component for Cache<'l> {
    #[inline]
    fn area(&self) -> f64 {
        unsafe { raw::Area_get_area(raw::Component_area(self.raw.0 as *mut _)) }
    }

    fn power(&self) -> Power {
        unsafe {
            let dynamic = {
                let raw = raw::Component_rt_power(self.raw.0 as *mut _);
                debug_assert!(!raw.is_null());
                let raw = raw::powerDef_readOp(raw);
                debug_assert!(!raw.is_null());
                raw::powerComponents_dynamic(raw) /
                raw::CacheDynParam_executionTime(raw::SharedCache_cachep(self.raw.0))
            };
            let leakage = {
                let raw = raw::Component_power(self.raw.0 as *mut _);
                debug_assert!(!raw.is_null());
                let raw = raw::powerDef_readOp(raw);
                debug_assert!(!raw.is_null());
                component::leakage(self.raw.1, raw)
            };
            Power { dynamic: dynamic, leakage: leakage }
        }
    }
}

#[inline]
pub fn from_raw<'l>(raw: (*mut raw::SharedCache, *mut raw::root_system)) -> Cache<'l> {
    Cache { raw: raw, phantom: PhantomData }
}
