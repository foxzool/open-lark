//! 向单个范围写入数据
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/write-data-to-a-single-range

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;
use crate::common::api_utils::*;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateValuesRequest {
    pub valueRange: ValueRange,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ValueRange {
    pub range: String,
    pub values: Vec<Vec<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateValuesResponse {
    pub spreadsheetToken: String,
    pub updatedRange: String,
    pub updatedRows: i32,
    pub updatedColumns: i32,
    pub updatedCells: i32,
    pub revision: i32,
}

impl ApiResponseTrait for UpdateValuesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 向单个范围写入数据
pub async fn values(
    spreadsheet_token: String,
    request: UpdateValuesRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<UpdateValuesResponse> {
    validate_required!(spreadsheet_token, "spreadsheet_token 不能为空");
    validate_required!(request.valueRange.range, "range 不能为空");
    validate_required!(request.valueRange.values, "values 不能为空");
    if request.valueRange.values.len() > 5000 {
        return Err(openlark_core::error::validation_error(
            "values",
            "单次写入不超过 5000 行",
        ));
    }
    for (r_idx, row) in request.valueRange.values.iter().enumerate() {
        if row.len() > 100 {
            return Err(openlark_core::error::validation_error(
                &format!("values[{}]", r_idx),
                "单次写入不超过 100 列",
            ));
        }
        for (c_idx, cell) in row.iter().enumerate() {
            if let serde_json::Value::String(s) = cell {
                if s.chars().count() > 50_000 {
                    return Err(openlark_core::error::validation_error(
                        &format!("values[{}][{}]", r_idx, c_idx),
                        "单个格子不超过 50000 字符",
                    ));
                }
            }
        }
    }

    let api_endpoint = CcmSheetApiOld::Values(spreadsheet_token);
    let mut api_request: ApiRequest<UpdateValuesResponse> = ApiRequest::put(&api_endpoint.to_url())
        .body(serialize_params(&request, "向单个范围写入数据")?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "向单个范围写入数据")
}

pub mod _range;
pub use _range::*;
