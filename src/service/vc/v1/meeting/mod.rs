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
                VC_MEETING_END, VC_MEETING_GET, VC_MEETING_INVITE, VC_MEETING_KICKOUT,
                VC_MEETING_LIST_BY_NO, VC_MEETING_SET_HOST,
            },
            EndpointBuilder,
        },
        http::Transport,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    },
    service::vc::models::{Meeting, UserIdType},
};

/// 会议管理服务
pub struct MeetingService {
    pub config: Config,
}

/// 邀请参会人请求
#[derive(Debug, Serialize, Deserialize)]
pub struct InviteMeetingRequest {
    /// 邀请的用户ID列表
    pub invitees: Vec<String>,
}

/// 邀请参会人响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteMeetingResponse {
    /// 邀请结果
    pub invite_results: Vec<InviteResult>,
}

/// 邀请结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteResult {
    /// 用户ID
    pub user_id: String,
    /// 邀请状态
    pub status: String,
    /// 错误信息
    pub error_msg: Option<String>,
}

impl ApiResponseTrait for InviteMeetingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 移除参会人请求
#[derive(Debug, Serialize, Deserialize)]
pub struct KickoutMeetingRequest {
    /// 要移除的用户ID列表
    pub kickout_users: Vec<String>,
}

/// 移除参会人响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KickoutMeetingResponse {
    /// 移除结果
    pub kickout_results: Vec<KickoutResult>,
}

/// 移除结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KickoutResult {
    /// 用户ID
    pub user_id: String,
    /// 移除状态
    pub status: String,
    /// 错误信息
    pub error_msg: Option<String>,
}

impl ApiResponseTrait for KickoutMeetingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 设置主持人请求
#[derive(Debug, Serialize, Deserialize)]
pub struct SetHostRequest {
    /// 新主持人用户ID
    pub host_user_id: String,
}

/// 获取会议详情响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMeetingResponse {
    /// 会议信息
    pub meeting: Meeting,
}

impl ApiResponseTrait for GetMeetingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取会议列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMeetingsByNoResponse {
    /// 会议列表
    pub meetings: Vec<Meeting>,
    /// 是否还有更多数据
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListMeetingsByNoResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 会议列表查询参数
#[derive(Debug, Default)]
pub struct ListMeetingsByNoParams {
    pub page_size: Option<i32>,
    pub page_token: Option<String>,
    pub user_id_type: Option<UserIdType>,
}

impl MeetingService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 邀请参会人
    pub async fn invite(
        &self,
        meeting_id: &str,
        request: InviteMeetingRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<InviteMeetingResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(VC_MEETING_INVITE, "{meeting_id}", meeting_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 移除参会人
    pub async fn kickout(
        &self,
        meeting_id: &str,
        request: KickoutMeetingRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<KickoutMeetingResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                VC_MEETING_KICKOUT,
                "{meeting_id}",
                meeting_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 设置主持人
    pub async fn set_host(
        &self,
        meeting_id: &str,
        request: SetHostRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                VC_MEETING_SET_HOST,
                "{meeting_id}",
                meeting_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 结束会议
    pub async fn end(
        &self,
        meeting_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(VC_MEETING_END, "{meeting_id}", meeting_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取会议详情
    pub async fn get(
        &self,
        meeting_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetMeetingResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(VC_MEETING_GET, "{meeting_id}", meeting_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取与会议号关联的会议列表
    pub async fn list_by_no(
        &self,
        meeting_no: &str,
        start_time: &str,
        end_time: &str,
        params: Option<ListMeetingsByNoParams>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListMeetingsByNoResponse>> {
        let mut query_params = HashMap::new();
        query_params.insert("meeting_no", meeting_no.to_string());
        query_params.insert("start_time", start_time.to_string());
        query_params.insert("end_time", end_time.to_string());

        if let Some(params) = params {
            if let Some(page_size) = params.page_size {
                query_params.insert("page_size", page_size.to_string());
            }
            if let Some(page_token) = params.page_token {
                query_params.insert("page_token", page_token);
            }
            if let Some(user_id_type) = params.user_id_type {
                query_params.insert("user_id_type", user_id_type.as_str().to_string());
            }
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: VC_MEETING_LIST_BY_NO.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

impl Service for MeetingService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "meeting"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use crate::service::vc::models::{Meeting, MeetingStatus, UserInfo};

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .base_url("https://test.example.com")
            .build()
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

    fn create_test_invite_result() -> InviteResult {
        InviteResult {
            user_id: "user456".to_string(),
            status: "success".to_string(),
            error_msg: None,
        }
    }

    fn create_test_kickout_result() -> KickoutResult {
        KickoutResult {
            user_id: "user789".to_string(),
            status: "success".to_string(),
            error_msg: None,
        }
    }

    #[test]
    fn test_meeting_service_new() {
        let config = create_test_config();
        let service = MeetingService::new(config.clone());

        assert_eq!(service.config.app_id, config.app_id);
        assert_eq!(service.config.app_secret, config.app_secret);
        assert_eq!(service.config.base_url, config.base_url);
    }

    #[test]
    fn test_meeting_service_config_independence() {
        let config1 = Config::builder()
            .app_id("app1")
            .app_secret("secret1")
            .build();
        let config2 = Config::builder()
            .app_id("app2")
            .app_secret("secret2")
            .build();

        let service1 = MeetingService::new(config1);
        let service2 = MeetingService::new(config2);

        assert_eq!(service1.config.app_id, "app1");
        assert_eq!(service2.config.app_id, "app2");
        assert_ne!(service1.config.app_id, service2.config.app_id);
    }

    #[test]
    fn test_invite_meeting_request_serialization() {
        let request = InviteMeetingRequest {
            invitees: vec![
                "user1".to_string(),
                "user2".to_string(),
                "user3".to_string(),
            ],
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"invitees\""));
        assert!(json.contains("\"user1\""));
        assert!(json.contains("\"user2\""));
        assert!(json.contains("\"user3\""));

        let deserialized: InviteMeetingRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.invitees.len(), 3);
        assert_eq!(deserialized.invitees[0], "user1");
        assert_eq!(deserialized.invitees[1], "user2");
        assert_eq!(deserialized.invitees[2], "user3");
    }

    #[test]
    fn test_invite_meeting_response_serialization() {
        let response = InviteMeetingResponse {
            invite_results: vec![
                create_test_invite_result(),
                InviteResult {
                    user_id: "user_fail".to_string(),
                    status: "failed".to_string(),
                    error_msg: Some("User not found".to_string()),
                },
            ],
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"invite_results\""));
        assert!(json.contains("\"user456\""));
        assert!(json.contains("\"user_fail\""));
        assert!(json.contains("\"success\""));
        assert!(json.contains("\"failed\""));
        assert!(json.contains("\"User not found\""));

        let deserialized: InviteMeetingResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.invite_results.len(), 2);
        assert_eq!(deserialized.invite_results[0].user_id, "user456");
        assert_eq!(
            deserialized.invite_results[1].error_msg,
            Some("User not found".to_string())
        );
    }

    #[test]
    fn test_invite_meeting_response_data_format() {
        assert_eq!(InviteMeetingResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_kickout_meeting_request_serialization() {
        let request = KickoutMeetingRequest {
            kickout_users: vec!["user1".to_string(), "user2".to_string()],
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"kickout_users\""));
        assert!(json.contains("\"user1\""));
        assert!(json.contains("\"user2\""));

        let deserialized: KickoutMeetingRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.kickout_users.len(), 2);
        assert_eq!(deserialized.kickout_users[0], "user1");
        assert_eq!(deserialized.kickout_users[1], "user2");
    }

    #[test]
    fn test_kickout_meeting_response_serialization() {
        let response = KickoutMeetingResponse {
            kickout_results: vec![
                create_test_kickout_result(),
                KickoutResult {
                    user_id: "user_fail".to_string(),
                    status: "failed".to_string(),
                    error_msg: Some("Permission denied".to_string()),
                },
            ],
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"kickout_results\""));
        assert!(json.contains("\"user789\""));
        assert!(json.contains("\"user_fail\""));
        assert!(json.contains("\"Permission denied\""));

        let deserialized: KickoutMeetingResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.kickout_results.len(), 2);
        assert_eq!(deserialized.kickout_results[0].user_id, "user789");
        assert_eq!(deserialized.kickout_results[1].status, "failed");
    }

    #[test]
    fn test_kickout_meeting_response_data_format() {
        assert_eq!(KickoutMeetingResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_set_host_request_serialization() {
        let request = SetHostRequest {
            host_user_id: "new_host_123".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"host_user_id\""));
        assert!(json.contains("\"new_host_123\""));

        let deserialized: SetHostRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.host_user_id, "new_host_123");
    }

    #[test]
    fn test_get_meeting_response_serialization() {
        let response = GetMeetingResponse {
            meeting: create_test_meeting(),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"meeting\""));
        assert!(json.contains("\"meeting123\""));
        assert!(json.contains("\"Test Meeting\""));
        assert!(json.contains("\"123456789\""));

        let deserialized: GetMeetingResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.meeting.id, "meeting123");
        assert_eq!(deserialized.meeting.topic, "Test Meeting");
        assert_eq!(deserialized.meeting.meeting_no, "123456789");
    }

    #[test]
    fn test_get_meeting_response_data_format() {
        assert_eq!(GetMeetingResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_list_meetings_by_no_response_serialization() {
        let response = ListMeetingsByNoResponse {
            meetings: vec![create_test_meeting()],
            has_more: true,
            page_token: Some("next_page_token".to_string()),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"meetings\""));
        assert!(json.contains("\"has_more\""));
        assert!(json.contains("\"page_token\""));
        assert!(json.contains("\"next_page_token\""));
        assert!(json.contains("true"));

        let deserialized: ListMeetingsByNoResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.meetings.len(), 1);
        assert!(deserialized.has_more);
        assert_eq!(deserialized.page_token, Some("next_page_token".to_string()));
    }

    #[test]
    fn test_list_meetings_by_no_response_data_format() {
        assert_eq!(
            ListMeetingsByNoResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_list_meetings_by_no_params_default() {
        let params = ListMeetingsByNoParams::default();
        assert!(params.page_size.is_none());
        assert!(params.page_token.is_none());
        assert!(params.user_id_type.is_none());
    }

    #[test]
    fn test_list_meetings_by_no_params_with_values() {
        let params = ListMeetingsByNoParams {
            page_size: Some(20),
            page_token: Some("test_token".to_string()),
            user_id_type: Some(UserIdType::UserId),
        };

        assert_eq!(params.page_size, Some(20));
        assert_eq!(params.page_token, Some("test_token".to_string()));
        assert_eq!(params.user_id_type, Some(UserIdType::UserId));
    }

    #[test]
    fn test_invite_result_with_error() {
        let result = InviteResult {
            user_id: "error_user".to_string(),
            status: "failed".to_string(),
            error_msg: Some("User is already in the meeting".to_string()),
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"error_user\""));
        assert!(json.contains("\"failed\""));
        assert!(json.contains("\"User is already in the meeting\""));

        let deserialized: InviteResult = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.user_id, "error_user");
        assert_eq!(deserialized.status, "failed");
        assert_eq!(
            deserialized.error_msg,
            Some("User is already in the meeting".to_string())
        );
    }

    #[test]
    fn test_kickout_result_with_error() {
        let result = KickoutResult {
            user_id: "error_user".to_string(),
            status: "failed".to_string(),
            error_msg: Some("User is not in the meeting".to_string()),
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"error_user\""));
        assert!(json.contains("\"failed\""));
        assert!(json.contains("\"User is not in the meeting\""));

        let deserialized: KickoutResult = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.user_id, "error_user");
        assert_eq!(deserialized.status, "failed");
        assert_eq!(
            deserialized.error_msg,
            Some("User is not in the meeting".to_string())
        );
    }

    #[test]
    fn test_meeting_service_with_empty_config() {
        let config = Config::default();
        let service = MeetingService::new(config);

        assert_eq!(service.config.app_id, "");
        assert_eq!(service.config.app_secret, "");
    }

    #[test]
    fn test_meeting_service_with_unicode_config() {
        let config = Config::builder()
            .app_id("测试应用")
            .app_secret("测试密钥")
            .base_url("https://测试域名.com")
            .build();
        let service = MeetingService::new(config);

        assert_eq!(service.config.app_id, "测试应用");
        assert_eq!(service.config.app_secret, "测试密钥");
        assert_eq!(service.config.base_url, "https://测试域名.com");
    }

    #[test]
    fn test_invite_meeting_request_empty_invitees() {
        let request = InviteMeetingRequest { invitees: vec![] };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"invitees\":[]"));

        let deserialized: InviteMeetingRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.invitees.len(), 0);
    }

    #[test]
    fn test_kickout_meeting_request_empty_users() {
        let request = KickoutMeetingRequest {
            kickout_users: vec![],
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"kickout_users\":[]"));

        let deserialized: KickoutMeetingRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.kickout_users.len(), 0);
    }

    #[test]
    fn test_list_meetings_by_no_response_empty_meetings() {
        let response = ListMeetingsByNoResponse {
            meetings: vec![],
            has_more: false,
            page_token: None,
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"meetings\":[]"));
        assert!(json.contains("\"has_more\":false"));
        assert!(json.contains("\"page_token\":null"));

        let deserialized: ListMeetingsByNoResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.meetings.len(), 0);
        assert!(!deserialized.has_more);
        assert!(deserialized.page_token.is_none());
    }

    #[test]
    fn test_invite_result_success_without_error() {
        let result = InviteResult {
            user_id: "success_user".to_string(),
            status: "success".to_string(),
            error_msg: None,
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"success_user\""));
        assert!(json.contains("\"success\""));
        assert!(json.contains("\"error_msg\":null"));

        let deserialized: InviteResult = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.user_id, "success_user");
        assert_eq!(deserialized.status, "success");
        assert!(deserialized.error_msg.is_none());
    }

    #[test]
    fn test_kickout_result_success_without_error() {
        let result = KickoutResult {
            user_id: "success_user".to_string(),
            status: "success".to_string(),
            error_msg: None,
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"success_user\""));
        assert!(json.contains("\"success\""));
        assert!(json.contains("\"error_msg\":null"));

        let deserialized: KickoutResult = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.user_id, "success_user");
        assert_eq!(deserialized.status, "success");
        assert!(deserialized.error_msg.is_none());
    }

    #[test]
    fn test_set_host_request_unicode_user_id() {
        let request = SetHostRequest {
            host_user_id: "用户_123".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"用户_123\""));

        let deserialized: SetHostRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.host_user_id, "用户_123");
    }

    #[test]
    fn test_meeting_service_multiple_instances() {
        let config = create_test_config();
        let service1 = MeetingService::new(config.clone());
        let service2 = MeetingService::new(config.clone());

        assert_eq!(service1.config.app_id, service2.config.app_id);
        assert_eq!(service1.config.app_secret, service2.config.app_secret);
    }

    #[test]
    fn test_invite_meeting_request_unicode_user_ids() {
        let request = InviteMeetingRequest {
            invitees: vec![
                "用户1".to_string(),
                "用户2".to_string(),
                "user_3".to_string(),
            ],
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"用户1\""));
        assert!(json.contains("\"用户2\""));
        assert!(json.contains("\"user_3\""));

        let deserialized: InviteMeetingRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.invitees.len(), 3);
        assert_eq!(deserialized.invitees[0], "用户1");
        assert_eq!(deserialized.invitees[1], "用户2");
        assert_eq!(deserialized.invitees[2], "user_3");
    }

    #[test]
    fn test_kickout_meeting_request_unicode_user_ids() {
        let request = KickoutMeetingRequest {
            kickout_users: vec!["用户1".to_string(), "用户2".to_string()],
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"用户1\""));
        assert!(json.contains("\"用户2\""));

        let deserialized: KickoutMeetingRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.kickout_users.len(), 2);
        assert_eq!(deserialized.kickout_users[0], "用户1");
        assert_eq!(deserialized.kickout_users[1], "用户2");
    }
}
