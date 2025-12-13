/// 增加行列 API
///
/// 增加行列 - 根据 spreadsheetToken 和长度，在末尾增加空行/列
///
/// 文档链接: https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheet-rowcol/add-rows-or-columns
///
/// # 参数说明
/// - `spreadsheet_token`: 电子表格令牌
/// - `params`: 增加行列请求参数
///
/// # 返回值
/// 返回增加行列的结果
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};

/// 增加行列请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddDimensionRangeParams {
    /// 主要维度：ROWS(行) 或 COLUMNS(列)
    #[serde(rename = "major_dimension")]
    pub major_dimension: String,
    /// 增加的长度
    pub length: i32,
}

/// 增加行列响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddDimensionRangeResponse {
    /// 更新结果
    pub data: Option<DimensionUpdateResult>,
}

/// 维度更新结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionUpdateResult {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 更新的范围
    #[serde(rename = "updated_range")]
    pub updated_range: String,
    /// 更新的行数
    #[serde(rename = "updated_rows")]
    pub updated_rows: i32,
    /// 更新的列数
    #[serde(rename = "updated_columns")]
    pub updated_columns: i32,
    /// 更新的单元格数
    #[serde(rename = "updated_cells")]
    pub updated_cells: i32,
}

impl ApiResponseTrait for AddDimensionRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 增加行列
///
/// 根据 spreadsheetToken 和长度，在末尾增加空行/列；单次操作不超过5000行或列。
///
/// # 示例
/// ```rust
/// use openlark_docs::ccm::ccm_sheet::old::v2::spreadsheets::dimension_range::*;
///
/// // 增加10行
/// let params = AddDimensionRangeParams {
///     major_dimension: "ROWS".to_string(),
///     length: 10,
/// };
///
/// let result = add_dimension_range(&config, "spreadsheet_token", params).await?;
/// ```
pub async fn add_dimension_range(
    config: &Config,
    spreadsheet_token: &str,
    params: AddDimensionRangeParams,
) -> SDKResult<AddDimensionRangeResponse> {
    // 验证必填字段
    validate_required_field("表格Token", Some(spreadsheet_token), "表格Token不能为空")?;
    validate_required_field(
        "主要维度",
        Some(&params.major_dimension),
        "主要维度不能为空",
    )?;

    if params.length <= 0 || params.length > 5000 {
        return Err(openlark_core::error::CoreError::validation(
            "length",
            "增加长度必须在1-5000之间",
        ));
    }

    if !["ROWS", "COLUMNS"].contains(&params.major_dimension.as_str()) {
        return Err(openlark_core::error::CoreError::validation(
            "major_dimension",
            "主要维度必须是 ROWS 或 COLUMNS",
        ));
    }

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::DimensionRange(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<AddDimensionRangeResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "增加行列")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "增加行列")
}

/// 插入行列
///
/// 根据 spreadsheetToken 和维度信息 插入空行/列。
/// 如 startIndex=3， endIndex=7，则从第 4 行开始开始插入行列，一直到第 7 行，共插入 4 行；
/// 单次操作不超过5000行或列。
///
/// # 示例
/// ```rust
/// use openlark_docs::ccm::ccm_sheet::old::v2::spreadsheets::dimension_range::*;
///
/// // 在第4行前插入5行
/// let params = InsertDimensionRangeParams {
///     major_dimension: "ROWS".to_string(),
///     start_index: 3,
///     end_index: 8,
/// };
///
/// let result = insert_dimension_range(&config, "spreadsheet_token", params).await?;
/// ```
pub async fn insert_dimension_range(
    config: &Config,
    spreadsheet_token: &str,
    params: InsertDimensionRangeParams,
) -> SDKResult<AddDimensionRangeResponse> {
    // 验证必填字段
    validate_required_field("表格Token", Some(spreadsheet_token), "表格Token不能为空")?;
    validate_required_field(
        "主要维度",
        Some(&params.major_dimension),
        "主要维度不能为空",
    )?;

    if params.start_index < 0 || params.end_index <= params.start_index {
        return Err(openlark_core::error::CoreError::validation(
            "indices",
            "起始索引必须小于结束索引且大于等于0",
        ));
    }

    let length = params.end_index - params.start_index;
    if length > 5000 {
        return Err(openlark_core::error::CoreError::validation(
            "length",
            "插入长度不能超过5000",
        ));
    }

    if !["ROWS", "COLUMNS"].contains(&params.major_dimension.as_str()) {
        return Err(openlark_core::error::CoreError::validation(
            "major_dimension",
            "主要维度必须是 ROWS 或 COLUMNS",
        ));
    }

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::InsertDimensionRange(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<AddDimensionRangeResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "插入行列")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "插入行列")
}

/// 插入行列请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertDimensionRangeParams {
    /// 主要维度：ROWS(行) 或 COLUMNS(列)
    #[serde(rename = "major_dimension")]
    pub major_dimension: String,
    /// 起始索引
    #[serde(rename = "start_index")]
    pub start_index: i32,
    /// 结束索引
    #[serde(rename = "end_index")]
    pub end_index: i32,
}
