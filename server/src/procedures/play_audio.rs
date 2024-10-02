use std::error::Error;

use crate::types::mpv_handler::{MpvRequest, MpvSend, Song};

pub async fn play_audio(send: &MpvSend, speaker: &str, song: Song) -> Result<(), Box<dyn Error>> {
    send.add_song_to_playlist(speaker, song).await?;
    send.play(speaker).await
}
