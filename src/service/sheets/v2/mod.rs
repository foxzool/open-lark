//! Sheets电子表格服务 v2
//!
//! 提供飞书电子表格v2版本的完整管理功能，包括：
//! - 创建和删除电子表格
//! - 电子表格信息查询和管理
//! - 工作表操作和单元格管理
//! - 工作表批量更新，支持增删改查操作
//! - 单元格内容更新和格式化
//! - 数据读写操作
//! - 图片写入和管理
//! - 单个范围数据写入
//! - 数据追加功能，支持CSV、HashMap等多种格式
//! - 数据前置插入功能，智能移动现有数据
//! - 单元格合并和拆分操作
//! - 行列插入、删除和移动操作
//! - 单元格样式设置和格式化
//! - 数据验证和下拉列表设置

pub mod sheet_cells;
pub mod batch_read;
pub mod batch_read_ranges;
pub mod batch_write;
pub mod image_write;
pub mod single_write;
pub mod sheet_management;
pub mod merge_cells;
pub mod dimension_operations;
pub mod style_operations;
pub mod data_validation;
pub mod values_append;
pub mod sheets_batch_update;
pub mod values_prepend;

// 重新导出所有服务类型
pub use sheet_cells::*;
pub use batch_read::*;
pub use batch_read_ranges::*;
pub use batch_write::*;
pub use image_write::*;
pub use single_write::*;
pub use sheet_management::*;
pub use merge_cells::*;
pub use dimension_operations::*;
pub use style_operations::*;
pub use data_validation::*;
pub use values_append::*;
pub use sheets_batch_update::*;
pub use values_prepend::*;

use crate::core::config::Config;

/// Sheets电子表格服务 v2版本
///
/// 提供飞书电子表格v2版本的统一入口，支持现代化的电子表格管理。
/// 包括创建、编辑、格式化、数据读写、图片管理、单个范围写入、数据追加、工作表批量更新等企业级功能。
#[derive(Debug, Clone)]
pub struct SheetsServiceV2 {
    config: Config,
    /// 单元格管理服务
    pub sheet_cells: SheetCellsService,
    /// 批量读取服务
    pub batch_read: BatchReadService,
    /// 批量范围读取服务
    pub batch_read_ranges: BatchReadRangesService,
    /// 批量写入服务
    pub batch_write: BatchWriteService,
    /// 图片写入服务
    pub image_write: ImageWriteService,
    /// 单个范围写入服务
    pub single_write: SingleWriteService,
    /// 工作表管理服务
    pub sheet_management: SheetManagementService,
    /// 单元格合并服务
    pub merge_cells: MergeCellsService,
    /// 行列操作服务
    pub dimension_operations: DimensionOperationsService,
    /// 样式操作服务
    pub style_operations: StyleOperationsService,
    /// 数据验证服务
    pub data_validation: DataValidationService,
    /// 数据追加服务
    pub values_append: ValuesAppendService,
    /// 工作表批量更新服务
    pub sheets_batch_update: SheetsBatchUpdateService,
    /// 数据前置插入服务
    pub values_prepend: ValuesPrependService,
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
            batch_read_ranges: BatchReadRangesService::new(config.clone()),
            batch_write: BatchWriteService::new(config.clone()),
            image_write: ImageWriteService::new(config.clone()),
            single_write: SingleWriteService::new(config.clone()),
            sheet_management: SheetManagementService::new(config.clone()),
            merge_cells: MergeCellsService::new(config.clone()),
            dimension_operations: DimensionOperationsService::new(config.clone()),
        style_operations: StyleOperationsService::new(config.clone()),
            data_validation: DataValidationService::new(config.clone()),
            values_append: ValuesAppendService::new(config.clone()),
            sheets_batch_update: SheetsBatchUpdateService::new(config.clone()),
            values_prepend: ValuesPrependService::new(config),
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
    fn test_batch_read_ranges_service_available() {
        let config = Config::default();
        let service = SheetsServiceV2::new(config);

        // 验证batch_read服务可用
        let batch_read_service_str = format!("{:?}", service.batch_read);
        assert!(!batch_read_service_str.is_empty());

        // 验证batch_read_ranges服务可用
        let batch_read_ranges_service_str = format!("{:?}", service.batch_read_ranges);
        assert!(!batch_read_ranges_service_str.is_empty());
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
        assert!(!format!("{:?}", service.single_write).is_empty());

        // 验证服务名
        assert_eq!(SheetsServiceV2::service_name(), "SheetsServiceV2");
        assert_eq!(service.config().app_id, "test_app_id");
    }

    #[test]
    fn test_single_write_service_available() {
        let config = Config::default();
        let service = SheetsServiceV2::new(config);

        // 验证single_write服务可用
        let single_write_service_str = format!("{:?}", service.single_write);
        assert!(!single_write_service_str.is_empty());

        // 验证服务名称和版本
        assert_eq!(crate::service::sheets::v2::single_write::SingleWriteService::service_name(), "SingleWriteService");
        assert_eq!(crate::service::sheets::v2::single_write::SingleWriteService::service_version(), "v2");
    }

    #[test]
    fn test_sheet_management_service_available() {
        let config = Config::default();
        let service = SheetsServiceV2::new(config);

        // 验证sheet_management服务可用
        let sheet_management_service_str = format!("{:?}", service.sheet_management);
        assert!(!sheet_management_service_str.is_empty());
    }

    #[test]
    fn test_merge_cells_service_available() {
        let config = Config::default();
        let service = SheetsServiceV2::new(config);

        // 验证merge_cells服务可用
        let merge_cells_service_str = format!("{:?}", service.merge_cells);
        assert!(!merge_cells_service_str.is_empty());
    }

    #[test]
    fn test_dimension_operations_service_available() {
        let config = Config::default();
        let service = SheetsServiceV2::new(config);

        // 验证dimension_operations服务可用
        let dimension_operations_service_str = format!("{:?}", service.dimension_operations);
        assert!(!dimension_operations_service_str.is_empty());
    }

    #[test]
    fn test_style_operations_service_available() {
        let config = Config::default();
        let service = SheetsServiceV2::new(config);

        // 验证style_operations服务可用
        let style_operations_service_str = format!("{:?}", service.style_operations);
        assert!(!style_operations_service_str.is_empty());
    }

    #[test]
    fn test_data_validation_service_available() {
        let config = Config::default();
        let service = SheetsServiceV2::new(config);

        // 验证data_validation服务可用
        let data_validation_service_str = format!("{:?}", service.data_validation);
        assert!(!data_validation_service_str.is_empty());
    }

    #[test]
    fn test_sheets_v2_single_write_integration() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = SheetsServiceV2::new(config);

        // 验证single_write服务集成
        assert!(!format!("{:?}", service.single_write).is_empty());

        // 验证配置传递正确
        assert_eq!(service.config().app_id, "test_app_id");
    }

    #[test]
    fn test_values_append_service_available() {
        let config = Config::default();
        let service = SheetsServiceV2::new(config);

        // 验证values_append服务可用
        let values_append_service_str = format!("{:?}", service.values_append);
        assert!(!values_append_service_str.is_empty());
    }

    #[test]
    fn test_sheets_batch_update_service_available() {
        let config = Config::default();
        let service = SheetsServiceV2::new(config);

        // 验证sheets_batch_update服务可用
        let sheets_batch_update_service_str = format!("{:?}", service.sheets_batch_update);
        assert!(!sheets_batch_update_service_str.is_empty());
    }

    #[test]
    fn test_values_prepend_service_available() {
        let config = Config::default();
        let service = SheetsServiceV2::new(config);

        // 验证values_append服务可用
        let values_append_service_str = format!("{:?}", service.values_append);
        assert!(!values_append_service_str.is_empty());

        // 验证values_prepend服务可用
        let values_prepend_service_str = format!("{:?}", service.values_prepend);
        assert!(!values_prepend_service_str.is_empty());
    }

    #[test]
    fn test_sheets_v2_complete_data_operations_integration() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = SheetsServiceV2::new(config);

        // 验证数据操作相关服务都可用
        assert!(!format!("{:?}", service.batch_read).is_empty());
        assert!(!format!("{:?}", service.batch_write).is_empty());
        assert!(!format!("{:?}", service.single_write).is_empty());
        assert!(!format!("{:?}", service.values_append).is_empty());
        assert!(!format!("{:?}", service.values_prepend).is_empty());
        assert!(!format!("{:?}", service.sheets_batch_update).is_empty());

        // 验证服务名和版本
        assert_eq!(SheetsServiceV2::service_name(), "SheetsServiceV2");
        assert_eq!(service.config().app_id, "test_app_id");

        // 验证数据前置插入服务配置
        let prepend_service_str = format!("{:?}", service.values_prepend);
        assert!(!prepend_service_str.is_empty());
    }
}