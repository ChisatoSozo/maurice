use actix_web::{
    web::{Data, Json},
    Error,
};

use log::error;
use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::Serialize;

use crate::{
    procedures::{get_audio_devices::AudioDevice, refresh_speakers::refresh_speakers},
    GlobalState,
};

#[derive(Debug, Serialize, Apiv2Schema)]
struct GetSpeakersReturn {
    devices: Vec<AudioDevice>,
}

#[api_v2_operation]
#[post("/api/get_speakers")]
pub fn get_speakers(gs: Data<GlobalState>) -> Result<Json<GetSpeakersReturn>, Error> {
    let speakers = gs.speakers.clone();
    let mut speakers_lock = speakers.lock().map_err(|e| {
        error!("Error getting speakers_lock: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Error getting speakers_lock: {}", e))
    })?;

    refresh_speakers(&mut speakers_lock).expect("Failed to refresh speakers");

    let devices = speakers_lock
        .list_speakers()
        .iter()
        .map(|name| AudioDevice { name: name.clone() })
        .collect();

    Ok(Json(GetSpeakersReturn { devices }))
}
