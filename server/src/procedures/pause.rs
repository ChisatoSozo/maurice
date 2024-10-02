use std::error::Error;

use crate::types::mpv_handler::{MpvRequest, MpvSend};

pub async fn pause(send: &MpvSend, speaker: &str) -> Result<(), Box<dyn Error>> {
    send.pause(speaker).await
}
