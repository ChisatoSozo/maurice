use actix_web::{
    web::{Data, Json},
    Error,
};

use log::error;
use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::Deserialize;

use crate::{procedures::refresh_speakers::refresh_speakers, types::speaker::Song, GlobalState};

#[derive(Debug, Deserialize, Apiv2Schema)]
struct AppendSongToPlaylistArgs {
    speaker: String,
    song: Song,
}

#[api_v2_operation]
#[post("/api/append_song_to_playlist")]
pub fn append_song_to_playlist(
    gs: Data<GlobalState>,
    body: Json<AppendSongToPlaylistArgs>,
) -> Result<Json<bool>, Error> {
    let speakers = gs.speakers.clone();
    let mut speakers_lock = speakers.lock().map_err(|e| {
        error!("Error getting speakers_lock: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Error getting speakers_lock: {}", e))
    })?;

    refresh_speakers(&mut speakers_lock).expect("Failed to refresh speakers");

    speakers_lock
        .append_song_to_playlist(&body.speaker, body.song.clone())
        .map_err(|e| {
            error!("Error getting volume: {}", e);
            actix_web::error::ErrorInternalServerError(format!("Error getting volume: {}", e))
        })?;

    Ok(Json(true))
}
