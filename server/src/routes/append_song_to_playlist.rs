use actix_web::{
    web::{Data, Json},
    Error,
};

use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::Deserialize;

use crate::{procedures, types::mpv_handler::Song, GlobalState};

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
    let send = &gs.mpv_send;

    procedures::refresh_speakers::refresh_speakers(send).await?;

    procedures::append_song_to_playlist::append_song_to_playlist(
        send,
        &body.speaker,
        body.song.clone(),
    )
    .await?;

    Ok(Json(true))
}
