use std::error::Error;

use crate::{
    procedures::get_audio_devices::get_audio_devices, types::mpv_handler::MpvRequest, MpvSend,
};

pub async fn refresh_speakers(send: &MpvSend) -> Result<(), Box<dyn Error>> {
    let devices = get_audio_devices()?;
    let speaker_names: Vec<String> = devices.into_iter().map(|device| device.name).collect();
    let mpv_speaker_names = send.list_devices().await?;

    // Remove speakers that are no longer available
    for speaker_name in mpv_speaker_names.iter() {
        if !speaker_names.contains(speaker_name) {
            send.remove_device(speaker_name).await?;
        }
    }
    // Add speakers that are new
    for speaker_name in speaker_names.iter() {
        if !mpv_speaker_names.contains(speaker_name) {
            send.add_device(speaker_name).await?;
        }
    }

    //list speakers again to make sure they are all there
    let mpv_speaker_names = send.list_devices().await?;

    //make a set from each list and compare them
    let speaker_names_set: std::collections::HashSet<String> = speaker_names.into_iter().collect();
    let mpv_speaker_names_set: std::collections::HashSet<String> =
        mpv_speaker_names.into_iter().collect();

    if speaker_names_set != mpv_speaker_names_set {
        return Err("Speakers do not match".into());
    }

    Ok(())
}
