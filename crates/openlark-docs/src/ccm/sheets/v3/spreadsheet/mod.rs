/// 电子表格管理模块
pub mod create;
pub mod get;
pub mod models;
pub mod patch;
pub mod sheet;

// 使用通配符重新导出所有子模块,避免手动维护大量重复的导出列表
// create 模块显式导出
pub use create::{};
// get 模块显式导出
pub use get::{};
// models 模块显式导出
pub use models::{
    CreateSpreadsheetParams,
    CreateSpreadsheetResponse,
    CreatedSpreadsheet,
    DimensionSource,
    FindCondition,
    FindParams,
    FindReplaceParams,
    FindReplaceResponse,
    FindResponse,
    FindResult,
    GetSheetResponse,
    GetSpreadsheetResponse,
    GridProperties,
    MergeRange,
    MoveDimensionParams,
    MoveDimensionResponse,
    QuerySheetResponse,
    Sheet,
    SpreadsheetInfo,
    UpdateSpreadsheetParams,
    UpdateSpreadsheetResponse,
};
// patch 模块显式导出
pub use patch::{};
// sheet 模块显式导出
pub use sheet::{
    CreateFilterConditionRequest,
    CreateFilterConditionResponse,
    CreateFilterRequest,
    CreateFilterResponse,
    CreateFilterViewRequest,
    CreateFilterViewResponse,
    CreateFloatImageRequest,
    CreateFloatImageResponse,
    DeleteFilterConditionResponse,
    DeleteFilterResponse,
    DeleteFilterViewResponse,
    DeleteFloatImageResponse,
    GetFilterConditionResponse,
    GetFilterResponse,
    GetFilterViewResponse,
    GetFloatImageResponse,
    QueryFilterConditionsResponse,
    QueryFilterViewsResponse,
    QueryFloatImagesResponse,
    UpdateFilterConditionRequest,
    UpdateFilterConditionResponse,
    UpdateFilterRequest,
    UpdateFilterResponse,
    UpdateFilterViewRequest,
    UpdateFilterViewResponse,
    UpdateFloatImageRequest,
    UpdateFloatImageResponse,
};
