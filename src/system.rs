use raw;
use std::fs;
use std::marker::PhantomData;
use std::path::Path;

use {Result, Processor};

/// A system.
pub struct System<'l> {
    raw: (*mut raw::ParseXML, *mut raw::root_system),
    phantom: PhantomData<(&'l raw::ParseXML, &'l raw::root_system)>,
}

impl<'l> System<'l> {
    /// Load the system from an XML file.
    pub fn new(path: &Path) -> Result<System> {
        if !exists(path) {
            raise!(NotFound, format!("{:?} does not exist", path));
        }
        unsafe {
            let raw = not_null!(raw::new_ParseXML());
            raw::ParseXML_parse(raw, path_to_c_str!(path) as *mut _);
            Ok(System {
                raw: (raw, debug_not_null!(raw::ParseXML_sys(raw))),
                phantom: PhantomData,
            })
        }
    }

    /// Compute the processor corresponding to the system.
    pub fn processor(&self) -> Result<Processor<'l>> {
        let raw = unsafe { not_null!(raw::new_Processor(self.raw.0)) };
        Ok(::processor::from_raw((raw, self.raw.1)))
    }

    /// Return the raw description of the system.
    #[inline(always)]
    pub fn raw(&self) -> &raw::root_system {
        unsafe { &*self.raw.1 }
    }
}

impl<'l> Drop for System<'l> {
    #[inline]
    fn drop(&mut self) {
        unsafe { raw::delete_ParseXML(debug_not_null!(self.raw.0)) };
    }
}

#[inline]
fn exists(path: &Path) -> bool {
    match fs::metadata(path) {
        Ok(metadata) => !metadata.is_dir(),
        Err(_) => false,
    }
}
