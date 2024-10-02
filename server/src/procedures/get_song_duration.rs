use std::error::Error;

use crate::types::mpv_handler::{MpvRequest, MpvSend};

pub async fn get_song_duration(send: &MpvSend, speaker: &str) -> Result<f64, Box<dyn Error>> {
    send.get_duration(speaker).await
}
