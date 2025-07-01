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
    service::vc::models::{Recording, UserIdType},
};

/// 录制服务
pub struct RecordingService {
    pub config: Config,
}

/// 开始录制请求
#[derive(Debug, Serialize, Deserialize)]
pub struct StartRecordingRequest {
    /// 录制标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// 开始录制响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartRecordingResponse {
    /// 录制信息
    pub recording: Recording,
}

impl ApiResponseTrait for StartRecordingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 停止录制响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopRecordingResponse {
    /// 录制信息
    pub recording: Recording,
}

impl ApiResponseTrait for StopRecordingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取录制详情响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRecordingResponse {
    /// 录制信息
    pub recording: Recording,
}

impl ApiResponseTrait for GetRecordingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 设置录制权限请求
#[derive(Debug, Serialize, Deserialize)]
pub struct SetRecordingPermissionRequest {
    /// 权限类型
    pub permission_type: String,
    /// 权限对象
    pub permission_objects: Vec<String>,
}

impl RecordingService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 开始录制
    pub async fn start(
        &self,
        meeting_id: &str,
        request: StartRecordingRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<StartRecordingResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert(
                "user_id_type".to_string(),
                user_id_type.as_str().to_string(),
            );
        }

        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: format!("/open-apis/vc/v1/meetings/{meeting_id}/recording/start"),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 停止录制
    pub async fn stop(
        &self,
        meeting_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<StopRecordingResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert(
                "user_id_type".to_string(),
                user_id_type.as_str().to_string(),
            );
        }

        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: format!("/open-apis/vc/v1/meetings/{meeting_id}/recording/stop"),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取录制文件
    pub async fn get(
        &self,
        meeting_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetRecordingResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert(
                "user_id_type".to_string(),
                user_id_type.as_str().to_string(),
            );
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: format!("/open-apis/vc/v1/meetings/{meeting_id}/recording"),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 设置录制文件权限
    pub async fn set_permission(
        &self,
        meeting_id: &str,
        request: SetRecordingPermissionRequest,
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
            http_method: Method::PATCH,
            api_path: format!("/open-apis/vc/v1/meetings/{meeting_id}/recording/set_permission"),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
