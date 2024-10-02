use std::error::Error;

use crate::types::mpv_handler::{MpvRequest, MpvSend};

pub async fn get_speakers(send: &MpvSend) -> Result<Vec<String>, Box<dyn Error>> {
    send.list_devices().await
}
