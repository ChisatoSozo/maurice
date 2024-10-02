use actix_web::{
    web::{Data, Json},
    Error,
};

use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::Deserialize;

use crate::{procedures, GlobalState};

#[derive(Debug, Deserialize, Apiv2Schema)]
struct StopArgs {
    speaker: String,
}

#[api_v2_operation]
#[post("/api/stop")]
pub fn stop(gs: Data<GlobalState>, body: Json<StopArgs>) -> Result<Json<bool>, Error> {
    let send = &gs.mpv_send;

    procedures::stop::stop(send, &body.speaker).await?;

    Ok(Json(true))
}
