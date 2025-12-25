//! 更新下拉列表设置
//!
//! docPath: /document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/datavalidation/update-datavalidation
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/datavalidation/update-datavalidation

use openlark_core::{
    api::ApiRequest,
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::api_utils::*;

/// 更新下拉列表设置（PUT）
///
/// 注意：该接口的请求体字段在不同历史版本中可能存在差异，先使用 JSON 透传。
pub async fn update(
    body: serde_json::Value,
    spreadsheet_token: String,
    sheet_id: String,
    data_validation_id: String,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<serde_json::Value> {
    use crate::common::api_endpoints::CcmSheetApiOld;

    let api_endpoint =
        CcmSheetApiOld::DataValidationUpdate(spreadsheet_token, sheet_id, data_validation_id);
    let mut api_request: ApiRequest<serde_json::Value> =
        ApiRequest::put(&api_endpoint.to_url()).body(body);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "更新下拉列表设置")
}
