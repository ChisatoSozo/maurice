use actix_web::{
    web::{Data, Json},
    Error,
};

use log::error;
use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::Deserialize;

use crate::{procedures, types::speaker::Song, GlobalState};

#[derive(Debug, Deserialize, Apiv2Schema)]
struct PlayAudioArgs {
    speaker: String,
    song: Song,
}

#[api_v2_operation]
#[post("/api/play_audio")]
pub fn play_audio(gs: Data<GlobalState>, body: Json<PlayAudioArgs>) -> Result<Json<bool>, Error> {
    let speakers = gs.speakers.clone();
    let mut speakers_lock = speakers.lock().map_err(|e| {
        error!("Error getting speakers_lock: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Error getting speakers_lock: {}", e))
    })?;

    procedures::play_audio::play_audio(&mut speakers_lock, &body.speaker, body.song.clone())
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(Json(true))
}
