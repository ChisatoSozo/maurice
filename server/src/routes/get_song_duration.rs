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
struct GetSongDurationReturn {
    duration: f64,
}

#[api_v2_operation]
#[post("/api/get_song_duration")]
pub fn get_song_duration(
    gs: Data<GlobalState>,
    body: Json<GetSongTimeArgs>,
) -> Result<Json<GetSongDurationReturn>, Error> {
    let send = &gs.mpv_send;

    procedures::refresh_speakers::refresh_speakers(send).await?;

    let duration = procedures::get_song_duration::get_song_duration(send, &body.speaker).await?;

    Ok(Json(GetSongDurationReturn { duration }))
}
