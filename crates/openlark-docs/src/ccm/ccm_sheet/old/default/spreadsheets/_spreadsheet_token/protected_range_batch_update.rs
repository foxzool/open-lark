//! 该接口用于根据 spreadsheetToken 和 protectionId 更新保护范围的保护信息。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/protect-range/udpate-protection-scopes

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchUpdateProtectedRangeRequest {
    pub requests: Vec<UpdateProtectedRangeRequest>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateProtectedRangeRequest {
    pub protectId: String,
    pub dimension: Option<Dimension>,
    pub editors: Option<Vec<i64>>,
    pub lockInfo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Dimension {
    pub sheetId: String,
    pub majorDimension: String,
    pub startIndex: i32,
    pub endIndex: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchUpdateProtectedRangeResponse {
    pub replies: Vec<ProtectedRangeReply>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProtectedRangeReply {
    pub protectId: String,
    pub dimension: Option<Dimension>,
    pub editors: Option<Vec<i64>>,
    pub lockInfo: Option<String>,
}

impl ApiResponseTrait for BatchUpdateProtectedRangeResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct BatchUpdateProtectedRangeBuilder {
    api_req: ApiRequest<BatchUpdateProtectedRangeRequest>,
}

impl BatchUpdateProtectedRangeBuilder {
    pub fn new(spreadsheet_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_protected_range_batch_update".to_string();
        builder.api_req.method = "POST".to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/protected_range_batch_update",
            spreadsheet_token.to_string()
        );
        builder.api_req.body = Some(BatchUpdateProtectedRangeRequest::default());
        builder
    }

    pub fn requests(mut self, requests: Vec<UpdateProtectedRangeRequest>) -> Self {
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
