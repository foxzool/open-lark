//! Sheets电子表格服务 v3
//!
//! 提供飞书电子表格v3版本的完整管理功能，包括：
//! - 创建和删除电子表格
//! - 电子表格信息查询和管理
//! - 工作表操作和单元格管理
//! - 数据格式化和样式设置

pub mod spreadsheet;

// 重新导出所有服务类型
pub use spreadsheet::*;

use crate::core::config::Config;

/// Sheets电子表格服务 v3版本
///
/// 提供飞书电子表格v3版本的统一入口，支持现代化的电子表格管理。
/// 包括创建、编辑、格式化、数据验证等企业级功能。
#[derive(Debug, Clone)]
pub struct SheetsServiceV3 {
    config: Config,
    /// 电子表格管理服务
    pub spreadsheet: SpreadsheetService,
}

impl SheetsServiceV3 {
    /// 创建Sheets v3服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v3::SheetsServiceV3;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = SheetsServiceV3::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            spreadsheet: SpreadsheetService::new(config),
        }
    }
}

impl crate::core::trait_system::Service for SheetsServiceV3 {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "SheetsServiceV3"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::trait_system::Service;

    #[test]
    fn test_sheets_v3_service_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = SheetsServiceV3::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_v3_service_trait_implementation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = SheetsServiceV3::new(config);

        // 测试Service trait的实现
        let config_ref = service.config();
        assert_eq!(config_ref.app_id, "test_app_id");
    }

    #[test]
    fn test_spreadsheet_service_available() {
        let config = Config::default();
        let service = SheetsServiceV3::new(config);

        // 验证spreadsheet服务可用
        let spreadsheet_service_str = format!("{:?}", service.spreadsheet);
        assert!(!spreadsheet_service_str.is_empty());
    }
}