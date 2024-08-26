use actix_web::{
    web::{Data, Json},
    Error,
};
use paperclip::actix::{api_v2_operation, post};
use crate::{
    function_types::types::{SendChatArgs, SendChatReturn},
    GlobalState,
};

#[api_v2_operation]
#[post("/api/send_chat")]
pub async fn send_chat(
    gs: Data<GlobalState>,
    body: Json<SendChatArgs>,
) -> Result<Json<SendChatReturn>, Error> {
    Ok(Json(gs.python.send_chat(body.into_inner()).await?))
}
