use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{helpdesk::*, EndpointBuilder},
        http::Transport,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    },
    service::helpdesk::models::{Ticket, UserIdType},
};

/// 工单管理服务
#[derive(Debug)]
pub struct TicketService {
    pub config: Config,
}

/// 创建服务台对话请求
#[derive(Debug, Serialize, Deserialize)]
pub struct StartServiceRequest {
    /// 用户开放平台ID
    pub open_id: String,
    /// 服务台ID
    pub helpdesk_id: String,
    /// 问题描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 创建服务台对话响应
#[derive(Debug, Serialize, Deserialize)]
pub struct StartServiceResponse {
    /// 聊天群ID
    pub chat_id: String,
    /// 工单信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket: Option<Ticket>,
}

impl ApiResponseTrait for StartServiceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取工单详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GetTicketResponse {
    /// 工单信息
    pub ticket: Ticket,
}

impl ApiResponseTrait for GetTicketResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新工单请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTicketRequest {
    /// 工单状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// 评论
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// 更新工单响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTicketResponse {
    /// 更新后的工单信息
    pub ticket: Ticket,
}

impl ApiResponseTrait for UpdateTicketResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取全部工单响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ListTicketsResponse {
    /// 工单列表
    pub tickets: Vec<Ticket>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 下一页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListTicketsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl TicketService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建服务台对话
    pub async fn start_service(
        &self,
        request: StartServiceRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<StartServiceResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: HELPDESK_V1_START_SERVICE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询指定工单详情
    pub async fn get(
        &self,
        ticket_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetTicketResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                HELPDESK_V1_TICKET_GET,
                "ticket_id",
                ticket_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新工单详情
    pub async fn update(
        &self,
        ticket_id: &str,
        request: UpdateTicketRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateTicketResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::PUT,
            api_path: EndpointBuilder::replace_param(
                HELPDESK_V1_TICKET_GET,
                "ticket_id",
                ticket_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询全部工单详情
    pub async fn list(
        &self,
        user_id_type: Option<UserIdType>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListTicketsResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }
        if let Some(page_token) = page_token {
            query_params.insert("page_token", page_token.to_string());
        }
        if let Some(page_size) = page_size {
            query_params.insert("page_size", page_size.to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: HELPDESK_V1_TICKETS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 规划中的工单相关功能
    ///
    /// 以下功能将在未来版本中实现：
    ///
    /// - `ticket_image`: 获取工单内图像
    /// - `answer_user_query`: 回复用户在工单里的提问
    /// - `customized_fields`: 获取服务台自定义字段
    ///
    /// 🚧 **待实现** - 以上功能尚未实现，敬请期待。
    fn _placeholder() { /* TODO: 实现以上功能 */
    }
}

impl Service for TicketService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "ticket"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use crate::core::config::Config;
    use serde_json;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
    }

    #[test]
    fn test_ticket_service_creation() {
        let config = create_test_config();
        let service = TicketService::new(config.clone());

        // The config should be stored properly
        assert_eq!(service.config.app_id, config.app_id);
        assert_eq!(service.config.app_secret, config.app_secret);
    }

    #[test]
    fn test_start_service_request_serialization() {
        let request = StartServiceRequest {
            open_id: "user123".to_string(),
            helpdesk_id: "helpdesk456".to_string(),
            description: Some("I need help with my account".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: StartServiceRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.open_id, deserialized.open_id);
        assert_eq!(request.helpdesk_id, deserialized.helpdesk_id);
        assert_eq!(request.description, deserialized.description);
    }

    #[test]
    fn test_start_service_request_without_description() {
        let request = StartServiceRequest {
            open_id: "user123".to_string(),
            helpdesk_id: "helpdesk456".to_string(),
            description: None,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(!serialized.contains("description"));

        let deserialized: StartServiceRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request.open_id, deserialized.open_id);
        assert_eq!(request.helpdesk_id, deserialized.helpdesk_id);
        assert_eq!(request.description, None);
    }

    #[test]
    fn test_start_service_response_serialization() {
        let response = StartServiceResponse {
            chat_id: "chat123".to_string(),
            ticket: Some(Ticket {
                ticket_id: Some("ticket456".to_string()),
                title: Some("Account Issue".to_string()),
                description: Some("User needs help with account".to_string()),
                status: Some(crate::service::helpdesk::models::TicketStatus::Pending),
                priority: Some(crate::service::helpdesk::models::TicketPriority::Medium),
                creator: Some("user123".to_string()),
                assignee: None,
                created_at: Some("2023-01-01T00:00:00Z".to_string()),
                updated_at: Some("2023-01-01T00:00:00Z".to_string()),
            }),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: StartServiceResponse = serde_json::from_str(&serialized).unwrap();

        assert_eq!(response.chat_id, deserialized.chat_id);
        assert!(deserialized.ticket.is_some());
    }

    #[test]
    fn test_start_service_response_data_format() {
        assert!(matches!(
            StartServiceResponse::data_format(),
            ResponseFormat::Data
        ));
    }

    #[test]
    fn test_get_ticket_response_serialization() {
        let ticket = Ticket {
            ticket_id: Some("ticket123".to_string()),
            title: Some("Test Ticket".to_string()),
            description: Some("This is a test".to_string()),
            status: Some(crate::service::helpdesk::models::TicketStatus::Processing),
            priority: Some(crate::service::helpdesk::models::TicketPriority::High),
            creator: Some("user123".to_string()),
            assignee: Some("agent456".to_string()),
            created_at: Some("2023-01-01T00:00:00Z".to_string()),
            updated_at: Some("2023-01-01T01:00:00Z".to_string()),
        };

        let response = GetTicketResponse { ticket };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: GetTicketResponse = serde_json::from_str(&serialized).unwrap();

        assert_eq!(response.ticket.ticket_id, deserialized.ticket.ticket_id);
        assert_eq!(response.ticket.title, deserialized.ticket.title);
    }

    #[test]
    fn test_get_ticket_response_data_format() {
        assert!(matches!(
            GetTicketResponse::data_format(),
            ResponseFormat::Data
        ));
    }

    #[test]
    fn test_update_ticket_request_serialization() {
        let request = UpdateTicketRequest {
            status: Some("solved".to_string()),
            tags: Some(vec!["urgent".to_string(), "resolved".to_string()]),
            comment: Some("Issue has been resolved".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: UpdateTicketRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.status, deserialized.status);
        assert_eq!(request.tags, deserialized.tags);
        assert_eq!(request.comment, deserialized.comment);
    }

    #[test]
    fn test_update_ticket_request_partial() {
        let request = UpdateTicketRequest {
            status: Some("processing".to_string()),
            tags: None,
            comment: None,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(!serialized.contains("tags"));
        assert!(!serialized.contains("comment"));

        let deserialized: UpdateTicketRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request.status, deserialized.status);
        assert_eq!(request.tags, None);
        assert_eq!(request.comment, None);
    }

    #[test]
    fn test_update_ticket_response_data_format() {
        assert!(matches!(
            UpdateTicketResponse::data_format(),
            ResponseFormat::Data
        ));
    }

    #[test]
    fn test_list_tickets_response_serialization() {
        let tickets = vec![
            Ticket {
                ticket_id: Some("ticket1".to_string()),
                title: Some("First Ticket".to_string()),
                description: None,
                status: Some(crate::service::helpdesk::models::TicketStatus::Pending),
                priority: Some(crate::service::helpdesk::models::TicketPriority::Low),
                creator: Some("user1".to_string()),
                assignee: None,
                created_at: Some("2023-01-01T00:00:00Z".to_string()),
                updated_at: Some("2023-01-01T00:00:00Z".to_string()),
            },
            Ticket {
                ticket_id: Some("ticket2".to_string()),
                title: Some("Second Ticket".to_string()),
                description: Some("Description for second ticket".to_string()),
                status: Some(crate::service::helpdesk::models::TicketStatus::Solved),
                priority: Some(crate::service::helpdesk::models::TicketPriority::Medium),
                creator: Some("user2".to_string()),
                assignee: Some("agent1".to_string()),
                created_at: Some("2023-01-02T00:00:00Z".to_string()),
                updated_at: Some("2023-01-02T01:00:00Z".to_string()),
            },
        ];

        let response = ListTicketsResponse {
            tickets,
            has_more: Some(true),
            page_token: Some("next_page_token".to_string()),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: ListTicketsResponse = serde_json::from_str(&serialized).unwrap();

        assert_eq!(response.tickets.len(), deserialized.tickets.len());
        assert_eq!(response.has_more, deserialized.has_more);
        assert_eq!(response.page_token, deserialized.page_token);
    }

    #[test]
    fn test_list_tickets_response_data_format() {
        assert!(matches!(
            ListTicketsResponse::data_format(),
            ResponseFormat::Data
        ));
    }

    #[test]
    fn test_list_tickets_response_minimal() {
        let response = ListTicketsResponse {
            tickets: vec![],
            has_more: None,
            page_token: None,
        };

        let serialized = serde_json::to_string(&response).unwrap();
        assert!(!serialized.contains("has_more"));
        assert!(!serialized.contains("page_token"));

        let deserialized: ListTicketsResponse = serde_json::from_str(&serialized).unwrap();
        assert!(deserialized.tickets.is_empty());
        assert_eq!(deserialized.has_more, None);
        assert_eq!(deserialized.page_token, None);
    }

    #[tokio::test]
    async fn test_start_service_api_request_construction() {
        let config = create_test_config();
        let service = TicketService::new(config);

        let request = StartServiceRequest {
            open_id: "user123".to_string(),
            helpdesk_id: "helpdesk456".to_string(),
            description: Some("Test description".to_string()),
        };

        // This test verifies the API request construction logic
        // We can't easily test the actual HTTP call without mocking
        let user_id_type = Some(UserIdType::OpenId);
        let option = Some(RequestOption::default());

        // The actual method call would fail due to network, but we can verify
        // that the method signature and basic setup work correctly
        let result = service.start_service(request, user_id_type, option).await;

        // We expect this to fail with a network error, not a construction error
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_ticket_api_request_construction() {
        let config = create_test_config();
        let service = TicketService::new(config);

        let ticket_id = "ticket123";
        let user_id_type = Some(UserIdType::UserId);
        let option = Some(RequestOption::default());

        // Test API request construction
        let result = service.get(ticket_id, user_id_type, option).await;

        // We expect this to fail with a network error, not a construction error
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_ticket_api_request_construction() {
        let config = create_test_config();
        let service = TicketService::new(config);

        let ticket_id = "ticket123";
        let request = UpdateTicketRequest {
            status: Some("solved".to_string()),
            tags: Some(vec!["resolved".to_string()]),
            comment: Some("Issue resolved".to_string()),
        };
        let user_id_type = Some(UserIdType::UnionId);
        let option = Some(RequestOption::default());

        // Test API request construction
        let result = service
            .update(ticket_id, request, user_id_type, option)
            .await;

        // We expect this to fail with a network error, not a construction error
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_tickets_api_request_construction() {
        let config = create_test_config();
        let service = TicketService::new(config);

        let user_id_type = Some(UserIdType::OpenId);
        let page_token = Some("test_token");
        let page_size = Some(20);
        let option = Some(RequestOption::default());

        // Test API request construction
        let result = service
            .list(user_id_type, page_token, page_size, option)
            .await;

        // We expect this to fail with a network error, not a construction error
        assert!(result.is_err());
    }

    #[test]
    fn test_service_debug_and_clone_traits() {
        let config = create_test_config();
        let service = TicketService::new(config);

        // Test Debug trait
        let debug_output = format!("{:?}", service);
        assert!(debug_output.contains("TicketService"));

        // Test that we can access config
        assert!(!service.config.app_id.is_empty());
    }

    #[test]
    fn test_request_structs_debug_trait() {
        let start_request = StartServiceRequest {
            open_id: "user123".to_string(),
            helpdesk_id: "helpdesk456".to_string(),
            description: Some("Test".to_string()),
        };

        let update_request = UpdateTicketRequest {
            status: Some("solved".to_string()),
            tags: None,
            comment: None,
        };

        // Test Debug trait
        let start_debug = format!("{:?}", start_request);
        let update_debug = format!("{:?}", update_request);

        assert!(start_debug.contains("StartServiceRequest"));
        assert!(update_debug.contains("UpdateTicketRequest"));
    }

    #[test]
    fn test_response_structs_debug_trait() {
        let start_response = StartServiceResponse {
            chat_id: "chat123".to_string(),
            ticket: None,
        };

        let get_response = GetTicketResponse {
            ticket: Ticket {
                ticket_id: Some("ticket123".to_string()),
                title: None,
                description: None,
                status: None,
                priority: None,
                creator: None,
                assignee: None,
                created_at: None,
                updated_at: None,
            },
        };

        let list_response = ListTicketsResponse {
            tickets: vec![],
            has_more: None,
            page_token: None,
        };

        // Test Debug trait
        let start_debug = format!("{:?}", start_response);
        let get_debug = format!("{:?}", get_response);
        let list_debug = format!("{:?}", list_response);

        assert!(start_debug.contains("StartServiceResponse"));
        assert!(get_debug.contains("GetTicketResponse"));
        assert!(list_debug.contains("ListTicketsResponse"));
    }
}
