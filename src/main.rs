mod app;
mod device;
mod utils;

use actix_cors::Cors;
use actix_web::{App, HttpServer, http::header};
use std::env;
use std::path::Path;
use tokio::sync::broadcast;
use tokio::task;

use crate::utils::config::Config;

fn initialize() {
    if !Path::new(Config::FILE_PATH).exists() {
        if let Err(e) = Config::create_default_config() {
            eprintln!("[ERROR] Failed to create default config: {}", e);
        } else {
            println!("[INFO] Created default config file: {}", Config::FILE_PATH);
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().unwrap();
    let port = env::var("PORT").unwrap_or("8080".to_string());

    println!("[INFO] Starting server...");
    initialize();

    // Create a broadcast channel for config reload signals.
    let (reload_tx, reload_rx) = broadcast::channel::<()>(1);

    // Spawn the automation background task with the receiver.
    task::spawn(async move {
        if let Err(e) = utils::automations::run_automation(reload_rx).await {
            eprintln!("Automation task failed: {}", e);
        }
    });

    // Store the sender in Actix's application data so handlers can trigger reloads.
    let app_data = actix_web::web::Data::new(reload_tx);

    println!("[INFO] Listening on http://0.0.0.0:{}", port);
    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")   // Vue dev server
            .allowed_origin("http://127.0.0.1:5173")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(app_data.clone())
            .configure(app::routs::init_routs)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await?;

    Ok(())
}