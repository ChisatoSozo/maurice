use actix_web::{
    web::{Data, Json},
    Error,
};

use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::{Deserialize, Serialize};

use crate::{procedures, GlobalState};

#[derive(Debug, Deserialize, Apiv2Schema)]
struct GetSongTimeArgs {
    speaker: String,
}

#[derive(Debug, Serialize, Apiv2Schema)]
struct GetSongTimeReturn {
    time: f64,
}

#[api_v2_operation]
#[post("/api/get_song_time")]
pub fn get_song_time(
    gs: Data<GlobalState>,
    body: Json<GetSongTimeArgs>,
) -> Result<Json<GetSongTimeReturn>, Error> {
    let send = &gs.mpv_send;

    procedures::refresh_speakers::refresh_speakers(send).await?;

    let time = procedures::get_song_time::get_song_time(send, &body.speaker).await?;

    Ok(Json(GetSongTimeReturn { time }))
}
