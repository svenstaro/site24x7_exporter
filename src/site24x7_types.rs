//! Module containing Site24x7 API-specific types.
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Deserializer};
use serde_repr::Deserialize_repr;
use strum_macros::Display;
use thiserror::Error;

pub static DATE_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.f%z";

#[derive(Clone, Deserialize, Debug)]
pub struct Site24x7ClientInfo {
    pub site24x7_endpoint: String,
    pub zoho_endpoint: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(untagged)]
pub enum CurrentStatusResponse {
    Success(CurrentStatusResponseInner),
    Error(ApiError),
}

#[derive(Clone, Deserialize, Debug)]
pub struct ApiError {
    pub error_code: u16,
    pub message: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct CurrentStatusResponseInner {
    pub data: CurrentStatusData,
}

#[derive(Clone, Deserialize_repr, Debug, PartialEq)]
#[repr(u8)]
pub enum Status {
    Down = 0,
    Up = 1,
    Trouble = 2,
    Critical = 3,
    Suspended = 5,
    Maintenance = 7,
    Discovery = 9,
    ConfigurationError = 10,
}

/// Default to `Status::ConfigurationError` as observation shows that this is the most probable
/// case if we don't see a proper value for this enum.
impl Default for Status {
    fn default() -> Self {
        Status::ConfigurationError
    }
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct CurrentStatusData {
    #[serde(default)]
    pub monitors: Vec<MonitorMaybe>,
    #[serde(default)]
    pub monitor_groups: Vec<MonitorGroup>,
}

#[derive(Error, Debug)]
pub enum CurrentStatusError {
    #[error("API auth error: {0}")]
    ApiAuthError(String),

    #[error("Unknown API error: {0}")]
    ApiUnknownError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[allow(clippy::unnecessary_wraps)]
fn from_attribute_value<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    // Site24x7 sends "-" as the latency value in `attribute_value` for monitors
    // that are down. This is a bit weird but it's their way of saying that no
    // latency measurment is possible for a down host.
    // We'll deal with this by setting `NaN` as that's what Prometheus recommends:
    // https://prometheus.io/docs/practices/instrumentation/#avoid-missing-metrics
    let v: Option<u64> = Deserialize::deserialize(deserializer).ok();
    Ok(v)
}

fn from_custom_dateformat<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<FixedOffset>>, D::Error>
where
    D: Deserializer<'de>,
{
    // Site24x7 sends a slightly weird RFC3339-ish date which we'll need to parse.
    let d: Option<String> = Option::deserialize(deserializer)?;
    if let Some(d) = d {
        return Ok(Some(
            DateTime::parse_from_str(&d, DATE_FORMAT).map_err(serde::de::Error::custom)?,
        ));
    }
    Ok(None)
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Location {
    #[serde(default)]
    pub status: Status,
    #[serde(default, deserialize_with = "from_attribute_value")]
    pub attribute_value: Option<u64>,
    pub location_name: String,
    #[serde(default, deserialize_with = "from_custom_dateformat")]
    pub last_polled_time: Option<DateTime<FixedOffset>>,
}

#[derive(Clone, Deserialize, Display, Debug, PartialEq)]
#[serde(tag = "monitor_type")]
pub enum MonitorMaybe {
    #[serde(rename = "URL")]
    Url(Monitor),
    #[serde(rename = "HOMEPAGE")]
    Homepage(Monitor),
    #[serde(rename = "REALBROWSER")]
    RealBrowser(Monitor),
    // SSL_CERT(Monitor),
    #[serde(other)]
    Unknown,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tag {
    pub key: String,
    pub value: String,
}

impl<'de> Deserialize<'de> for Tag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;

        let mut parts = s.splitn(2, ':').fuse();

        let key = parts.next().unwrap_or_default().to_string();
        let value = parts.next().unwrap_or_default().to_string();

        Ok(Tag { key, value })
    }
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Monitor {
    pub name: String,
    pub unit: Option<String>,
    pub attribute_key: Option<String>,
    pub status: Status,
    pub locations: Vec<Location>,
    #[serde(rename = "attributeName")]
    pub attribute_name: String,
    // pub attribute_label: String,
    #[serde(default, deserialize_with = "from_attribute_value")]
    pub attribute_value: Option<u64>,
    pub monitor_id: String,
    #[serde(default)]
    pub tags: Vec<Tag>,
    #[serde(default, deserialize_with = "from_custom_dateformat")]
    pub last_polled_time: Option<DateTime<FixedOffset>>,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct MonitorGroup {
    #[serde(default)]
    pub monitors: Vec<MonitorMaybe>,
    pub group_id: String,
    pub group_name: String,
}
