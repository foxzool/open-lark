//! 用户邮箱规则模块

use crate::common::{api_endpoints::MailApiV1, api_utils::*};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// ===== 数据模型 =====

/// 创建用户邮箱规则请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateUserMailboxRuleBody {
    /// 规则名称
    pub name: String,
    /// 匹配条件
    pub conditions: RuleConditions,
    /// 执行动作
    pub actions: Vec<RuleAction>,
    /// 是否启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// 规则匹配条件
#[derive(Debug, Clone, Serialize, Default)]
pub struct RuleConditions {
    /// 发件人匹配
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<String>>,
    /// 主题包含
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_contains: Option<Vec<String>>,
}

/// 规则执行动作
#[derive(Debug, Clone, Serialize, Default)]
pub struct RuleAction {
    /// 动作类型
    pub action_type: String,
    /// 目标文件夹 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_folder_id: Option<String>,
}

/// 创建用户邮箱规则响应
#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserMailboxRuleResponse {
    /// 规则 ID
    pub rule_id: String,
    /// 规则名称
    pub name: String,
    /// 创建时间
    pub create_time: String,
}

/// 获取用户邮箱规则列表响应
#[derive(Debug, Clone, Deserialize)]
pub struct UserMailboxRuleListResponse {
    /// 规则列表
    pub items: Vec<UserMailboxRuleItem>,
}

/// 用户邮箱规则项目
#[derive(Debug, Clone, Deserialize)]
pub struct UserMailboxRuleItem {
    /// 规则 ID
    pub rule_id: String,
    /// 规则名称
    pub name: String,
    /// 是否启用
    pub enabled: bool,
    /// 创建时间
    pub create_time: String,
}

/// 删除用户邮箱规则响应
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteUserMailboxRuleResponse {
    /// 是否删除成功
    pub success: bool,
}

/// 更新用户邮箱规则请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateUserMailboxRuleBody {
    /// 规则名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 匹配条件
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<RuleConditions>,
    /// 执行动作
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<RuleAction>>,
    /// 是否启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// 更新用户邮箱规则响应
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateUserMailboxRuleResponse {
    /// 规则 ID
    pub rule_id: String,
    /// 更新时间
    pub update_time: String,
}

/// 重新排序用户邮箱规则请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct ReorderUserMailboxRuleBody {
    /// 规则 ID 列表（按优先级排序）
    pub rule_ids: Vec<String>,
}

/// 重新排序用户邮箱规则响应
#[derive(Debug, Clone, Deserialize)]
pub struct ReorderUserMailboxRuleResponse {
    /// 是否重新排序成功
    pub success: bool,
}

/// Rule：用户邮箱规则资源
#[derive(Clone)]
pub struct Rule {
    config: Arc<Config>,
    mailbox_id: String,
}

impl Rule {
    pub fn new(config: Arc<Config>, mailbox_id: String) -> Self {
        Self { config, mailbox_id }
    }

    /// 创建规则
    pub fn create(&self) -> CreateUserMailboxRuleRequest {
        CreateUserMailboxRuleRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 获取规则列表
    pub fn list(&self) -> UserMailboxRuleListRequest {
        UserMailboxRuleListRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 删除规则
    pub fn delete(&self, rule_id: impl Into<String>) -> DeleteUserMailboxRuleRequest {
        DeleteUserMailboxRuleRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            rule_id.into(),
        )
    }

    /// 更新规则
    pub fn update(&self, rule_id: impl Into<String>) -> UpdateUserMailboxRuleRequest {
        UpdateUserMailboxRuleRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            rule_id.into(),
        )
    }

    /// 重新排序规则
    pub fn reorder(&self) -> ReorderUserMailboxRuleRequest {
        ReorderUserMailboxRuleRequest::new(self.config.clone(), self.mailbox_id.clone())
    }
}

/// 创建用户邮箱规则请求
#[derive(Debug, Clone)]
pub struct CreateUserMailboxRuleRequest {
    config: Arc<Config>,
    mailbox_id: String,
    body: CreateUserMailboxRuleBody,
}

impl CreateUserMailboxRuleRequest {
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
            body: CreateUserMailboxRuleBody::default(),
        }
    }

    /// 设置规则名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = name.into();
        self
    }

    /// 设置匹配条件
    pub fn conditions(mut self, conditions: RuleConditions) -> Self {
        self.body.conditions = conditions;
        self
    }

    /// 设置执行动作
    pub fn actions(mut self, actions: Vec<RuleAction>) -> Self {
        self.body.actions = actions;
        self
    }

    /// 设置是否启用
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.body.enabled = Some(enabled);
        self
    }

    pub async fn execute(self) -> SDKResult<CreateUserMailboxRuleResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateUserMailboxRuleResponse> {
        validate_required!(self.body.name.trim(), "规则名称不能为空");

        let api_endpoint = MailApiV1::UserMailboxRuleCreate(self.mailbox_id.clone());
        let mut request = ApiRequest::<CreateUserMailboxRuleResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "创建用户邮箱规则")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建用户邮箱规则")
    }
}

impl ApiResponseTrait for CreateUserMailboxRuleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取用户邮箱规则列表请求
#[derive(Debug, Clone)]
pub struct UserMailboxRuleListRequest {
    config: Arc<Config>,
    mailbox_id: String,
}

impl UserMailboxRuleListRequest {
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<UserMailboxRuleListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UserMailboxRuleListResponse> {
        let api_endpoint = MailApiV1::UserMailboxRuleList(self.mailbox_id.clone());
        let request = ApiRequest::<UserMailboxRuleListResponse>::get(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取用户邮箱规则列表")
    }
}

impl ApiResponseTrait for UserMailboxRuleListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除用户邮箱规则请求
#[derive(Debug, Clone)]
pub struct DeleteUserMailboxRuleRequest {
    config: Arc<Config>,
    mailbox_id: String,
    rule_id: String,
}

impl DeleteUserMailboxRuleRequest {
    pub fn new(config: Arc<Config>, mailbox_id: String, rule_id: String) -> Self {
        Self {
            config,
            mailbox_id,
            rule_id,
        }
    }

    pub async fn execute(self) -> SDKResult<DeleteUserMailboxRuleResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteUserMailboxRuleResponse> {
        let api_endpoint = MailApiV1::UserMailboxRuleDelete(self.mailbox_id.clone());
        let mut request = ApiRequest::<DeleteUserMailboxRuleResponse>::delete(api_endpoint.to_url());
        request = request.query("rule_id", &self.rule_id);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "删除用户邮箱规则")
    }
}

impl ApiResponseTrait for DeleteUserMailboxRuleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新用户邮箱规则请求
#[derive(Debug, Clone)]
pub struct UpdateUserMailboxRuleRequest {
    config: Arc<Config>,
    mailbox_id: String,
    rule_id: String,
    body: UpdateUserMailboxRuleBody,
}

impl UpdateUserMailboxRuleRequest {
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>, rule_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
            rule_id: rule_id.into(),
            body: UpdateUserMailboxRuleBody::default(),
        }
    }

    /// 设置规则名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = Some(name.into());
        self
    }

    /// 设置匹配条件
    pub fn conditions(mut self, conditions: RuleConditions) -> Self {
        self.body.conditions = Some(conditions);
        self
    }

    /// 设置执行动作
    pub fn actions(mut self, actions: Vec<RuleAction>) -> Self {
        self.body.actions = Some(actions);
        self
    }

    /// 设置是否启用
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.body.enabled = Some(enabled);
        self
    }

    pub async fn execute(self) -> SDKResult<UpdateUserMailboxRuleResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateUserMailboxRuleResponse> {
        let api_endpoint = MailApiV1::UserMailboxRuleUpdate(self.mailbox_id.clone());
        let mut request = ApiRequest::<UpdateUserMailboxRuleResponse>::put(api_endpoint.to_url());
        request = request.query("rule_id", &self.rule_id);

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "更新用户邮箱规则")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "更新用户邮箱规则")
    }
}

impl ApiResponseTrait for UpdateUserMailboxRuleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 重新排序用户邮箱规则请求
#[derive(Debug, Clone)]
pub struct ReorderUserMailboxRuleRequest {
    config: Arc<Config>,
    mailbox_id: String,
    body: ReorderUserMailboxRuleBody,
}

impl ReorderUserMailboxRuleRequest {
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
            body: ReorderUserMailboxRuleBody::default(),
        }
    }

    /// 设置规则 ID 列表（按优先级排序）
    pub fn rule_ids(mut self, rule_ids: Vec<String>) -> Self {
        self.body.rule_ids = rule_ids;
        self
    }

    pub async fn execute(self) -> SDKResult<ReorderUserMailboxRuleResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ReorderUserMailboxRuleResponse> {
        let api_endpoint = MailApiV1::UserMailboxRuleReorder(self.mailbox_id.clone());
        let mut request = ApiRequest::<ReorderUserMailboxRuleResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "重新排序用户邮箱规则")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "重新排序用户邮箱规则")
    }
}

impl ApiResponseTrait for ReorderUserMailboxRuleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user_mailbox_rule_request() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let conditions = RuleConditions {
            from: Some(vec!["boss@example.com".to_string()]),
            subject_contains: None,
        };

        let actions = vec![RuleAction {
            action_type: "move_to_folder".to_string(),
            target_folder_id: Some("folder_123".to_string()),
        }];

        let request = CreateUserMailboxRuleRequest::new(config, "mailbox_123".to_string())
            .name("老板邮件")
            .conditions(conditions)
            .actions(actions)
            .enabled(true);

        assert_eq!(request.mailbox_id, "mailbox_123");
        assert_eq!(request.body.name, "老板邮件");
        assert_eq!(request.body.enabled, Some(true));
    }
}
