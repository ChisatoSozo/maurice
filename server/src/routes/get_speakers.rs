use actix_web::{
    web::{Data, Json},
    Error,
};

use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::Serialize;

use crate::{
    procedures::{self, get_audio_devices::AudioDevice, refresh_speakers::refresh_speakers},
    GlobalState,
};

#[derive(Debug, Serialize, Apiv2Schema)]
struct GetSpeakersReturn {
    devices: Vec<AudioDevice>,
}

#[api_v2_operation]
#[post("/api/get_speakers")]
pub fn get_speakers(gs: Data<GlobalState>) -> Result<Json<GetSpeakersReturn>, Error> {
    let send = &gs.mpv_send;
    refresh_speakers(send).await?;
    let devices = procedures::get_speakers::get_speakers(send).await?;

    Ok(Json(GetSpeakersReturn {
        devices: devices
            .iter()
            .map(|d| AudioDevice { name: d.clone() })
            .collect(),
    }))
}
