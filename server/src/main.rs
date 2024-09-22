#![allow(non_camel_case_types)]
use std::sync::{Arc, Mutex};

use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web::Data, App, HttpServer};

use diesel::{Connection, PgConnection};
use function_routes::add_function_routes::AddFunctionRoutes;
use function_types::python_functions::Python;
use logger::init_logger;
use paperclip::actix::OpenApiExt;
use routes::{
    append_song_to_playlist::append_song_to_playlist, edit_file::edit_file,
    get_playlist::get_playlist, get_song_time::get_song_time, get_speakers::get_speakers,
    get_volume::get_volume, get_youtube_videos::get_youtube_videos, is_locked::is_locked,
    pause::pause, play_audio::play_audio,
    remove_song_from_playlist_at_index::remove_song_from_playlist_at_index, resume::resume,
    set_song_time::set_song_time, set_volume::set_volume,
};
use types::speaker::MultiSpeaker;

pub mod function_routes;
pub mod function_types;
pub mod logger;
pub mod model;
pub mod procedures;
pub mod routes;
pub mod schema;
pub mod types;

const JSON_SPEC_PATH: &str = "/api/spec/v2.json";

struct GlobalState {
    speakers: Arc<Mutex<MultiSpeaker>>,
    python: Python,
    db: Arc<Mutex<PgConnection>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger with custom format

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    init_logger();
    let speakers = Arc::new(Mutex::new(MultiSpeaker::new()));

    HttpServer::new(move || {
        let database_url = "postgres:///maurice";

        let db = PgConnection::establish(database_url)
            .expect(&format!("Error connecting to {}", database_url));

        let global_state = Data::new(GlobalState {
            speakers: speakers.clone(),
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
            .service(actix_files::Files::new("/files", "./files"))
            .wrap_api()
            .with_json_spec_at(JSON_SPEC_PATH)
            .add_function_routes()
            .service(get_youtube_videos)
            .service(get_speakers)
            .service(play_audio)
            .service(get_volume)
            .service(set_volume)
            .service(remove_song_from_playlist_at_index)
            .service(get_playlist)
            .service(append_song_to_playlist)
            .service(pause)
            .service(resume)
            .service(edit_file)
            .service(is_locked)
            .service(get_song_time)
            .service(set_song_time)
            .build()
    })
    .workers(4)
    .client_request_timeout(std::time::Duration::from_secs(600))
    .client_disconnect_timeout(std::time::Duration::from_secs(600))
    .bind((host.clone(), 8080))?
    .run()
    .await
}
