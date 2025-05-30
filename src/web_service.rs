//! Module containing the web service.
use std::sync::Arc;

use hyper::{header, Body, Method, Request, Response, StatusCode};
use log::{debug, error, info};
use prometheus::{Encoder, TextEncoder};
use tokio::sync::RwLock;

use crate::api_communication::fetch_current_status;
use crate::metrics::update_metrics_from_current_status;
use crate::{api_communication::get_access_token, geodata, site24x7_types, CLIENT};

pub async fn hyper_service(
    req: Request<Body>,
    site24x7_client_info: &site24x7_types::Site24x7ClientInfo,
    refresh_token: &str,
    access_token: Arc<RwLock<String>>,
    metrics_path: &str,
    geolocation_path: &str,
) -> Result<Response<Body>, hyper::Error> {
    // Serve geolocation data.
    if req.method() == Method::GET && req.uri().path() == geolocation_path {
        info!("Serving geolocation info");
        return Ok(Response::builder()
            .header("Content-Type", "application/json")
            .header("Access-Control-Allow-Origin", "*")
            .body(Body::from(
                serde_json::to_string_pretty(&geodata::get_geolocation_info()).unwrap(),
            ))
            .unwrap());
    }

    // Serve default path.
    if req.method() != Method::GET || req.uri().path() != metrics_path {
        info!("Serving default path");
        return Ok(Response::new(
            format!("site24x7_exporter\n\nTry {metrics_path}").into(),
        ));
    }

    info!("Serving metrics");
    let current_status;
    {
        let access_token_read = access_token.read().await;

        current_status = fetch_current_status(
            &CLIENT,
            &site24x7_client_info.site24x7_endpoint,
            &access_token_read,
        )
        .await;
    }

    let current_status_data = match current_status {
        Ok(ref current_status_data) => {
            debug!(
                "Successfully deserialized into this data structure: \n{:#?}",
                &current_status
            );
            current_status_data.clone()
        }
        // If there was an auth error, maybe the token was old. We'll try to get a new token.
        // If we also get an auth error the second time, probably something is wrong with the
        // refresh token and we'll just give up.
        Err(site24x7_types::CurrentStatusError::ApiAuthError(_)) => {
            info!(
                "Couldn't get status update due to an authentication error. \
                Probably the access token has timed out. Trying to get a new one."
            );
            let mut access_token_write = access_token.write().await;
            let access_token_res =
                get_access_token(&CLIENT, site24x7_client_info, refresh_token).await;
            *access_token_write = match access_token_res {
                Ok(access_token) => access_token,
                Err(e) => {
                    error!("Failed to renew access token");
                    error!("{:?}", e);
                    return Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from(e.to_string()))
                        .unwrap());
                }
            };

            match fetch_current_status(
                &CLIENT,
                &site24x7_client_info.site24x7_endpoint,
                &access_token_write,
            )
            .await
            {
                Ok(current_status_data) => current_status_data,
                Err(e) => {
                    error!("An unexpected error occurred after renewing access token.");
                    error!("{:?}", e);
                    return Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from(e.to_string()))
                        .unwrap());
                }
            }
        }
        Err(e) => {
            error!("An unexpected error occurred.");
            error!("{:?}", e);
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(e.to_string()))
                .unwrap());
        }
    };

    update_metrics_from_current_status(&current_status_data);

    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, encoder.format_type())
        .body(Body::from(buffer))
        .unwrap())
}
