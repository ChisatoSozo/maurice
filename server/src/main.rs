use std::sync::{Arc, Mutex};

use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web::Data, App, HttpServer};

use diesel::{Connection, PgConnection};
use function_routes::add_function_routes::AddFunctionRoutes;
use function_types::python_functions::Python;
use logger::init_logger;
use paperclip::actix::OpenApiExt;

pub mod function_routes;
pub mod function_types;
pub mod logger;
pub mod model;
pub mod routes;
pub mod schema;

const JSON_SPEC_PATH: &str = "/api/spec/v2.json";

struct GlobalState {
    python: Python,
    db: Arc<Mutex<PgConnection>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger with custom format

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    init_logger();

    HttpServer::new(move || {
        let database_url = "postgres:///maurice";

        let db = PgConnection::establish(database_url)
            .expect(&format!("Error connecting to {}", database_url));

        let global_state = Data::new(GlobalState {
            python: Python::new().unwrap(),
            db: Arc::new(Mutex::new(db)),
        });
        App::new()
            .app_data(global_state)
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
            .add_function_routes()
            .build()
    })
    .workers(4)
    .client_request_timeout(std::time::Duration::from_secs(600))
    .client_disconnect_timeout(std::time::Duration::from_secs(600))
    .bind((host.clone(), 8080))?
    .run()
    .await
}
