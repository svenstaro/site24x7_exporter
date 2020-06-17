use anyhow::{anyhow, Context, Result};
use http::uri::PathAndQuery;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Response, Server};
use log::{debug, info};
use prometheus::register_int_gauge;
use simplelog::{LevelFilter, TermLogger};
use std::net::SocketAddr;
use structopt::StructOpt;

mod site24x7_types;
mod zoho_types;

#[derive(StructOpt, Clone, Debug)]
#[structopt(
    name = "site24x7_exporter",
    author,
    about,
    global_settings = &[structopt::clap::AppSettings::ColoredHelp],
)]
pub struct Config {
    /// API endpoint to use (depends on region, see https://site24x7.com/help/api)
    #[structopt(long, default_value = "site24x7.com",
        possible_values = &["site24x7.com", "site24x7.eu", "site24x7.cn", "site24x7.in", "site24x7.net.au"])]
    pub site24x7_endpoint: String,

    /// Address on which to expose metrics and web interface
    #[structopt(long = "web.listen-address", default_value = "0.0.0.0:9803")]
    pub listen_address: SocketAddr,

    /// Path under which to expose metrics
    #[structopt(long = "web.telemetry-path", default_value = "/metrics")]
    pub metrics_path: PathAndQuery,

    /// Only log messages with the given severity or above
    #[structopt(
        long = "log.level",
        default_value = "info",
        possible_values = &["error", "warn", "info", "debug", "trace"],
    )]
    pub loglevel: LevelFilter,
}

async fn get_access_token(
    client: &reqwest::Client,
    zoho_endpoint: &str,
    client_id: &str,
    client_secret: &str,
    refresh_token: &str,
) -> Result<String> {
    let access_token_request = zoho_types::AccessTokenRequest {
        client_id: client_id.into(),
        client_secret: client_secret.into(),
        refresh_token: refresh_token.into(),
        grant_type: "refresh_token".into(),
    };

    let access_token_resp = client
        .post(&format!("{}/oauth/v2/token", &zoho_endpoint))
        .form(&access_token_request)
        .send()
        .await?;

    let access_token_resp_text = access_token_resp.text().await?;

    let access_token_resp_parsed =
        serde_json::from_str(&access_token_resp_text).context(format!(
            "Couldn't parse server response while getting access token. Server replied: '{}",
            access_token_resp_text
        ))?;
    match access_token_resp_parsed {
        zoho_types::AccessTokenResponse::Success(inner) => Ok(inner.access_token),
        zoho_types::AccessTokenResponse::Error(e) => Err(anyhow!(
            "Error while getting access token. Server replied '{}'",
            e.error
        )),
    }
}

async fn fetch_current_status(
    client: &reqwest::Client,
    site24x7_endpoint: &str,
    access_token: &str,
) -> Result<site24x7_types::CurrentStatusData, site24x7_types::CurrentStatusError> {
    let current_status_resp = client
        .get(&format!("{}/current_status", site24x7_endpoint))
        .header("Accept", "application/json; version=2.0")
        .header("Authorization", format!("Zoho-oauthtoken {}", access_token))
        .send()
        .await
        .context("Error during web request to fetch curent status.")?;

    let current_status_resp_text = current_status_resp
        .text()
        .await
        .context("Couldn't stream text from response")?;

    let deserializer = &mut serde_json::Deserializer::from_str(&current_status_resp_text);
    let current_status_resp_result = serde_path_to_error::deserialize(deserializer);

    if current_status_resp_result.is_err() {
        let v: serde_json::Value =
            serde_json::from_str(&current_status_resp_text).context("JSON seems invalid.")?;
        eprintln!(
            "{}",
            serde_json::to_string_pretty(&v).context("Couldn't format JSON for debug output")?
        );
    }
    let current_status_resp_parsed: site24x7_types::CurrentStatusResponse =
        current_status_resp_result
            .map_err(|e| {
                anyhow!(site24x7_types::CurrentStatusError::ParseError(
                    e.to_string()
                ))
            })
            .context(format!(
                "Couldn't parse server response while fetching monitors."
            ))?;

    match current_status_resp_parsed {
        site24x7_types::CurrentStatusResponse::Success(inner) => Ok(inner.data),
        site24x7_types::CurrentStatusResponse::Error(e) => {
            if e.message == "OAuth Access Token is invalid or has expired." {
                Err(site24x7_types::CurrentStatusError::ApiAuthError(e.message))
            } else {
                Err(site24x7_types::CurrentStatusError::ApiUnknownError(
                    e.message,
                ))
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Config::from_args();

    dotenv::dotenv()?;

    let client_id = std::env::var("ZOHO_CLIENT_ID").context("ZOHO_CLIENT_ID must be set")?;
    let client_secret =
        std::env::var("ZOHO_CLIENT_SECRET").context("ZOHO_CLIENT_SECRET must be set")?;
    let refresh_token =
        std::env::var("ZOHO_REFRESH_TOKEN").context("ZOHO_REFRESH_TOKEN must be set")?;

    TermLogger::init(
        args.loglevel,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
    )?;

    let client = reqwest::Client::new();

    let site24x7_endpoint = format!("https://{}/api", args.site24x7_endpoint);

    // Figure out Zoho accounts endpoint.
    let zoho_endpoint = format!(
        "https://accounts.zoho.{}",
        args.site24x7_endpoint.splitn(2, ".").last().unwrap()
    );
    info!("Using site24x7 endpoint: {}", site24x7_endpoint);
    info!("Using Zoho endpoint: {}", zoho_endpoint);

    // An access token is only available for a period of time.
    // We sometimes have to refresh it.
    let mut access_token = get_access_token(
        &client,
        &zoho_endpoint,
        &client_id,
        &client_secret,
        &refresh_token,
    )
    .await?;

    // Create metric
    let random_value_metric = register_int_gauge!("random_value_metric", "will set a random value")
        .expect("can not create gauge random_value_metric");

    // Update metric with random value.
    random_value_metric.set(0);

    // Try updating the monitors.
    let current_status = fetch_current_status(&client, &site24x7_endpoint, &access_token).await;
    dbg!(&current_status);
    // TODO Fix error handling
    let current_status_data = match current_status {
        Ok(current_status_data) => current_status_data,
        // If there was an auth error, maybe the token was old. We'll try to get a new token.
        // If we also get an auth error the second time, probably something is wrong with the
        // refresh token and we'll just give up.
        Err(site24x7_types::CurrentStatusError::ApiAuthError(_)) => {
            access_token = get_access_token(
                &client,
                &zoho_endpoint,
                &client_id,
                &client_secret,
                &refresh_token,
            )
            .await?;
            fetch_current_status(&client, &site24x7_endpoint, &access_token).await?
        }
        Err(e) => Err(e)?,
    };

    let make_service = make_service_fn(move |_conn| async move {
        Ok::<_, hyper::Error>(service_fn(move |req| async move {
            match (req.method(), req.uri().path()) {
                (&Method::GET, "/metrics") => {
                    Ok::<_, hyper::Error>(Response::new(Body::from("lol")))
                }
                _ => Ok::<_, hyper::Error>(Response::new("Trying /metrics".into())),
            }
        }))
    });

    let server = Server::bind(&args.listen_address).serve(make_service);

    server.await.context("Server error")
}
