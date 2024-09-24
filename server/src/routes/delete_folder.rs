use actix_web::{web::Json, Error};

use log::error;
use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::Deserialize;

use crate::procedures;

#[derive(Debug, Deserialize, Apiv2Schema)]
struct DeleteFolderArgs {
    path: String,
}

#[api_v2_operation]
#[post("/api/delete_folder")]
pub async fn delete_folder(body: Json<DeleteFolderArgs>) -> Result<Json<bool>, Error> {
    procedures::delete_folder::delete_folder(body.path.clone()).map_err(|e| {
        error!("Error deleting folder: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Error deleting folder: {}", e))
    })?;

    Ok(Json(true))
}
