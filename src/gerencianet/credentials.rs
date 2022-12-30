use crate::gerencianet::{Configuration};
use serde::{Deserialize};
use std::path::Path;
use secrecy::Secret;


#[derive(Debug, Deserialize, Clone)]
pub struct Credentials {
    pub client_id: Secret<String>,
    pub client_secret: Secret<String>,
    pub certificado_pix: String,
}

impl Credentials {
    pub fn new(configuration: &Configuration) -> Result<Credentials, anyhow::Error> {
        let credentials = configuration.credentials.clone();

        if (Path::new(&credentials.certificado_pix).exists() == false){
            return Err(anyhow::anyhow!(format!("`certificado_pix` file does not exists at `{}`", &credentials.certificado_pix)))
        }

        return Ok( credentials );
    }
    
}

