use std::{
    ffi::OsString,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    path::PathBuf,
};
use url::Url;

/// A lookup of default values
pub trait ConfigOptDefaults {
    /// Lookup a default value for the path to an argument
    fn arg_default(&self, arg_path: &[String]) -> Option<OsString>;
}

/// Implement `ConfigOptDefaults` for a type that implements `ToString`.
///
/// Due to lack of specialization this trait is needed.
pub trait ConfigOptToString: ToString {}

impl ConfigOptToString for Url {}

impl<T: ConfigOptToString> ConfigOptDefaults for T {
    fn arg_default(&self, _arg_path: &[String]) -> Option<OsString> {
        Some(self.to_string().into())
    }
}

macro_rules! configopt_defaults_to_string {
    ($rust_type:ty) => {
        impl ConfigOptDefaults for $rust_type {
            fn arg_default(&self, _arg_path: &[String]) -> Option<OsString> {
                Some(self.to_string().into())
            }
        }
    };
}

configopt_defaults_to_string!(usize);
configopt_defaults_to_string!(u8);
configopt_defaults_to_string!(u64);
configopt_defaults_to_string!(u32);
configopt_defaults_to_string!(u16);
configopt_defaults_to_string!(u128);
configopt_defaults_to_string!(String);
configopt_defaults_to_string!(str);
configopt_defaults_to_string!(SocketAddr);
configopt_defaults_to_string!(isize);
configopt_defaults_to_string!(Ipv6Addr);
configopt_defaults_to_string!(Ipv4Addr);
configopt_defaults_to_string!(IpAddr);
configopt_defaults_to_string!(i8);
configopt_defaults_to_string!(i64);
configopt_defaults_to_string!(i32);
configopt_defaults_to_string!(i16);
configopt_defaults_to_string!(i128);
configopt_defaults_to_string!(f64);
configopt_defaults_to_string!(f32);
configopt_defaults_to_string!(char);
configopt_defaults_to_string!(bool);
configopt_defaults_to_string!(&str);

impl ConfigOptDefaults for PathBuf {
    fn arg_default(&self, _arg_path: &[String]) -> Option<OsString> {
        Some(self.clone().into_os_string())
    }
}

impl<T: ConfigOptDefaults> ConfigOptDefaults for Option<T> {
    fn arg_default(&self, _arg_path: &[String]) -> Option<OsString> {
        self.as_ref().and_then(|v| v.arg_default(&[]))
    }
}

impl<T: ConfigOptDefaults> ConfigOptDefaults for Vec<T> {
    fn arg_default(&self, _arg_path: &[String]) -> Option<OsString> {
        let mut result = OsString::new();
        let mut first = true;
        for v in self.iter().map(|v| v.arg_default(&[])) {
            if !first {
                result.push(",");
            }
            if let Some(v) = v {
                result.push(&v)
            }
            first = false;
        }
        Some(result)
    }
}
