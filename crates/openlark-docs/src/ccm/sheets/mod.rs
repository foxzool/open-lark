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
//! ```rust,ignore
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("app_id", "app_secret").with_app_type(AppType::SelfBuild);
//!
//! // v3版本 - 稳定现代化调用方式（推荐）
//! let sheets = client.sheets.v3.sheet.query_sheets(&spreadsheet_token).await?;
//!
//! let charts = client.sheets.v3.charts.create_chart(&chart_request).await?;
//!
//! // v2版本 - 实验性功能
//! #[cfg(feature = "ccm-sheets-v2")]
//! {
//!     let sheet_info = client
//!         .sheets
//!         .v2
//!         .single_range_read
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
// v3 模块显式导出
pub use v3::{
    CellPosition,
    CellReference,
    CreateFilterConditionRequest,
    CreateFilterConditionResponse,
    CreateFilterRequest,
    CreateFilterResponse,
    CreateFilterViewRequest,
    CreateFilterViewResponse,
    CreateFloatImageRequest,
    CreateFloatImageResponse,
    CreateSpreadsheetParams,
    CreateSpreadsheetRequest,
    CreateSpreadsheetResponse,
    CreatedSpreadsheet,
    DeleteFilterConditionResponse,
    DeleteFilterResponse,
    DeleteFilterViewResponse,
    DeleteFloatImageResponse,
    DimensionSource,
    FilterCondition,
    FilterInfo,
    FilterViewCondition,
    FilterViewId,
    FilterViewInfo,
    FindCellsRequest,
    FindCondition,
    FindParams,
    FindReplaceParams,
    FindReplaceResponse,
    FindResponse,
    FindResult,
    FloatImageId,
    FloatImageInfo,
    FloatImageToken,
    GetFilterConditionResponse,
    GetFilterResponse,
    GetFilterViewResponse,
    GetFloatImageResponse,
    GetSheetResponse,
    GetSpreadsheetResponse,
    GridProperties,
    Locale,
    MergeRange,
    MoveDimensionParams,
    MoveDimensionRequest,
    MoveDimensionResponse,
    PagedResponse,
    QueryFilterConditionsResponse,
    QueryFilterViewsResponse,
    QueryFloatImagesResponse,
    QuerySheetResponse,
    Range,
    ReplaceCellsRequest,
    ReplaceResult,
    Sheet,
    SheetId,
    SheetInfo,
    SheetProperty,
    SheetsResponse,
    SpreadsheetInfo,
    SpreadsheetToken,
    TimeZone,
    UpdateFilterConditionRequest,
    UpdateFilterConditionResponse,
    UpdateFilterRequest,
    UpdateFilterResponse,
    UpdateFilterViewRequest,
    UpdateFilterViewResponse,
    UpdateFloatImageRequest,
    UpdateFloatImageResponse,
    UpdateSpreadsheetParams,
    UpdateSpreadsheetRequest,
    UpdateSpreadsheetResponse,
    create_filter,
    create_filter_condition,
    create_filter_condition_with_options,
    create_filter_view,
    create_filter_view_with_options,
    create_filter_with_options,
    create_float_image,
    create_float_image_with_options,
    create_spreadsheet,
    create_spreadsheet_with_options,
    delete_filter,
    delete_filter_condition,
    delete_filter_condition_with_options,
    delete_filter_view,
    delete_filter_view_with_options,
    delete_filter_with_options,
    delete_float_image,
    delete_float_image_with_options,
    find_cells,
    find_cells_with_options,
    get_filter,
    get_filter_condition,
    get_filter_condition_with_options,
    get_filter_view,
    get_filter_view_with_options,
    get_filter_with_options,
    get_float_image,
    get_float_image_with_options,
    get_sheet,
    get_sheet_with_options,
    get_spreadsheet,
    get_spreadsheet_with_options,
    move_dimension,
    move_dimension_with_options,
    query_filter_conditions,
    query_filter_conditions_with_options,
    query_filter_views,
    query_filter_views_with_options,
    query_float_images,
    query_float_images_with_options,
    query_sheets,
    query_sheets_with_options,
    replace_cells,
    replace_cells_with_options,
    update_filter,
    update_filter_condition,
    update_filter_condition_with_options,
    update_filter_view,
    update_filter_view_with_options,
    update_filter_with_options,
    update_float_image,
    update_float_image_with_options,
    update_spreadsheet,
    update_spreadsheet_with_options,
};

// V2模块已弃用，已移除 feature 支持
// 如需使用 v2 功能，请联系维护团队评估迁移需求
//
// 注意：SheetsService 已从 v3 模块重新导出，无需在此重复定义
