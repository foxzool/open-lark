//! Sheets电子表格服务
//!
//! 提供飞书电子表格的完整管理功能，包括：
//! - 创建和删除电子表格
//! - 读取和更新电子表格内容
//! - 单元格格式化操作
//! - 工作表管理
//! - 数据验证和过滤
//!
//! # 服务架构
//!
//! ## v3版本
//! - [`v3::spreadsheet`] - 电子表格基础管理服务，提供创建、查询等核心功能
//!
//! # 使用示例
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuild)
//!     .build();
//!
//! // 创建电子表格
//! let spreadsheet = client.sheets.v3.spreadsheet
//!     .create_spreadsheet_builder()
//!     .title("财务报表")
//!     .folder_token("folder_token")
//!     .execute(&client.sheets.v3.spreadsheet)
//!     .await?;
//! ```

pub mod v3;

// 重新导出所有服务类型
pub use v3::*;

use crate::core::config::Config;

/// Sheets电子表格服务
///
/// 提供飞书电子表格的统一入口，支持电子表格的全生命周期管理。
/// 包括创建、编辑、分享、协作等企业级功能。
#[derive(Debug, Clone)]
pub struct SheetsService {
    config: Config,
    /// v3版本服务
    pub v3: v3::SheetsServiceV3,
}

impl SheetsService {
    /// 创建Sheets服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::SheetsService;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = SheetsService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            #[cfg(feature = "collaboration")]
            v3: v3::SheetsServiceV3::new(config),
        }
    }
}

impl crate::core::trait_system::Service for SheetsService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "SheetsService"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::trait_system::Service;

    #[test]
    fn test_sheets_service_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = SheetsService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_service_trait_implementation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = SheetsService::new(config);

        // 测试Service trait的实现
        let config_ref = service.config();
        assert_eq!(config_ref.app_id, "test_app_id");
    }
}
