#[derive(Debug, Clone, PartialEq)]
pub enum HelpdeskApiV1 {
    TicketCreate,
    TicketGet(String),
    TicketUpdate(String),
    TicketList,
}

impl HelpdeskApiV1 {
    pub fn to_url(&self) -> String {
        match self {
            HelpdeskApiV1::TicketCreate => "/open-apis/helpdesk/v1/tickets".to_string(),
            HelpdeskApiV1::TicketGet(id) => format!("/open-apis/helpdesk/v1/tickets/{}", id),
            HelpdeskApiV1::TicketUpdate(id) => format!("/open-apis/helpdesk/v1/tickets/{}", id),
            HelpdeskApiV1::TicketList => "/open-apis/helpdesk/v1/tickets".to_string(),
        }
    }
}
