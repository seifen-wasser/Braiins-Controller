use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShellyConfig {
    pub shelly_ip: String,
}

pub struct Config {
    pub max_watts_braiins: f64,   // The max power the miner should consume
    pub start_watts_braiins: f64, // The power where the miner should start mining
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BraiinsConfig {
    pub braiins_ip: String,
    pub braiins_username: String,
    pub braiins_password: String,
}

impl ShellyConfig {
    const FILE_PATH: &'static str = "shelly_config.json";

    /// Returns `true` if the config file exists, `false` otherwise.
    /// Propagates IO errors other than `NotFound`.
    pub fn check() -> Result<bool> {
        match fs::metadata(Self::FILE_PATH) {
            Ok(_) => Ok(true),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    pub fn load() -> Result<Self> {
        if let Ok(data) = fs::read_to_string(Self::FILE_PATH) {
            Ok(serde_json::from_str(&data)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        fs::write(Self::FILE_PATH, data)?;
        Ok(())
    }
}

impl Default for ShellyConfig {
    fn default() -> Self {
        Self {
            shelly_ip: "0.0.0.0".into(),
        }
    }
}

impl BraiinsConfig {
    const FILE_PATH: &'static str = "braiins_config.json";

    pub fn load() -> Result<Self> {
        if let Ok(data) = fs::read_to_string(Self::FILE_PATH) {
            Ok(serde_json::from_str(&data)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        fs::write(Self::FILE_PATH, data)?;
        Ok(())
    }
}

impl Default for BraiinsConfig {
    fn default() -> Self {
        Self {
            braiins_ip: "0.0.0.0".into(),
            braiins_username: "root".into(),
            braiins_password: "admin".into(),
        }
    }
}