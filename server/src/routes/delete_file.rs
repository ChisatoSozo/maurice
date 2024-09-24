use actix_web::{web::Json, Error, HttpRequest};

use log::error;
use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::Deserialize;

use crate::procedures;

#[derive(Debug, Deserialize, Apiv2Schema)]
struct DeleteFileArgs {
    path: String,
}

#[api_v2_operation]
#[post("/api/delete_file")]
pub async fn delete_file(
    body: Json<DeleteFileArgs>,
    req: HttpRequest,
) -> Result<Json<bool>, Error> {
    let requestor_ip = req
        .connection_info()
        .peer_addr()
        .ok_or("No peer address")
        .map_err(|e| {
            error!("Error getting peer address: {}", e);
            actix_web::error::ErrorInternalServerError(format!("Error getting peer address: {}", e))
        })?
        .to_string();
    procedures::delete_file::delete_file(requestor_ip.clone(), body.path.clone()).map_err(|e| {
        error!("Error checking if file locked: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Error checking if file locked: {}", e))
    })?;

    Ok(Json(true))
}
