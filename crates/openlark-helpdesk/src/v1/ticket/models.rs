use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateTicketBody {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTicketResponse {
    pub ticket_id: String,
    pub title: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetTicketResponse {
    pub ticket_id: String,
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateTicketBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTicketResponse {
    pub ticket_id: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteTicketResponse {
    pub success: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TicketListResponse {
    pub tickets: Vec<TicketItem>,
    #[serde(default)]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TicketItem {
    pub ticket_id: String,
    pub title: String,
    pub status: String,
}
