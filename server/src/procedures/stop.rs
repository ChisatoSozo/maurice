use std::error::Error;

use crate::types::mpv_handler::{MpvRequest, MpvSend};

pub async fn stop(send: &MpvSend, speaker: &str) -> Result<(), Box<dyn Error>> {
    send.stop(speaker).await
}
