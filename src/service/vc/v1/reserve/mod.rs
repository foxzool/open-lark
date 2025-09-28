use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{
            vc::{
                VC_RESERVE_CREATE, VC_RESERVE_DELETE, VC_RESERVE_GET,
                VC_RESERVE_GET_ACTIVE_MEETING, VC_RESERVE_UPDATE,
            },
            EndpointBuilder,
        },
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::vc::models::{Meeting, Reserve, UserIdType},
};

/// 预约服务
pub struct ReserveService {
    pub config: Config,
}

/// 预约会议请求
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplyReserveRequest {
    /// 会议主题
    pub topic: String,
    /// 开始时间
    pub start_time: String,
    /// 结束时间
    pub end_time: String,
    /// 主持人用户ID
    pub host_user_id: String,
    /// 会议密码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// 参会人ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participants: Option<Vec<String>>,
    /// 会议室ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_id: Option<String>,
}

/// 预约会议响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyReserveResponse {
    /// 预约信息
    pub reserve: Reserve,
}

impl ApiResponseTrait for ApplyReserveResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新预约请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateReserveRequest {
    /// 会议主题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    /// 开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 结束时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 主持人用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_user_id: Option<String>,
    /// 会议密码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

/// 更新预约响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateReserveResponse {
    /// 预约信息
    pub reserve: Reserve,
}

impl ApiResponseTrait for UpdateReserveResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取预约响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetReserveResponse {
    /// 预约信息
    pub reserve: Reserve,
}

impl ApiResponseTrait for GetReserveResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取活跃会议响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetActiveMeetingResponse {
    /// 会议信息
    pub meeting: Meeting,
}

impl ApiResponseTrait for GetActiveMeetingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ReserveService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 预约会议
    pub async fn apply(
        &self,
        request: ApplyReserveRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ApplyReserveResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: VC_RESERVE_CREATE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除预约
    pub async fn delete(
        &self,
        reserve_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(VC_RESERVE_DELETE, "{reserve_id}", reserve_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新预约
    pub async fn update(
        &self,
        reserve_id: &str,
        request: UpdateReserveRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateReserveResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(VC_RESERVE_UPDATE, "{reserve_id}", reserve_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取预约
    pub async fn get(
        &self,
        reserve_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetReserveResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(VC_RESERVE_GET, "{reserve_id}", reserve_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取活跃会议
    pub async fn get_active_meeting(
        &self,
        reserve_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetActiveMeetingResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                VC_RESERVE_GET_ACTIVE_MEETING,
                "{reserve_id}",
                reserve_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use crate::service::vc::models::{Meeting, MeetingStatus, Reserve, UserInfo};

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .base_url("https://test.example.com")
            .build()
    }

    fn create_test_reserve() -> Reserve {
        Reserve {
            id: "reserve123".to_string(),
            topic: "Test Reserve Meeting".to_string(),
            meeting_no: "123456789".to_string(),
            password: Some("test123".to_string()),
            start_time: "2024-01-01T10:00:00Z".to_string(),
            end_time: "2024-01-01T11:00:00Z".to_string(),
            host_user: Some(UserInfo {
                id: "host123".to_string(),
                name: Some("Test Host".to_string()),
                avatar_url: Some("https://example.com/avatar.jpg".to_string()),
            }),
            status: MeetingStatus::NotStarted,
            meeting_type: crate::service::vc::models::MeetingType::Scheduled,
            create_time: Some("2024-01-01T09:00:00Z".to_string()),
        }
    }

    fn create_test_meeting() -> Meeting {
        Meeting {
            id: "meeting123".to_string(),
            topic: "Test Meeting".to_string(),
            meeting_no: "123456789".to_string(),
            password: Some("test123".to_string()),
            start_time: "2024-01-01T10:00:00Z".to_string(),
            end_time: Some("2024-01-01T11:00:00Z".to_string()),
            host_user: Some(UserInfo {
                id: "user123".to_string(),
                name: Some("Test Host".to_string()),
                avatar_url: Some("https://example.com/avatar.jpg".to_string()),
            }),
            status: MeetingStatus::InProgress,
            participant_count: Some(5),
            create_time: Some("2024-01-01T09:00:00Z".to_string()),
        }
    }

    #[test]
    fn test_reserve_service_new() {
        let config = create_test_config();
        let service = ReserveService::new(config.clone());

        assert_eq!(service.config.app_id, config.app_id);
        assert_eq!(service.config.app_secret, config.app_secret);
        assert_eq!(service.config.base_url, config.base_url);
    }

    #[test]
    fn test_apply_reserve_request_serialization() {
        let request = ApplyReserveRequest {
            topic: "Team Meeting".to_string(),
            start_time: "2024-01-01T10:00:00Z".to_string(),
            end_time: "2024-01-01T11:00:00Z".to_string(),
            host_user_id: "host123".to_string(),
            password: Some("password123".to_string()),
            participants: Some(vec!["user1".to_string(), "user2".to_string()]),
            room_id: Some("room456".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"topic\""));
        assert!(json.contains("\"Team Meeting\""));
        assert!(json.contains("\"start_time\""));
        assert!(json.contains("\"end_time\""));
        assert!(json.contains("\"host_user_id\""));
        assert!(json.contains("\"password\""));
        assert!(json.contains("\"participants\""));
        assert!(json.contains("\"room_id\""));

        let deserialized: ApplyReserveRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.topic, "Team Meeting");
        assert_eq!(deserialized.host_user_id, "host123");
        assert_eq!(deserialized.password, Some("password123".to_string()));
        assert_eq!(deserialized.participants.as_ref().unwrap().len(), 2);
        assert_eq!(deserialized.room_id, Some("room456".to_string()));
    }

    #[test]
    fn test_apply_reserve_request_without_optional_fields() {
        let request = ApplyReserveRequest {
            topic: "Simple Meeting".to_string(),
            start_time: "2024-01-01T10:00:00Z".to_string(),
            end_time: "2024-01-01T11:00:00Z".to_string(),
            host_user_id: "host123".to_string(),
            password: None,
            participants: None,
            room_id: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(!json.contains("\"password\""));
        assert!(!json.contains("\"participants\""));
        assert!(!json.contains("\"room_id\""));

        let deserialized: ApplyReserveRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.topic, "Simple Meeting");
        assert!(deserialized.password.is_none());
        assert!(deserialized.participants.is_none());
        assert!(deserialized.room_id.is_none());
    }

    #[test]
    fn test_apply_reserve_response_serialization() {
        let response = ApplyReserveResponse {
            reserve: create_test_reserve(),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"reserve\""));
        assert!(json.contains("\"reserve123\""));
        assert!(json.contains("\"Test Reserve Meeting\""));

        let deserialized: ApplyReserveResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.reserve.id, "reserve123");
        assert_eq!(deserialized.reserve.topic, "Test Reserve Meeting");
    }

    #[test]
    fn test_apply_reserve_response_data_format() {
        assert_eq!(ApplyReserveResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_update_reserve_request_serialization() {
        let request = UpdateReserveRequest {
            topic: Some("Updated Meeting".to_string()),
            start_time: Some("2024-01-01T14:00:00Z".to_string()),
            end_time: Some("2024-01-01T15:00:00Z".to_string()),
            host_user_id: Some("new_host".to_string()),
            password: Some("new_password".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"Updated Meeting\""));
        assert!(json.contains("\"2024-01-01T14:00:00Z\""));
        assert!(json.contains("\"new_host\""));
        assert!(json.contains("\"new_password\""));

        let deserialized: UpdateReserveRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.topic, Some("Updated Meeting".to_string()));
        assert_eq!(deserialized.host_user_id, Some("new_host".to_string()));
    }

    #[test]
    fn test_update_reserve_request_partial_update() {
        let request = UpdateReserveRequest {
            topic: Some("Only Topic Updated".to_string()),
            start_time: None,
            end_time: None,
            host_user_id: None,
            password: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"Only Topic Updated\""));
        assert!(!json.contains("\"start_time\""));
        assert!(!json.contains("\"end_time\""));
        assert!(!json.contains("\"host_user_id\""));
        assert!(!json.contains("\"password\""));

        let deserialized: UpdateReserveRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.topic, Some("Only Topic Updated".to_string()));
        assert!(deserialized.start_time.is_none());
    }

    #[test]
    fn test_update_reserve_response_serialization() {
        let response = UpdateReserveResponse {
            reserve: create_test_reserve(),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"reserve\""));
        assert!(json.contains("\"reserve123\""));

        let deserialized: UpdateReserveResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.reserve.id, "reserve123");
    }

    #[test]
    fn test_update_reserve_response_data_format() {
        assert_eq!(UpdateReserveResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_get_reserve_response_serialization() {
        let response = GetReserveResponse {
            reserve: create_test_reserve(),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"reserve\""));
        assert!(json.contains("\"reserve123\""));

        let deserialized: GetReserveResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.reserve.id, "reserve123");
    }

    #[test]
    fn test_get_reserve_response_data_format() {
        assert_eq!(GetReserveResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_get_active_meeting_response_serialization() {
        let response = GetActiveMeetingResponse {
            meeting: create_test_meeting(),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"meeting\""));
        assert!(json.contains("\"meeting123\""));
        assert!(json.contains("\"Test Meeting\""));

        let deserialized: GetActiveMeetingResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.meeting.id, "meeting123");
        assert_eq!(deserialized.meeting.topic, "Test Meeting");
    }

    #[test]
    fn test_get_active_meeting_response_data_format() {
        assert_eq!(
            GetActiveMeetingResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_reserve_service_config_independence() {
        let config1 = Config::builder()
            .app_id("app1")
            .app_secret("secret1")
            .build();
        let config2 = Config::builder()
            .app_id("app2")
            .app_secret("secret2")
            .build();

        let service1 = ReserveService::new(config1);
        let service2 = ReserveService::new(config2);

        assert_eq!(service1.config.app_id, "app1");
        assert_eq!(service2.config.app_id, "app2");
        assert_ne!(service1.config.app_id, service2.config.app_id);
    }

    #[test]
    fn test_apply_reserve_request_unicode_content() {
        let request = ApplyReserveRequest {
            topic: "团队会议".to_string(),
            start_time: "2024-01-01T10:00:00Z".to_string(),
            end_time: "2024-01-01T11:00:00Z".to_string(),
            host_user_id: "主持人123".to_string(),
            password: Some("密码123".to_string()),
            participants: Some(vec!["用户1".to_string(), "用户2".to_string()]),
            room_id: Some("会议室456".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"团队会议\""));
        assert!(json.contains("\"主持人123\""));
        assert!(json.contains("\"密码123\""));
        assert!(json.contains("\"用户1\""));
        assert!(json.contains("\"会议室456\""));

        let deserialized: ApplyReserveRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.topic, "团队会议");
        assert_eq!(deserialized.host_user_id, "主持人123");
        assert_eq!(deserialized.password, Some("密码123".to_string()));
    }

    #[test]
    fn test_reserve_service_with_empty_config() {
        let config = Config::default();
        let service = ReserveService::new(config);

        assert_eq!(service.config.app_id, "");
        assert_eq!(service.config.app_secret, "");
    }

    #[test]
    fn test_reserve_service_multiple_instances() {
        let config = create_test_config();
        let service1 = ReserveService::new(config.clone());
        let service2 = ReserveService::new(config.clone());

        assert_eq!(service1.config.app_id, service2.config.app_id);
        assert_eq!(service1.config.app_secret, service2.config.app_secret);
    }

    #[test]
    fn test_apply_reserve_request_empty_participants() {
        let request = ApplyReserveRequest {
            topic: "Empty Participants Meeting".to_string(),
            start_time: "2024-01-01T10:00:00Z".to_string(),
            end_time: "2024-01-01T11:00:00Z".to_string(),
            host_user_id: "host123".to_string(),
            password: None,
            participants: Some(vec![]),
            room_id: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"participants\":[]"));

        let deserialized: ApplyReserveRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.participants.as_ref().unwrap().len(), 0);
    }

    #[test]
    fn test_update_reserve_request_all_none() {
        let request = UpdateReserveRequest {
            topic: None,
            start_time: None,
            end_time: None,
            host_user_id: None,
            password: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert_eq!(json, "{}");

        let deserialized: UpdateReserveRequest = serde_json::from_str(&json).unwrap();
        assert!(deserialized.topic.is_none());
        assert!(deserialized.start_time.is_none());
        assert!(deserialized.end_time.is_none());
        assert!(deserialized.host_user_id.is_none());
        assert!(deserialized.password.is_none());
    }

    #[test]
    fn test_reserve_service_with_unicode_config() {
        let config = Config::builder()
            .app_id("测试应用")
            .app_secret("测试密钥")
            .base_url("https://测试域名.com")
            .build();
        let service = ReserveService::new(config);

        assert_eq!(service.config.app_id, "测试应用");
        assert_eq!(service.config.app_secret, "测试密钥");
        assert_eq!(service.config.base_url, "https://测试域名.com");
    }
}
