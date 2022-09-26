#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};
mod api;
mod data;
mod entities;
pub mod schema;
use dotenv::dotenv;

use std::{error::Error, net::Ipv4Addr};

// Add error and info logging macro usings here.
use log::{error, info};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    // Apply any ending database migrations upon startup of our application.
    match data::run_migrations() {
        Ok(()) => info!("Succesfully applied pending migrations (if any)"),
        Err(_) => error!("Unable to apply pending migrations"),
    }

    HttpServer::new(|| App::new().configure(api::todo_controller::configure()))
        .bind((Ipv4Addr::UNSPECIFIED, 8080))?
        .run()
        .await
}
