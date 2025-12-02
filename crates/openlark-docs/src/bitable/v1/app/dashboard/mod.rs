//! App Dashboard API v1
//!
//! 提供飞书多维表格仪表盘的完整管理功能，包括：
//! - 列出仪表盘列表
//! - 复制仪表盘
//! - 仪表盘权限管理

use serde::{Deserialize, Serialize};

pub mod copy;
pub mod list;

// 重新导出所有服务类型
pub use copy::*;
pub use list::*;

use openlark_core::config::Config;

/// 仪表盘信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Dashboard {
    /// 仪表盘ID
    pub block_id: String,
    /// 仪表盘名称
    pub name: String,
    /// 仪表盘描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 仪表盘URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 创建者信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<Creator>,
    /// 仪表盘权限信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<Permission>>,
}

impl Default for Dashboard {
    fn default() -> Self {
        Self {
            block_id: String::new(),
            name: String::new(),
            description: None,
            url: None,
            create_time: None,
            update_time: None,
            creator: None,
            permissions: None,
        }
    }
}

/// 创建者信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Creator {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 用户头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

impl Default for Creator {
    fn default() -> Self {
        Self {
            user_id: None,
            name: None,
            avatar: None,
        }
    }
}

/// 权限信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Permission {
    /// 权限类型
    pub permission_type: String,
    /// 权限实体
    pub entity: String,
    /// 权限值
    pub value: String,
}

impl Default for Permission {
    fn default() -> Self {
        Self {
            permission_type: String::new(),
            entity: String::new(),
            value: String::new(),
        }
    }
}

/// AppDashboard v1版本服务
///
/// 提供飞书多维表格仪表盘v1版本的统一入口，支持现代化的仪表盘管理。
/// 包括列表查询、复制、权限管理等企业级功能。
pub struct AppDashboardService {
    config: Config,
}

impl AppDashboardService {
    /// 创建AppDashboard v1服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::bitable::v1::app_dashboard::AppDashboardService;
    ///
    /// let config = openlark_core::config::Config::new(app_id, app_secret);
    /// let service = AppDashboardService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl openlark_core::trait_system::Service for AppDashboardService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "AppDashboardService"
    }
}

