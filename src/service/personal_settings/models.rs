use serde::{Deserialize, Serialize};

/// 系统状态信息
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemStatus {
    /// 系统状态ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_status_id: Option<String>,
    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 国际化标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_title: Option<I18nContent>,
    /// 图标样式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_style: Option<String>,
    /// 图标链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// 优先级，数字越小，优先级越高
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// 是否开启，true-开启，false-关闭
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_open: Option<bool>,
    /// 创建时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

/// 国际化内容
#[derive(Debug, Serialize, Deserialize)]
pub struct I18nContent {
    /// 中文内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 英文内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
    /// 日文内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ja_jp: Option<String>,
}

/// 创建系统状态请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSystemStatusRequest {
    /// 标题
    pub title: String,
    /// 国际化标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_title: Option<I18nContent>,
    /// 图标样式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_style: Option<String>,
    /// 图标链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// 优先级，数字越小，优先级越高，默认为1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

/// 更新系统状态请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSystemStatusRequest {
    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 国际化标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_title: Option<I18nContent>,
    /// 图标样式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_style: Option<String>,
    /// 图标链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// 优先级，数字越小，优先级越高
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

/// 批量操作系统状态请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchSystemStatusRequest {
    /// 系统状态ID列表
    pub system_status_ids: Vec<String>,
}

/// 系统状态列表查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct ListSystemStatusRequest {
    /// 页码，从1开始
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    /// 每页数量，默认20，最大100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}
