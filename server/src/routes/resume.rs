use actix_web::{
    web::{Data, Json},
    Error,
};

use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::Deserialize;

use crate::{
    procedures,
    GlobalState,
};

#[derive(Debug, Deserialize, Apiv2Schema)]
struct ResumeArgs {
    speaker: String,
}

#[api_v2_operation]
#[post("/api/resume")]
pub fn resume(gs: Data<GlobalState>, body: Json<ResumeArgs>) -> Result<Json<bool>, Error> {
    let send = &gs.mpv_send;

    procedures::resume::resume(send, &body.speaker).await?;

    Ok(Json(true))
}
