use raw;
use std::marker::PhantomData;

use {Raw, Phantom};
use component::{Component, Power};

/// A cache.
pub struct Cache<'l> {
    raw: Raw<raw::SharedCache>,
    phantom: Phantom<'l, raw::SharedCache>,
}

/// An L3 cache.
pub type L3<'l> = Cache<'l>;

impl<'l> Component for Cache<'l> {
    #[inline]
    fn power(&self) -> Power {
        unsafe {
            let dynamic = {
                let raw = raw::SharedCache_rt_power(self.raw.0);
                debug_assert!(!raw.is_null());
                let raw = raw::powerDef_readOp(raw);
                debug_assert!(!raw.is_null());
                raw::powerComponents_dynamic(raw) /
                raw::CacheDynParam_executionTime(raw::SharedCache_cachep(self.raw.0))
            };
            let leakage = {
                let raw = raw::SharedCache_power(self.raw.0);
                debug_assert!(!raw.is_null());
                let raw = raw::powerDef_readOp(raw);
                debug_assert!(!raw.is_null());
                let subthreshold = if (&*self.raw.1).longer_channel_device > 0 {
                    raw::powerComponents_longer_channel_leakage(raw)
                } else {
                    raw::powerComponents_leakage(raw)
                };
                subthreshold + raw::powerComponents_gate_leakage(raw)
            };
            Power { dynamic: dynamic, leakage: leakage }
        }
    }
}

#[inline]
pub fn from_raw<'l>(raw: (*mut raw::SharedCache, *mut raw::root_system)) -> Cache<'l> {
    Cache { raw: raw, phantom: PhantomData }
}
