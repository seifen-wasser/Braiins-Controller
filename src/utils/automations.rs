use anyhow::Result;
use tokio::sync::broadcast;
use tokio::time::Duration;

use crate::device::braiins::Braiins;
use crate::device::shelly::ShellyClient;
use crate::utils::config::Config;

pub async fn run_automation(mut reload_rx: broadcast::Receiver<()>) -> Result<()> {
    loop {
        // Reload configuration – either on a reload signal or every 60 seconds.
        let config = tokio::select! {
            _ = reload_rx.recv() => {
                println!("[AUTO] Reloading config due to API change.");
                match Config::load() {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("Failed to load config after reload: {}", e);
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        continue;
                    }
                }
            }
            _ = tokio::time::sleep(Duration::from_secs(60)) => {
                match Config::load() {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("Failed to load config on timer: {}", e);
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        continue;
                    }
                }
            }
        };

        let shelly_ip = config.shelly.shelly_ip;
        let client = ShellyClient::new(&shelly_ip);

        // Build list of miners
        let mut miners = Vec::new();
        for device in config.braiins.devices {
            let mut miner = Braiins::new(
                device.braiins_ip.clone(),
                device.braiins_username.clone(),
                device.braiins_password.clone(),
            );

            if let Err(e) = miner.login().await {
                eprintln!("Failed to login to {}: {}", device.braiins_ip, e);
                continue;
            }

            let is_mining = match miner.get_mining_status().await {
                Ok(status) => status,
                Err(e) => {
                    eprintln!("Failed to get status for {}: {}", device.braiins_ip, e);
                    false
                }
            };

            miners.push((miner, device.braiins_ip, is_mining));
        }

        if miners.is_empty() {
            eprintln!("No miners available. Retrying in 30 seconds...");
            tokio::time::sleep(Duration::from_secs(30)).await;
            continue;
        }

        let max_watts = config.max_watts_braiins;
        let start_watts = config.start_watts_braiins;

        // Run with the current set of miners, but break on a reload signal.
        let mut iteration_count = 0;
        let max_iterations = 60; // about 60 seconds before forced reload
        loop {
            tokio::select! {
                _ = reload_rx.recv() => {
                    println!("[AUTO] Reload signalled, restarting with fresh config.");
                    break;
                }
                _ = tokio::time::sleep(Duration::from_secs(1)) => {
                    if iteration_count >= max_iterations {
                        println!("[AUTO] 60 seconds passed, reloading config.");
                        break;
                    }

                    let current = match ShellyClient::get_status(&client).await {
                        Ok(status) => status.em.total_current,
                        Err(e) => {
                            eprintln!("Failed to get Shelly status: {}", e);
                            iteration_count += 1;
                            continue;
                        }
                    };

                    let should_pause = current > max_watts;
                    let should_resume = current < start_watts;

                    for (miner, ip, is_mining) in &mut miners {
                        if should_pause && *is_mining {
                            if let Err(e) = miner.pause_miner().await {
                                eprintln!("Failed to pause miner {}: {}", ip, e);
                            } else {
                                *is_mining = false;
                                println!("Paused miner {}", ip);
                            }
                        } else if should_resume && !*is_mining {
                            if let Err(e) = miner.resume_miner().await {
                                eprintln!("Failed to resume miner {}: {}", ip, e);
                            } else {
                                *is_mining = true;
                                println!("Resumed miner {}", ip);
                            }
                        }
                    }

                    iteration_count += 1;
                }
            }
        }
    }
}