use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::helpdesk::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::helpdesk::models::{AgentSkill, UserIdType},
};

/// 客服技能规则服务
pub struct AgentSkillRuleService {
    pub config: Config,
}

/// 客服技能及运算符响应
#[derive(Debug, Serialize, Deserialize)]
pub struct OperatorOptionsResponse {
    /// 技能列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skills: Option<Vec<AgentSkill>>,
    /// 运算符列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operators: Option<Vec<String>>,
}

impl ApiResponseTrait for OperatorOptionsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取客服技能列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ListAgentSkillRulesResponse {
    /// 技能列表
    pub skills: Vec<AgentSkill>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 下一页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListAgentSkillRulesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl AgentSkillRuleService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 客服技能及运算符
    ///
    /// 该接口用于获取客服技能及运算符信息。
    ///
    /// # 参数
    ///
    /// - `user_id_type`: 用户ID类型
    /// - `option`: 请求选项
    ///
    /// # 错误
    ///
    /// - 权限不足
    /// - 参数无效
    pub async fn operator_options(
        &self,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<OperatorOptionsResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: HELPDESK_V1_AGENT_SKILL_RULES_OPERATOR_OPTIONS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取客服技能列表
    ///
    /// 该接口用于获取客服技能列表。
    ///
    /// # 参数
    ///
    /// - `user_id_type`: 用户ID类型
    /// - `page_token`: 分页标记
    /// - `page_size`: 分页大小
    /// - `option`: 请求选项
    ///
    /// # 错误
    ///
    /// - 权限不足
    /// - 参数无效
    pub async fn list(
        &self,
        user_id_type: Option<UserIdType>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListAgentSkillRulesResponse>> {
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
            api_path: HELPDESK_V1_AGENT_SKILL_RULES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
