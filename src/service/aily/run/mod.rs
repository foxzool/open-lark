use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpoints},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::aily::models::{
        PageResponse, Run, RunCancelRequest, RunCreateRequest, RunGetRequest, RunListRequest,
    },
};

/// 运行管理服务
pub struct RunService {
    pub config: Config,
}

/// 运行创建响应
#[derive(Debug, Serialize, Deserialize)]
pub struct RunCreateResponse {
    /// 运行信息
    #[serde(flatten)]
    pub run: Run,
}

impl ApiResponseTrait for RunCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 运行查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct RunGetResponse {
    /// 运行信息
    #[serde(flatten)]
    pub run: Run,
}

impl ApiResponseTrait for RunGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 运行列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct RunListResponse {
    /// 分页响应数据
    #[serde(flatten)]
    pub page_response: PageResponse<Run>,
}

impl ApiResponseTrait for RunListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 运行取消响应
#[derive(Debug, Serialize, Deserialize)]
pub struct RunCancelResponse {
    /// 运行信息
    #[serde(flatten)]
    pub run: Run,
}

impl ApiResponseTrait for RunCancelResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl RunService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建运行
    ///
    /// 该接口用于在指定的智能伙伴会话中创建一个新的运行。
    ///
    /// # 参数
    ///
    /// - `request`: 运行创建请求参数
    /// - `option`: 可选的请求配置
    pub async fn create_run(
        &self,
        request: RunCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RunCreateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                Endpoints::AILY_V1_RUNS,
                "session_id",
                &request.session_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&serde_json::json!({
                "app_id": request.app_id,
                "instructions": request.instructions,
                "model": request.model,
                "additional_messages": request.additional_messages,
                "tool_set": request.tool_set
            }))?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取运行
    ///
    /// 该接口用于获取指定的智能伙伴运行详情。
    ///
    /// # 参数
    ///
    /// - `request`: 运行查询请求参数
    /// - `option`: 可选的请求配置
    pub async fn get_run(
        &self,
        request: RunGetRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RunGetResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::AILY_V1_RUN_GET,
                &[
                    ("session_id", &request.session_id),
                    ("run_id", &request.run_id),
                ],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        api_req.query_params.insert("app_id", request.app_id);

        Transport::request(api_req, &self.config, option).await
    }

    /// 列出运行
    ///
    /// 该接口用于获取指定会话的运行列表。
    ///
    /// # 参数
    ///
    /// - `request`: 运行列表查询请求参数
    /// - `option`: 可选的请求配置
    pub async fn list_runs(
        &self,
        request: RunListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RunListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::AILY_V1_RUNS,
                "session_id",
                &request.session_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        api_req.query_params.insert("app_id", request.app_id);

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }
        if let Some(order) = request.order {
            api_req.query_params.insert("order", order);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 取消运行
    ///
    /// 该接口用于取消指定的智能伙伴运行。
    ///
    /// # 参数
    ///
    /// - `request`: 运行取消请求参数
    /// - `option`: 可选的请求配置
    pub async fn cancel_run(
        &self,
        request: RunCancelRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RunCancelResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::AILY_V1_RUN_CANCEL,
                &[
                    ("session_id", &request.session_id),
                    ("run_id", &request.run_id),
                ],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&serde_json::json!({
                "app_id": request.app_id
            }))?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
