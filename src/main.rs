use std::env;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, middleware, Responder};
use actix_cors::Cors;

use sea_orm::Database;
use tracing;
mod core;
mod config;

use crate::config::api_config::AppState;
use crate::core::constants;

async fn health_check() -> impl Responder {
  HttpResponse::Ok()
}
async fn set_app_state() -> AppState {
  AppState {
    app_name: "Actix Web".to_string(),
  }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    dotenvy::dotenv().ok();

    let hostname: &str =constants::HOST_NAME; 
    let db_url:String = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    set_environment();
    tracing_debug_print();
    startup(db_url.as_str(),hostname);

    let conn = Database::connect(&db_url).await.unwrap();

    HttpServer::new(move|| {
        App::new()
        .app_data(web::Data::new(set_app_state()))
        .app_data(middleware::Logger::default()) // enable logger
            .route("/health_check", web::get().to(health_check))
    })
    .bind(hostname)?
    .run()
    .await
}

async fn tracing_debug_print() {
  tracing_subscriber::fmt()
  .with_max_level(tracing::Level::DEBUG)
  .with_test_writer()
  .init();
}
async fn set_environment() {
  if std::env::var_os("RUST_LOG").is_none() {
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
  }

  if std::env::var_os("RUST_LOG_STYLE").is_none() {
      std::env::set_var("RUST_LOG_STYLE", "always");
  }
}
async fn startup(db_url: &str, hostname: &str) {
  println!("Database loaded on {}", db_url);
  println!("\n🚀 Web Server successfully started at: http://{}\n", hostname);

}