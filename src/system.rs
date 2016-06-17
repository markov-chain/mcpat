use raw;
use std::fs;
use std::marker::PhantomData;
use std::path::Path;

use {Raw, Result, Processor};

/// A system.
pub struct System {
    raw: Raw<raw::ParseXML>,
    phantom: PhantomData<raw::ParseXML>,
}

impl System {
    /// Load a system from a file.
    ///
    /// The file is a configuration file of McPAT.
    pub fn open<T: AsRef<Path>>(path: T) -> Result<System> {
        let path = path.as_ref();
        if !exists(path) {
            raise!(NotFound, format!("the file {:?} does not exist", path));
        }
        unsafe {
            let raw = not_null!(raw::new_ParseXML());
            raw::ParseXML_parse(raw, path_to_cstr!(path).as_ptr() as *mut _);
            Ok(System {
                raw: (raw, debug_not_null!(raw::ParseXML_sys(raw))),
                phantom: PhantomData,
            })
        }
    }

    /// Perform optimization and produce an instance of the system.
    pub fn compute<'l>(&'l self) -> Result<Processor<'l>> {
        let raw = unsafe { not_null!(raw::new_Processor(self.raw.0)) };
        Ok(::processor::from_raw((raw, self.raw.1)))
    }

    /// Return the raw specification of the system.
    #[inline(always)]
    pub fn raw(&self) -> &raw::root_system {
        unsafe { &*self.raw.1 }
    }
}

impl Drop for System {
    #[inline]
    fn drop(&mut self) {
        unsafe { raw::delete_ParseXML(debug_not_null!(self.raw.0)) };
    }
}

#[inline]
fn exists<T: AsRef<Path>>(path: T) -> bool {
    match fs::metadata(path) {
        Ok(metadata) => !metadata.is_dir(),
        Err(_) => false,
    }
}
