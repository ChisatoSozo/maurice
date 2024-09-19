use std::error::Error;

use crate::types::speaker::MultiSpeaker;

pub fn get_volume(speakers: &MultiSpeaker, speaker_name: &str) -> Result<f64, Box<dyn Error>> {
    Ok(speakers.get_volume(speaker_name)?)
}
