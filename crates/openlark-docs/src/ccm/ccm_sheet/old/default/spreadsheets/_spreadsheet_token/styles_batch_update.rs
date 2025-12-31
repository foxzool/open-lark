//! 批量设置单元格样式
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/batch-set-cell-style

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
pub struct BatchSetStyleRequest {
    pub data: Vec<BatchStyleData>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchStyleData {
    pub ranges: Vec<String>,
    pub style: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchSetStyleResponse {
    pub revision: i32,
    pub spreadsheetToken: String,
    pub totalUpdatedRows: i32,
    pub totalUpdatedColumns: i32,
    pub totalUpdatedCells: i32,
}

impl ApiResponseTrait for BatchSetStyleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量设置单元格样式
pub async fn styles_batch_update(
    spreadsheet_token: String,
    request: BatchSetStyleRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<BatchSetStyleResponse> {
    validate_required!(spreadsheet_token, "spreadsheet_token 不能为空");
    validate_required!(request.data, "data 不能为空");
    for (idx, item) in request.data.iter().enumerate() {
        if item.ranges.is_empty() {
            return Err(openlark_core::error::validation_error(
                &format!("data[{}].ranges", idx),
                "ranges 不能为空",
            ));
        }
        for (ridx, r) in item.ranges.iter().enumerate() {
            if r.is_empty() {
                return Err(openlark_core::error::validation_error(
                    &format!("data[{}].ranges[{}]", idx, ridx),
                    "range 不能为空",
                ));
            }
        }
        if item.style.is_null() {
            return Err(openlark_core::error::validation_error(
                &format!("data[{}].style", idx),
                "style 不能为空",
            ));
        }
    }

    let api_endpoint = CcmSheetApiOld::StylesBatchUpdate(spreadsheet_token);
    let mut api_request: ApiRequest<BatchSetStyleResponse> =
        ApiRequest::put(&api_endpoint.to_url())
            .body(serialize_params(&request, "批量设置单元格样式")?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "批量设置单元格样式")
}
