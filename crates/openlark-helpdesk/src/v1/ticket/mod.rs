pub mod create;
pub mod get;
pub mod list;
pub mod models;
pub mod update;
pub mod message;

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

    pub fn message(&self) -> message::TicketMessage {
        message::TicketMessage::new(self)
    }
}

pub use create::CreateTicketRequest;
pub use get::GetTicketRequest;
pub use list::TicketListRequest;
pub use message::{CreateTicketMessageRequest, CreateTicketMessageRequestBuilder, ListTicketMessageRequest, ListTicketMessageRequestBuilder};
pub use models::*;
pub use update::UpdateTicketRequest;
