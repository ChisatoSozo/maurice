use std::error::Error;

use crate::types::speaker::MultiSpeaker;

pub fn set_volume(
    speakers: &mut MultiSpeaker,
    speaker_name: &str,
    volume: f64,
) -> Result<(), Box<dyn Error>> {
    Ok(speakers.set_volume(speaker_name, volume)?)
}
