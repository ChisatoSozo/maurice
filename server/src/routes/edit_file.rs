use actix_web::{web::Json, Error, HttpRequest};

use base64::{engine::general_purpose, Engine as _};
use log::error;
use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::Deserialize;

use crate::procedures;

#[derive(Debug, Deserialize, Apiv2Schema)]
struct EditFileArgs {
    path: String,
    content: String,
}

#[api_v2_operation]
#[post("/api/edit_file")]
pub async fn edit_file(body: Json<EditFileArgs>, req: HttpRequest) -> Result<Json<bool>, Error> {
    let file_content = general_purpose::STANDARD
        .decode(body.content.as_bytes())
        .map_err(|e| {
            error!("Error decoding base64: {}", e);
            actix_web::error::ErrorInternalServerError(format!("Error decoding base64: {}", e))
        })?;
    let requestor_ip = req
        .connection_info()
        .peer_addr()
        .ok_or("No peer address")
        .map_err(|e| {
            error!("Error getting peer address: {}", e);
            actix_web::error::ErrorInternalServerError(format!("Error getting peer address: {}", e))
        })?
        .to_string();
    procedures::edit_file::edit_file(requestor_ip, body.path.clone(), file_content)
        .await
        .map_err(|e| {
            error!("Error editing file: {}", e);
            actix_web::error::ErrorInternalServerError(format!("Error editing file: {}", e))
        })?;

    Ok(Json(true))
}
