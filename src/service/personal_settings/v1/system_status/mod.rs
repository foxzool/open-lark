use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::personal_settings::models::{
        BatchSystemStatusRequest, CreateSystemStatusRequest, ListSystemStatusRequest, SystemStatus,
        UpdateSystemStatusRequest,
    },
};

/// 系统状态服务
pub struct SystemStatusService {
    pub config: Config,
}

/// 创建系统状态响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSystemStatusResponse {
    /// 系统状态信息
    pub system_status: SystemStatus,
}

impl ApiResponseTrait for CreateSystemStatusResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取系统状态列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ListSystemStatusResponse {
    /// 系统状态列表
    pub items: Vec<SystemStatus>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListSystemStatusResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 空响应（用于删除、修改、批量操作等）
#[derive(Debug, Serialize, Deserialize)]
pub struct EmptySystemStatusResponse {}

impl ApiResponseTrait for EmptySystemStatusResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SystemStatusService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建系统状态
    ///
    /// 该接口用于创建系统状态，设置用户的个人状态显示。
    ///
    /// 注意事项：
    /// - 需要申请相应权限
    /// - 每个应用最多可创建100个系统状态
    ///
    /// # 参数
    ///
    /// - `request`: 创建系统状态请求参数
    /// - `option`: 可选的请求配置
    pub async fn create(
        &self,
        request: CreateSystemStatusRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateSystemStatusResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: "/open-apis/personal_settings/v1/system_statuses".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除系统状态
    ///
    /// 该接口用于删除指定的系统状态。
    ///
    /// # 参数
    ///
    /// - `system_status_id`: 系统状态ID
    /// - `option`: 可选的请求配置
    pub async fn delete(
        &self,
        system_status_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptySystemStatusResponse>> {
        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: format!("/open-apis/personal_settings/v1/system_statuses/{system_status_id}"),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 修改系统状态
    ///
    /// 该接口用于修改指定的系统状态信息。
    ///
    /// # 参数
    ///
    /// - `system_status_id`: 系统状态ID
    /// - `request`: 更新系统状态请求参数
    /// - `option`: 可选的请求配置
    pub async fn patch(
        &self,
        system_status_id: &str,
        request: UpdateSystemStatusRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateSystemStatusResponse>> {
        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: format!("/open-apis/personal_settings/v1/system_statuses/{system_status_id}"),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取系统状态列表
    ///
    /// 该接口用于获取应用创建的所有系统状态列表。
    ///
    /// # 参数
    ///
    /// - `request`: 查询参数（可选）
    /// - `option`: 可选的请求配置
    pub async fn list(
        &self,
        request: Option<ListSystemStatusRequest>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListSystemStatusResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: "/open-apis/personal_settings/v1/system_statuses".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            ..Default::default()
        };

        // 如果有查询参数，添加到URL中
        if let Some(req) = request {
            let mut query_params = Vec::new();
            if let Some(page) = req.page {
                query_params.push(format!("page={page}"));
            }
            if let Some(page_size) = req.page_size {
                query_params.push(format!("page_size={page_size}"));
            }
            if !query_params.is_empty() {
                api_req.api_path = format!("{}?{}", api_req.api_path, query_params.join("&"));
            }
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 批量开启系统状态
    ///
    /// 该接口用于批量开启指定的系统状态。
    ///
    /// # 参数
    ///
    /// - `request`: 批量操作请求参数
    /// - `option`: 可选的请求配置
    pub async fn batch_open(
        &self,
        request: BatchSystemStatusRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptySystemStatusResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: "/open-apis/personal_settings/v1/system_statuses/batch_open".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 批量关闭系统状态
    ///
    /// 该接口用于批量关闭指定的系统状态。
    ///
    /// # 参数
    ///
    /// - `request`: 批量操作请求参数
    /// - `option`: 可选的请求配置
    pub async fn batch_close(
        &self,
        request: BatchSystemStatusRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptySystemStatusResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: "/open-apis/personal_settings/v1/system_statuses/batch_close".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
