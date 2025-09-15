use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpoints},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::mail::models::{Rule, UserIdType},
};

/// 收信规则服务
pub struct RuleService {
    pub config: Config,
}

/// 创建收信规则请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRuleRequest {
    /// 规则名称
    pub rule_name: String,
    /// 是否启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// 条件列表
    pub conditions: Vec<serde_json::Value>,
    /// 动作列表
    pub actions: Vec<serde_json::Value>,
}

/// 创建收信规则响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRuleResponse {
    /// 创建的规则
    pub rule: Rule,
}

impl ApiResponseTrait for CreateRuleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新收信规则请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRuleRequest {
    /// 规则名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// 是否启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// 条件列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<serde_json::Value>>,
    /// 动作列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<serde_json::Value>>,
}

/// 更新收信规则响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRuleResponse {
    /// 更新后的规则
    pub rule: Rule,
}

impl ApiResponseTrait for UpdateRuleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 列出收信规则响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ListRulesResponse {
    /// 规则列表
    pub rules: Vec<Rule>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 下一页标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListRulesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 排序收信规则请求
#[derive(Debug, Serialize, Deserialize)]
pub struct ReorderRulesRequest {
    /// 规则ID列表（按优先级排序）
    pub rule_ids: Vec<String>,
}

impl RuleService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建收信规则
    pub async fn create(
        &self,
        user_mailbox_id: &str,
        request: CreateRuleRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateRuleResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                Endpoints::MAIL_V1_USER_MAILBOX_RULES,
                "user_mailbox_id",
                user_mailbox_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除收信规则
    pub async fn delete(
        &self,
        user_mailbox_id: &str,
        rule_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(
                &EndpointBuilder::replace_param(
                    Endpoints::MAIL_V1_USER_MAILBOX_RULE,
                    "user_mailbox_id",
                    user_mailbox_id,
                ),
                "rule_id",
                rule_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新收信规则
    pub async fn update(
        &self,
        user_mailbox_id: &str,
        rule_id: &str,
        request: UpdateRuleRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateRuleResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::PUT,
            api_path: EndpointBuilder::replace_param(
                &EndpointBuilder::replace_param(
                    Endpoints::MAIL_V1_USER_MAILBOX_RULE,
                    "user_mailbox_id",
                    user_mailbox_id,
                ),
                "rule_id",
                rule_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 列出收信规则
    pub async fn list(
        &self,
        user_mailbox_id: &str,
        page_size: Option<i32>,
        page_token: Option<String>,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListRulesResponse>> {
        let mut query_params = HashMap::new();
        if let Some(page_size) = page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = page_token {
            query_params.insert("page_token", page_token);
        }
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::MAIL_V1_USER_MAILBOX_RULES,
                "user_mailbox_id",
                user_mailbox_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 对收信规则进行排序
    pub async fn reorder(
        &self,
        user_mailbox_id: &str,
        request: ReorderRulesRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                Endpoints::MAIL_V1_USER_MAILBOX_RULES_REORDER,
                "user_mailbox_id",
                user_mailbox_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
