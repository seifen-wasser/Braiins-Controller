use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ShellyStatus {
    pub em: EmStatus,
    pub emdata: EmData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmData {
    pub id: u32,
    pub a_total_act_energy: f64,
    pub a_total_act_ret_energy: f64,
    pub b_total_act_energy: f64,
    pub b_total_act_ret_energy: f64,
    pub c_total_act_energy: f64,
    pub c_total_act_ret_energy: f64,
    pub total_act: f64,
    pub total_act_ret: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmStatus {
    pub id: u32,
    pub a_current: f64,
    pub a_voltage: f64,
    pub a_act_power: f64,
    pub a_aprt_power: f64,
    pub a_pf: f64,
    pub a_freq: f64,
    pub b_current: f64,
    pub b_voltage: f64,
    pub b_act_power: f64,
    pub b_aprt_power: f64,
    pub b_pf: f64,
    pub b_freq: f64,
    pub c_current: f64,
    pub c_voltage: f64,
    pub c_act_power: f64,
    pub c_aprt_power: f64,
    pub c_pf: f64,
    pub c_freq: f64,
    pub n_current: Option<f64>,
    pub total_current: f64,
    pub total_act_power: f64,
    pub total_aprt_power: f64,
    pub user_calibrated_phase: Vec<String>,
}

pub struct ShellyClient {
    client: Client,
    base_url: String,
}

impl ShellyClient {
    pub fn new(ip: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: format!("http://{}", ip),
        }
    }

    pub async fn test_shelly(ip: &str) -> bool {
        let client = Self::new(ip);
        client.get_status().await.is_ok()
    }

    pub async fn get_status(&self) -> Result<ShellyStatus> {
        let url = format!("{}/rpc/Shelly.GetStatus", self.base_url);
        let resp = self.client.get(&url).send().await?;
        let mut status: serde_json::Value = resp.json().await?;

        let em_value = status
            .get_mut("em:0")
            .ok_or_else(|| anyhow!("Missing 'em:0' field in response"))?
            .take();
        let emdata_value = status
            .get_mut("emdata:0")
            .ok_or_else(|| anyhow!("Missing 'emdata:0' field in response"))?
            .take();

        let em: EmStatus = serde_json::from_value(em_value)?;
        let emdata: EmData = serde_json::from_value(emdata_value)?;

        Ok(ShellyStatus { em, emdata })
    }
}
