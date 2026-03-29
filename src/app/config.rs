use crate::device::shelly::ShellyClient;
use crate::device::braiins::Braiins;
use crate::utils::config::Config;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use tokio::sync::broadcast;

// --- Existing handlers ---
pub async fn get_config() -> impl Responder {
    match Config::load() {
        Ok(config) => HttpResponse::Ok().json(config),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to load config: {}", e)),
    }
}

pub async fn set_config(
    config: web::Json<Config>,
    reload_tx: web::Data<broadcast::Sender<()>>,
) -> impl Responder {
    if !ShellyClient::test_shelly(&config.shelly.shelly_ip).await {
        eprintln!("Failed to connect to Shelly at {}", config.shelly.shelly_ip);
        return HttpResponse::BadRequest().body("Invalid Shelly IP: cannot connect");
    }

    if let Err(e) = config.save() {
        eprintln!("Failed to save config: {}", e);
        return HttpResponse::InternalServerError().body("Failed to save configuration");
    }

    // Notify automation to reload config immediately
    let _ = reload_tx.send(());
    println!("Configuration saved successfully.");
    HttpResponse::Ok().json(json!({ "status": "success" }))
}

// --- New handlers ---

/// GET /api/status
/// Returns Shelly power and each miner's mining status.
pub async fn get_status() -> impl Responder {
    let config = match Config::load() {
        Ok(c) => c,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Config load error: {}", e)),
    };

    // Get Shelly power
    let shelly_client = ShellyClient::new(&config.shelly.shelly_ip);
    let power = match shelly_client.get_status().await {
        Ok(s) => s.em.total_current,
        Err(e) => {
            eprintln!("Shelly status error: {}", e);
            -1.0
        }
    };

    // Get status of each miner (temporary connections)
    let mut miners_status = Vec::new();
    for device in config.braiins.devices {
        let mut miner = Braiins::new(
            device.braiins_ip.clone(),
            device.braiins_username.clone(),
            device.braiins_password.clone(),
        );
        let mining = if let Ok(()) = miner.login().await {
            miner.get_mining_status().await.unwrap_or(false)
        } else {
            false
        };
        miners_status.push(json!({
            "ip": device.braiins_ip,
            "username": device.braiins_username,
            "mining": mining
        }));
    }

    HttpResponse::Ok().json(json!({
        "shelly_power_watts": power,
        "max_watts": config.max_watts_braiins,
        "start_watts": config.start_watts_braiins,
        "miners": miners_status
    }))
}

/// GET /api/miners
/// Returns the list of configured miners (without status).
pub async fn get_miners() -> impl Responder {
    let config = match Config::load() {
        Ok(c) => c,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Config load error: {}", e)),
    };
    HttpResponse::Ok().json(config.braiins.devices)
}

/// POST /api/miners/{ip}/pause
pub async fn pause_miner(ip: web::Path<String>) -> impl Responder {
    let ip = ip.into_inner();
    let config = match Config::load() {
        Ok(c) => c,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Config load error: {}", e)),
    };

    // Find the device with matching IP
    let device = config.braiins.devices.iter().find(|d| d.braiins_ip == ip);
    if device.is_none() {
        return HttpResponse::NotFound().body("Miner not found in config");
    }
    let device = device.unwrap();

    let mut miner = Braiins::new(
        device.braiins_ip.clone(),
        device.braiins_username.clone(),
        device.braiins_password.clone(),
    );
    if let Err(e) = miner.login().await {
        return HttpResponse::InternalServerError().body(format!("Login failed: {}", e));
    }
    if let Err(e) = miner.pause_miner().await {
        return HttpResponse::InternalServerError().body(format!("Pause failed: {}", e));
    }
    HttpResponse::Ok().json(json!({ "status": "paused", "ip": ip }))
}

/// POST /api/miners/{ip}/resume
pub async fn resume_miner(ip: web::Path<String>) -> impl Responder {
    let ip = ip.into_inner();
    let config = match Config::load() {
        Ok(c) => c,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Config load error: {}", e)),
    };

    let device = config.braiins.devices.iter().find(|d| d.braiins_ip == ip);
    if device.is_none() {
        return HttpResponse::NotFound().body("Miner not found in config");
    }
    let device = device.unwrap();

    let mut miner = Braiins::new(
        device.braiins_ip.clone(),
        device.braiins_username.clone(),
        device.braiins_password.clone(),
    );
    if let Err(e) = miner.login().await {
        return HttpResponse::InternalServerError().body(format!("Login failed: {}", e));
    }
    if let Err(e) = miner.resume_miner().await {
        return HttpResponse::InternalServerError().body(format!("Resume failed: {}", e));
    }
    HttpResponse::Ok().json(json!({ "status": "resumed", "ip": ip }))
}