use configopt::{ConfigOpt, ConfigOptType};
use serde::{Deserialize, Serialize};
use std::{fmt, num::ParseFloatError, str::FromStr, time::Duration};
use structopt::StructOpt;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(from = "f64", into = "f64")]
pub struct DurationProxy(Duration);

impl From<DurationProxy> for f64 {
    fn from(d: DurationProxy) -> Self {
        d.0.as_secs_f64()
    }
}

impl From<f64> for DurationProxy {
    fn from(f: f64) -> Self {
        Self(Duration::from_secs_f64(f))
    }
}

impl From<DurationProxy> for Duration {
    fn from(d: DurationProxy) -> Self {
        d.0
    }
}

impl From<Duration> for DurationProxy {
    fn from(d: Duration) -> Self {
        Self(d)
    }
}

impl FromStr for DurationProxy {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<f64>()?.into())
    }
}

impl fmt::Display for DurationProxy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", f64::from(*self))
    }
}

#[derive(ConfigOpt, StructOpt, Debug, Deserialize, PartialEq)]
#[configopt(derive(Debug, PartialEq), attrs(serde))]
#[serde(deny_unknown_fields)]
struct MyStruct {
    #[structopt(long)]
    period: DurationProxy,
}

#[test]
fn test_duration_type() {
    let c = ConfigOptMyStruct {
        period: Some(DurationProxy::from(19.345)),
    };
    let s = MyStruct::try_from_iter_with_defaults(&["app"], &c).unwrap();
    assert_eq!(s.period.0.as_secs_f64(), 19.345);
    let s = MyStruct::try_from_iter_with_defaults(&["app", "--period", "5.789"], &c).unwrap();
    assert_eq!(s.period.0.as_secs_f64(), 5.789);
    assert_eq!(c.toml_config(), "period = 19.345\n\n");
}
