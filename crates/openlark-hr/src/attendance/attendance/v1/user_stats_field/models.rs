//! 查询统计字段定义模型
//!
//! docPath: https://open.feishu.cn/document/attendance-v1/user_stats_field/query

use serde::{Deserialize, Serialize};

/// 查询统计字段定义请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 考勤组 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_id: Option<String>,
    /// 统计类型，可选值：daily（日度统计）、monthly（月度统计）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stat_type: Option<String>,
    /// 用户 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// 查询的起始日期，格式为 yyyy-MM-dd
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 查询的结束日期，格式为 yyyy-MM-dd
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 是否包含下属，默认值为 false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_include_subordinate: Option<bool>,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 统计字段信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatField {
    /// 字段 ID
    pub field_id: String,
    /// 字段名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    /// 字段类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_type: Option<String>,
    /// 字段描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_desc: Option<String>,
}

/// 查询统计字段定义响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 统计字段列表
    pub stat_fields: Vec<StatField>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}
