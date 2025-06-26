use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
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
            query_params.insert(
                "user_id_type".to_string(),
                user_id_type.as_str().to_string(),
            );
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: "/open-apis/vc/v1/reserves".to_string(),
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
            query_params.insert(
                "user_id_type".to_string(),
                user_id_type.as_str().to_string(),
            );
        }

        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: format!("/open-apis/vc/v1/reserves/{}", reserve_id),
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
            query_params.insert(
                "user_id_type".to_string(),
                user_id_type.as_str().to_string(),
            );
        }

        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: format!("/open-apis/vc/v1/reserves/{}", reserve_id),
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
            query_params.insert(
                "user_id_type".to_string(),
                user_id_type.as_str().to_string(),
            );
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: format!("/open-apis/vc/v1/reserves/{}", reserve_id),
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
            query_params.insert(
                "user_id_type".to_string(),
                user_id_type.as_str().to_string(),
            );
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: format!(
                "/open-apis/vc/v1/reserves/{}/get_active_meeting",
                reserve_id
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
