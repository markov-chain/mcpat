use raw;
use std::fs;
use std::marker::PhantomData;
use std::path::Path;

use Result;

/// A system.
pub struct System<'l> {
    raw: *mut raw::ParseXML,
    phantom: PhantomData<&'l raw::ParseXML>,
}

impl<'l> System<'l> {
    /// Load the system from an XML file.
    pub fn new(path: &Path) -> Result<System> {
        if !exists(path) {
            raise!(NotFound, format!("{:?} does not exist", path));
        }
        let raw = unsafe { raw::new_ParseXML() };
        if raw.is_null() {
            raise!(NoMemory, "cannot allocate memory for an XML parser");
        }
        unsafe { raw::ParseXML_parse(raw, path_to_c_str!(path) as *mut _) };
        Ok(System { raw: raw, phantom: PhantomData })
    }
}

impl<'l> Drop for System<'l> {
    #[inline]
    fn drop(&mut self) {
        debug_assert!(!self.raw.is_null());
        unsafe { raw::delete_ParseXML(self.raw) };
    }
}

#[inline]
pub fn as_raw(system: &System) -> *mut raw::ParseXML {
    system.raw
}

#[inline]
fn exists(path: &Path) -> bool {
    match fs::metadata(path) {
        Ok(metadata) => !metadata.is_dir(),
        Err(_) => false,
    }
}
