use actix_web::{
    web::{Data, Json},
    Error,
};

use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::{Deserialize, Serialize};

use crate::{procedures, GlobalState};

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
    let send = &gs.mpv_send;

    let volume = procedures::get_volume::get_volume(send, &body.speaker).await?;

    Ok(Json(GetVolumeReturn { volume }))
}
