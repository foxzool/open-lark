//! Report Rule 数据模型定义
//!
//! 定义报告规则相关的请求和响应数据结构，包括：
//! - 规则查询和删除请求
//! - 规则列表和详情响应
//! - 规则状态和配置信息
//! - 完整的序列化和验证支持

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 查询报告规则请求
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct QueryRuleRequest {
    /// 规则类型过滤
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,
    /// 规则状态过滤
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 创建者用户ID过滤
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 分页大小，默认20，最大100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页令牌
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl Default for QueryRuleRequest {
    fn default() -> Self {
        Self {
            rule_type: None,
            status: None,
            creator_id: None,
            user_id_type: None,
            page_size: Some(20),
            page_token: None,
        }
    }
}

impl QueryRuleRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        // 验证分页大小
        if let Some(page_size) = self.page_size {
            if page_size < 1 || page_size > 100 {
                return Err("page_size必须在1-100之间".to_string());
            }
        }

        Ok(())
    }
}

/// 删除报告规则视图请求
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct RemoveRuleViewRequest {
    /// 规则ID
    pub rule_id: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl RemoveRuleViewRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.rule_id.trim().is_empty() {
            return Err("rule_id不能为空".to_string());
        }

        Ok(())
    }
}

/// 规则信息
#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct RuleInfo {
    /// 规则ID
    #[serde(rename = "rule_id")]
    pub rule_id: String,
    /// 规则名称
    #[serde(rename = "rule_name")]
    pub rule_name: String,
    /// 规则描述
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// 规则类型
    #[serde(rename = "rule_type")]
    pub rule_type: String,
    /// 规则状态：enabled/disabled
    #[serde(rename = "status")]
    pub status: String,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub create_time: String,
    /// 更新时间
    #[serde(rename = "update_time")]
    pub update_time: String,
    /// 创建者信息
    #[serde(rename = "creator")]
    pub creator: Option<CreatorInfo>,
    /// 规则配置
    #[serde(rename = "rule_config")]
    pub rule_config: Option<serde_json::Value>,
}

/// 创建者信息
#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct CreatorInfo {
    /// 用户ID
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// 用户名称
    #[serde(rename = "name")]
    pub name: String,
    /// 邮箱
    #[serde(rename = "email")]
    pub email: Option<String>,
}

/// 规则查询响应数据
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct QueryRuleData {
    /// 规则列表
    #[serde(rename = "items")]
    pub items: Vec<RuleInfo>,
    /// 规则总数
    #[serde(rename = "total")]
    pub total: i32,
    /// 下一页令牌
    #[serde(rename = "page_token")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(rename = "has_more")]
    pub has_more: bool,
}

/// 查询报告规则响应
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct QueryRuleResponse {
    /// 规则数据
    #[serde(rename = "data")]
    pub data: Option<QueryRuleData>,
}

impl ApiResponseTrait for QueryRuleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl Default for QueryRuleResponse {
    fn default() -> Self {
        Self { data: None }
    }
}

/// 删除报告规则响应
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct RemoveRuleViewResponse {
    /// 删除结果
    #[serde(rename = "data")]
    pub data: Option<RemoveRuleViewData>,
}

impl ApiResponseTrait for RemoveRuleViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl Default for RemoveRuleViewResponse {
    fn default() -> Self {
        Self { data: None }
    }
}

/// 删除操作结果数据
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct RemoveRuleViewData {
    /// 删除是否成功
    #[serde(rename = "success")]
    pub success: bool,
    /// 删除的规则ID
    #[serde(rename = "rule_id")]
    pub rule_id: String,
}
