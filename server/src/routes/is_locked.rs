use actix_web::{web::Json, Error, HttpRequest};

use log::error;
use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::{Deserialize, Serialize};

use crate::procedures;

#[derive(Debug, Deserialize, Apiv2Schema)]
struct EditFileArgs {
    path: String,
}

#[derive(Debug, Serialize, Apiv2Schema)]
struct EditFileResponse {
    locked: bool,
    locked_by_me: bool,
}

#[api_v2_operation]
#[post("/api/is_locked")]
pub async fn is_locked(
    body: Json<EditFileArgs>,
    req: HttpRequest,
) -> Result<Json<EditFileResponse>, Error> {
    let requestor_ip = req
        .connection_info()
        .peer_addr()
        .ok_or("No peer address")
        .map_err(|e| {
            error!("Error getting peer address: {}", e);
            actix_web::error::ErrorInternalServerError(format!("Error getting peer address: {}", e))
        })?
        .to_string();
    let locked = procedures::edit_file::is_locked(&body.path, &requestor_ip).map_err(|e| {
        error!("Error editing file: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Error editing file: {}", e))
    })?;

    Ok(Json(EditFileResponse {
        locked: locked.0,
        locked_by_me: locked.1,
    }))
}
