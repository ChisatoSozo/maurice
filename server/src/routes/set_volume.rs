use actix_web::{
    web::{Data, Json},
    Error,
};

use log::error;
use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::Deserialize;

use crate::{procedures, GlobalState};

#[derive(Debug, Deserialize, Apiv2Schema)]
struct SetVolumeArgs {
    speaker: String,
    volume: f64,
}

#[api_v2_operation]
#[post("/api/set_volume")]
pub fn set_volume(gs: Data<GlobalState>, body: Json<SetVolumeArgs>) -> Result<Json<bool>, Error> {
    let speakers = gs.speakers.clone();
    let mut speakers_lock = speakers.lock().map_err(|e| {
        error!("Error getting speakers_lock: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Error getting speakers_lock: {}", e))
    })?;

    procedures::set_volume::set_volume(&mut speakers_lock, &body.speaker, body.volume.clone())
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(Json(true))
}
