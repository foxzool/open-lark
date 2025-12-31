//! 删除 Sheet
//!
//! docPath: /document/ukTMukTMukTM/uUTNzUjL1UzM14SN1MTN/delete-sheet
//! doc: https://open.feishu.cn/document/ukTMukTMukTM/uUTNzUjL1UzM14SN1MTN/delete-sheet

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::CcmDriveExplorerApiOld, api_utils::*};

/// 删除 Sheet 响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSpreadsheetResp {
    /// 被删除的对象 id
    pub id: String,
    /// 是否成功
    pub result: bool,
}

impl ApiResponseTrait for DeleteSpreadsheetResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除 Sheet 请求
pub struct DeleteSpreadsheetRequest {
    config: Config,
    spreadsheet_token: String,
}

impl DeleteSpreadsheetRequest {
    pub fn new(config: Config, spreadsheet_token: impl Into<String>) -> Self {
        Self {
            config,
            spreadsheet_token: spreadsheet_token.into(),
        }
    }

    pub async fn send(self) -> SDKResult<DeleteSpreadsheetResp> {
        validate_required!(self.spreadsheet_token, "spreadsheetToken 不能为空");

        let api_request: ApiRequest<DeleteSpreadsheetResp> = ApiRequest::delete(
            &CcmDriveExplorerApiOld::FileSpreadsheets(self.spreadsheet_token).to_url(),
        );

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "删除Sheet")
    }
}
