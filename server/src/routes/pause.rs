use actix_web::{
    web::{Data, Json},
    Error,
};

use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::Deserialize;

use crate::{procedures, GlobalState};

#[derive(Debug, Deserialize, Apiv2Schema)]
struct PauseArgs {
    speaker: String,
}

#[api_v2_operation]
#[post("/api/pause")]
pub fn pause(gs: Data<GlobalState>, body: Json<PauseArgs>) -> Result<Json<bool>, Error> {
    let send = &gs.mpv_send;

    procedures::pause::pause(send, &body.speaker).await?;

    Ok(Json(true))
}
