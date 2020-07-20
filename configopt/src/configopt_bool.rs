use crate::configopt_arg_to_os_string::ConfigOptArgToOsString;
use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use std::{
    ops::{Deref, DerefMut},
    str::{FromStr, ParseBoolError},
};

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(from = "Option<bool>", into = "Option<bool>")]
pub struct ConfigOptBool(pub Option<bool>);

impl ConfigOptBool {
    pub fn from_flag(b: bool) -> Self {
        if b {
            true.into()
        } else {
            None.into()
        }
    }
}

impl FromStr for ConfigOptBool {
    type Err = ParseBoolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b: bool = s.parse()?;
        Ok(b.into())
    }
}

impl From<bool> for ConfigOptBool {
    fn from(other: bool) -> Self {
        Self(Some(other))
    }
}

impl From<Option<bool>> for ConfigOptBool {
    fn from(other: Option<bool>) -> Self {
        Self(other)
    }
}

impl From<ConfigOptBool> for Option<bool> {
    fn from(other: ConfigOptBool) -> Self {
        other.0
    }
}

impl Deref for ConfigOptBool {
    type Target = Option<bool>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ConfigOptBool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ConfigOptArgToOsString for ConfigOptBool {
    fn arg_to_os_string(&self, _arg_path: &[String]) -> Option<OsString> {
        self.and_then(|b| (&b).arg_to_os_string(&[]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn config_opt_bool_from_str() {
        assert_eq!(
            ConfigOptBool::from_str("true").unwrap(),
            ConfigOptBool(Some(true))
        );
        assert_eq!(
            ConfigOptBool::from_str("false").unwrap(),
            ConfigOptBool(Some(false))
        );
        assert!(ConfigOptBool::from_str("bad input").is_err());
    }

    #[test]
    fn config_opt_bool_from_flag() {
        assert_eq!(ConfigOptBool::from_flag(false), ConfigOptBool(None));
        assert_eq!(ConfigOptBool::from_flag(true), ConfigOptBool(Some(true)));
    }

    #[test]
    fn config_opt_bool_serde() {
        assert_eq!(
            serde_json::from_str::<ConfigOptBool>("true").unwrap(),
            ConfigOptBool(Some(true))
        );
        assert_eq!(
            serde_json::from_str::<ConfigOptBool>("false").unwrap(),
            ConfigOptBool(Some(false))
        );
        assert_eq!(
            serde_json::from_str::<ConfigOptBool>("null").unwrap(),
            ConfigOptBool(None)
        );
        assert!(serde_json::from_str::<ConfigOptBool>("bad input").is_err(),);
        assert_eq!(serde_json::to_string(&ConfigOptBool(None)).unwrap(), "null");
        assert_eq!(
            serde_json::to_string(&ConfigOptBool(Some(true))).unwrap(),
            "true"
        );
        assert_eq!(
            serde_json::to_string(&ConfigOptBool(Some(false))).unwrap(),
            "false"
        );
    }
}
