use actix_web::{
    web::{Data, Json},
    Error,
};
use paperclip::actix::{api_v2_operation, post};
use crate::{
    function_types::types::{ListChatModelsArgs, ListChatModelsReturn},
    GlobalState,
};

#[api_v2_operation]
#[post("/api/list_chat_models")]
pub async fn list_chat_models(
    gs: Data<GlobalState>,
    body: Json<ListChatModelsArgs>,
) -> Result<Json<ListChatModelsReturn>, Error> {
    Ok(Json(gs.python.list_chat_models(body.into_inner()).await?))
}
