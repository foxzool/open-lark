//! 插入数据
//!
//! docPath: /document/ukTMukTMukTM/uIjMzUjLyIzM14iMyMTN

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
pub struct ValuesPrependRequest {
    pub valueRange: ValueRange,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ValueRange {
    pub range: String,
    pub values: Vec<Vec<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ValuesPrependResponse {
    pub spreadsheetToken: String,
    pub tableRange: String,
    pub revision: i32,
    pub updates: UpdateResult,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateResult {
    pub spreadsheetToken: String,
    pub updatedRange: String,
    pub updatedRows: i32,
    pub updatedColumns: i32,
    pub updatedCells: i32,
    pub revision: i32,
}

impl ApiResponseTrait for ValuesPrependResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 插入数据
pub async fn values_prepend(
    spreadsheet_token: String,
    request: ValuesPrependRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<ValuesPrependResponse> {
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

    let api_endpoint = CcmSheetApiOld::ValuesPrepend(spreadsheet_token);
    let mut api_request: ApiRequest<ValuesPrependResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&request, "插入数据")?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "插入数据")
}
