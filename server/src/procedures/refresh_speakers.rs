use std::error::Error;

use crate::{procedures::get_audio_devices::get_audio_devices, types::speaker::MultiSpeaker};

pub fn refresh_speakers(speakers: &mut MultiSpeaker) -> Result<(), Box<dyn Error>> {
    let devices = get_audio_devices()?;
    let speaker_names: Vec<String> = devices.into_iter().map(|device| device.name).collect();
    for speaker_name in speaker_names {
        if !speakers.has_speaker(&speaker_name) {
            speakers.add_speaker(speaker_name.clone(), speaker_name)?;
        }
    }
    Ok(())
}
