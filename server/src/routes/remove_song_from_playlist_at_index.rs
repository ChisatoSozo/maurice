use actix_web::{
    web::{Data, Json},
    Error,
};

use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::Deserialize;

use crate::{procedures, GlobalState};

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
    let send = &gs.mpv_send;

    procedures::remove_song_from_playlist_at_index::remove_song_from_playlist_at_index(
        send,
        &body.speaker,
        body.index,
    )
    .await?;

    Ok(Json(true))
}
