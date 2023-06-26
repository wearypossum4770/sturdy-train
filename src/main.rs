use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::{
    get, http::header, middleware, web, App, HttpResponse, HttpServer, Responder, Result,
};
use std::env;
use tracing;
use uuid::Uuid;

mod schema;

mod analytics;
mod config;
mod core;

use crate::analytics::routes::page_routes;
use crate::config::{api_config::AppState, postgres::get_pool};
use crate::core::constants;

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
fn set_app_state() -> AppState {
    let mut state = AppState::new();
    state.set_app_name("Active Web").clone()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    dotenvy::dotenv().ok();

    let hostname: &str = constants::HOST_NAME;
    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    set_environment();
    tracing_debug_print();
    startup(db_url.as_str(), hostname);
    HttpServer::new(move || {
        App::new()
            // .default_service(web::to(move || async { "Hello, cross-origin world!" }))
            .app_data(web::Data::new(set_app_state()))
            .app_data(web::Data::new(get_pool()))
            .app_data(set_cors())
            .app_data(middleware::Logger::default()) // enable logger
            .app_data(Logger::new("  %a %t '%r' %s %b '%{Referer}i' '%{User-Agent}i' %T"))
            // .service(Files::new("", ".").show_files_listing())
            .service(index)
            // .route("/blog", web::get().to(blog_index))
            // .service(Files::new("", "./templates/index.html"))
            .route("/debug", web::get().to(app_state_debug))
            .service(
                web::scope("/page_view")
                    .service(page_routes::create_new_page_view)
                    .service(page_routes::new_page_view),
            )
            .route("/health_check", web::get().to(health_check))
    })
    .bind(hostname)?
    .run()
    .await
}

#[get("/")]
async fn index() -> Result<impl Responder> {
    Ok(NamedFile::open_async("src/templates/index.html").await)
}
async fn blog_index() -> Result<NamedFile, std::io::Error> {
    NamedFile::open("./src/blog_post.html")
}
pub async fn app_state_debug(data: web::Data<AppState>) -> impl Responder {
    println!("server config {:?}", data.into_inner());
    HttpResponse::Ok().content_type("text/html").body("<!doctypehtml><html lang=en><meta charset=UTF-8><meta content='width=device-width,initial-scale=1'name=viewport><title>Debugger</title><h2 style=margin:auto;height:80%;width:300px>Nothing to see here</h2>")
}
fn tracing_debug_print() {
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).with_test_writer().init();
}
fn set_environment() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    }

    if std::env::var_os("RUST_LOG_STYLE").is_none() {
        std::env::set_var("RUST_LOG_STYLE", "always");
    }
}
fn startup(db_url: &str, hostname: &str) {
    println!("Database loaded on {}", db_url);
    println!(
        "\n🚀 Web Server successfully started at: http://{}\n",
        hostname
    );
}
fn set_cors() -> Cors {
    Cors::default()
        // add specific origin to allowed origin list
        .allowed_origin("http://project.local:8080")
        // allow any port on localhost
        .allowed_origin_fn(|origin, _req_head| {
            origin.as_bytes().starts_with(b"http://localhost")

            // manual alternative:
            // unwrapping is acceptable on the origin header since this function is
            // only called when it exists
            // req_head
            //     .headers()
            //     .get(header::ORIGIN)
            //     .unwrap()
            //     .as_bytes()
            //     .starts_with(b"http://localhost")
        })
        // set allowed methods list
        .allowed_methods(vec!["GET", "POST"])
        // set allowed request header list
        .allowed_headers(&[header::AUTHORIZATION, header::ACCEPT])
        // add header to allowed list
        .allowed_header(header::CONTENT_TYPE)
        // set list of headers that are safe to expose
        .expose_headers(&[header::CONTENT_DISPOSITION])
        // allow cURL/HTTPie from working without providing Origin headers
        .block_on_origin_mismatch(false)
        // set preflight cache TTL
        .max_age(3600)
}
