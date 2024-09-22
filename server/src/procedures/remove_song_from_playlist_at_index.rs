use std::error::Error;

use crate::types::speaker::MultiSpeaker;

pub fn remove_song_from_playlist_at_index(
    speakers: &mut MultiSpeaker,
    speaker_name: &str,
    index: usize,
) -> Result<(), Box<dyn Error>> {
    Ok(speakers.remove_song_from_playlist_at_index(speaker_name, index)?)
}
