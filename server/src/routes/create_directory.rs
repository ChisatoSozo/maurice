use actix_web::{web::Json, Error};

use log::error;
use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::Deserialize;

use crate::procedures;

#[derive(Debug, Deserialize, Apiv2Schema)]
struct CreateDirectoryArgs {
    path: String,
}

#[api_v2_operation]
#[post("/api/create_directory")]
pub async fn create_directory(body: Json<CreateDirectoryArgs>) -> Result<Json<bool>, Error> {
    procedures::create_directory::create_directory(body.path.clone()).map_err(|e| {
        error!("Error creating directory: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Error creating directory: {}", e))
    })?;

    Ok(Json(true))
}
