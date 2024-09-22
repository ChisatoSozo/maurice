use actix_web::{
    web::{Data, Json},
    Error,
};

use log::error;
use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::Deserialize;

use crate::{procedures::refresh_speakers::refresh_speakers, GlobalState};

#[derive(Debug, Deserialize, Apiv2Schema)]
struct RemoveSongFromPlaylistAtIndexArgs {
    speaker: String,
    index: usize,
}

#[api_v2_operation]
#[post("/api/remove_song_from_playlist_at_index")]
pub fn remove_song_from_playlist_at_index(
    gs: Data<GlobalState>,
    body: Json<RemoveSongFromPlaylistAtIndexArgs>,
) -> Result<Json<bool>, Error> {
    let speakers = gs.speakers.clone();
    let mut speakers_lock = speakers.lock().map_err(|e| {
        error!("Error getting speakers_lock: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Error getting speakers_lock: {}", e))
    })?;

    refresh_speakers(&mut speakers_lock).expect("Failed to refresh speakers");

    speakers_lock
        .remove_song_from_playlist_at_index(&body.speaker, body.index)
        .map_err(|e| {
            error!("Error getting volume: {}", e);
            actix_web::error::ErrorInternalServerError(format!("Error getting volume: {}", e))
        })?;

    Ok(Json(true))
}
