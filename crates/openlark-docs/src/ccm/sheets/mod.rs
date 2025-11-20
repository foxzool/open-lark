//! Sheets电子表格服务
//!
//! 提供飞书电子表格的完整管理功能，支持多版本API：
//!
//! # 版本支持
//!
//! ## v3版本 - 稳定版本（推荐）
//! - 完整的企业级功能
//! - 现代化API设计
//! - 构建器模式支持
//! - 高级分析和可视化功能
//! - 100% API覆盖率（27/27个API）
//! - 生产环境就绪
//!
//! ## v2版本 - 实验性功能
//! - 基础电子表格操作
//! - 单元格读写和格式化
//! - 工作表管理
//! - 数据验证和样式设置
//! - 正在修复中，部分功能可用
//! - 30/30个API已实现
//!
//! # 使用方法
//!
//! ## 启用所需功能
//!
//! 在 `Cargo.toml` 中添加相应的功能标志：
//!
//! ```toml
//! [dependencies]
//! open-lark = { version = "0.15.0", features = ["ccm-sheets"] }
//!
//! # 仅使用v3版本（推荐）
//! open-lark = { version = "0.15.0", features = ["ccm-sheets-v3"] }
//!
//! # 实验性v2版本
//! open-lark = { version = "0.15.0", features = ["ccm-sheets-v2"] }
//!
//! # 同时启用v2和v3
//! open-lark = { version = "0.15.0", features = ["ccm-sheets-v2", "ccm-sheets-v3"] }
//! ```
//!
//! ## 基础示例
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuild)
//!     .build();
//!
//! // v3版本 - 稳定现代化调用方式（推荐）
//! let sheets = client.sheets.v3.sheet
//!     .query_sheets(&spreadsheet_token)
//!     .await?;
//!
//! let charts = client.sheets.v3.charts
//!     .create_chart(&chart_request)
//!     .await?;
//!
//! // v2版本 - 实验性功能
//! #[cfg(feature = "ccm-sheets-v2")]
//! {
//!     let sheet_info = client.sheets.v2.single_range_read
//!         .read_range(&spreadsheet_token, "Sheet1!A1:C10")
//!         .await?;
//! }
//! ```
//!
//! # 功能对比
//!
//! | 功能特性 | v2版本 | v3版本 |
//! |---------|--------|--------|
//! | 基础CRUD | ✅ | ✅ |
//! | 构建器模式 | ❌ | ✅ |
//! | 条件格式 | 基础 | 高级 |
//! | 图表支持 | ❌ | ✅ |
//! | 数据透视表 | ✅ | ✅ |
//! | 评论协作 | ❌ | ✅ |
//! | 宏自动化 | ❌ | ✅ |
//! | 筛选视图 | ❌ | ✅ |
//! | 工作表保护 | ❌ | ✅ |
//! | 查找替换 | ❌ | ✅ |
//! | 状态稳定性 | 实验性 | 稳定 |
//!
//! # 版本选择建议
//!
//! ## 推荐方案
//! - **新项目**: 直接使用v3版本，功能完整且稳定
//! - **现有项目**: 建议迁移到v3版本，获得更好的功能支持
//! - **兼容性需求**: 可同时启用v2和v3，但建议使用v3作为主要版本
//!
//! ## 迁移指南
//!
//! 从v2迁移到v3的主要变更：
//! 1. API路径从 `/open-apis/sheets/v2/` 改为 `/open-apis/sheets/v3/`
//! 2. 请求和响应结构有所变化
//! 3. v3版本提供构建器模式，使用更便捷
//! 4. v3版本错误处理更加完善
//!
//! # 架构设计
//!
//! Sheets服务采用分层架构设计：
//!
//! ```text
//! SheetsService (主入口)
//! ├── v2/ (v2版本实现)
//! │   ├── single_range_read
//! │   ├── batch_read
//! │   ├── sheet_management
//! │   └── ...
//! └── v3/ (v3版本实现)
//!     ├── spreadsheet
//!     ├── sheet
//!     ├── charts
//!     ├── conditional_format
//!     └── ...
//! ```
//!
//! # 最佳实践
//!
//! ## 1. 版本选择建议
//! - **新项目**: 推荐使用v3版本，功能更完整
//! - **现有项目**: 可继续使用v2版本，或逐步迁移到v3
//! - **混合使用**: 支持同时启用v2和v3，灵活选择
//!
//! ## 2. 性能优化
//! - 合理使用批量操作API
//! - 设置适当的分页参数
//! - 启用客户端缓存
//!
//! ## 3. 错误处理
//! - 实现统一的错误处理机制
//! - 记录详细的操作日志
//! - 实现适当的重试策略

// 启用v3模块作为稳定版本
#[cfg(any(feature = "ccm-sheets", feature = "ccm-sheets-v3"))]
pub mod v3;

#[cfg(any(feature = "ccm-sheets", feature = "ccm-sheets-v3"))]
pub use v3::*;

// V2模块作为实验性功能，单独启用
#[cfg(feature = "ccm-sheets-v2")]
pub mod v2;

#[cfg(feature = "ccm-sheets-v2")]
pub use v2::*;

use openlark_core::config::Config;

/// Sheets电子表格服务
///
/// 基础服务架构，具体功能在后续版本中实现。
#[derive(Clone, Debug)]
pub struct SheetsService {
    config: Config,
}

impl SheetsService {
    /// 创建Sheets服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl openlark_core::trait_system::Service for SheetsService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "SheetsService"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::trait_system::Service;

    #[test]
    fn test_sheets_service_creation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = SheetsService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_service_trait_implementation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = SheetsService::new(config);

        let config_ref = service.config();
        assert_eq!(config_ref.app_id, "test_app_id");
    }
}
