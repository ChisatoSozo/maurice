use actix_web::{
    web::{Data, Json},
    Error,
};

use log::error;
use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::Deserialize;

use crate::{procedures::refresh_speakers::refresh_speakers, GlobalState};

#[derive(Debug, Deserialize, Apiv2Schema)]
struct ResumeArgs {
    speaker: String,
}

#[api_v2_operation]
#[post("/api/resume")]
pub fn resume(gs: Data<GlobalState>, body: Json<ResumeArgs>) -> Result<Json<bool>, Error> {
    let speakers = gs.speakers.clone();
    let mut speakers_lock = speakers.lock().map_err(|e| {
        error!("Error getting speakers_lock: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Error getting speakers_lock: {}", e))
    })?;

    refresh_speakers(&mut speakers_lock).expect("Failed to refresh speakers");

    speakers_lock.resume(&body.speaker).map_err(|e| {
        error!("Error resuming: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Error resuming: {}", e))
    })?;

    Ok(Json(true))
}
