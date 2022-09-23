#[macro_use]
extern crate diesel;
use actix_web::{App, HttpServer};
mod api;
mod data;
mod entities;
pub mod schema;
use dotenv::dotenv;

use std::net::Ipv4Addr;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| App::new().configure(api::todo_controller::configure()))
        .bind((Ipv4Addr::UNSPECIFIED, 8080))?
        .run()
        .await
}
