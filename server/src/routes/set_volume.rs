use actix_web::{
    web::{Data, Json},
    Error,
};

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
    let send = &gs.mpv_send;

    procedures::set_volume::set_volume(send, &body.speaker, body.volume).await?;

    Ok(Json(true))
}
