use std::error::Error;

use crate::types::speaker::{MultiSpeaker, Song};

pub fn append_song_to_playlist(
    speakers: &mut MultiSpeaker,
    speaker_name: &str,
    song: Song,
) -> Result<(), Box<dyn Error>> {
    Ok(speakers.append_song_to_playlist(speaker_name, song)?)
}
