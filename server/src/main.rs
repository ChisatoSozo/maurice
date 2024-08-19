use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, App, HttpServer};

use logger::init_logger;
use paperclip::actix::OpenApiExt;
use routes::say_hello::say_hello;

pub mod logger;
pub mod routes;

const JSON_SPEC_PATH: &str = "/api/spec/v2.json";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger with custom format

    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    init_logger();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .wrap_api()
            .with_json_spec_at(JSON_SPEC_PATH)
            .service(say_hello)
            .build()
    })
    .workers(4)
    .client_request_timeout(std::time::Duration::from_secs(600))
    .client_disconnect_timeout(std::time::Duration::from_secs(600))
    .bind((host.clone(), 8080))?
    .run()
    .await
}
