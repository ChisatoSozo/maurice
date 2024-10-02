use std::error::Error;

use crate::types::mpv_handler::{MpvRequest, MpvSend};

pub async fn set_volume(send: &MpvSend, speaker: &str, volume: f64) -> Result<(), Box<dyn Error>> {
    send.set_volume(speaker, volume).await
}
