use actix_web::{App, HttpServer};
mod api;
use std::net::Ipv4Addr;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(api::todo_controller::configure()))
        .bind((Ipv4Addr::UNSPECIFIED, 8080))?
        .run()
        .await
}
