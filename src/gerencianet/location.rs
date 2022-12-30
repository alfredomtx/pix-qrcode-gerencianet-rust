
use serde::{Deserialize};
use crate::gerencianet::{Configuration};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocationResponse {
    pub qrcode: String,
    pub imagem_qrcode: String,
}

pub async fn generate_qr_code(client: &reqwest::Client, configuration: &Configuration, location_id: i64) -> Result<LocationResponse, anyhow::Error>{
    let response = client
        .get(format!("{}/v2/loc/{}/qrcode", &configuration.api_url, location_id))
        .send()
        .await
        .map_err(|e| anyhow::anyhow!(format!("Failed to perform request: {}", e)))?;

    let response_body = response.text().await
        .map_err(|e| anyhow::anyhow!(format!("Failed to get the response body: {}", e)))?;

    let location_response: LocationResponse = serde_json::from_str(&response_body)
        .map_err(|e| anyhow::anyhow!(format!("Failed to parse `response_body` to `LocationResponse`: {}\n\nResponse:\n{}\n", e, response_body)))?;

    return Ok(location_response);
}