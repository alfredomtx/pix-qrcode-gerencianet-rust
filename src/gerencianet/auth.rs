
use crate::gerencianet::{Configuration, Client};
use serde::{Deserialize};
use serde_json::json;

#[derive(Deserialize, Debug)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub scope: String,
}

pub async fn get_token(configuration: &Configuration) -> Result<String, anyhow::Error> {
    let client = Client::new(&configuration.credentials, None)
        .map_err(|e| anyhow::anyhow!(format!("Failed to instantiate `Client`: {}", e)))?;

    let body = json!({"grant_type": "client_credentials"});

    let response = client
        .post(format!("{}/oauth/token", &configuration.api_url))
        .json(&body)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!(format!("Failed to perform request: {}", e)))?;

    let response_body = response.text().await
        .map_err(|e| anyhow::anyhow!(format!("Failed to get the response body: {}", e)))?;

    let auth_response: AuthResponse = serde_json::from_str(&response_body)
        .map_err(|e| anyhow::anyhow!(format!("Failed to parse `response_body` to `AuthResponse`: {}\n\nResponse:\n{}\n", e, response_body)))?;

    return Ok(auth_response.access_token);
}