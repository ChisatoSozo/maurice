use std::error::Error;

use crate::types::mpv_handler::{MpvRequest, MpvSend};

pub async fn set_song_time(send: &MpvSend, speaker: &str, time: f64) -> Result<(), Box<dyn Error>> {
    send.seek(speaker, time).await
}
