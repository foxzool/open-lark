//! 批量更新数据
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/write-data-to-multiple-ranges

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
pub struct BatchUpdateValuesRequest {
    pub valueRanges: Vec<ValueRange>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ValueRange {
    pub range: String,
    pub values: Vec<Vec<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchUpdateValuesResponse {
    #[serde(default)]
    pub responses: Vec<UpdateResponse>,
    pub revision: i32,
    pub spreadsheetToken: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateResponse {
    pub spreadsheetToken: String,
    pub updatedRange: String,
    pub updatedRows: i32,
    pub updatedColumns: i32,
    pub updatedCells: i32,
}

impl ApiResponseTrait for BatchUpdateValuesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量更新数据
pub async fn values_batch_update(
    spreadsheet_token: String,
    request: BatchUpdateValuesRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<BatchUpdateValuesResponse> {
    validate_required!(spreadsheet_token, "spreadsheet_token 不能为空");
    validate_required!(request.valueRanges, "valueRanges 不能为空");
    for (idx, vr) in request.valueRanges.iter().enumerate() {
        if vr.range.is_empty() {
            return Err(openlark_core::error::validation_error(
                &format!("valueRanges[{}].range", idx),
                "range 不能为空",
            ));
        }
        if vr.values.is_empty() {
            return Err(openlark_core::error::validation_error(
                &format!("valueRanges[{}].values", idx),
                "values 不能为空",
            ));
        }
        // 文档限制：单次写入不超过 5000 行、100 列；每个格子不超过 5 万字符
        if vr.values.len() > 5000 {
            return Err(openlark_core::error::validation_error(
                &format!("valueRanges[{}].values", idx),
                "单次写入不超过 5000 行",
            ));
        }
        let mut max_cols = 0usize;
        for (r_idx, row) in vr.values.iter().enumerate() {
            max_cols = max_cols.max(row.len());
            if row.len() > 100 {
                return Err(openlark_core::error::validation_error(
                    &format!("valueRanges[{}].values[{}]", idx, r_idx),
                    "单次写入不超过 100 列",
                ));
            }
            for (c_idx, cell) in row.iter().enumerate() {
                if let serde_json::Value::String(s) = cell {
                    if s.chars().count() > 50_000 {
                        return Err(openlark_core::error::validation_error(
                            &format!("valueRanges[{}].values[{}][{}]", idx, r_idx, c_idx),
                            "单个格子不超过 50000 字符",
                        ));
                    }
                }
            }
        }
        if max_cols == 0 {
            return Err(openlark_core::error::validation_error(
                &format!("valueRanges[{}].values", idx),
                "values 不能为空",
            ));
        }
    }

    let api_endpoint = CcmSheetApiOld::ValuesBatchUpdate(spreadsheet_token);
    let mut api_request: ApiRequest<BatchUpdateValuesResponse> =
        ApiRequest::post(&api_endpoint.to_url())
            .body(serialize_params(&request, "向多个范围写入数据")?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "向多个范围写入数据")
}
