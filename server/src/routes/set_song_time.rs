use actix_web::{
    web::{Data, Json},
    Error,
};

use log::error;
use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::Deserialize;

use crate::{procedures::refresh_speakers::refresh_speakers, GlobalState};

#[derive(Debug, Deserialize, Apiv2Schema)]
struct SetSongTimeArgs {
    speaker: String,
    song_time: f64,
}

#[api_v2_operation]
#[post("/api/set_song_time")]
pub fn set_song_time(
    gs: Data<GlobalState>,
    body: Json<SetSongTimeArgs>,
) -> Result<Json<bool>, Error> {
    let speakers = gs.speakers.clone();
    let mut speakers_lock = speakers.lock().map_err(|e| {
        error!("Error getting speakers_lock: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Error getting speakers_lock: {}", e))
    })?;

    refresh_speakers(&mut speakers_lock).expect("Failed to refresh speakers");

    speakers_lock
        .set_song_time(&body.speaker, body.song_time)
        .map_err(|e| {
            error!("Error getting song_time: {}", e);
            actix_web::error::ErrorInternalServerError(format!("Error getting song_time: {}", e))
        })?;

    Ok(Json(true))
}
