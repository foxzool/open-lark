/// CCM Sheet API Old V2 模块
///
/// 包含所有表格操作相关的API实现
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

    // 所有API实现已移除，因为对应的子模块已在清理中被删除
    // 如果需要以前的功能，请参考git历史恢复相关文件
}

// 导出所有API模块
// pub mod adddimensionrange; // Generated: Module file not found
// pub mod addprotectedrange; // Generated: Module file not found
// pub mod appendvalues; // Generated: Module file not found
// pub mod batchsetstyle; // Generated: Module file not found
// pub mod batchwriteranges; // Generated: Module file not found
// pub mod createconditionformat; // Generated: Module file not found
// pub mod deleteconditionformat; // Generated: Module file not found
// pub mod deletedimensionrange; // Generated: Module file not found
// pub mod deletedropdown; // Generated: Module file not found
// pub mod deleteprotectedrange; // Generated: Module file not found
// pub mod getconditionformat; // Generated: Module file not found
// pub mod getdropdown; // Generated: Module file not found
// pub mod getimportresult; // Generated: Module file not found
// pub mod getprotectedrange; // Generated: Module file not found
// pub mod getspreadsheetmeta; // Generated: Module file not found
// pub mod importspreadsheet; // Generated: Module file not found
// pub mod insertdimensionrange; // Generated: Module file not found
// pub mod insertvalues; // Generated: Module file not found
// pub mod mergecells; // Generated: Module file not found
// pub mod operatesheets; // Generated: Module file not found
// pub mod readmultipleranges; // Generated: Module file not found
// pub mod readsinglerange; // Generated: Module file not found
// pub mod setdropdown; // Generated: Module file not found
// pub mod setstyle; // Generated: Module file not found
// pub mod unmergecells; // Generated: Module file not found
// pub mod updateconditionformat; // Generated: Module file not found
// pub mod updatedimensionrange; // Generated: Module file not found
// pub mod updatedropdown; // Generated: Module file not found
// pub mod updateprotectedrange; // Generated: Module file not found
// pub mod updatesheetproperties; // Generated: Module file not found
// pub mod updatespreadsheetproperties; // Generated: Module file not found
// pub mod writeimage; // Generated: Module file not found
// pub mod writesinglerange; // Generated: Module file not found

// 新增的函数式API模块
// pub mod spreadsheets; // Generated: Module file not found

// 重新导出所有函数式API，方便外部调用
pub use crate::ccm::sheets::*;
