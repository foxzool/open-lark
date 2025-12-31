//! 更新行列
//!
//! docPath: /document/ukTMukTMukTM/uYjMzUjL2IzM14iNyMTN

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;

use crate::common::api_endpoints::CcmSheetApiOld;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateDimensionRangeRequest {
    pub dimension: DimensionUpdate,
    pub dimensionProperties: DimensionProperties,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DimensionUpdate {
    pub sheetId: String,
    pub majorDimension: String,
    pub startIndex: i32,
    pub endIndex: i32,
}

/// 行列属性（dimensionProperties）
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DimensionProperties {
    pub visible: Option<bool>,
    pub fixedSize: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateDimensionRangeResponse {}

impl ApiResponseTrait for UpdateDimensionRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新行列
pub async fn dimension_range(
    spreadsheet_token: String,
    request: UpdateDimensionRangeRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<UpdateDimensionRangeResponse> {
    validate_required!(spreadsheet_token, "spreadsheet_token 不能为空");
    validate_required!(request.dimension.sheetId, "sheetId 不能为空");
    validate_required!(request.dimension.majorDimension, "majorDimension 不能为空");
    if request.dimension.majorDimension != "ROWS" && request.dimension.majorDimension != "COLUMNS" {
        return Err(openlark_core::error::validation_error(
            "majorDimension",
            "majorDimension 必须为 ROWS 或 COLUMNS",
        ));
    }
    if request.dimension.startIndex <= 0 {
        return Err(openlark_core::error::validation_error(
            "startIndex",
            "startIndex 必须 >= 1",
        ));
    }
    if request.dimension.endIndex < request.dimension.startIndex {
        return Err(openlark_core::error::validation_error(
            "endIndex",
            "endIndex 必须 >= startIndex",
        ));
    }
    if request.dimension.endIndex - request.dimension.startIndex + 1 > 5000 {
        return Err(openlark_core::error::validation_error(
            "endIndex",
            "单次操作不超过 5000 行或列",
        ));
    }
    if request.dimensionProperties.visible.is_none()
        && request.dimensionProperties.fixedSize.is_none()
    {
        return Err(openlark_core::error::validation_error(
            "dimensionProperties",
            "dimensionProperties 至少填写 visible 或 fixedSize",
        ));
    }
    if let Some(fixed_size) = request.dimensionProperties.fixedSize {
        if fixed_size < 0 {
            return Err(openlark_core::error::validation_error(
                "fixedSize",
                "fixedSize 必须 >= 0",
            ));
        }
    }

    let api_endpoint = CcmSheetApiOld::DimensionRangeUpdate(spreadsheet_token);
    let mut api_request: ApiRequest<UpdateDimensionRangeResponse> =
        ApiRequest::put(&api_endpoint.to_url()).body(serialize_params(&request, "更新行列")?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "更新行列")
}
