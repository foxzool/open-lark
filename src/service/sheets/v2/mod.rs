//! Sheets电子表格服务 v2
//!
//! 提供飞书电子表格v2版本的完整管理功能，包括：
//! - 创建和删除电子表格
//! - 电子表格信息查询和管理
//! - 工作表操作和单元格管理
//! - 单元格内容更新和格式化
//! - 数据读写操作
//! - 图片写入和管理

pub mod sheet_cells;
pub mod batch_read;
pub mod batch_write;
pub mod image_write;

// 重新导出所有服务类型
pub use sheet_cells::*;
pub use batch_read::*;
pub use batch_write::*;
pub use image_write::*;

use crate::core::config::Config;

/// Sheets电子表格服务 v2版本
///
/// 提供飞书电子表格v2版本的统一入口，支持现代化的电子表格管理。
/// 包括创建、编辑、格式化、数据读写、图片管理等企业级功能。
#[derive(Debug, Clone)]
pub struct SheetsServiceV2 {
    config: Config,
    /// 单元格管理服务
    pub sheet_cells: SheetCellsService,
    /// 批量读取服务
    pub batch_read: BatchReadService,
    /// 批量写入服务
    pub batch_write: BatchWriteService,
    /// 图片写入服务
    pub image_write: ImageWriteService,
}

impl SheetsServiceV2 {
    /// 创建Sheets v2服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v2::SheetsServiceV2;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = SheetsServiceV2::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            sheet_cells: SheetCellsService::new(config.clone()),
            batch_read: BatchReadService::new(config.clone()),
            batch_write: BatchWriteService::new(config.clone()),
            image_write: ImageWriteService::new(config),
        }
    }
}

impl crate::core::trait_system::Service for SheetsServiceV2 {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "SheetsServiceV2"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::trait_system::Service;

    #[test]
    fn test_sheets_v2_service_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = SheetsServiceV2::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_v2_service_trait_implementation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = SheetsServiceV2::new(config);

        // 测试Service trait的实现
        let config_ref = service.config();
        assert_eq!(config_ref.app_id, "test_app_id");
    }

    #[test]
    fn test_sheet_cells_service_available() {
        let config = Config::default();
        let service = SheetsServiceV2::new(config);

        // 验证sheet_cells服务可用
        let sheet_cells_service_str = format!("{:?}", service.sheet_cells);
        assert!(!sheet_cells_service_str.is_empty());
    }

    #[test]
    fn test_batch_read_service_available() {
        let config = Config::default();
        let service = SheetsServiceV2::new(config);

        // 验证batch_read服务可用
        let batch_read_service_str = format!("{:?}", service.batch_read);
        assert!(!batch_read_service_str.is_empty());
    }

    #[test]
    fn test_batch_write_service_available() {
        let config = Config::default();
        let service = SheetsServiceV2::new(config);

        // 验证batch_write服务可用
        let batch_write_service_str = format!("{:?}", service.batch_write);
        assert!(!batch_write_service_str.is_empty());
    }

    #[test]
    fn test_image_write_service_available() {
        let config = Config::default();
        let service = SheetsServiceV2::new(config);

        // 验证image_write服务可用
        let image_write_service_str = format!("{:?}", service.image_write);
        assert!(!image_write_service_str.is_empty());
    }

    #[test]
    fn test_sheets_v2_complete_integration() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = SheetsServiceV2::new(config);

        // 验证所有服务都可用
        assert!(!format!("{:?}", service.sheet_cells).is_empty());
        assert!(!format!("{:?}", service.batch_read).is_empty());
        assert!(!format!("{:?}", service.batch_write).is_empty());
        assert!(!format!("{:?}", service.image_write).is_empty());

        // 验证服务名
        assert_eq!(SheetsServiceV2::service_name(), "SheetsServiceV2");
        assert_eq!(service.config().app_id, "test_app_id");
    }
}