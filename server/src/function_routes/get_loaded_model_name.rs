use crate::{
    function_types::types::{GetLoadedModelNameArgs, GetLoadedModelNameReturn},
    GlobalState,
};
use actix_web::{
    web::{Data, Json},
    Error,
};
use paperclip::actix::{api_v2_operation, post};

#[api_v2_operation]
#[post("/api/get_loaded_model_name")]
pub async fn get_loaded_model_name(
    gs: Data<GlobalState>,
    body: Json<GetLoadedModelNameArgs>,
) -> Result<Json<GetLoadedModelNameReturn>, Error> {
    Ok(Json(
        gs.python.get_loaded_model_name(body.into_inner()).await?,
    ))
}
