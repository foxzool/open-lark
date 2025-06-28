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

/// 工作台访问数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkplaceAccessData {
    /// 数据ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_id: Option<String>,
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 访问时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_time: Option<i64>,
    /// 访问类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_type: Option<String>,
    /// 访问次数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_count: Option<i32>,
    /// 访问时长（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// 访问的应用ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// 平台类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// 设备类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
}

/// 定制工作台访问数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomWorkplaceAccessData {
    /// 数据ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_id: Option<String>,
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 定制工作台ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_workplace_id: Option<String>,
    /// 访问时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_time: Option<i64>,
    /// 访问次数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_count: Option<i32>,
    /// 访问时长（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// 操作类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
}

/// 定制工作台小组件访问数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomWorkplaceWidgetAccessData {
    /// 数据ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_id: Option<String>,
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 定制工作台ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_workplace_id: Option<String>,
    /// 小组件ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget_id: Option<String>,
    /// 小组件名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget_name: Option<String>,
    /// 访问时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_time: Option<i64>,
    /// 访问次数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_count: Option<i32>,
    /// 点击次数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_count: Option<i32>,
}

/// 应用信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo {
    /// 应用ID
    pub app_id: String,
    /// 应用名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    /// 应用描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_description: Option<String>,
    /// 应用图标URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_icon_url: Option<String>,
    /// 应用类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_type: Option<String>,
    /// 应用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 用户常用应用
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavouriteApp {
    /// 应用ID
    pub app_id: String,
    /// 应用信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_info: Option<AppInfo>,
    /// 添加到常用的时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favourited_at: Option<i64>,
    /// 使用频率
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_frequency: Option<i32>,
    /// 最后使用时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<i64>,
}

/// 推荐应用
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendedApp {
    /// 应用ID
    pub app_id: String,
    /// 应用信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_info: Option<AppInfo>,
    /// 推荐原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommend_reason: Option<String>,
    /// 推荐分数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommend_score: Option<f64>,
    /// 推荐时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_at: Option<i64>,
    /// 推荐规则ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
}

/// 应用推荐规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppRecommendRule {
    /// 规则ID
    pub rule_id: String,
    /// 规则名称
    pub rule_name: String,
    /// 规则描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_description: Option<String>,
    /// 规则类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,
    /// 规则状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 适用的应用ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_ids: Option<Vec<String>>,
    /// 适用的用户ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// 适用的部门ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 优先级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// 生效开始时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 生效结束时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
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
