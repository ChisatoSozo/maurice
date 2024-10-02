use actix_web::{
    web::{Data, Json},
    Error,
};

use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::{Deserialize, Serialize};

use crate::{procedures, types::mpv_handler::Song, GlobalState};

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
    let send = &gs.mpv_send;

    procedures::refresh_speakers::refresh_speakers(send).await?;

    let songs = procedures::get_playlist::get_playlist(send, &body.speaker).await?;

    Ok(Json(GetPlaylistReturn { songs }))
}
