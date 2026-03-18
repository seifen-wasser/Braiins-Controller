use crate::device::shelly::ShellyClient;
use crate::utils::config::{ShellyConfig, BraiinsConfig};
use actix_web::{HttpResponse, Responder, web};

pub async fn set_config_shelly(config: web::Json<ShellyConfig>) -> impl Responder {
    if ShellyClient::test_shelly(&config.shelly_ip).await {
        println!("Successfully connected to Shelly at {}", config.shelly_ip);
        config.save().unwrap();

        HttpResponse::Ok().json("status: success")
    } else {
        println!("Failed to connect to Shelly at {}", config.shelly_ip);
        HttpResponse::BadRequest().body("Failed to connect to Shelly with provided IP")
    }
}

pub async fn get_config_shelly() -> impl Responder {
    let config = ShellyConfig::load().unwrap();
    HttpResponse::Ok().json(config)
}

pub async fn set_config_braiins(config: web::Json<BraiinsConfig>) -> impl Responder {
    println!("Successfully connected to Braiins at {}", config.braiins_ip);
    config.save().unwrap();

    HttpResponse::Ok().json("status: success")
}

pub async fn get_config_braiins() -> impl Responder {
    let config = BraiinsConfig::load().unwrap();
    HttpResponse::Ok().json(config)
}