/// Sheets V2 通用模型模块。
pub mod models;
/// Sheets V2 接口模块。
pub mod v2;

/// 重新导出 Sheets V2 通用模型。
pub use models::{
    AppendDataRequest, AppendDataResponse, BatchUpdateRequest, BatchUpdateSheetRequest,
    BatchUpdateSheetResponse, CellStyle, Color, DeleteDimensionRequest, DeleteDimensionResponse,
    GetSheetMetaRequest, GetSheetMetaResponse, GridProperties, InsertDimensionRequest,
    InsertDimensionResponse, ReadMultipleRangesRequest, ReadMultipleRangesResponse,
    ReadSingleRangeRequest, ReadSingleRangeResponse, SetCellStyleRequest, SetCellStyleResponse,
    SheetInfo, SheetProperties, UpdateInfo, ValueRange, WriteData, WriteMultipleRangesRequest,
    WriteMultipleRangesResponse, WriteSingleRangeRequest, WriteSingleRangeResponse,
};

/// 重新导出 Sheets V2 接口类型。
pub use v2::*;
