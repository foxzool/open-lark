//! 电子表格（Sheets）服务
//!
//! 提供飞书电子表格的完整API功能，支持数据读写、格式设置、公式计算等。
//! 是处理结构化数据和报表的强大工具。
//!
//! # API版本
//!
//! - **v2**: 基础电子表格操作
//! - **v3**: 增强功能和高级特性（推荐）
//!
//! # 主要功能
//!
//! ## V2 版本功能
//! - 📊 数据读写操作
//! - 🎨 单元格样式设置
//! - 🔗 单元格合并拆分
//! - 📏 行列操作
//! - 📋 工作表管理
//!
//! ## V3 版本功能（更强大）
//! - 📈 电子表格完整生命周期
//! - 🔍 数据查找和替换
//! - 🎯 条件格式化
//! - ✅ 数据验证规则
//! - 🔒 单元格保护
//! - 🖼️ 浮动图片
//! - 🔽 筛选器和筛选视图
//!
//! # 快速开始
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuild)
//!     .build();
//!
//! // V3 版本 - 创建电子表格
//! // let create_request = CreateSpreadsheetRequest::builder()
//! //     .title("销售数据统计")
//! //     .folder_token("folder_token")
//! //     .build();
//! // let spreadsheet = client.sheets.v3.spreadsheet.create(create_request, None).await?;
//!
//! // V3 版本 - 写入数据
//! // let write_request = WriteDataToMultipleRangesRequest::builder()
//! //     .spreadsheet_token("spreadsheet_token")
//! //     .value_ranges(vec![...])
//! //     .build();
//! // client.sheets.v3.data_operation.write_data_to_multiple_ranges(write_request, None).await?;
//! ```

/// Sheets API v2版本
pub mod v2;
/// Sheets API v3版本
pub mod v3;

/// 电子表格服务
///
/// 聚合所有Sheets相关的API版本，提供统一的电子表格操作接口。
/// 推荐使用v3版本获得最新功能和最佳性能。
pub struct SheetsService {
    /// Sheets API v2版本服务
    pub v2: v2::V2,
    /// Sheets API v3版本服务（推荐）
    pub v3: v3::V3,
}

impl SheetsService {
    /// 创建新的Sheets服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置
    pub fn new(config: crate::core::config::Config) -> Self {
        Self {
            v2: v2::V2::new(config.clone()),
            v3: v3::V3::new(config.clone()),
        }
    }
}
