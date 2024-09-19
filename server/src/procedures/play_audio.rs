use std::error::Error;

use crate::types::speaker::{MultiSpeaker, Song};

pub fn play_audio(
    speakers: &mut MultiSpeaker,
    speaker_name: &str,
    song: Song,
) -> Result<(), Box<dyn Error>> {
    let names = speakers.list_speakers();
    println!("Speakers: {:?}", names);
    if !speakers.has_speaker(speaker_name) {
        return Err("Speaker not found".into());
    }

    speakers.play(speaker_name, song)?;

    Ok(())
}
