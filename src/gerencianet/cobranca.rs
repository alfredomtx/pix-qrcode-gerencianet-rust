
use serde::{Serialize, Deserialize};
use serde_json::{json};

use super::Configuration;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CobrancaRequest {
    pub calendario: Calendario,
    pub devedor: Devedor,
    pub valor: Valor,
    pub chave: String,
    pub solicitacao_pagador: String,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CobrancaResponse {
    pub calendario: CalendarioResponse,
    pub txid: String,
    pub loc: LocationResponse,
    pub status: String,
    pub devedor: Devedor,
    pub valor: Valor,
    pub chave: String,
    pub solicitacao_pagador: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Calendario {
    pub expiracao: i64,
}

#[derive(Deserialize, Debug)]
pub struct CalendarioResponse {
    pub criacao: String,
    pub expiracao: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Devedor {
    pub cpf: String,
    pub nome: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Valor {
    pub original: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct LocationResponse {
    pub id: i64,
    pub location: String,
    pub tipo_cob: String,
    pub criacao: String,
}


pub async fn do_cobranca(client: &reqwest::Client, configuration: &Configuration) -> Result<CobrancaResponse, anyhow::Error>{
    let calendario = Calendario {
        expiracao: 3600,
    };
    
    let devedor = Devedor {
        cpf: "02902720092".to_string(),
        nome: "Francisco da Silva".to_string(),
    };

    let valor = Valor {
        original: "30.00".to_string(),
    };

    let cobranca = CobrancaRequest {
        calendario,
        devedor,
        valor,
        chave: configuration.chave.clone(),
        solicitacao_pagador: "Test".to_string(),
    };

    let json = serde_json::to_value(&cobranca)
        .map_err(|e| anyhow::anyhow!(format!("Failed to parse `cobranca` to json: {}", e)))?;

    let body = json!(json);

    let response = client
        .post(format!("{}/v2/cob", &configuration.api_url))
        .json(&body)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!(format!("Failed to perform request: {}", e)))?;

    let response_body = response.text().await
        .map_err(|e| anyhow::anyhow!(format!("Failed to get the response body: {}", e)))?;

    let cobranca_response: CobrancaResponse = serde_json::from_str(&response_body)
        .map_err(|e| anyhow::anyhow!(format!("Failed to parse `response_body` to `CobrancaResponse`: {}\n\nResponse:\n{}\n", e, response_body)))?;

    return Ok(cobranca_response);
}