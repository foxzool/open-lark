//! Bitable App Workflow API 数据模型
//!
//! 提供多维表格工作流管理相关的数据结构，支持工作流的查询、
//! 更新等操作。

use serde_json::Value;
use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 列出工作流请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListWorkflowRequest {
    /// 应用token
    pub app_token: String,
    /// 页面大小（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页面token（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ListWorkflowRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if let Some(page_size) = self.page_size {
            if page_size <= 0 || page_size > 1000 {
                return Err("页面大小必须在1-1000之间".to_string());
            }
        }
        Ok(())
    }
}

/// 列出工作流响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListWorkflowResponse {
    /// 工作流信息列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflows: Option<Vec<WorkflowInfo>>,
    /// 分页token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多页面
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListWorkflowResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新工作流请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateWorkflowRequest {
    /// 应用token
    pub app_token: String,
    /// 工作流ID
    pub workflow_id: String,
    /// 工作流名称（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 工作流描述（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 工作流配置（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Value>,
    /// 工作流状态（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<WorkflowStatus>,
}

impl UpdateWorkflowRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.workflow_id.trim().is_empty() {
            return Err("工作流ID不能为空".to_string());
        }
        if let Some(ref name) = self.name {
            if name.trim().is_empty() {
                return Err("工作流名称不能为空".to_string());
            }
            if name.len() > 100 {
                return Err("工作流名称不能超过100个字符".to_string());
            }
        }
        if let Some(ref description) = self.description {
            if description.len() > 1000 {
                return Err("工作流描述不能超过1000个字符".to_string());
            }
        }
        Ok(())
    }
}

/// 更新工作流响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UpdateWorkflowResponse {
    /// 工作流信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<WorkflowInfo>,
}

impl ApiResponseTrait for UpdateWorkflowResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 工作流信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WorkflowInfo {
    /// 工作流ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
    /// 工作流名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 工作流描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 工作流状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<WorkflowStatus>,
    /// 工作流配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Value>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 创建者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<UserInfo>,
    /// 触发条件
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<WorkflowTrigger>>,
    /// 操作列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<WorkflowAction>>,
}

/// 工作流状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkflowStatus {
    /// 草稿
    #[serde(rename = "draft")]
    Draft,
    /// 已启用
    #[serde(rename = "enabled")]
    Enabled,
    /// 已禁用
    #[serde(rename = "disabled")]
    Disabled,
    /// 已删除
    #[serde(rename = "deleted")]
    Deleted,
}

/// 工作流触发器
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WorkflowTrigger {
    /// 触发器ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_id: Option<String>,
    /// 触发器类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<String>,
    /// 触发器配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Value>,
    /// 触发条件
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Value>,
}

/// 工作流操作
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WorkflowAction {
    /// 操作ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    /// 操作类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    /// 操作配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Value>,
    /// 操作顺序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
}

/// 用户信息（重用自其他模块）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UserInfo {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 用户邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 用户头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}
