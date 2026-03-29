use actix_web::{web, HttpResponse, Responder};
use crate::app::config::{
    get_config, set_config,
    get_status, get_miners, pause_miner, resume_miner,
};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "message": "Server is running"
    }))
}

fn config_rout(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/config")
            .route("/get", web::get().to(get_config))
            .route("/set", web::post().to(set_config)),
    );
}

fn miner_rout(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/miners")
            .route("", web::get().to(get_miners))
            .route("/{ip}/pause", web::post().to(pause_miner))
            .route("/{ip}/resume", web::post().to(resume_miner)),
    );
}

pub fn init_routs(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(config_rout)
            .configure(miner_rout)
            .route("/health", web::get().to(health_check))
            .route("/status", web::get().to(get_status)),
    );
}