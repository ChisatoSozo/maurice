use std::error::Error;

use crate::types::mpv_handler::{MpvRequest, MpvSend};

pub async fn remove_song_from_playlist_at_index(
    send: &MpvSend,
    speaker: &str,
    index: usize,
) -> Result<(), Box<dyn Error>> {
    send.remove_song(speaker, index).await
}
