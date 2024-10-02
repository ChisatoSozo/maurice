use std::error::Error;

use crate::types::mpv_handler::{MpvRequest, MpvSend};

pub async fn next_song(send: &MpvSend, speaker: &str) -> Result<(), Box<dyn Error>> {
    send.next(speaker).await
}
