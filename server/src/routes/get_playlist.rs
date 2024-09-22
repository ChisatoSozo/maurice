use actix_web::{
    web::{Data, Json},
    Error,
};

use log::error;
use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::{Deserialize, Serialize};

use crate::{procedures::refresh_speakers::refresh_speakers, types::speaker::Song, GlobalState};

#[derive(Debug, Deserialize, Apiv2Schema)]
struct GetPlaylistArgs {
    speaker: String,
}

#[derive(Debug, Serialize, Apiv2Schema)]
struct GetPlaylistReturn {
    songs: Vec<Song>,
}

#[api_v2_operation]
#[post("/api/get_playlist")]
pub fn get_playlist(
    gs: Data<GlobalState>,
    body: Json<GetPlaylistArgs>,
) -> Result<Json<GetPlaylistReturn>, Error> {
    let speakers = gs.speakers.clone();
    let mut speakers_lock = speakers.lock().map_err(|e| {
        error!("Error getting speakers_lock: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Error getting speakers_lock: {}", e))
    })?;

    refresh_speakers(&mut speakers_lock).expect("Failed to refresh speakers");

    let songs = speakers_lock.get_playlist(&body.speaker).map_err(|e| {
        error!("Error getting songs: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Error getting songs: {}", e))
    })?;

    Ok(Json(GetPlaylistReturn { songs }))
}
