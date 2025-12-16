//! 将本地表格导入到云空间上
//!
//! doc: https://open.feishu.cn/document/server-docs/historic-version/docs/sheets/sheet-operation/import-spreadsheet

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ImportSheetRequest {
    pub file: Vec<u8>,
    pub name: String,
    pub folder_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ImportSheetResponse {
    pub ticket: String,
}

impl ApiResponseTrait for ImportSheetResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct ImportSheetBuilder {
    api_req: ApiRequest<ImportSheetRequest>,
}

impl ImportSheetBuilder {
    pub fn new() -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_import".to_string();
        builder.api_req.method = "POST".to_string();
        builder.api_req.url = "https://open.feishu.cn/open-apis/sheets/v2/import".to_string();
        builder.api_req.body = Some(ImportSheetRequest::default());
        builder
    }

    pub fn file(mut self, file: Vec<u8>) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.file = file;
        }
        self
    }

    pub fn name(mut self, name: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.name = name.to_string();
        }
        self
    }

    pub fn folder_token(mut self, folder_token: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.folder_token = folder_token.to_string();
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
