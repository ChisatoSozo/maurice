use std::error::Error;

use crate::types::speaker::MultiSpeaker;

pub fn set_song_time(
    speakers: &mut MultiSpeaker,
    speaker_name: &str,
    time: f64,
) -> Result<(), Box<dyn Error>> {
    Ok(speakers.set_song_time(speaker_name, time)?)
}
