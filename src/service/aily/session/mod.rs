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
        Session, SessionCreateRequest, SessionDeleteRequest, SessionGetRequest,
        SessionUpdateRequest,
    },
};

/// 会话管理服务
pub struct SessionService {
    pub config: Config,
}

/// 会话创建响应
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionCreateResponse {
    /// 会话信息
    #[serde(flatten)]
    pub session: Session,
}

impl ApiResponseTrait for SessionCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 会话更新响应
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionUpdateResponse {
    /// 会话信息
    #[serde(flatten)]
    pub session: Session,
}

impl ApiResponseTrait for SessionUpdateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 会话查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionGetResponse {
    /// 会话信息
    #[serde(flatten)]
    pub session: Session,
}

impl ApiResponseTrait for SessionGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 会话删除响应
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionDeleteResponse {
    /// 删除成功标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl ApiResponseTrait for SessionDeleteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SessionService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建会话
    ///
    /// 该接口用于创建一个新的智能伙伴会话。
    ///
    /// # 参数
    ///
    /// - `request`: 会话创建请求参数
    /// - `option`: 可选的请求配置
    pub async fn create_session(
        &self,
        request: SessionCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SessionCreateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::AILY_V1_SESSIONS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&serde_json::json!({
                "app_id": request.app_id,
                "metadata": request.metadata,
                "tool_set": request.tool_set
            }))?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新会话
    ///
    /// 该接口用于更新指定的智能伙伴会话。
    ///
    /// # 参数
    ///
    /// - `request`: 会话更新请求参数
    /// - `option`: 可选的请求配置
    pub async fn update_session(
        &self,
        request: SessionUpdateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SessionUpdateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                Endpoints::AILY_V1_SESSION_OPERATION,
                "session_id",
                &request.session_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&serde_json::json!({
                "app_id": request.app_id,
                "metadata": request.metadata,
                "tool_set": request.tool_set
            }))?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取会话
    ///
    /// 该接口用于获取指定的智能伙伴会话信息。
    ///
    /// # 参数
    ///
    /// - `request`: 会话查询请求参数
    /// - `option`: 可选的请求配置
    pub async fn get_session(
        &self,
        request: SessionGetRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SessionGetResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::AILY_V1_SESSION_OPERATION,
                "session_id",
                &request.session_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        api_req.query_params.insert("app_id", request.app_id);

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除会话
    ///
    /// 该接口用于删除指定的智能伙伴会话。
    ///
    /// # 参数
    ///
    /// - `request`: 会话删除请求参数
    /// - `option`: 可选的请求配置
    pub async fn delete_session(
        &self,
        request: SessionDeleteRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SessionDeleteResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(
                Endpoints::AILY_V1_SESSION_OPERATION,
                "session_id",
                &request.session_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        api_req.query_params.insert("app_id", request.app_id);

        Transport::request(api_req, &self.config, option).await
    }
}
