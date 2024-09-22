use actix_web::{
    web::{Data, Json},
    Error,
};

use log::error;
use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::{Deserialize, Serialize};

use crate::{procedures::refresh_speakers::refresh_speakers, GlobalState};

#[derive(Debug, Deserialize, Apiv2Schema)]
struct GetSongTimeArgs {
    speaker: String,
}

#[derive(Debug, Serialize, Apiv2Schema)]
struct GetSongTimeReturn {
    song_time: f64,
    song_duration: f64,
}

#[api_v2_operation]
#[post("/api/get_song_time")]
pub fn get_song_time(
    gs: Data<GlobalState>,
    body: Json<GetSongTimeArgs>,
) -> Result<Json<GetSongTimeReturn>, Error> {
    let speakers = gs.speakers.clone();
    let mut speakers_lock = speakers.lock().map_err(|e| {
        error!("Error getting speakers_lock: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Error getting speakers_lock: {}", e))
    })?;

    refresh_speakers(&mut speakers_lock).expect("Failed to refresh speakers");

    let song_time = speakers_lock.get_song_time(&body.speaker).map_err(|e| {
        error!("Error getting song_time: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Error getting song_time: {}", e))
    })?;

    let song_duration = speakers_lock.get_duration(&body.speaker).map_err(|e| {
        error!("Error getting song_duration: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Error getting song_duration: {}", e))
    })?;

    Ok(Json(GetSongTimeReturn {
        song_time,
        song_duration,
    }))
}
