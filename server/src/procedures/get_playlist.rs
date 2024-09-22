use std::error::Error;

use crate::types::speaker::{MultiSpeaker, Song};

pub fn get_playlist(
    speakers: &mut MultiSpeaker,
    speaker_name: &str,
) -> Result<Vec<Song>, Box<dyn Error>> {
    Ok(speakers.get_playlist(speaker_name)?)
}
