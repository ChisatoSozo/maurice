use std::error::Error;

use crate::types::speaker::MultiSpeaker;

pub fn resume(speakers: &mut MultiSpeaker, speaker_name: &str) -> Result<(), Box<dyn Error>> {
    Ok(speakers.resume(speaker_name)?)
}
