use crate::gerencianet::Credentials;
use std::env;
use serde::{Deserialize};
use serde_yaml;
use std::fs::File;

/// The possible runtime environment for our application.
#[derive(Debug, Clone, Deserialize)]
pub enum Environment {
    Development,
    Production
}

#[derive(Debug, Deserialize)]
pub struct ConfigurationFile {
    pub gerencianet: Configuration,
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub api_url: String,
    pub chave: String,
    pub sandbox: bool,
    pub debug: bool,
    pub credentials: Credentials,
}


impl Configuration {

    pub fn new() -> Result<Configuration, anyhow::Error> {
        let base_path = env::current_dir()
            .map_err(|e| anyhow::anyhow!(format!("Failed to determine the current directory: {}", e)))?;
        let configuration_directory = base_path.join("configuration");

        // Detect the running environment
        // Default to `development` if unspecified
        let environment: Environment = env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "development".into())
            .try_into()
            .map_err(|e| anyhow::anyhow!(format!("Failed to parse APP_ENVIRONMENT: {}", e)))?;

        let environment_filename = format!("{}.yaml", environment.as_str());

        let file = File::open(&configuration_directory.join(&environment_filename))
            .map_err(|e| anyhow::anyhow!(format!("Failed to open config file: {}", e)))?;

        let yaml: ConfigurationFile = serde_yaml::from_reader(file)
            .map_err(|e| anyhow::anyhow!(format!("Failed to parse YAML file to `ConfigurationFile`: {}", e)))?;

        let configuration = yaml.gerencianet;
        return Ok( configuration );
    }
    
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        return match self {
            Environment::Development => "development",
            Environment::Production => "production"
        };
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        return match s.to_lowercase().as_str() {
            "development" => Ok(Self::Development),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either 'development' or 'production'.",
                other
            )),
        };
    }

}
