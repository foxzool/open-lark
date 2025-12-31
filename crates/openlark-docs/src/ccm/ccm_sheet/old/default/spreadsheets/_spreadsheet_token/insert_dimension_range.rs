//! 插入行列
//!
//! docPath: /document/ukTMukTMukTM/uQjMzUjL0IzM14CNyMTN

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;

use crate::common::api_endpoints::CcmSheetApiOld;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct InsertDimensionRangeRequest {
    pub dimension: DimensionInsert,
    pub inheritStyle: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DimensionInsert {
    pub sheetId: String,
    pub majorDimension: String,
    pub startIndex: i32,
    pub endIndex: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct InsertDimensionRangeResponse {}

impl ApiResponseTrait for InsertDimensionRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 插入行列
pub async fn insert_dimension_range(
    spreadsheet_token: String,
    request: InsertDimensionRangeRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<InsertDimensionRangeResponse> {
    validate_required!(spreadsheet_token, "spreadsheet_token 不能为空");
    validate_required!(request.dimension.sheetId, "sheetId 不能为空");
    validate_required!(request.dimension.majorDimension, "majorDimension 不能为空");
    if request.dimension.majorDimension != "ROWS" && request.dimension.majorDimension != "COLUMNS" {
        return Err(openlark_core::error::validation_error(
            "majorDimension",
            "majorDimension 必须为 ROWS 或 COLUMNS",
        ));
    }
    if request.dimension.startIndex < 0 {
        return Err(openlark_core::error::validation_error(
            "startIndex",
            "startIndex 必须 >= 0",
        ));
    }
    if request.dimension.endIndex <= request.dimension.startIndex {
        return Err(openlark_core::error::validation_error(
            "endIndex",
            "endIndex 必须 > startIndex",
        ));
    }
    if request.dimension.endIndex - request.dimension.startIndex > 5000 {
        return Err(openlark_core::error::validation_error(
            "endIndex",
            "单次操作不超过 5000 行或列",
        ));
    }
    if let Some(inherit_style) = request.inheritStyle.as_deref() {
        if !inherit_style.is_empty()
            && inherit_style != "BEFORE"
            && inherit_style != "AFTER"
        {
            return Err(openlark_core::error::validation_error(
                "inheritStyle",
                "inheritStyle 仅支持 BEFORE 或 AFTER（或不传/传空表示不继承）",
            ));
        }
    }

    let api_endpoint = CcmSheetApiOld::InsertDimensionRange(spreadsheet_token);
    let mut api_request: ApiRequest<InsertDimensionRangeResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&request, "插入行列")?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "插入行列")
}
