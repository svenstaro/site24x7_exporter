//! This module contains functions for communicating with the Site24x7 and Zoho APIs.
use anyhow::{anyhow, Context, Result};
use log::{info, debug};

use crate::{site24x7_types, zoho_types};
use crate::parsing::parse_current_status;

/// Acquire the access token.
///
/// An access token is a short-lived token that can be used to query the
/// API multiple times. It will become invalidated after a short period of
/// time.
/// See https://www.site24x7.com/help/api/index.html#authentication
pub async fn get_access_token(
    client: &reqwest::Client,
    site24x7_client_info: &site24x7_types::Site24x7ClientInfo,
    refresh_token: &str,
) -> Result<String> {
    let access_token_request = zoho_types::AccessTokenRequest {
        client_id: site24x7_client_info.client_id.clone(),
        client_secret: site24x7_client_info.client_secret.clone(),
        refresh_token: refresh_token.into(),
        grant_type: "refresh_token".into(),
    };

    let access_token_endpoint = format!("{}/oauth/v2/token", &site24x7_client_info.zoho_endpoint);
    info!("Requesting access token from {}", access_token_endpoint);
    debug!(
        "Getting access token with info:\n{:#?}",
        access_token_request
    );
    let access_token_resp = client
        .post(&access_token_endpoint)
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
        zoho_types::AccessTokenResponse::Success(inner) => {
            info!("Successfully acquired access token");
            debug!("Access token value: {}", inner.access_token);
            Ok(inner.access_token)
        }
        zoho_types::AccessTokenResponse::Error(e) => Err(anyhow!(
            "Error while getting access token. Server replied '{}'",
            e.error
        )),
    }
}

/// Receive an update for all monitor statuses.
///
/// Given a valid `access_token`, this will try to get a new set of fresh monitor data.
pub async fn fetch_current_status(
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

    parse_current_status(&current_status_resp_text)
}
