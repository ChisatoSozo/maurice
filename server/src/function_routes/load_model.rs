use actix_web::{
    web::{Data, Json},
    Error,
};
use paperclip::actix::{api_v2_operation, post};
use crate::{
    function_types::types::{LoadModelArgs, LoadModelReturn},
    GlobalState,
};

#[api_v2_operation]
#[post("/api/load_model")]
pub async fn load_model(
    gs: Data<GlobalState>,
    body: Json<LoadModelArgs>,
) -> Result<Json<LoadModelReturn>, Error> {
    Ok(Json(gs.python.load_model(body.into_inner()).await?))
}
