pub mod answer_user_query;
pub mod create;
pub mod customized_fields;
pub mod get;
pub mod list;
pub mod message;
pub mod models;
pub mod start_service;
pub mod ticket_image;
pub mod update;

use openlark_core::config::Config;
use std::sync::Arc;

#[derive(Clone)]
pub struct Ticket {
    config: Arc<Config>,
}

impl Ticket {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn create(&self) -> create::CreateTicketRequest {
        create::CreateTicketRequest::new(self.config.clone())
    }

    pub fn get(&self, ticket_id: impl Into<String>) -> get::GetTicketRequest {
        get::GetTicketRequest::new(self.config.clone(), ticket_id.into())
    }

    pub fn update(&self, ticket_id: impl Into<String>) -> update::UpdateTicketRequest {
        update::UpdateTicketRequest::new(self.config.clone(), ticket_id.into())
    }

    pub fn list(&self) -> list::TicketListRequest {
        list::TicketListRequest::new(self.config.clone())
    }

    pub fn start_service(&self) -> start_service::StartServiceRequest {
        start_service::StartServiceRequest::new(self.config.clone())
    }

    pub fn answer_user_query(
        &self,
        ticket_id: impl Into<String>,
    ) -> answer_user_query::AnswerUserQueryRequest {
        answer_user_query::AnswerUserQueryRequest::new(self.config.clone(), ticket_id)
    }

    pub fn ticket_image(
        &self,
        ticket_id: impl Into<String>,
        image_key: impl Into<String>,
    ) -> ticket_image::GetTicketImageRequest {
        ticket_image::GetTicketImageRequest::new(self.config.clone(), ticket_id, image_key)
    }

    pub fn customized_fields(&self) -> customized_fields::GetCustomizedFieldsRequest {
        customized_fields::GetCustomizedFieldsRequest::new(self.config.clone())
    }

    #[allow(mismatched_lifetime_syntaxes)]
    pub fn message(&self) -> message::TicketMessage<'_> {
        message::TicketMessage::new(self)
    }
}

pub use answer_user_query::AnswerUserQueryRequest;
pub use create::CreateTicketRequest;
pub use customized_fields::GetCustomizedFieldsRequest;
pub use get::GetTicketRequest;
pub use list::TicketListRequest;
pub use message::{
    CreateTicketMessageRequest, CreateTicketMessageRequestBuilder, ListTicketMessageRequest,
    ListTicketMessageRequestBuilder,
};
pub use models::*;
pub use start_service::StartServiceRequest;
pub use ticket_image::GetTicketImageRequest;
pub use update::UpdateTicketRequest;
