use crate::{
    model::{Content, Message},
    schema::{contents, message_contents, messages},
    GlobalState,
};
use actix_web::{
    web::{Data, Json},
    Error,
};
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use log::error;
use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Apiv2Schema)]
pub struct GetHistoryArgs {
    pub num_messages: usize,
    pub include_voice: bool,
}

#[derive(Debug, Serialize, Apiv2Schema)]
pub struct HistoryData {
    pub data: Vec<u8>,
    pub mime_type: String,
}

#[derive(Debug, Serialize, Apiv2Schema)]
pub struct HistoryMessage {
    pub id: i32,
    pub user_id: Option<i32>,
    pub timestamp: usize,
    pub message: String,
    pub datas: Vec<HistoryData>,
}

#[derive(Debug, Serialize, Apiv2Schema)]
pub struct GetHistoryReturn {
    pub messages: Vec<HistoryMessage>,
}

#[api_v2_operation]
#[post("/api/get_history")]
pub async fn get_history(
    gs: Data<GlobalState>,
    body: Json<GetHistoryArgs>,
) -> Result<Json<GetHistoryReturn>, Error> {
    let db = gs.db.clone();
    let mut pg_conn = db.lock().map_err(|e| {
        error!("Error getting db lock: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Error getting db lock: {}", e))
    })?;

    let messages = pg_conn
        .build_transaction()
        .read_only()
        .run(|pg_conn| {
            let messages: Vec<Message> = messages::table
                .select(messages::all_columns)
                .order(messages::timestamp.desc())
                .limit(body.num_messages as i64)
                .load(pg_conn)?;

            let history_messages: Result<Vec<HistoryMessage>, diesel::result::Error> = messages
                .into_iter()
                .map(|message| {
                    // Manually join message_contents and contents tables
                    let contents: Vec<Content> = message_contents::table
                        .inner_join(contents::table)
                        .filter(message_contents::message_id.eq(message.id))
                        .select(contents::all_columns)
                        .load(pg_conn)?;

                    let mut text_content = String::new();
                    let mut datas = Vec::new();

                    for content in contents {
                        match content.type_.as_str() {
                            "text" => {
                                if let Some(text) = content.text_content {
                                    text_content.push_str(&text);
                                    text_content.push('\n');
                                }
                            }
                            "voice" if body.include_voice => {
                                if let Some(binary_data) = content.binary_data {
                                    datas.push(HistoryData {
                                        data: binary_data,
                                        mime_type: content.mime_type,
                                    });
                                }
                            }
                            _ => {}
                        }
                    }

                    Ok(HistoryMessage {
                        id: message.id,
                        user_id: message.user_id,
                        timestamp: message.timestamp.timestamp() as usize,
                        message: text_content.trim().to_string(),
                        datas,
                    })
                })
                .collect();

            history_messages
        })
        .map_err(|e| {
            error!("Error getting history: {}", e);
            actix_web::error::ErrorInternalServerError(e)
        })?;

    Ok(Json(GetHistoryReturn { messages }))
}
