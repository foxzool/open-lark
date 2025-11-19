//! Report Task 数据模型定义
//!
//! 定义报告任务相关的请求和响应数据结构，包括：
//! - 任务查询和状态请求
//! - 任务列表和详情响应
//! - 任务执行历史和结果信息
//! - 完整的序列化和验证支持

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 查询报告任务请求
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct QueryTaskRequest {
    /// 任务类型过滤
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// 任务状态过滤：pending/running/completed/failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 规则ID过滤
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    /// 创建者用户ID过滤
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    /// 开始时间过滤（时间戳，毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 结束时间过滤（时间戳，毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
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

impl Default for QueryTaskRequest {
    fn default() -> Self {
        Self {
            task_type: None,
            status: None,
            rule_id: None,
            creator_id: None,
            start_time: None,
            end_time: None,
            user_id_type: None,
            page_size: Some(20),
            page_token: None,
        }
    }
}

impl QueryTaskRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        // 验证分页大小
        if let Some(page_size) = self.page_size {
            if page_size < 1 || page_size > 100 {
                return Err("page_size必须在1-100之间".to_string());
            }
        }

        // 验证时间范围
        if let (Some(start_time), Some(end_time)) = (self.start_time, self.end_time) {
            if start_time > end_time {
                return Err("start_time不能大于end_time".to_string());
            }
        }

        Ok(())
    }
}

/// 任务信息
#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct TaskInfo {
    /// 任务ID
    #[serde(rename = "task_id")]
    pub task_id: String,
    /// 任务名称
    #[serde(rename = "task_name")]
    pub task_name: String,
    /// 任务类型
    #[serde(rename = "task_type")]
    pub task_type: String,
    /// 任务状态：pending/running/completed/failed
    #[serde(rename = "status")]
    pub status: String,
    /// 任务描述
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// 规则ID
    #[serde(rename = "rule_id")]
    pub rule_id: Option<String>,
    /// 规则名称
    #[serde(rename = "rule_name")]
    pub rule_name: Option<String>,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub create_time: String,
    /// 开始执行时间
    #[serde(rename = "start_time")]
    pub start_time: Option<String>,
    /// 完成时间
    #[serde(rename = "finish_time")]
    pub finish_time: Option<String>,
    /// 执行时长（毫秒）
    #[serde(rename = "duration")]
    pub duration: Option<i64>,
    /// 创建者信息
    #[serde(rename = "creator")]
    pub creator: Option<CreatorInfo>,
    /// 任务配置
    #[serde(rename = "task_config")]
    pub task_config: Option<serde_json::Value>,
    /// 执行结果
    #[serde(rename = "result")]
    pub result: Option<TaskResult>,
    /// 错误信息
    #[serde(rename = "error_message")]
    pub error_message: Option<String>,
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

/// 任务执行结果
#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct TaskResult {
    /// 结果类型
    #[serde(rename = "result_type")]
    pub result_type: String,
    /// 结果数据
    #[serde(rename = "data")]
    pub data: Option<serde_json::Value>,
    /// 文件路径（如果结果保存为文件）
    #[serde(rename = "file_path")]
    pub file_path: Option<String>,
    /// 文件大小
    #[serde(rename = "file_size")]
    pub file_size: Option<i64>,
    /// 下载链接
    #[serde(rename = "download_url")]
    pub download_url: Option<String>,
}

/// 任务查询响应数据
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct QueryTaskData {
    /// 任务列表
    #[serde(rename = "items")]
    pub items: Vec<TaskInfo>,
    /// 任务总数
    #[serde(rename = "total")]
    pub total: i32,
    /// 下一页令牌
    #[serde(rename = "page_token")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(rename = "has_more")]
    pub has_more: bool,
}

/// 查询报告任务响应
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct QueryTaskResponse {
    /// 任务数据
    #[serde(rename = "data")]
    pub data: Option<QueryTaskData>,
}

impl ApiResponseTrait for QueryTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl Default for QueryTaskResponse {
    fn default() -> Self {
        Self { data: None }
    }
}
