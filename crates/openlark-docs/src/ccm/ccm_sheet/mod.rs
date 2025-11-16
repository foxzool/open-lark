//! CCM Sheet API模块
//!
//! 提供电子表格操作相关的功能，包括：
//! - 单元格读写操作（单个范围、多个范围）
//! - 批量数据处理（批量写入、追加数据）
//! - 工作表管理（批量更新）
//! - 维度操作（插入行列、删除行列）
//! - 样式设置（单元格样式、格式设置）
//! - 表格元数据管理
//!
//! # 示例
//!
//! ```rust
//! use openlark_docs::ccm::ccm_sheet::{CcmSheetService, ReadSingleRangeRequest};
//!
//! let service = CcmSheetService::new(config);
//!
//! // 读取单个范围
//! let response = service
//!     .read_single_range_builder()
//!     .spreadsheet_token("token_xxx")
//!     .range("Sheet1!A1:C10")
//!     .include_format(true)
//!     .execute(&service)
//!     .await?;
//!
//! // 写入数据
//! let write_response = service
//!     .write_single_range_builder()
//!     .spreadsheet_token("token_xxx")
//!     .range("Sheet1!D1:D10")
//!     .values(vec![
//!         vec!["姓名".into(), "年龄".into(), "城市".into()],
//!         vec!["张三".into(), 25.into(), "北京".into()],
//!         vec!["李四".into(), 30.into(), "上海".into()],
//!     ])
//!     .execute(&service)
//!     .await?;
//!
//! println!("写入了 {} 个单元格", write_response.updated_cells.unwrap_or(0));
//! ```

/// 数据模型定义
pub mod models;

/// API服务实现
pub mod services;

// 重新导出主要类型
pub use models::*;
pub use services::{
    AppendDataRequestBuilder, CcmSheetService, ReadSingleRangeRequestBuilder,
    WriteSingleRangeRequestBuilder,
};
