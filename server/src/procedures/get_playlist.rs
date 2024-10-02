use std::error::Error;

use crate::types::mpv_handler::{MpvRequest, MpvSend, Song};

pub async fn get_playlist(send: &MpvSend, speaker: &str) -> Result<Vec<Song>, Box<dyn Error>> {
    send.list_playlist(speaker).await
}
