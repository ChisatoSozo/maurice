use std::path::Path;

use actix_web::{web::Json, Error};

use log::error;
use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::{Deserialize, Serialize};

use crate::procedures::{self};

#[derive(Debug, Deserialize, Apiv2Schema)]
struct ListFilesArgs {
    path: String,
}

#[derive(Debug, Serialize, Apiv2Schema)]
struct ListFilesResponse {
    files: Vec<String>,
}

#[api_v2_operation]
#[post("/api/list_files")]
pub async fn list_files(body: Json<ListFilesArgs>) -> Result<Json<ListFilesResponse>, Error> {
    let files =
        procedures::list_files::list_files(&Path::new(&("./files".to_string() + &body.path)))
            .map_err(|e| {
                error!("Error listing files: {}", e);
                actix_web::error::ErrorInternalServerError(format!("Error listing files: {}", e))
            })?;

    Ok(Json(ListFilesResponse { files }))
}
