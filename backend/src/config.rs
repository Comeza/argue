use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub address: String,
    pub serve_dir: PathBuf,

    pub openai: EndpointConfig,
    pub google_gemini: EndpointConfig,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EndpointConfig {
    pub enabled: bool,
    pub token: String,
    pub endpoint: String,
    pub rate_limit: Option<u32>,
}

impl Default for EndpointConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            token: String::new(),
            endpoint: String::new(),
            rate_limit: Some(0),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            address: "0.0.0.0:3000".into(),
            serve_dir: "argue-react/dist".into(),
            google_gemini: Default::default(),
            openai: Default::default(),
        }
    }
}
