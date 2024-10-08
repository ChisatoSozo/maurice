use actix_web::{web::Json, Error, HttpRequest};

use log::error;
use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::{Deserialize, Serialize};

use crate::procedures;

#[derive(Debug, Deserialize, Apiv2Schema)]
struct IsLockedArgs {
    path: String,
}

#[derive(Debug, Serialize, Apiv2Schema)]
struct IsLockedResponse {
    locked: bool,
    locked_by_me: bool,
}

#[api_v2_operation]
#[post("/api/is_locked")]
pub async fn is_locked(
    body: Json<IsLockedArgs>,
    req: HttpRequest,
) -> Result<Json<IsLockedResponse>, Error> {
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
        error!("Error checking if file locked: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Error checking if file locked: {}", e))
    })?;

    Ok(Json(IsLockedResponse {
        locked: locked.0,
        locked_by_me: locked.1,
    }))
}
