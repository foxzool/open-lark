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

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_create_ticket_body_serialize() {
        let body = CreateTicketBody {
            title: "无法登录".to_string(),
            description: Some("账号被锁定".to_string()),
            priority: Some("high".to_string()),
        };

        let value = serde_json::to_value(body).expect("serialize body");
        assert_eq!(
            value,
            json!({
                "title": "无法登录",
                "description": "账号被锁定",
                "priority": "high"
            })
        );
    }

    #[test]
    fn test_ticket_list_response_deserialize() {
        let value = json!({
            "tickets": [
                {
                    "ticket_id": "t_1",
                    "title": "无法登录",
                    "status": "open"
                }
            ],
            "page_token": "next"
        });

        let resp: TicketListResponse =
            serde_json::from_value(value).expect("deserialize ticket list");
        assert_eq!(resp.tickets.len(), 1);
        assert_eq!(resp.tickets[0].status, "open");
    }
}
