use actix_web::{
    web::{Data, Json},
    Error,
};

use log::error;
use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::{Deserialize, Serialize};

use crate::{procedures::refresh_speakers::refresh_speakers, GlobalState};

#[derive(Debug, Deserialize, Apiv2Schema)]
struct GetVolumeArgs {
    speaker: String,
}

#[derive(Debug, Serialize, Apiv2Schema)]
struct GetVolumeReturn {
    volume: f64,
}

#[api_v2_operation]
#[post("/api/get_volume")]
pub fn get_volume(
    gs: Data<GlobalState>,
    body: Json<GetVolumeArgs>,
) -> Result<Json<GetVolumeReturn>, Error> {
    let speakers = gs.speakers.clone();
    let mut speakers_lock = speakers.lock().map_err(|e| {
        error!("Error getting speakers_lock: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Error getting speakers_lock: {}", e))
    })?;

    refresh_speakers(&mut speakers_lock).expect("Failed to refresh speakers");

    let volume = speakers_lock.get_volume(&body.speaker).map_err(|e| {
        error!("Error getting volume: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Error getting volume: {}", e))
    })?;

    Ok(Json(GetVolumeReturn { volume }))
}
