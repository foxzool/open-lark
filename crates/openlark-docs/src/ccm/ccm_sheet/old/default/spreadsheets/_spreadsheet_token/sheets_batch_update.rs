//! 根据 spreadsheetToken 更新工作表属性。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet/operate-sheets

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SheetsBatchUpdateRequest {
    pub requests: Vec<RequestItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RequestItem {
    pub addSheet: Option<AddSheetProperty>,
    pub updateSheet: Option<UpdateSheetProperty>,
    pub deleteSheet: Option<DeleteSheetProperty>,
    pub copySheet: Option<CopySheetProperty>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AddSheetProperty {
    pub properties: Option<SheetProperty>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateSheetProperty {
    pub properties: Option<SheetProperty>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteSheetProperty {
    pub sheetId: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CopySheetProperty {
    pub sourceSheetId: String,
    pub destinationSheetId: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SheetProperty {
    pub sheetId: Option<String>,
    pub title: Option<String>,
    pub index: Option<i32>,
    pub hidden: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SheetsBatchUpdateResponse {
    pub replies: Vec<ReplyItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ReplyItem {
    pub addSheet: Option<SheetProperty>,
    pub updateSheet: Option<SheetProperty>,
    pub deleteSheet: Option<DeleteSheetResult>,
    pub copySheet: Option<SheetProperty>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteSheetResult {
    pub result: bool,
    pub sheetId: String,
}

impl ApiResponseTrait for SheetsBatchUpdateResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct SheetsBatchUpdateBuilder {
    api_req: ApiRequest<SheetsBatchUpdateRequest>,
}

impl SheetsBatchUpdateBuilder {
    pub fn new(spreadsheet_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_sheets_batch_update".to_string();
        builder.api_req.method = "POST".to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/sheets_batch_update",
            spreadsheet_token.to_string()
        );
        builder.api_req.body = Some(SheetsBatchUpdateRequest::default());
        builder
    }

    pub fn requests(mut self, requests: Vec<RequestItem>) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.requests = requests;
        }
        self
    }

    pub fn build(
        self,
        config: &openlark_core::config::Config,
        option: &RequestOption,
    ) -> Result<RequestBuilder, LarkAPIError> {
        let mut req = self.api_req;
        req.build(AccessTokenType::Tenant, config, option)
    }
}
