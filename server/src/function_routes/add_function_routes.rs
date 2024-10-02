use actix_web::dev::{ServiceFactory, ServiceRequest};
use paperclip::actix::App;

use super::complete::complete;
use super::get_loaded_chat_model_name::get_loaded_chat_model_name;
use super::get_loaded_completion_model_name::get_loaded_completion_model_name;
use super::list_chat_models::list_chat_models;
use super::list_completion_models::list_completion_models;
use super::load_chat_model::load_chat_model;
use super::load_completion_model::load_completion_model;
use super::send_chat::send_chat;

pub trait AddFunctionRoutes {
    fn add_function_routes(self) -> Self;
}

impl<T> AddFunctionRoutes for App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = actix_web::Error, InitError = ()>,
{
    fn add_function_routes(self) -> Self {
        self
            .service(complete)
            .service(get_loaded_chat_model_name)
            .service(get_loaded_completion_model_name)
            .service(list_chat_models)
            .service(list_completion_models)
            .service(load_chat_model)
            .service(load_completion_model)
            .service(send_chat)
    }
}
