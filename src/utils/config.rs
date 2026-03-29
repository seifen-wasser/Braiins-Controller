use anyhow::Result;               // only import Result, not Ok
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShellyConfig {
    pub shelly_ip: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BraiinsDevice {
    pub braiins_ip: String,
    pub braiins_username: String,
    pub braiins_password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BraiinsConfig {
    pub devices: Vec<BraiinsDevice>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub braiins: BraiinsConfig,
    pub shelly: ShellyConfig,
    pub max_watts_braiins: f64,
    pub start_watts_braiins: f64,
}

impl Config {
    pub const FILE_PATH: &'static str = "config.json";

    pub fn create_default_config() -> Result<()> {
        let json_string = serde_json::to_string_pretty(&Self::default())?;
        fs::write(Self::FILE_PATH, json_string)?;
        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        fs::write(Self::FILE_PATH, data)?;
        Ok(())
    }

    pub fn load() -> Result<Self> {
        let data = fs::read_to_string(Self::FILE_PATH)?;
        // Try to deserialize as the new format (with devices list)
        match serde_json::from_str::<Self>(&data) {
            Ok(config) => Ok(config),   // Ok is now the standard enum variant
            Err(_) => {
                // Fallback to old format (single device)
                #[derive(Deserialize)]
                struct OldConfig {
                    braiins: OldBraiinsConfig,
                    shelly: ShellyConfig,
                    max_watts_braiins: f64,
                    start_watts_braiins: f64,
                }
                #[derive(Deserialize)]
                struct OldBraiinsConfig {
                    braiins_ip: String,
                    braiins_username: String,
                    braiins_password: String,
                }

                let old: OldConfig = serde_json::from_str(&data)?;
                Ok(Config {
                    braiins: BraiinsConfig {
                        devices: vec![BraiinsDevice {
                            braiins_ip: old.braiins.braiins_ip,
                            braiins_username: old.braiins.braiins_username,
                            braiins_password: old.braiins.braiins_password,
                        }],
                    },
                    shelly: old.shelly,
                    max_watts_braiins: old.max_watts_braiins,
                    start_watts_braiins: old.start_watts_braiins,
                })
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            braiins: BraiinsConfig {
                devices: vec![BraiinsDevice {
                    braiins_ip: "192.0.0.123".to_string(),
                    braiins_username: "root".to_string(),
                    braiins_password: "".to_string(),
                }],
            },
            shelly: ShellyConfig {
                shelly_ip: "192.0.0.124".to_string(),
            },
            max_watts_braiins: -1000.0,
            start_watts_braiins: 100.0,
        }
    }
}