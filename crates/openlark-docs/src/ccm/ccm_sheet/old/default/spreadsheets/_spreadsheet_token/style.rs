//! 根据 spreadsheetToken 、range 和样式信息更新单元格样式；单次写入不超过5000行，100列。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/set-cell-style

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SetStyleRequest {
    pub appendStyle: Option<serde_json::Value>,
    pub range: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SetStyleResponse {
    pub spreadsheetToken: String,
    pub updatedRange: String,
}

impl ApiResponseTrait for SetStyleResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct SetStyleBuilder {
    api_req: ApiRequest<SetStyleRequest>,
}

impl SetStyleBuilder {
    pub fn new(spreadsheet_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_style_set".to_string();
        builder.api_req.method = "PUT".to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/style",
            spreadsheet_token.to_string()
        );
        builder.api_req.body = Some(SetStyleRequest::default());
        builder
    }

    pub fn append_style(mut self, style: serde_json::Value) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.appendStyle = Some(style);
        }
        self
    }

    pub fn range(mut self, range: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.range = range.to_string();
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
