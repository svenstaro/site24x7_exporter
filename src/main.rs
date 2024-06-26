use anyhow::{Context, Result};
use clap::{crate_name, crate_version, Parser};
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use lazy_static::lazy_static;
use log::{debug, info};
use prometheus::{GaugeVec, IntGaugeVec};
use simplelog::TermLogger;
use tokio::sync::RwLock;

use std::sync::Arc;

mod api_communication;
mod args;
mod geodata;
mod metrics;
mod parsing;
mod site24x7_types;
mod web_service;
mod zoho_types;

lazy_static! {
    pub static ref CLIENT: reqwest::Client = reqwest::Client::new();
    pub static ref MONITOR_UP_GAUGE: IntGaugeVec = prometheus::register_int_gauge_vec!(
        "site24x7_monitor_up",
        "Current health status of the monitor (1 = UP, 0 = DOWN).",
        &["monitor_type", "monitor_name", "monitor_group", "location"]
    )
    .expect("Couldn't create monitor_up metric");
    pub static ref MONITOR_LATENCY_SECONDS_GAUGE: GaugeVec = prometheus::register_gauge_vec!(
        "site24x7_monitor_latency_seconds",
        "Last measured latency in seconds.",
        &["monitor_type", "monitor_name", "monitor_group", "location"]
    )
    .expect("Couldn't create monitor_latency_seconds metric");
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = args::Config::parse();

    TermLogger::init(
        args.loglevel,
        simplelog::ConfigBuilder::new()
            .set_thread_level(simplelog::LevelFilter::Trace)
            .build(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )?;

    dotenv::dotenv().ok();

    info!("{} {}", crate_name!(), crate_version!());

    let client_id = std::env::var("ZOHO_CLIENT_ID").context("ZOHO_CLIENT_ID must be set")?;
    let client_secret =
        std::env::var("ZOHO_CLIENT_SECRET").context("ZOHO_CLIENT_SECRET must be set")?;
    let refresh_token =
        std::env::var("ZOHO_REFRESH_TOKEN").context("ZOHO_REFRESH_TOKEN must be set")?;

    let site24x7_client_info = site24x7_types::Site24x7ClientInfo {
        site24x7_endpoint: format!("https://www.{}/api", args.site24x7_endpoint),
        zoho_endpoint: format!(
            "https://accounts.zoho.{}",
            args.site24x7_endpoint
                .to_string()
                .splitn(2, '.')
                .last()
                .unwrap()
        ),
        client_id,
        client_secret,
    };

    // Figure out Zoho accounts endpoint.
    info!(
        "Using site24x7 endpoint: {}",
        site24x7_client_info.site24x7_endpoint
    );
    info!(
        "Using Zoho endpoint: {}",
        site24x7_client_info.zoho_endpoint
    );

    // Info print used proxies if there are any.
    // Currently we have to do this in a stupid backwards way by parsing the debug output.
    // Hopefully, we'll be able to do this properly once this is fixed:
    // https://github.com/seanmonstar/reqwest/issues/967
    let debug_output = format!("{:?}", *CLIENT);
    let re = regex::Regex::new(r"^.*System\(\{(.*?)\}").unwrap();
    if let Some(caps) = re.captures(&debug_output) {
        if let Some(cap) = caps.get(1) {
            if cap.as_str().is_empty() {
                info!("Not using any proxies");
            } else {
                info!("Picked up proxies: {}", &caps[1]);
            }
        }
    }

    debug!("Reqwest client:\n{:#?}", *CLIENT);

    // An access token is only available for a period of time.
    // We sometimes have to refresh it.
    let access_token = Arc::new(RwLock::new(
        api_communication::get_access_token(&CLIENT, &site24x7_client_info, &refresh_token).await?,
    ));

    let metrics_path = args.metrics_path.to_string();
    let geolocation_path = args.geolocation_path.to_string();
    let make_service = make_service_fn(move |_conn| {
        let site24x7_client_info = site24x7_client_info.clone();
        let refresh_token = refresh_token.clone();
        let access_token = access_token.clone();
        let metrics_path = metrics_path.clone();
        let geolocation_path = geolocation_path.clone();
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                let site24x7_client_info = site24x7_client_info.clone();
                let refresh_token = refresh_token.clone();
                let access_token = access_token.clone();
                let metrics_path = metrics_path.clone();
                let geolocation_path = geolocation_path.clone();
                async move {
                    web_service::hyper_service(
                        req,
                        &site24x7_client_info,
                        &refresh_token,
                        access_token,
                        &metrics_path,
                        &geolocation_path,
                    )
                    .await
                }
            }))
        }
    });

    let server = Server::bind(&args.listen_address).serve(make_service);

    server.await.context("Server error")
}
