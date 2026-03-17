/// Sheets V2 API 模块（旧版）
///
/// 电子表格操作API实现，包含全面的表格功能：
///
/// ## 数据读写API (8个)
pub mod models;
pub mod v2;

// 使用通配符导出所有子模块
// models 模块显式导出
pub use models::{
    AppendDataRequest, AppendDataResponse, BatchUpdateRequest, BatchUpdateSheetRequest,
    BatchUpdateSheetResponse, CellStyle, Color, DeleteDimensionRequest, DeleteDimensionResponse,
    GetSheetMetaRequest, GetSheetMetaResponse, GridProperties, InsertDimensionRequest,
    InsertDimensionResponse, ReadMultipleRangesRequest, ReadMultipleRangesResponse,
    ReadSingleRangeRequest, ReadSingleRangeResponse, SetCellStyleRequest, SetCellStyleResponse,
    SheetInfo, SheetProperties, UpdateInfo, ValueRange, WriteData, WriteMultipleRangesRequest,
    WriteMultipleRangesResponse, WriteSingleRangeRequest, WriteSingleRangeResponse,
};

// 重新导出 v2 模块的所有内容
pub use v2::*;
