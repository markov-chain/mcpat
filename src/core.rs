use raw;
use std::marker::PhantomData;

use {Raw, Phantom};
use component::{Component, Power};

/// A core.
pub struct Core<'l> {
    raw: Raw<raw::Core>,
    phantom: Phantom<'l, raw::Core>,
}

impl<'l> Component for Core<'l> {
    #[inline]
    fn power(&self) -> Power {
        unsafe {
            let raw = raw::Core_power(self.raw.0);
            debug_assert!(!raw.is_null());
            let raw = raw::powerDef_readOp(raw);
            debug_assert!(!raw.is_null());
            Power {
                dynamic: raw::powerComponents_dynamic(raw) * raw::Core_clockRate(self.raw.0),
                leakage: if (&*self.raw.1).longer_channel_device > 0 {
                            raw::powerComponents_longer_channel_leakage(raw)
                         } else {
                            raw::powerComponents_leakage(raw)
                         } + raw::powerComponents_gate_leakage(raw),
            }
        }
    }
}

#[inline]
pub fn from_raw<'l>(raw: (*mut raw::Core, *mut raw::root_system)) -> Core<'l> {
    Core { raw: raw, phantom: PhantomData }
}
