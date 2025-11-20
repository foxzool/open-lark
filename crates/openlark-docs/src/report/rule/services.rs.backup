//! Report Rule API 服务实现
//!
//! 提供报告规则管理相关的API服务，包括：
//! - 规则查询和过滤
//! - 规则删除和清理
//! - 规则状态管理
//! - 完整的错误处理和参数验证
use std::collections::HashMap;

use openlark_core::{
    api::ApiRequest, config::Config, constants::AccessTokenType, error::LarkAPIError,
    http::Transport, SDKResult,
};

use super::models::*;

/// 报告规则管理服务
#[derive(Debug, Clone)]
pub struct RuleService {
    config: Config,
}

impl RuleService {
    /// 创建新的报告规则管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 查询报告规则
    ///
    /// 查询符合条件的报告规则列表
    ///
    /// # 参数
    /// * `request` - 查询报告规则请求
    ///
    /// # 返回
    /// 返回报告规则列表数据
    pub async fn query_rule(&self, request: &QueryRuleRequest) -> SDKResult<QueryRuleResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "查询报告规则: rule_type={:?}, status={:?}, page_size={:?}",
            request.rule_type,
            request.status,
            request.page_size
        );

        // 构建查询参数
        let mut query_params = HashMap::new();

        if let Some(ref rule_type) = request.rule_type {
            query_params.insert("rule_type", rule_type.clone());
        }
        if let Some(ref status) = request.status {
            query_params.insert("status", status.clone());
        }
        if let Some(ref creator_id) = request.creator_id {
            query_params.insert("creator_id", creator_id.clone());
        }
        if let Some(ref user_id_type) = request.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }
        if let Some(page_size) = request.page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = request.page_token {
            query_params.insert("page_token", page_token.clone());
        }

        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: "/open-apis/report/v1/rules/query".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            query_params,
            ..Default::default()
        };

        // 发送请求
        let resp = Transport::<QueryRuleResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "查询报告规则完成: total={}, has_more={}",
            response.data.as_ref().map(|d| d.total).unwrap_or(0),
            response.data.as_ref().map(|d| d.has_more).unwrap_or(false)
        );

        Ok(response)
    }

    /// 删除报告规则视图
    ///
    /// 删除指定的报告规则
    ///
    /// # 参数
    /// * `request` - 删除报告规则请求
    ///
    /// # 返回
    /// 返回删除操作结果
    pub async fn remove_rule_view(
        &self,
        request: &RemoveRuleViewRequest,
    ) -> SDKResult<RemoveRuleViewResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!("删除报告规则: rule_id={}", request.rule_id);

        // 构建查询参数
        let mut query_params = HashMap::new();
        query_params.insert("rule_id", request.rule_id.clone());

        if let Some(ref user_id_type) = request.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::DELETE,
            api_path: "/open-apis/report/v1/rules/remove".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            query_params,
            ..Default::default()
        };

        // 发送请求
        let resp =
            Transport::<RemoveRuleViewResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "删除报告规则完成: rule_id={}, success={:?}",
            request.rule_id,
            response.data.as_ref().map(|d| d.success).unwrap_or(false)
        );

        Ok(response)
    }
}

// 构建器模式实现
pub struct QueryRuleRequestBuilder {
    request: QueryRuleRequest,
}

impl QueryRuleRequestBuilder {
    pub fn new() -> Self {
        Self {
            request: QueryRuleRequest::default(),
        }
    }

    pub fn rule_type(mut self, rule_type: impl Into<String>) -> Self {
        self.request.rule_type = Some(rule_type.into());
        self
    }

    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.request.status = Some(status.into());
        self
    }

    pub fn creator_id(mut self, creator_id: impl Into<String>) -> Self {
        self.request.creator_id = Some(creator_id.into());
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    pub async fn execute(self, service: &RuleService) -> SDKResult<QueryRuleResponse> {
        service.query_rule(&self.request).await
    }
}

pub struct RemoveRuleViewRequestBuilder {
    request: RemoveRuleViewRequest,
}

impl RemoveRuleViewRequestBuilder {
    pub fn new(rule_id: impl Into<String>) -> Self {
        Self {
            request: RemoveRuleViewRequest {
                rule_id: rule_id.into(),
                user_id_type: None,
            },
        }
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    pub async fn execute(self, service: &RuleService) -> SDKResult<RemoveRuleViewResponse> {
        service.remove_rule_view(&self.request).await
    }
}
