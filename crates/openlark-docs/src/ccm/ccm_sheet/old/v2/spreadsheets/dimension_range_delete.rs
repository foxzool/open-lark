//! 删除行列 API
//!
/// 删除行列 - 根据 spreadsheetToken 和维度信息删除行/列
///
/// 文档链接: https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheet-rowcol/-delete-rows-or-columns
///
/// # 参数说明
/// - `spreadsheet_token`: 电子表格令牌
/// - `params`: 删除行列请求参数
///
/// # 返回值
/// 返回删除行列的结果

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};

/// 删除行列请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDimensionRangeParams {
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

/// 删除行列响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDimensionRangeResponse {
    /// 删除结果
    pub data: Option<DimensionDeleteResult>,
}

/// 维度删除结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionDeleteResult {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 删除的范围
    #[serde(rename = "deleted_range")]
    pub deleted_range: String,
    /// 删除的行数
    #[serde(rename = "deleted_rows")]
    pub deleted_rows: i32,
    /// 删除的列数
    #[serde(rename = "deleted_columns")]
    pub deleted_columns: i32,
    /// 删除的单元格数
    #[serde(rename = "deleted_cells")]
    pub deleted_cells: i32,
}

impl ApiResponseTrait for DeleteDimensionRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除行列
///
/// 该接口用于根据 spreadsheetToken 和维度信息删除行/列。单次删除最大5000行/列。
///
/// # 示例
/// ```rust
/// use openlark_docs::ccm::ccm_sheet::old::v2::spreadsheets::dimension_range_delete::*;
///
/// // 删除第3行到第7行（共5行）
/// let params = DeleteDimensionRangeParams {
///     major_dimension: "ROWS".to_string(),
///     start_index: 2,
///     end_index: 7,
/// };
///
/// let result = delete_dimension_range(&config, "spreadsheet_token", params).await?;
/// ```
pub async fn delete_dimension_range(
    config: &Config,
    spreadsheet_token: &str,
    params: DeleteDimensionRangeParams,
) -> SDKResult<DeleteDimensionRangeResponse> {
    // 验证必填字段
    validate_required_field("表格Token", Some(spreadsheet_token), "表格Token不能为空")?;
    validate_required_field("主要维度", Some(&params.major_dimension), "主要维度不能为空")?;

    if params.start_index < 0 || params.end_index <= params.start_index {
        return Err(openlark_core::error::CoreError::validation(
            "indices",
            "起始索引必须小于结束索引且大于等于0"
        ));
    }

    let length = params.end_index - params.start_index;
    if length > 5000 {
        return Err(openlark_core::error::CoreError::validation(
            "length",
            "删除长度不能超过5000"
        ));
    }

    if !["ROWS", "COLUMNS"].contains(&params.major_dimension.as_str()) {
        return Err(openlark_core::error::CoreError::validation(
            "major_dimension",
            "主要维度必须是 ROWS 或 COLUMNS"
        ));
    }

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::DimensionRangeDelete(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<DeleteDimensionRangeResponse> =
        ApiRequest::delete(&api_endpoint.to_url())
            .body(serialize_params(&params, "删除行列")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "删除行列")
}