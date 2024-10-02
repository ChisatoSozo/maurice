use std::error::Error;

use crate::types::mpv_handler::{MpvRequest, MpvSend};

pub async fn resume(send: &MpvSend, speaker: &str) -> Result<(), Box<dyn Error>> {
    send.play(speaker).await
}
