//! 设置单元格样式
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/set-cell-style

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
pub struct SetStyleRequest {
    pub appendStyle: AppendStyle,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AppendStyle {
    /// 设置样式的范围（格式：`<sheetId>!<开始位置>:<结束位置>`）
    pub range: String,
    /// 需要更新的样式
    pub style: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SetStyleResponse {
    pub revision: i32,
    pub spreadsheetToken: String,
    pub updatedRange: String,
}

impl ApiResponseTrait for SetStyleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 设置单元格样式
pub async fn style(
    spreadsheet_token: String,
    request: SetStyleRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<SetStyleResponse> {
    validate_required!(spreadsheet_token, "spreadsheet_token 不能为空");
    validate_required!(request.appendStyle.range, "range 不能为空");
    if request.appendStyle.style.is_null() {
        return Err(openlark_core::error::validation_error(
            "style",
            "style 不能为空",
        ));
    }

    let api_endpoint = CcmSheetApiOld::Style(spreadsheet_token);
    let mut api_request: ApiRequest<SetStyleResponse> =
        ApiRequest::put(&api_endpoint.to_url()).body(serialize_params(&request, "设置单元格样式")?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "设置单元格样式")
}
