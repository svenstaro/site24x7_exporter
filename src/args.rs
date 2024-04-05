use clap::{Parser, ValueEnum};
use http::uri::PathAndQuery;
use simplelog::LevelFilter;
use strum::Display;

use std::net::SocketAddr;

#[derive(Debug, Clone, ValueEnum, Display)]
pub enum Endpoint {
    #[value(name = "site24x7.com")]
    Com,
    #[value(name = "site24x7.eu")]
    Eu,
    #[value(name = "site24x7.cn")]
    Cn,
    #[value(name = "site24x7.in")]
    In,
    #[value(name = "site24x7.net.au")]
    NetAu,
}

#[derive(Parser)]
#[command(name = "site24x7_exporter", author, about, version)]
pub struct Config {
    /// API endpoint to use (depends on region, see https://site24x7.com/help/api)
    #[arg(long, default_value = "site24x7.com")]
    pub site24x7_endpoint: Endpoint,

    /// Address on which to expose metrics and web interface
    #[arg(long = "web.listen-address", default_value = "0.0.0.0:9803")]
    pub listen_address: SocketAddr,

    /// Path under which to expose metrics
    #[arg(long = "web.telemetry-path", default_value = "/metrics")]
    pub metrics_path: PathAndQuery,

    /// Path under which to expose geolocation information
    #[arg(long = "web.geolocation-path", default_value = "/geolocation")]
    pub geolocation_path: PathAndQuery,

    /// Only log messages with the given severity or above
    #[arg(long = "log.level", default_value = "info")]
    pub loglevel: LevelFilter,
}
