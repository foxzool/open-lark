pub use app_table_field::*;
pub use app_table_record::*;

// 为了避免名称冲突，使用模块路径而不是全局导入
pub mod app;
pub mod app_table;

use crate::core::config::Config;

mod app_table_field;
mod app_table_record;

mod share;
pub use share::*;

pub struct V1 {
    /// 多维表格
    pub app: app::AppService,
    /// 数据表
    pub app_table: app_table::AppTableService,
    /// 字段
    pub app_table_field: AppTableFieldService,
    /// 记录
    pub app_table_record: AppTableRecordService,
}

impl V1 {
    pub fn new(config: Config) -> Self {
        Self {
            app: app::AppService::new(config.clone()),
            app_table: app_table::AppTableService::new(config.clone()),
            app_table_field: AppTableFieldService::new(config.clone()),
            app_table_record: AppTableRecordService::new(config),
        }
    }
}
