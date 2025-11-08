//! Sheets电子表格服务 v3
//!
//! 提供飞书电子表格v3版本的完整管理功能，包括：
//! - 创建和删除电子表格
//! - 电子表格信息查询和管理
//! - 工作表操作和单元格管理
//! - 数据格式化和样式设置
//! - 筛选视图和筛选条件管理

pub mod spreadsheet;
pub mod sheet;
pub mod filter_views;
pub mod data_filter;
pub mod conditional_format;
pub mod charts;

// 重新导出所有服务类型
pub use spreadsheet::*;
pub use sheet::*;
pub use filter_views::*;
pub use data_filter::*;
pub use conditional_format::*;
pub use charts::*;

use crate::core::config::Config;

/// Sheets电子表格服务 v3版本
///
/// 提供飞书电子表格v3版本的统一入口，支持现代化的电子表格管理。
/// 包括创建、编辑、格式化、数据验证、筛选视图、数据过滤器、条件格式、图表等企业级功能。
#[derive(Debug, Clone)]
pub struct SheetsServiceV3 {
    config: Config,
    /// 电子表格管理服务
    pub spreadsheet: SpreadsheetService,
    /// 工作表管理服务
    pub sheet: SheetService,
    /// 筛选视图管理服务
    pub filter_views: FilterViewsService,
    /// 数据过滤器管理服务
    pub data_filter: DataFilterService,
    /// 条件格式管理服务
    pub conditional_format: ConditionalFormatService,
    /// 图表管理服务
    pub charts: ChartService,
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
            spreadsheet: SpreadsheetService::new(config.clone()),
            sheet: SheetService::new(config.clone()),
            filter_views: FilterViewsService::new(config.clone()),
            data_filter: DataFilterService::new(config.clone()),
            conditional_format: ConditionalFormatService::new(config.clone()),
            charts: ChartService::new(config),
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

    #[test]
    fn test_filter_views_service_available() {
        let config = Config::default();
        let service = SheetsServiceV3::new(config);

        // 验证filter_views服务可用
        let filter_views_service_str = format!("{:?}", service.filter_views);
        assert!(!filter_views_service_str.is_empty());
    }

    #[test]
    fn test_data_filter_service_available() {
        let config = Config::default();
        let service = SheetsServiceV3::new(config);

        // 验证data_filter服务可用
        let data_filter_service_str = format!("{:?}", service.data_filter);
        assert!(!data_filter_service_str.is_empty());
    }

    #[test]
    fn test_conditional_format_service_available() {
        let config = Config::default();
        let service = SheetsServiceV3::new(config);

        // 验证conditional_format服务可用
        let conditional_format_service_str = format!("{:?}", service.conditional_format);
        assert!(!conditional_format_service_str.is_empty());
    }

    #[test]
    fn test_charts_service_available() {
        let config = Config::default();
        let service = SheetsServiceV3::new(config);

        // 验证charts服务可用
        let charts_service_str = format!("{:?}", service.charts);
        assert!(!charts_service_str.is_empty());
    }

    #[test]
    fn test_sheets_v3_complete_integration() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = SheetsServiceV3::new(config);

        // 验证所有服务都可用
        assert!(!format!("{:?}", service.spreadsheet).is_empty());
        assert!(!format!("{:?}", service.sheet).is_empty());
        assert!(!format!("{:?}", service.filter_views).is_empty());
        assert!(!format!("{:?}", service.data_filter).is_empty());
        assert!(!format!("{:?}", service.conditional_format).is_empty());
        assert!(!format!("{:?}", service.charts).is_empty());

        // 验证服务名
        assert_eq!(SheetsServiceV3::service_name(), "SheetsServiceV3");
        assert_eq!(service.config().app_id, "test_app_id");
    }
}
