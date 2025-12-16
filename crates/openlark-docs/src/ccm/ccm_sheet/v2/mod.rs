/// CCM Sheet V2 API 主模块
///
/// 包含所有V2版本表格相关API的模块：
/// - data_io: 数据读写操作 (8个API)
/// - sheet_operations: 表格操作 (7个API)
/// - filter: 筛选功能 (4个API)
/// - float_image: 浮图功能 (4个API)
/// - spreadsheet: 表格基础操作 (3个API)
/// - sheet: 工作表操作 (4个API)
///
/// 总计：30个核心表格API
use openlark_core::config::Config;

/// CCM Sheet V2 API访问器
#[derive(Debug, Clone)]
pub struct CcmSheetV2 {
    config: Config,
}

impl CcmSheetV2 {
    /// 创建新的V2 API访问器实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取数据读写API
    pub fn data_io(&self) -> crate::ccm::ccm_sheet::v2::data_io::DataIOApi {
        crate::ccm::ccm_sheet::v2::data_io::DataIOApi::new(self.config.clone())
    }

    /// 获取表格操作API
    pub fn sheet_operations(
        &self,
    ) -> crate::ccm::ccm_sheet::v2::sheet_operations::SheetOperationsApi {
        crate::ccm::ccm_sheet::v2::sheet_operations::SheetOperationsApi::new(self.config.clone())
    }

    /// 获取筛选功能API
    pub fn filter(&self) -> crate::ccm::ccm_sheet::v2::filter::FilterApi {
        crate::ccm::ccm_sheet::v2::filter::FilterApi::new(self.config.clone())
    }

    /// 获取浮图功能API
    pub fn float_image(&self) -> crate::ccm::ccm_sheet::v2::float_image::FloatImageApi {
        crate::ccm::ccm_sheet::v2::float_image::FloatImageApi::new(self.config.clone())
    }

    /// 获取表格基础API
    pub fn spreadsheet(&self) -> crate::ccm::ccm_sheet::v2::spreadsheet::SpreadsheetApi {
        crate::ccm::ccm_sheet::v2::spreadsheet::SpreadsheetApi::new(self.config.clone())
    }

    /// 获取工作表API
    pub fn sheet(&self) -> crate::ccm::ccm_sheet::v2::sheet::SheetApi {
        crate::ccm::ccm_sheet::v2::sheet::SheetApi::new(self.config.clone())
    }
}

// 导出所有子模块
pub mod data_io;
pub mod filter;
pub mod float_image;
pub mod sheet;
pub mod sheet_operations;
pub mod spreadsheet;

// 重新导出主要的API结构体
pub use data_io::DataIOApi;
pub use filter::FilterApi;
pub use float_image::FloatImageApi;
pub use sheet::SheetApi;
pub use sheet_operations::SheetOperationsApi;
pub use self::spreadsheet::SpreadsheetApi;
