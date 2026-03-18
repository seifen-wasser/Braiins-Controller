
use crate::utils::config::ShellyConfig;
use crate::{api::config, device::shelly::ShellyClient};
use actix_web::{HttpResponse, Responder, web};

async fn shelly_status() -> impl Responder {
    let shelly_config = ShellyConfig::load().unwrap();
    let shelly_ip = shelly_config.shelly_ip;
    let client = ShellyClient::new(&shelly_ip);
    match client.get_status().await {
        Ok(status) => HttpResponse::Ok().json(serde_json::json!({
                "status":  "ok",
                "data": {
                    "total_current": status.em.total_current,
                    "total_act_power": status.em.total_act_power,
                    "total_aprt_power": status.em.total_aprt_power
                }
        })),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "message": "Server is running"
    }))
}

fn shelly(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/shelly")
            .route("/status", web::get().to(shelly_status))
            .route("/getConfig", web::get().to(config::get_config_shelly))
            .route("/setConfig", web::post().to(config::set_config_shelly)),
    );
}

fn braiins(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/braiins")
            .route("/getConfig", web::get().to(config::get_config_braiins))
            .route("/setConfig", web::post().to(config::set_config_braiins)),
    );
}

pub fn init_routs(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(shelly)
            .configure(braiins)
            .route("/health", web::get().to(health_check))
    );
}
