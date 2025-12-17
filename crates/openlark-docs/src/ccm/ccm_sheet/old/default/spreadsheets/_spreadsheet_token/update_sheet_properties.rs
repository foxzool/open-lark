//! 更新工作表属性
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet/update-sheet-properties

use openlark_core::{
    api::{ApiRequest, Response},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::api_endpoints::CcmSheetApiOld;

use super::sheets_batch_update::{SheetsBatchUpdateReq, SheetsBatchUpdateResponse};

/// 更新工作表属性
pub async fn update_sheet_properties(
    spreadsheet_token: String,
    request: SheetsBatchUpdateReq,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<SheetsBatchUpdateResponse>> {
    let api_endpoint = CcmSheetApiOld::UpdateSheetProperties(spreadsheet_token);
    let mut api_request: ApiRequest<SheetsBatchUpdateResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serde_json::to_value(request)?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

