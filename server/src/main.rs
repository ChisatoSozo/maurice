#![allow(non_camel_case_types)]
use std::{
    error::Error,
    sync::{Arc, Mutex},
    time::Duration,
};

use actix_cors::Cors;
use actix_web::{http::header, web::Data, App, HttpServer};

use diesel::{Connection, PgConnection};
use function_routes::add_function_routes::AddFunctionRoutes;
use function_types::python_functions::Python;
use logger::init_logger;
use paperclip::actix::OpenApiExt;
use procedures::get_audio_devices::AudioDevice;
use routes::{
    append_song_to_playlist::append_song_to_playlist, create_directory::create_directory,
    create_file::create_file, delete_file::delete_file, delete_folder::delete_folder,
    edit_file::edit_file, get_playlist::get_playlist, get_song_duration::get_song_duration,
    get_song_time::get_song_time, get_speakers::get_speakers, get_volume::get_volume,
    get_youtube_videos::get_youtube_videos, is_locked::is_locked, list_files::list_files,
    pause::pause, play_audio::play_audio,
    remove_song_from_playlist_at_index::remove_song_from_playlist_at_index, resume::resume,
    set_song_time::set_song_time, set_volume::set_volume, stop::stop,
};
use types::mpv_handler::MpvSend;

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
    mpv_send: MpvSend,
    python: Arc<Python>,
    db: Arc<Mutex<PgConnection>>,
}

async fn handle_devices(
    mpv_send: &MpvSend,
    devices: Vec<AudioDevice>,
) -> Result<(), Box<dyn Error>> {
    for device in devices {
        let mut broken_pipe = false;
        {
            let duration =
                procedures::get_song_duration::get_song_duration(&mpv_send, &device.name).await;

            if let Err(e) = duration {
                if e.to_string().contains("Broken pipe") {
                    broken_pipe = true;
                }
            }
        }

        if broken_pipe {
            let result = procedures::next_song::next_song(&mpv_send, &device.name).await;
            if let Err(e) = result {
                log::error!("Error in skipping to next song: {}", e);
            }
        }
    }
    //sleep for TIME_TO_SKIP_TO_NEXT_SONG / 2
    tokio::time::sleep(Duration::from_millis(100)).await;
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger with custom format

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    init_logger();

    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        let mut mpv_handler =
            types::mpv_handler::MpvHandler::new(rx).expect("Failed to create mpv handler");
        mpv_handler.run();
    });

    let tx_clone = tx.clone();

    tokio::spawn(async move {
        loop {
            let devices =
                procedures::get_audio_devices::get_audio_devices().expect("Failed to get speakers");
            let result = handle_devices(&tx_clone, devices).await;
            if let Err(e) = result {
                log::error!("Error: {}", e);
            }
        }
    });

    let python = Arc::new(Python::new().unwrap());

    let port = std::env::var("PORT").unwrap_or_else(|_| "9090".to_string());
    let port_num = port.parse::<u16>().expect("Failed to parse port");

    HttpServer::new(move || {
        let database_url = "postgres:///maurice";

        let db = PgConnection::establish(database_url)
            .expect(&format!("Error connecting to {}", database_url));

        let global_state = Data::new(GlobalState {
            mpv_send: tx.clone(),
            python: python.clone(),
            db: Arc::new(Mutex::new(db)),
        });

        App::new()
            .app_data(global_state)
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
            .service(get_song_duration)
            .service(set_song_time)
            .service(list_files)
            .service(create_file)
            .service(delete_file)
            .service(delete_folder)
            .service(create_directory)
            .service(stop)
            .build()
    })
    .workers(4)
    .client_request_timeout(std::time::Duration::from_secs(600))
    .client_disconnect_timeout(std::time::Duration::from_secs(600))
    .bind((host.clone(), port_num))?
    .run()
    .await
}
