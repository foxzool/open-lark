//! 策略管理 V1 API
//!
//! 提供访问控制策略的创建、更新、删除和评估功能。

use crate::acs::service::{
    AccessControlService, CreatePolicyRequest, CreatePolicyResponse, DeletePolicyRequest,
    DeletePolicyResponse, PolicyEffect, PolicyEvaluationRequest, PolicyEvaluationResponse,
    PolicyRule, PolicyType, UpdatePolicyRequest, UpdatePolicyResponse,
};
use crate::error::{ErrorContext, SecurityError, SecurityResult};
use crate::models::{DeviceInfo, DeviceType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, error, info, warn};

/// 策略管理API v1
pub struct PolicyV1API {
    service: Box<dyn AccessControlService>,
}

impl PolicyV1API {
    /// 创建新的策略API实例
    pub fn new(service: Box<dyn AccessControlService>) -> Self {
        Self { service }
    }

    /// 创建默认策略API实例
    pub fn default() -> Self {
        Self::new(Box::new(
            crate::acs::service::DefaultAccessControlService::with_default_config(),
        ))
    }
}

impl PolicyV1API {
    /// 创建访问控制策略
    ///
    /// # 参数
    /// - `request`: 策略创建请求
    ///
    /// # 返回值
    /// 返回策略创建结果，包含策略ID和创建时间
    ///
    /// # 示例
    /// ```rust
    /// let api = PolicyV1API::default();
    /// let rule = PolicyRuleBuilder::new()
    ///     .name("read_rule")
    ///     .effect(PolicyEffect::Allow)
    ///     .build()?;
    /// let request = CreatePolicyRequestBuilder::new()
    ///     .policy_name("文档读取策略")
    ///     .description("允许管理员读取所有文档")
    ///     .add_rule(rule)
    ///     .creator_id("admin_001")
    ///     .policy_type(PolicyType::RBAC)
    ///     .build()?;
    /// let response = api.create_policy(request).await?;
    /// println!("策略创建成功: policy_id={}", response.policy_id);
    /// ```
    pub async fn create_policy(
        &self,
        request: CreatePolicyRequest,
    ) -> SecurityResult<CreatePolicyResponse> {
        let context = ErrorContext::new("create_policy")
            .with_user_id(&request.creator_id)
            .with_extra("policy_name", &request.policy_name);

        info!(
            "开始创建访问控制策略: name={}, creator={}",
            request.policy_name, request.creator_id
        );

        // 验证请求参数
        validate_create_policy_request(&request)?;

        // 调用服务层
        match self.service.create_policy(request.clone()).await {
            Ok(response) => {
                info!(
                    "访问控制策略创建完成: policy_id={}, name={}",
                    response.policy_id, response.policy_name
                );
                Ok(response)
            }
            Err(e) => {
                error!(
                    "访问控制策略创建失败: name={}, error={}",
                    request.policy_name, e
                );
                Err(e)
            }
        }
    }

    /// 删除访问控制策略
    ///
    /// # 参数
    /// - `request`: 策略删除请求
    ///
    /// # 返回值
    /// 返回策略删除结果
    ///
    /// # 示例
    /// ```rust
    /// let api = PolicyV1API::default();
    /// let request = DeletePolicyRequestBuilder::new()
    ///     .policy_id("policy_001")
    ///     .operator_id("admin_001")
    ///     .build()?;
    /// let response = api.delete_policy(request).await?;
    /// if response.deleted {
    ///     println!("策略删除成功");
    /// }
    /// ```
    pub async fn delete_policy(
        &self,
        request: DeletePolicyRequest,
    ) -> SecurityResult<DeletePolicyResponse> {
        let context = ErrorContext::new("delete_policy")
            .with_user_id(&request.operator_id)
            .with_extra("policy_id", &request.policy_id);

        info!(
            "开始删除访问控制策略: policy_id={}, operator={}",
            request.policy_id, request.operator_id
        );

        // 验证请求参数
        validate_delete_policy_request(&request)?;

        // 调用服务层
        match self.service.delete_policy(request.clone()).await {
            Ok(response) => {
                info!("访问控制策略删除完成: policy_id={}", response.policy_id);
                Ok(response)
            }
            Err(e) => {
                error!(
                    "访问控制策略删除失败: policy_id={}, error={}",
                    request.policy_id, e
                );
                Err(e)
            }
        }
    }

    /// 更新访问控制策略
    ///
    /// # 参数
    /// - `request`: 策略更新请求
    ///
    /// # 返回值
    /// 返回策略更新结果
    ///
    /// # 示例
    /// ```rust
    /// let api = PolicyV1API::default();
    /// let request = UpdatePolicyRequestBuilder::new()
    ///     .policy_id("policy_001")
    ///     .policy_name("更新后的策略名称")
    ///     .description("更新后的策略描述")
    ///     .operator_id("admin_001")
    ///     .build()?;
    /// let response = api.update_policy(request).await?;
    /// if response.updated {
    ///     println!("策略更新成功");
    /// }
    /// ```
    pub async fn update_policy(
        &self,
        request: UpdatePolicyRequest,
    ) -> SecurityResult<UpdatePolicyResponse> {
        let context = ErrorContext::new("update_policy")
            .with_user_id(&request.operator_id)
            .with_extra("policy_id", &request.policy_id);

        info!(
            "开始更新访问控制策略: policy_id={}, operator={}",
            request.policy_id, request.operator_id
        );

        // 验证请求参数
        validate_update_policy_request(&request)?;

        // 调用服务层
        match self.service.update_policy(request.clone()).await {
            Ok(response) => {
                info!("访问控制策略更新完成: policy_id={}", response.policy_id);
                Ok(response)
            }
            Err(e) => {
                error!(
                    "访问控制策略更新失败: policy_id={}, error={}",
                    request.policy_id, e
                );
                Err(e)
            }
        }
    }

    /// 评估访问控制策略
    ///
    /// # 参数
    /// - `request`: 策略评估请求
    ///
    /// # 返回值
    /// 返回策略评估结果，包含是否允许访问的详细信息
    ///
    /// # 示例
    /// ```rust
    /// let api = PolicyV1API::default();
    /// let request = PolicyEvaluationRequestBuilder::new()
    ///     .user_id("user_001")
    ///     .resource("document_123")
    ///     .action("read")
    ///     .add_context("department", "技术部")
    ///     .build()?;
    /// let response = api.evaluate_policy(request).await?;
    /// if response.allowed {
    ///     println!("访问允许");
    /// } else {
    ///     println!("访问被拒绝: {:?}", response.reason);
    /// }
    /// ```
    pub async fn evaluate_policy(
        &self,
        request: PolicyEvaluationRequest,
    ) -> SecurityResult<PolicyEvaluationResponse> {
        let context = ErrorContext::new("evaluate_policy")
            .with_user_id(&request.user_id)
            .with_extra("resource", &request.resource)
            .with_extra("action", &request.action);

        info!(
            "开始评估访问控制策略: user_id={}, resource={}, action={}",
            request.user_id, request.resource, request.action
        );

        // 验证请求参数
        validate_policy_evaluation_request(&request)?;

        // 调用服务层
        match self.service.evaluate_policy(request.clone()).await {
            Ok(response) => {
                info!(
                    "访问控制策略评估完成: user_id={}, allowed={}",
                    response.user_id, response.allowed
                );
                Ok(response)
            }
            Err(e) => {
                error!(
                    "访问控制策略评估失败: user_id={}, error={}",
                    request.user_id, e
                );
                Err(e)
            }
        }
    }
}

// 构建器模式实现

/// 策略创建请求构建器
#[derive(Debug, Clone)]
pub struct CreatePolicyRequestBuilder {
    request: CreatePolicyRequest,
}

impl CreatePolicyRequestBuilder {
    /// 创建新的策略创建请求构建器
    pub fn new() -> Self {
        Self {
            request: CreatePolicyRequest {
                policy_name: String::new(),
                description: None,
                rules: Vec::new(),
                creator_id: String::new(),
                policy_type: PolicyType::RBAC,
            },
        }
    }

    /// 设置策略名称
    pub fn policy_name(mut self, policy_name: impl Into<String>) -> Self {
        self.request.policy_name = policy_name.into();
        self
    }

    /// 设置策略描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    /// 设置策略规则列表
    pub fn rules(mut self, rules: Vec<PolicyRule>) -> Self {
        self.request.rules = rules;
        self
    }

    /// 添加策略规则
    pub fn add_rule(mut self, rule: PolicyRule) -> Self {
        self.request.rules.push(rule);
        self
    }

    /// 设置创建者ID
    pub fn creator_id(mut self, creator_id: impl Into<String>) -> Self {
        self.request.creator_id = creator_id.into();
        self
    }

    /// 设置策略类型
    pub fn policy_type(mut self, policy_type: PolicyType) -> Self {
        self.request.policy_type = policy_type;
        self
    }

    /// 构建策略创建请求
    pub fn build(self) -> SecurityResult<CreatePolicyRequest> {
        validate_create_policy_request(&self.request)?;
        Ok(self.request)
    }
}

impl Default for CreatePolicyRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 策略删除请求构建器
#[derive(Debug, Clone)]
pub struct DeletePolicyRequestBuilder {
    request: DeletePolicyRequest,
}

impl DeletePolicyRequestBuilder {
    /// 创建新的策略删除请求构建器
    pub fn new() -> Self {
        Self {
            request: DeletePolicyRequest {
                policy_id: String::new(),
                operator_id: String::new(),
            },
        }
    }

    /// 设置策略ID
    pub fn policy_id(mut self, policy_id: impl Into<String>) -> Self {
        self.request.policy_id = policy_id.into();
        self
    }

    /// 设置操作者ID
    pub fn operator_id(mut self, operator_id: impl Into<String>) -> Self {
        self.request.operator_id = operator_id.into();
        self
    }

    /// 构建策略删除请求
    pub fn build(self) -> SecurityResult<DeletePolicyRequest> {
        validate_delete_policy_request(&self.request)?;
        Ok(self.request)
    }
}

impl Default for DeletePolicyRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 策略更新请求构建器
#[derive(Debug, Clone)]
pub struct UpdatePolicyRequestBuilder {
    request: UpdatePolicyRequest,
}

impl UpdatePolicyRequestBuilder {
    /// 创建新的策略更新请求构建器
    pub fn new() -> Self {
        Self {
            request: UpdatePolicyRequest {
                policy_id: String::new(),
                policy_name: None,
                description: None,
                rules: None,
                operator_id: String::new(),
            },
        }
    }

    /// 设置策略ID
    pub fn policy_id(mut self, policy_id: impl Into<String>) -> Self {
        self.request.policy_id = policy_id.into();
        self
    }

    /// 设置策略名称
    pub fn policy_name(mut self, policy_name: impl Into<String>) -> Self {
        self.request.policy_name = Some(policy_name.into());
        self
    }

    /// 设置策略描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    /// 设置策略规则列表
    pub fn rules(mut self, rules: Vec<PolicyRule>) -> Self {
        self.request.rules = Some(rules);
        self
    }

    /// 设置操作者ID
    pub fn operator_id(mut self, operator_id: impl Into<String>) -> Self {
        self.request.operator_id = operator_id.into();
        self
    }

    /// 构建策略更新请求
    pub fn build(self) -> SecurityResult<UpdatePolicyRequest> {
        validate_update_policy_request(&self.request)?;
        Ok(self.request)
    }
}

impl Default for UpdatePolicyRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 策略评估请求构建器
#[derive(Debug, Clone)]
pub struct PolicyEvaluationRequestBuilder {
    request: PolicyEvaluationRequest,
}

impl PolicyEvaluationRequestBuilder {
    /// 创建新的策略评估请求构建器
    pub fn new() -> Self {
        Self {
            request: PolicyEvaluationRequest {
                user_id: String::new(),
                resource: String::new(),
                action: String::new(),
                context: None,
                policy_ids: None,
            },
        }
    }

    /// 设置用户ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.request.user_id = user_id.into();
        self
    }

    /// 设置资源标识符
    pub fn resource(mut self, resource: impl Into<String>) -> Self {
        self.request.resource = resource.into();
        self
    }

    /// 设置操作类型
    pub fn action(mut self, action: impl Into<String>) -> Self {
        self.request.action = action.into();
        self
    }

    /// 设置上下文信息
    pub fn context(mut self, context: HashMap<String, String>) -> Self {
        self.request.context = Some(context);
        self
    }

    /// 添加上下文信息
    pub fn add_context(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        if self.request.context.is_none() {
            self.request.context = Some(HashMap::new());
        }
        if let Some(ref mut context) = self.request.context {
            context.insert(key.into(), value.into());
        }
        self
    }

    /// 设置要评估的策略ID列表
    pub fn policy_ids(mut self, policy_ids: Vec<String>) -> Self {
        self.request.policy_ids = Some(policy_ids);
        self
    }

    /// 添加策略ID
    pub fn add_policy_id(mut self, policy_id: impl Into<String>) -> Self {
        if self.request.policy_ids.is_none() {
            self.request.policy_ids = Some(Vec::new());
        }
        if let Some(ref mut policy_ids) = self.request.policy_ids {
            policy_ids.push(policy_id.into());
        }
        self
    }

    /// 构建策略评估请求
    pub fn build(self) -> SecurityResult<PolicyEvaluationRequest> {
        validate_policy_evaluation_request(&self.request)?;
        Ok(self.request)
    }
}

impl Default for PolicyEvaluationRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 策略规则构建器
#[derive(Debug, Clone)]
pub struct PolicyRuleBuilder {
    rule: PolicyRule,
}

impl PolicyRuleBuilder {
    /// 创建新的策略规则构建器
    pub fn new() -> Self {
        Self {
            rule: PolicyRule {
                rule_id: uuid::Uuid::new_v4().to_string(),
                name: String::new(),
                conditions: Vec::new(),
                effect: PolicyEffect::Allow,
                priority: 100,
            },
        }
    }

    /// 设置规则名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.rule.name = name.into();
        self
    }

    /// 设置规则ID
    pub fn rule_id(mut self, rule_id: impl Into<String>) -> Self {
        self.rule.rule_id = rule_id.into();
        self
    }

    /// 设置规则条件列表
    pub fn conditions(mut self, conditions: Vec<crate::acs::service::PolicyCondition>) -> Self {
        self.rule.conditions = conditions;
        self
    }

    /// 添加规则条件
    pub fn add_condition(mut self, condition: crate::acs::service::PolicyCondition) -> Self {
        self.rule.conditions.push(condition);
        self
    }

    /// 设置规则效果
    pub fn effect(mut self, effect: PolicyEffect) -> Self {
        self.rule.effect = effect;
        self
    }

    /// 设置规则优先级
    pub fn priority(mut self, priority: u32) -> Self {
        self.rule.priority = priority;
        self
    }

    /// 构建策略规则
    pub fn build(self) -> SecurityResult<PolicyRule> {
        if self.rule.name.is_empty() {
            return Err(SecurityError::InvalidParameter {
                parameter: "name".to_string(),
                reason: "规则名称不能为空".to_string(),
            });
        }

        Ok(self.rule)
    }
}

impl Default for PolicyRuleBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 验证函数

/// 验证策略创建请求
fn validate_create_policy_request(request: &CreatePolicyRequest) -> SecurityResult<()> {
    if request.policy_name.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "policy_name".to_string(),
            reason: "策略名称不能为空".to_string(),
        });
    }

    if request.policy_name.len() > 200 {
        return Err(SecurityError::InvalidParameter {
            parameter: "policy_name".to_string(),
            reason: "策略名称长度不能超过200个字符".to_string(),
        });
    }

    if request.creator_id.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "creator_id".to_string(),
            reason: "创建者ID不能为空".to_string(),
        });
    }

    if request.rules.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "rules".to_string(),
            reason: "策略规则列表不能为空".to_string(),
        });
    }

    if request.rules.len() > 100 {
        return Err(SecurityError::InvalidParameter {
            parameter: "rules".to_string(),
            reason: "策略规则数量不能超过100个".to_string(),
        });
    }

    // 验证每个规则
    for (index, rule) in request.rules.iter().enumerate() {
        if rule.name.is_empty() {
            return Err(SecurityError::InvalidParameter {
                parameter: format!("rules[{}].name", index),
                reason: "规则名称不能为空".to_string(),
            });
        }

        if rule.conditions.is_empty() {
            return Err(SecurityError::InvalidParameter {
                parameter: format!("rules[{}].conditions", index),
                reason: "规则条件列表不能为空".to_string(),
            });
        }
    }

    Ok(())
}

/// 验证策略删除请求
fn validate_delete_policy_request(request: &DeletePolicyRequest) -> SecurityResult<()> {
    if request.policy_id.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "policy_id".to_string(),
            reason: "策略ID不能为空".to_string(),
        });
    }

    if request.operator_id.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "operator_id".to_string(),
            reason: "操作者ID不能为空".to_string(),
        });
    }

    Ok(())
}

/// 验证策略更新请求
fn validate_update_policy_request(request: &UpdatePolicyRequest) -> SecurityResult<()> {
    if request.policy_id.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "policy_id".to_string(),
            reason: "策略ID不能为空".to_string(),
        });
    }

    if request.operator_id.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "operator_id".to_string(),
            reason: "操作者ID不能为空".to_string(),
        });
    }

    // 验证策略名称
    if let Some(ref policy_name) = request.policy_name {
        if policy_name.is_empty() {
            return Err(SecurityError::InvalidParameter {
                parameter: "policy_name".to_string(),
                reason: "策略名称不能为空".to_string(),
            });
        }

        if policy_name.len() > 200 {
            return Err(SecurityError::InvalidParameter {
                parameter: "policy_name".to_string(),
                reason: "策略名称长度不能超过200个字符".to_string(),
            });
        }
    }

    // 验证规则列表
    if let Some(ref rules) = request.rules {
        if rules.len() > 100 {
            return Err(SecurityError::InvalidParameter {
                parameter: "rules".to_string(),
                reason: "策略规则数量不能超过100个".to_string(),
            });
        }

        // 验证每个规则
        for (index, rule) in rules.iter().enumerate() {
            if rule.name.is_empty() {
                return Err(SecurityError::InvalidParameter {
                    parameter: format!("rules[{}].name", index),
                    reason: "规则名称不能为空".to_string(),
                });
            }

            if rule.conditions.is_empty() {
                return Err(SecurityError::InvalidParameter {
                    parameter: format!("rules[{}].conditions", index),
                    reason: "规则条件列表不能为空".to_string(),
                });
            }
        }
    }

    Ok(())
}

/// 验证策略评估请求
fn validate_policy_evaluation_request(request: &PolicyEvaluationRequest) -> SecurityResult<()> {
    if request.user_id.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "user_id".to_string(),
            reason: "用户ID不能为空".to_string(),
        });
    }

    if request.resource.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "resource".to_string(),
            reason: "资源标识符不能为空".to_string(),
        });
    }

    if request.action.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "action".to_string(),
            reason: "操作类型不能为空".to_string(),
        });
    }

    // 验证策略ID列表
    if let Some(ref policy_ids) = request.policy_ids {
        if policy_ids.len() > 50 {
            return Err(SecurityError::InvalidParameter {
                parameter: "policy_ids".to_string(),
                reason: "策略ID数量不能超过50个".to_string(),
            });
        }

        for (index, policy_id) in policy_ids.iter().enumerate() {
            if policy_id.is_empty() {
                return Err(SecurityError::InvalidParameter {
                    parameter: format!("policy_ids[{}]", index),
                    reason: "策略ID不能为空".to_string(),
                });
            }
        }
    }

    Ok(())
}

// 便捷函数

/// 创建策略创建请求
pub fn create_create_policy_request(
    policy_name: impl Into<String>,
    creator_id: impl Into<String>,
) -> CreatePolicyRequestBuilder {
    CreatePolicyRequestBuilder::new()
        .policy_name(policy_name)
        .creator_id(creator_id)
}

/// 创建策略删除请求
pub fn create_delete_policy_request(
    policy_id: impl Into<String>,
    operator_id: impl Into<String>,
) -> DeletePolicyRequestBuilder {
    DeletePolicyRequestBuilder::new()
        .policy_id(policy_id)
        .operator_id(operator_id)
}

/// 创建策略更新请求
pub fn create_update_policy_request(
    policy_id: impl Into<String>,
    operator_id: impl Into<String>,
) -> UpdatePolicyRequestBuilder {
    UpdatePolicyRequestBuilder::new()
        .policy_id(policy_id)
        .operator_id(operator_id)
}

/// 创建策略评估请求
pub fn create_policy_evaluation_request(
    user_id: impl Into<String>,
    resource: impl Into<String>,
    action: impl Into<String>,
) -> PolicyEvaluationRequestBuilder {
    PolicyEvaluationRequestBuilder::new()
        .user_id(user_id)
        .resource(resource)
        .action(action)
}

/// 创建策略规则
pub fn create_policy_rule(name: impl Into<String>) -> PolicyRuleBuilder {
    PolicyRuleBuilder::new().name(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::acs::service::{DefaultAccessControlService, PolicyCondition};

    #[tokio::test]
    async fn test_create_policy() {
        let api = PolicyV1API::default();
        let condition = PolicyCondition {
            condition_type: "user_role".to_string(),
            value: serde_json::json!("admin"),
            operator: "equals".to_string(),
        };

        let rule = PolicyRuleBuilder::new()
            .name("admin_access")
            .effect(PolicyEffect::Allow)
            .add_condition(condition)
            .build()
            .unwrap();

        let request = CreatePolicyRequestBuilder::new()
            .policy_name("管理员策略")
            .creator_id("admin_001")
            .add_rule(rule)
            .policy_type(PolicyType::RBAC)
            .build()
            .unwrap();

        let response = api.create_policy(request).await.unwrap();
        assert!(!response.policy_id.is_empty());
        assert_eq!(response.policy_name, "管理员策略");
    }

    #[tokio::test]
    async fn test_evaluate_policy() {
        let api = PolicyV1API::default();
        let request = PolicyEvaluationRequestBuilder::new()
            .user_id("user_001")
            .resource("document_123")
            .action("read")
            .add_context("department", "技术部")
            .build()
            .unwrap();

        let response = api.evaluate_policy(request).await.unwrap();
        assert_eq!(response.user_id, "user_001");
        assert_eq!(response.resource, "document_123");
        assert_eq!(response.action, "read");
    }

    #[test]
    fn test_policy_rule_builder() {
        let condition = PolicyCondition {
            condition_type: "time".to_string(),
            value: serde_json::json!("09:00-18:00"),
            operator: "between".to_string(),
        };

        let rule = PolicyRuleBuilder::new()
            .name("business_hours")
            .effect(PolicyEffect::Allow)
            .priority(1)
            .add_condition(condition)
            .build()
            .unwrap();

        assert_eq!(rule.name, "business_hours");
        assert!(matches!(rule.effect, PolicyEffect::Allow));
        assert_eq!(rule.priority, 1);
        assert_eq!(rule.conditions.len(), 1);
    }

    #[test]
    fn test_validation() {
        // 测试空策略名称
        let request = CreatePolicyRequestBuilder::new()
            .creator_id("admin_001")
            .build();
        assert!(request.is_err());

        // 测试空规则列表
        let request = CreatePolicyRequestBuilder::new()
            .policy_name("测试策略")
            .creator_id("admin_001")
            .build();
        assert!(request.is_err());
    }
}
