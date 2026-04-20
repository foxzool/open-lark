/// 创建电子表格接口。
pub mod create;
/// 获取电子表格接口。
pub mod get;
/// 电子表格通用模型模块。
pub mod models;
/// 更新电子表格接口。
pub mod patch;
/// 工作表子资源模块。
pub mod sheet;

/// 重新导出电子表格模型与请求类型。
pub use models::{
    CreateSpreadsheetParams, CreateSpreadsheetResponse, CreatedSpreadsheet, DimensionSource,
    FindCondition, FindParams, FindReplaceParams, FindReplaceResponse, FindResponse, FindResult,
    GetSheetResponse, GetSpreadsheetResponse, GridProperties, MergeRange, MoveDimensionParams,
    MoveDimensionResponse, QuerySheetResponse, Sheet, SpreadsheetInfo, UpdateSpreadsheetParams,
    UpdateSpreadsheetResponse,
};

/// 重新导出工作表子资源类型。
pub use sheet::{
    CreateFilterConditionRequest, CreateFilterConditionResponse, CreateFilterRequest,
    CreateFilterResponse, CreateFilterViewRequest, CreateFilterViewResponse,
    CreateFloatImageRequest, CreateFloatImageResponse, DeleteFilterConditionResponse,
    DeleteFilterResponse, DeleteFilterViewResponse, DeleteFloatImageResponse,
    GetFilterConditionResponse, GetFilterResponse, GetFilterViewResponse, GetFloatImageResponse,
    QueryFilterConditionsResponse, QueryFilterViewsResponse, QueryFloatImagesResponse,
    UpdateFilterConditionRequest, UpdateFilterConditionResponse, UpdateFilterRequest,
    UpdateFilterResponse, UpdateFilterViewRequest, UpdateFilterViewResponse,
    UpdateFloatImageRequest, UpdateFloatImageResponse,
};
