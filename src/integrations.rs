use std::collections::HashMap;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Debug)]
pub enum IntegrationError {
    IO(tokio::io::Error),
    Serde(serde_json::Error),
    CredentialsNotFound
}

#[derive(Debug,Clone,Deserialize)]
pub struct GOGTokensData {
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: String
}

pub async fn load_refresh_token() -> Result<String, IntegrationError> {
    let xdg_config = match std::env::var("XDG_CONFIG_HOME") {
        Ok(home) => PathBuf::from(home),
        Err(err) => {
            let home = PathBuf::from(std::env::var("HOME").unwrap());
            home.join(".config")
        }
    };

    let heroic_config = xdg_config.join("heroic/gog_store/auth.json");

    if heroic_config.exists() {
        let config = load_heroic_config(heroic_config).await;
        if let Ok(config) = config {
            return Ok(config.refresh_token)
        }
    }

    Err(IntegrationError::CredentialsNotFound)
}

async fn load_heroic_config(heroic_config: PathBuf) -> Result<GOGTokensData, IntegrationError> {
    let data = fs::read_to_string(heroic_config).await.map_err(|err| IntegrationError::IO(err))?;
    let tokens: HashMap<String, GOGTokensData> = serde_json::from_str(&data).map_err(|err| IntegrationError::Serde(err))?;

    if let Some(token) = tokens.get("46899977096215655") {
       return Ok(token.clone());
    }
    Err(IntegrationError::CredentialsNotFound)
}