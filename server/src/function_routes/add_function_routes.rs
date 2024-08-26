use actix_web::dev::{ServiceFactory, ServiceRequest};
use paperclip::actix::App;

use super::get_loaded_model_name::get_loaded_model_name;
use super::list_chat_models::list_chat_models;
use super::load_model::load_model;
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
            .service(get_loaded_model_name)
            .service(list_chat_models)
            .service(load_model)
            .service(send_chat)
    }
}
