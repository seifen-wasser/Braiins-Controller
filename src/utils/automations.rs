use anyhow::Result;
use std::thread::sleep;
use tokio::time::Duration;

use crate::device::braiins::{self, Braiins};
use crate::device::shelly::ShellyClient;
use crate::utils::config::{BraiinsConfig, Config, ShellyConfig};

pub async fn run_automation() -> Result<()> {
    let mut miner = Braiins::new(
        BraiinsConfig::load().unwrap().braiins_ip,
        BraiinsConfig::load().unwrap().braiins_username,
        BraiinsConfig::load().unwrap().braiins_password,
    );

    let _ = miner.login().await;

    let shelly_config = ShellyConfig::load().unwrap();
    let shelly_ip = shelly_config.shelly_ip;
    let client = ShellyClient::new(&shelly_ip);

    loop {
        let max: f64 = -100.0;

        let current = ShellyClient::get_status(&client)
            .await
            .unwrap()
            .em
            .total_current;

        if max > current {
            println!(
                "Current power ({:.2}W) is below max ({:.2}W). Starting miner...",
                current, max
            );
            let _ = miner.resume_miner().await;

        } else {
            println!(
                "Current power ({:.2}W) is above max ({:.2}W). Stopping miner...",
                current, max
            );
            let _ = miner.pause_miner().await;
        }
        sleep(Duration::from_secs(1));
    }
}
