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
        SDKResult,
    },
    service::helpdesk::models::{Agent, UserIdType},
};

/// 客服功能管理服务
pub struct AgentService {
    pub config: Config,
}

/// 更新客服信息请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAgentRequest {
    /// 客服状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 客服名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
}

/// 更新客服信息响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAgentResponse {
    /// 更新后的客服信息
    pub agent: Agent,
}

impl ApiResponseTrait for UpdateAgentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取客服邮箱响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GetAgentEmailResponse {
    /// 客服邮箱
    pub agent_email: String,
}

impl ApiResponseTrait for GetAgentEmailResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl AgentService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 更新客服信息
    ///
    /// 该接口用于更新指定客服的基本信息，如状态、名称等。
    ///
    /// # 参数
    ///
    /// - `agent_id`: 客服ID
    /// - `request`: 更新请求参数
    /// - `user_id_type`: 用户ID类型
    /// - `option`: 请求选项
    ///
    /// # 错误
    ///
    /// - 参数无效
    /// - 权限不足
    /// - 客服不存在
    pub async fn patch(
        &self,
        agent_id: &str,
        request: UpdateAgentRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateAgentResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(HELPDESK_V1_AGENT_GET, "agent_id", agent_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取客服邮箱
    ///
    /// 该接口用于获取指定客服的邮箱地址。
    ///
    /// # 参数
    ///
    /// - `agent_id`: 客服ID
    /// - `user_id_type`: 用户ID类型
    /// - `option`: 请求选项
    ///
    /// # 错误
    ///
    /// - 参数无效
    /// - 权限不足
    /// - 客服不存在
    pub async fn agent_email(
        &self,
        agent_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetAgentEmailResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(HELPDESK_V1_AGENT_EMAIL, "agent_id", agent_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
