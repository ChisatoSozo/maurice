use actix_web::{
    web::{Data, Json},
    Error,
};

use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::Deserialize;

use crate::{
    procedures,
    GlobalState,
};

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
    let send = &gs.mpv_send;

    procedures::set_song_time::set_song_time(send, &body.speaker, body.song_time).await?;

    Ok(Json(true))
}
