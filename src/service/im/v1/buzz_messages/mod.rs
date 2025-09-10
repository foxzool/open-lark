use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        http::Transport,
        req_option::RequestOption,
        standard_response::StandardResponse,
        SDKResult,
    },
    service::im::v1::models::UserIdType,
};

/// 消息加急服务
pub struct BuzzMessagesService {
    pub config: Config,
}

/// 发送应用内加急请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UrgentAppRequest {
    /// 用户ID列表
    pub user_id_list: Vec<String>,
}

/// 发送短信加急请求  
#[derive(Debug, Serialize, Deserialize)]
pub struct UrgentSmsRequest {
    /// 用户ID列表
    pub user_id_list: Vec<String>,
}

/// 发送电话加急请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UrgentPhoneRequest {
    /// 用户ID列表
    pub user_id_list: Vec<String>,
}

/// 消息加急响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrgentResponse {
    /// 无效的用户ID列表
    pub invalid_user_id_list: Vec<String>,
}

impl ApiResponseTrait for UrgentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl BuzzMessagesService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 发送应用内加急
    pub async fn urgent_app(
        &self,
        message_id: &str,
        user_id_type: UserIdType,
        request: UrgentAppRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<UrgentResponse> {
        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: format!("/open-apis/im/v1/messages/{message_id}/urgent_app"),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params: HashMap::from([(
                "user_id_type".to_string(),
                user_id_type.as_str().to_string(),
            )]),
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        let api_resp: BaseResponse<UrgentResponse> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }

    /// 发送短信加急
    pub async fn urgent_sms(
        &self,
        message_id: &str,
        user_id_type: UserIdType,
        request: UrgentSmsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<UrgentResponse> {
        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: format!("/open-apis/im/v1/messages/{message_id}/urgent_sms"),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params: HashMap::from([(
                "user_id_type".to_string(),
                user_id_type.as_str().to_string(),
            )]),
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        let api_resp: BaseResponse<UrgentResponse> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }

    /// 发送电话加急
    pub async fn urgent_phone(
        &self,
        message_id: &str,
        user_id_type: UserIdType,
        request: UrgentPhoneRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<UrgentResponse> {
        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: format!("/open-apis/im/v1/messages/{message_id}/urgent_phone"),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params: HashMap::from([(
                "user_id_type".to_string(),
                user_id_type.as_str().to_string(),
            )]),
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        let api_resp: BaseResponse<UrgentResponse> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }
}
