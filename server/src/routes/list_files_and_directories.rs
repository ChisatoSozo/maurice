use std::path::Path;

use actix_web::{web::Json, Error};

use log::error;
use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::{Deserialize, Serialize};

use crate::procedures::{self, list_files_and_directories::FilesAndDirectories};

#[derive(Debug, Deserialize, Apiv2Schema)]
struct ListFilesAndDirectoriesArgs {
    path: String,
}

#[derive(Debug, Serialize, Apiv2Schema)]
struct ListFilesAndDirectoriesResponse {
    files_and_directories: FilesAndDirectories,
}

#[api_v2_operation]
#[post("/api/list_files_and_directories")]
pub async fn list_files_and_directories(
    body: Json<ListFilesAndDirectoriesArgs>,
) -> Result<Json<ListFilesAndDirectoriesResponse>, Error> {
    let files_and_directories = procedures::list_files_and_directories::list_files_directory(
        &Path::new(&("./files".to_string() + &body.path)),
    )
    .map_err(|e| {
        error!("Error listing files and directories: {}", e);
        actix_web::error::ErrorInternalServerError(format!(
            "Error listing files and directories: {}",
            e
        ))
    })?;

    Ok(Json(ListFilesAndDirectoriesResponse {
        files_and_directories,
    }))
}
