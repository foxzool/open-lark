use serde::{Deserialize, Serialize};

/// 分页响应基础结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResponse<T> {
    /// 数据项列表
    pub items: Vec<T>,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 汇报规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportRule {
    /// 规则ID
    pub rule_id: String,
    /// 规则名称
    pub name: String,
    /// 规则描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 规则类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,
    /// 规则状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 汇报频率
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    /// 汇报时间设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<ReportSchedule>,
    /// 适用范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<ReportScope>,
    /// 汇报模板
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<ReportTemplate>,
    /// 创建者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 汇报时间设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSchedule {
    /// 汇报时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_time: Option<String>,
    /// 提醒时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminder_time: Option<String>,
    /// 时区
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// 工作日设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekdays: Option<Vec<i32>>,
}

/// 汇报适用范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportScope {
    /// 适用的部门ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 适用的用户ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// 适用的角色列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
}

/// 汇报模板
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportTemplate {
    /// 模板ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// 模板名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    /// 模板内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 字段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<ReportField>>,
}

/// 汇报字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportField {
    /// 字段ID
    pub field_id: String,
    /// 字段名称
    pub field_name: String,
    /// 字段类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_type: Option<String>,
    /// 是否必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// 字段描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 规则看板
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleView {
    /// 看板ID
    pub view_id: String,
    /// 规则ID
    pub rule_id: String,
    /// 看板名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 看板类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_type: Option<String>,
    /// 看板配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    /// 创建者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
}

/// 汇报任务
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportTask {
    /// 任务ID
    pub task_id: String,
    /// 规则ID
    pub rule_id: String,
    /// 任务名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 任务状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 任务类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// 汇报者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporter_id: Option<String>,
    /// 汇报者信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporter_info: Option<UserInfo>,
    /// 预期汇报时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_report_time: Option<i64>,
    /// 实际汇报时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_report_time: Option<i64>,
    /// 汇报内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 用户邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 用户头像URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}
