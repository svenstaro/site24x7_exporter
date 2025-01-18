//! Module containing Zoho API-specific types.
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct AccessTokenRequest {
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
    pub grant_type: String,
}

#[derive(Deserialize, Debug)]
pub struct AccessTokenResponseInner {
    pub access_token: String,
    #[allow(dead_code)]
    pub expires_in: u32,
    #[allow(dead_code)]
    pub api_domain: String,
    #[allow(dead_code)]
    pub token_type: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum AccessTokenResponse {
    Success(AccessTokenResponseInner),
    Error(ApiError),
}

#[derive(Deserialize, Debug)]
pub struct ApiError {
    pub error: String,
}
