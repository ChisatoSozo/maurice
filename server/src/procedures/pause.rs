use std::error::Error;

use crate::types::speaker::MultiSpeaker;

pub fn pause(speakers: &mut MultiSpeaker, speaker_name: &str) -> Result<(), Box<dyn Error>> {
    Ok(speakers.pause(speaker_name)?)
}
