
use crate::gerencianet::Credentials;
use base64;
use secrecy::ExposeSecret;
use std::fs::File;
use std::io::Read;
use reqwest::header::{self, HeaderMap, HeaderValue};

#[derive(Debug)]
pub struct Client(reqwest::Client);

impl Client {

    pub fn new(credentials: &Credentials, access_token: Option<String>) -> Result<reqwest::Client, anyhow::Error> {
        let mut buf = Vec::new();
        File::open(&credentials.certificado_pix)
            .map_err(|e| anyhow::anyhow!(format!("Failed to open .p12 file: {}", e)))?
            .read_to_end(&mut buf)
            .map_err(|e| anyhow::anyhow!(format!("Failed to read .p12 file: {}", e)))?;

        let identity = reqwest::Identity::from_pkcs12_der(&buf, "")
            .map_err(|e| anyhow::anyhow!(format!("Failed to create identity from .p12 file.: {}", e)))?;

        // if `access_token` is none, we will add in the headers the `client_id` and `client_secret`
        // in the format that GerenciaNet expects
        let auth_header: HeaderValue;
        match access_token {
            Some(token) => {
                // `Bearer` Authorization header with access token
                let bearer = format!("Bearer {}", token);
                auth_header = header::HeaderValue::from_str(bearer.as_str())
                    .map_err(|e| anyhow::anyhow!(format!("Failed to parse `bearer` to `HeaderValue`: {}", e)))?;
            },
            None => {
                // `Basic` Authorization header base64 encoded
                let basic_base64 = base64::encode(format!("{}:{}", credentials.client_id.expose_secret(), credentials.client_secret.expose_secret()));
                let basic = format!("Basic {}", basic_base64);
                auth_header = header::HeaderValue::from_str(basic.as_str())
                    .map_err(|e| anyhow::anyhow!(format!("Failed to parse `basic` to `HeaderValue`: {}", e)))?;
            }
        };

        let mut headers = HeaderMap::new();
        headers.insert(header::AUTHORIZATION, auth_header);
        headers.insert(header::CONTENT_TYPE, "application/json".parse()? );

        let client = reqwest::ClientBuilder::new()
            .identity(identity)
            .default_headers(headers)
            .https_only(true)
            .build()
            .map_err(|e| anyhow::anyhow!(format!("Failed to create reqwest client: {}", e)))?;

        return Ok(client);
    }
    
}