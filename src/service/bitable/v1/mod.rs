pub use app::*;
pub use app_table_field::*;
pub use app_table_record::*;

use crate::core::config::Config;

mod app;
mod app_table_field;
mod app_table_record;

pub struct V1 {
    /// 多维表格
    pub app: AppService,
    /// 字段
    pub app_table_field: AppTableFieldService,
    /// 记录
    pub app_table_record: AppTableRecordService,
}

impl V1 {
    pub fn new(config: Config) -> Self {
        Self {
            app: AppService::new(config.clone()),
            app_table_field: AppTableFieldService::new(config.clone()),
            app_table_record: AppTableRecordService::new(config),
        }
    }
}
