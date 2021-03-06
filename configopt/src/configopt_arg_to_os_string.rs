use std::{ffi::OsString, path::PathBuf};

/// A lookup of default values
pub trait ConfigOptArgToOsString {
    /// Lookup a default value for the path to an argument
    fn arg_to_os_string(&self, arg_path: &[String]) -> Option<OsString>;
}

// Use this trick[1] to get around the lack of specialization.
//
// [1] https://github.com/dtolnay/case-studies/blob/master/autoref-specialization/README.md
impl<T: ToString> ConfigOptArgToOsString for &T {
    fn arg_to_os_string(&self, _arg_path: &[String]) -> Option<OsString> {
        Some(self.to_string().into())
    }
}

impl ConfigOptArgToOsString for PathBuf {
    fn arg_to_os_string(&self, _arg_path: &[String]) -> Option<OsString> {
        Some(self.clone().into_os_string())
    }
}
