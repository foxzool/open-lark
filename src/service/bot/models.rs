use serde::{Deserialize, Serialize};

/// 机器人信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bot {
    /// 机器人名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    /// 机器人头像URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// 机器人IP白名单
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_white_list: Option<Vec<String>>,
    /// 机器人应用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_status: Option<AppStatus>,
    /// 机器人的open_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
}

/// 应用状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppStatus {
    /// 未知状态
    #[serde(rename = "unknown")]
    Unknown,
    /// 正常
    #[serde(rename = "normal")]
    Normal,
    /// 停用
    #[serde(rename = "disabled")]
    Disabled,
}

/// 机器人自定义菜单事件内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotMenuEvent {
    /// 事件ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// 事件类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// 租户信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_key: Option<String>,
    /// 应用ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// 操作者信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<EventOperator>,
    /// 菜单事件详情
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<MenuEventDetail>,
}

/// 事件操作者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventOperator {
    /// 操作者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<OperatorId>,
    /// 操作者类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_type: Option<String>,
}

/// 操作者ID信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorId {
    /// Open ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    /// Union ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    /// User ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// 菜单事件详情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuEventDetail {
    /// 菜单事件key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_key: Option<String>,
    /// 时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// 菜单ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_id: Option<String>,
}
