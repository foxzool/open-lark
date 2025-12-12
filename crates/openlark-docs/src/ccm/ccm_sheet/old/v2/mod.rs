//! CCM Sheet API Old V2 模块
//!
//! 包含所有表格操作相关的API实现

use openlark_core::config::Config;

/// 表格服务
#[derive(Debug, Clone)]
pub struct CcmSheetOldV2 {
    config: Config,
}

impl CcmSheetOldV2 {
    /// 创建新的表格服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 操作工作表
    pub fn operatesheets(&self) -> operatesheets::OperateSheetsRequest {
        operatesheets::OperateSheetsRequest::new(self.config.clone())
    }

    /// 读取单个范围
    pub fn readsinglerange(&self) -> readsinglerange::ReadSingleRangeRequest {
        readsinglerange::ReadSingleRangeRequest::new(self.config.clone())
    }

    /// 写入单个范围
    pub fn writesinglerange(&self) -> writesinglerange::WriteSingleRangeRequest {
        writesinglerange::WriteSingleRangeRequest::new(self.config.clone())
    }

    /// 合并单元格
    pub fn mergecells(&self) -> mergecells::MergeCellsRequest {
        mergecells::MergeCellsRequest::new(self.config.clone())
    }

    /// 拆分单元格
    pub fn unmergecells(&self) -> unmergecells::UnmergeCellsRequest {
        unmergecells::UnmergeCellsRequest::new(self.config.clone())
    }

    /// 更新工作表属性
    pub fn updatesheetproperties(&self) -> updatesheetproperties::UpdateSheetPropertiesRequest {
        updatesheetproperties::UpdateSheetPropertiesRequest::new(self.config.clone())
    }

    /// 增加行列
    pub fn adddimensionrange(&self) -> adddimensionrange::AddDimensionRangeRequest {
        adddimensionrange::AddDimensionRangeRequest::new(self.config.clone())
    }

    /// 插入行列
    pub fn insertdimensionrange(&self) -> insertdimensionrange::InsertDimensionRangeRequest {
        insertdimensionrange::InsertDimensionRangeRequest::new(self.config.clone())
    }

    /// 更新行列
    pub fn updatedimensionrange(&self) -> updatedimensionrange::UpdateDimensionRangeRequest {
        updatedimensionrange::UpdateDimensionRangeRequest::new(self.config.clone())
    }

    /// 删除行列
    pub fn deletedimensionrange(&self) -> deletedimensionrange::DeleteDimensionRangeRequest {
        deletedimensionrange::DeleteDimensionRangeRequest::new(self.config.clone())
    }

    /// 设置单元格样式
    pub fn setstyle(&self) -> setstyle::SetStyleRequest {
        setstyle::SetStyleRequest::new(self.config.clone())
    }

    /// 插入数据
    pub fn insertvalues(&self) -> insertvalues::InsertValuesRequest {
        insertvalues::InsertValuesRequest::new(self.config.clone())
    }

    /// 读取多个范围
    pub fn readmultipleranges(&self) -> readmultipleranges::ReadMultipleRangesRequest {
        readmultipleranges::ReadMultipleRangesRequest::new(self.config.clone())
    }

    /// 批量写入范围
    pub fn batchwriteranges(&self) -> batchwriteranges::BatchWriteRangesRequest {
        batchwriteranges::BatchWriteRangesRequest::new(self.config.clone())
    }

    /// 写入图片
    pub fn writeimage(&self) -> writeimage::WriteImageRequest {
        writeimage::WriteImageRequest::new(self.config.clone())
    }

    /// 批量设置单元格样式
    pub fn batchsetstyle(&self) -> batchsetstyle::BatchSetStyleRequest {
        batchsetstyle::BatchSetStyleRequest::new(self.config.clone())
    }

    /// 追加数据
    pub fn appendvalues(&self) -> appendvalues::AppendValuesRequest {
        appendvalues::AppendValuesRequest::new(self.config.clone())
    }

    /// 增加保护范围
    pub fn addprotectedrange(&self) -> addprotectedrange::AddProtectedRangeRequest {
        addprotectedrange::AddProtectedRangeRequest::new(self.config.clone())
    }

    /// 更新保护范围
    pub fn updateprotectedrange(&self) -> updateprotectedrange::UpdateProtectedRangeRequest {
        updateprotectedrange::UpdateProtectedRangeRequest::new(self.config.clone())
    }

    /// 获取保护范围
    pub fn getprotectedrange(&self) -> getprotectedrange::GetProtectedRangeRequest {
        getprotectedrange::GetProtectedRangeRequest::new(self.config.clone())
    }

    /// 删除保护范围
    pub fn deleteprotectedrange(&self) -> deleteprotectedrange::DeleteProtectedRangeRequest {
        deleteprotectedrange::DeleteProtectedRangeRequest::new(self.config.clone())
    }

    /// 创建数据验证规则
    pub fn setdropdown(&self) -> setdropdown::SetDropdownRequest {
        setdropdown::SetDropdownRequest::new(self.config.clone())
    }

    /// 更新数据验证规则
    pub fn updatedropdown(&self) -> updatedropdown::UpdateDropdownRequest {
        updatedropdown::UpdateDropdownRequest::new(self.config.clone())
    }

    /// 获取数据验证规则
    pub fn getdropdown(&self) -> getdropdown::GetDropdownRequest {
        getdropdown::GetDropdownRequest::new(self.config.clone())
    }

    /// 删除数据验证规则
    pub fn deletedropdown(&self) -> deletedropdown::DeleteDropdownRequest {
        deletedropdown::DeleteDropdownRequest::new(self.config.clone())
    }

    /// 批量创建条件格式
    pub fn createconditionformat(&self) -> createconditionformat::CreateConditionFormatRequest {
        createconditionformat::CreateConditionFormatRequest::new(self.config.clone())
    }

    /// 批量更新条件格式
    pub fn updateconditionformat(&self) -> updateconditionformat::UpdateConditionFormatRequest {
        updateconditionformat::UpdateConditionFormatRequest::new(self.config.clone())
    }

    /// 获取条件格式
    pub fn getconditionformat(&self) -> getconditionformat::GetConditionFormatRequest {
        getconditionformat::GetConditionFormatRequest::new(self.config.clone())
    }

    /// 批量删除条件格式
    pub fn deleteconditionformat(&self) -> deleteconditionformat::DeleteConditionFormatRequest {
        deleteconditionformat::DeleteConditionFormatRequest::new(self.config.clone())
    }

    /// 获取表格元数据
    pub fn getspreadsheetmeta(&self) -> getspreadsheetmeta::GetSpreadsheetMetaRequest {
        getspreadsheetmeta::GetSpreadsheetMetaRequest::new(self.config.clone())
    }

    /// 更新表格属性
    pub fn updatespreadsheetproperties(&self) -> updatespreadsheetproperties::UpdateSpreadsheetPropertiesRequest {
        updatespreadsheetproperties::UpdateSpreadsheetPropertiesRequest::new(self.config.clone())
    }

    /// 导入表格
    pub fn importspreadsheet(&self) -> importspreadsheet::ImportSpreadsheetRequest {
        importspreadsheet::ImportSpreadsheetRequest::new(self.config.clone())
    }

    /// 查询导入结果
    pub fn getimportresult(&self) -> getimportresult::GetImportResultRequest {
        getimportresult::GetImportResultRequest::new(self.config.clone())
    }
}

// 导出所有API模块
pub mod operatesheets;
pub mod updatesheetproperties;
pub mod adddimensionrange;
pub mod insertdimensionrange;
pub mod updatedimensionrange;
pub mod deletedimensionrange;
pub mod mergecells;
pub mod unmergecells;
pub mod setstyle;
pub mod batchsetstyle;
pub mod insertvalues;
pub mod appendvalues;
pub mod writeimage;
pub mod readsinglerange;
pub mod readmultipleranges;
pub mod writesinglerange;
pub mod batchwriteranges;
pub mod addprotectedrange;
pub mod updateprotectedrange;
pub mod getprotectedrange;
pub mod deleteprotectedrange;
pub mod setdropdown;
pub mod updatedropdown;
pub mod getdropdown;
pub mod deletedropdown;
pub mod createconditionformat;
pub mod updateconditionformat;
pub mod getconditionformat;
pub mod deleteconditionformat;
pub mod getspreadsheetmeta;
pub mod updatespreadsheetproperties;
pub mod importspreadsheet;
pub mod getimportresult;

// 新增的函数式API模块
pub mod spreadsheets;

// 重新导出所有函数式API，方便外部调用
pub use spreadsheets::*;