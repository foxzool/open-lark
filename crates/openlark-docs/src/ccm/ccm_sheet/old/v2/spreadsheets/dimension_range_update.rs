//! 更新行列 API
//!
/// 更新行列 - 根据 spreadsheetToken 和维度信息更新隐藏行列、单元格大小
///
/// 文档链接: https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheet-rowcol/update-rows-or-columns
///
/// # 参数说明
/// - `spreadsheet_token`: 电子表格令牌
/// - `params`: 更新行列请求参数
///
/// # 返回值
/// 返回更新行列的结果

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};

/// 更新行列请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDimensionRangeParams {
    /// 主要维度：ROWS(行) 或 COLUMNS(列)
    #[serde(rename = "major_dimension")]
    pub major_dimension: String,
    /// 起始索引
    #[serde(rename = "start_index")]
    pub start_index: i32,
    /// 结束索引
    #[serde(rename = "end_index")]
    pub end_index: i32,
    /// 行列属性
    pub properties: DimensionProperties,
}

/// 行列属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionProperties {
    /// 是否隐藏
    #[serde(rename = "hidden_by_user", skip_serializing_if = "Option::is_none")]
    pub hidden_by_user: Option<bool>,
    /// 像素大小
    #[serde(rename = "pixel_size", skip_serializing_if = "Option::is_none")]
    pub pixel_size: Option<i32>,
}

/// 更新行列响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDimensionRangeResponse {
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

impl ApiResponseTrait for UpdateDimensionRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新行列
///
/// 根据 spreadsheetToken 和维度信息更新隐藏行列、单元格大小；单次操作不超过5000行或列。
///
/// # 示例
/// ```rust
/// use openlark_docs::ccm::ccm_sheet::old::v2::spreadsheets::dimension_range_update::*;
///
/// // 隐藏第3行到第7行
/// let params = UpdateDimensionRangeParams {
///     major_dimension: "ROWS".to_string(),
///     start_index: 2,
///     end_index: 7,
///     properties: DimensionProperties {
///         hidden_by_user: Some(true),
///         pixel_size: None,
///     },
/// };
///
/// let result = update_dimension_range(&config, "spreadsheet_token", params).await?;
/// ```
pub async fn update_dimension_range(
    config: &Config,
    spreadsheet_token: &str,
    params: UpdateDimensionRangeParams,
) -> SDKResult<UpdateDimensionRangeResponse> {
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
            "更新长度不能超过5000"
        ));
    }

    if !["ROWS", "COLUMNS"].contains(&params.major_dimension.as_str()) {
        return Err(openlark_core::error::CoreError::validation(
            "major_dimension",
            "主要维度必须是 ROWS 或 COLUMNS"
        ));
    }

    // 验证属性配置
    if params.properties.hidden_by_user.is_none() && params.properties.pixel_size.is_none() {
        return Err(openlark_core::error::CoreError::validation(
            "properties",
            "必须指定至少一个属性：hidden_by_user 或 pixel_size"
        ));
    }

    // 验证像素大小
    if let Some(pixel_size) = params.properties.pixel_size {
        if pixel_size <= 0 || pixel_size > 4096 {
            return Err(openlark_core::error::CoreError::validation(
                "pixel_size",
                "像素大小必须在1-4096之间"
            ));
        }
    }

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::DimensionRangeUpdate(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<UpdateDimensionRangeResponse> =
        ApiRequest::put(&api_endpoint.to_url())
            .body(serialize_params(&params, "更新行列")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "更新行列")
}