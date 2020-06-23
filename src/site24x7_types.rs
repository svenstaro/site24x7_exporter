//! Module containing Site24x7 API-specific types.
use serde::{Deserialize, Deserializer};
use serde_repr::Deserialize_repr;
use thiserror::Error;
use strum_macros::Display;

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

#[derive(Clone, Deserialize_repr, Debug)]
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

#[derive(Clone, Deserialize, Debug)]
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

fn from_attribute_value<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    // Site24x7 sends "-" as the latency value in `attribute_value` for monitors
    // that are down. This is a bit weird but it's their way of saying that no
    // latency measurment is possible for a down host.
    // We'll deal with this by sending 0 as that's what Prometheus recommends:
    // https://prometheus.io/docs/practices/instrumentation/#avoid-missing-metrics
    let v: u64 = Deserialize::deserialize(deserializer).unwrap_or(0);
    Ok(v)
}

#[derive(Clone, Deserialize, Debug)]
pub struct Location {
    pub status: Status,
    #[serde(deserialize_with = "from_attribute_value")]
    pub attribute_value: u64,
    pub location_name: String,
}

#[derive(Clone, Deserialize, Display, Debug)]
#[serde(tag = "monitor_type")]
pub enum MonitorMaybe {
    URL(Monitor),
    HOMEPAGE(Monitor),
    REALBROWSER(Monitor),
    // SSL_CERT(Monitor),
    #[serde(other)]
    Unknown,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Monitor {
    pub name: String,
    pub unit: String,
    pub attribute_key: String,
    pub status: Status,
    pub locations: Vec<Location>,
    #[serde(rename = "attributeName")]
    pub attribute_name: String,
    pub attribute_label: String,
    #[serde(deserialize_with = "from_attribute_value")]
    pub attribute_value: u64,
    pub monitor_id: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct MonitorGroup {
    #[serde(default)]
    pub monitors: Vec<MonitorMaybe>,
    pub group_id: String,
    pub group_name: String,
}
