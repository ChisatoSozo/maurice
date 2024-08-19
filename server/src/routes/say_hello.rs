use actix_web::{web::Json, Error};
use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Apiv2Schema)]
struct SayHelloInput {
    pub name: String,
}

#[derive(Serialize, Apiv2Schema)]
struct SayHelloOutput {
    pub message: String,
}

#[api_v2_operation]
#[post("/api/say_hello")]
pub fn say_hello(body: Json<SayHelloInput>) -> Result<Json<SayHelloOutput>, Error> {
    let name = body.into_inner().name;
    Ok(Json(SayHelloOutput {
        message: format!("Hello, {}!", name),
    }))
}
