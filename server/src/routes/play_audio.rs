use actix_web::{
    web::{Data, Json},
    Error,
};

use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::Deserialize;

use crate::{procedures, types::mpv_handler::Song, GlobalState};

#[derive(Debug, Deserialize, Apiv2Schema)]
struct PlayAudioArgs {
    speaker: String,
    song: Song,
}

#[api_v2_operation]
#[post("/api/play_audio")]
pub fn play_audio(gs: Data<GlobalState>, body: Json<PlayAudioArgs>) -> Result<Json<bool>, Error> {
    let send = &gs.mpv_send;

    procedures::play_audio::play_audio(send, &body.speaker, body.song.clone()).await?;

    Ok(Json(true))
}
