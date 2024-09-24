use actix_web::{web::Json, Error};

use log::error;
use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::Deserialize;

use crate::procedures;

#[derive(Debug, Deserialize, Apiv2Schema)]
struct CreateFileArgs {
    path: String,
}

#[api_v2_operation]
#[post("/api/create_file")]
pub async fn create_file(body: Json<CreateFileArgs>) -> Result<Json<bool>, Error> {
    procedures::create_file::create_file(body.path.clone()).map_err(|e| {
        error!("Error creating file: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Error creating file: {}", e))
    })?;

    Ok(Json(true))
}
