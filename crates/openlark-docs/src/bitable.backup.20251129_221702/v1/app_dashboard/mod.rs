//! App Dashboard API v1
//!
//! 提供飞书多维表格仪表盘的完整管理功能，包括：
//! - 列出仪表盘列表
//! - 复制仪表盘
//! - 仪表盘权限管理

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

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
#[derive(Clone)]
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
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
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

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::prelude::Service;

    #[test]
    fn test_app_dashboard_service_creation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = AppDashboardService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_v1_service_trait_implementation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = AppDashboardService::new(config);

        // 测试Service trait的实现
        let config_ref = service.config();
        assert_eq!(config_ref.app_id, "test_app_id");
    }

    #[test]
    fn test_dashboard_default_creation() {
        let dashboard = Dashboard::default();
        assert_eq!(dashboard.block_id, "");
        assert_eq!(dashboard.name, "");
        assert_eq!(dashboard.description, None);
        assert_eq!(dashboard.url, None);
        assert_eq!(dashboard.create_time, None);
        assert_eq!(dashboard.update_time, None);
        assert_eq!(dashboard.creator, None);
        assert_eq!(dashboard.permissions, None);
    }

    #[test]
    fn test_dashboard_with_data() {
        let creator = Creator {
            user_id: Some("user_123".to_string()),
            name: Some("张三".to_string()),
            avatar: Some("avatar_url".to_string()),
        };

        let permission = Permission {
            permission_type: "read".to_string(),
            entity: "dashboard".to_string(),
            value: "allowed".to_string(),
        };

        let dashboard = Dashboard {
            block_id: "block_789".to_string(),
            name: "销售仪表盘".to_string(),
            description: Some("月度销售数据分析".to_string()),
            url: Some("https://example.com/dashboard".to_string()),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-01-15T00:00:00Z".to_string()),
            creator: Some(creator),
            permissions: Some(vec![permission]),
        };

        assert_eq!(dashboard.block_id, "block_789");
        assert_eq!(dashboard.name, "销售仪表盘");
        assert_eq!(dashboard.description, Some("月度销售数据分析".to_string()));
        assert_eq!(dashboard.url, Some("https://example.com/dashboard".to_string()));
        assert_eq!(
            dashboard.create_time,
            Some("2023-01-01T00:00:00Z".to_string())
        );
        assert_eq!(
            dashboard.update_time,
            Some("2023-01-15T00:00:00Z".to_string())
        );
        assert_eq!(dashboard.creator.as_ref().unwrap().user_id, Some("user_123".to_string()));
        assert_eq!(dashboard.creator.as_ref().unwrap().name, Some("张三".to_string()));
        assert_eq!(dashboard.permissions.as_ref().unwrap()[0].permission_type, "read");
    }

    #[test]
    fn test_creator_default_creation() {
        let creator = Creator::default();
        assert_eq!(creator.user_id, None);
        assert_eq!(creator.name, None);
        assert_eq!(creator.avatar, None);
    }

    #[test]
    fn test_permission_default_creation() {
        let permission = Permission::default();
        assert_eq!(permission.permission_type, "");
        assert_eq!(permission.entity, "");
        assert_eq!(permission.value, "");
    }

    #[test]
    fn test_dashboard_serialization() {
        let dashboard = Dashboard {
            block_id: "test_block".to_string(),
            name: "测试仪表盘".to_string(),
            description: Some("测试描述".to_string()),
            url: Some("https://test.com".to_string()),
            ..Default::default()
        };

        let serialized = serde_json::to_string(&dashboard).unwrap();
        let deserialized: Dashboard = serde_json::from_str(&serialized).unwrap();

        assert_eq!(dashboard.block_id, deserialized.block_id);
        assert_eq!(dashboard.name, deserialized.name);
        assert_eq!(dashboard.description, deserialized.description);
        assert_eq!(dashboard.url, deserialized.url);
    }

    #[test]
    fn test_response_trait_implementation() {
        use openlark_core::service::cloud_docs::bitable::v1::app_dashboard::list::ListDashboardsResponse;
        assert_eq!(ListDashboardsResponse::data_format(), ResponseFormat::Data);
    }
}