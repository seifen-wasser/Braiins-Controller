mod api;
mod device;
mod utils;

use crate::{device::shelly::ShellyClient, utils::config::ShellyConfig};
use actix_web::{App, HttpServer, web};

async fn init_automations() -> Result<(), Box<dyn std::error::Error>> {

    //Shelly
    let mut shelly: ShellyClient = ShellyClient::new("192.0.0.182");

    println!("Shelly status: {:?}", shelly.get_status().await);

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //print!("\x1B[2J\x1B[1;1H");
    println!("[INFO] Starting");
    
    //init_automations();

    /*actix_web::rt::spawn(async {
        if let Err(e) = utils::automations::run_automation().await {
            eprintln!("Automation task failed: {}", e);
        }
    });*/

    println!("[INFO] http://127.0.0.1:8080");
    HttpServer::new(move || App::new().configure(api::routs::init_routs))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
