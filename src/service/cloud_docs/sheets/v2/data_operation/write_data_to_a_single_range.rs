use serde::Serialize;

use crate::{
    core::{
        api_req::ApiRequest, api_resp::BaseResponse, constants::AccessTokenType, req_option,
        SDKResult,
    },
    service::sheets::v2::{
        data_operation::{SheetDataUpdates, ValueRangeRequest},
        SpreadsheetService,
    },
};

/// 向单个范围写入数据 请求体
#[derive(Serialize, Debug, Default)]
pub struct WriteDataToSingleRangeRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    #[serde(rename = "valueRange")]
    value_range_request: ValueRangeRequest,
}

impl WriteDataToSingleRangeRequest {
    pub fn builder() -> WriteDataToSingleRangeBuilder {
        WriteDataToSingleRangeBuilder::default()
    }
}

#[derive(Default)]
pub struct WriteDataToSingleRangeBuilder {
    request: WriteDataToSingleRangeRequest,
}

impl WriteDataToSingleRangeBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn range(mut self, range: impl ToString) -> Self {
        self.request.value_range_request.range = range.to_string();
        self
    }

    pub fn values(mut self, values: serde_json::Value) -> Self {
        self.request.value_range_request.values = values;
        self
    }

    pub fn build(mut self) -> WriteDataToSingleRangeRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 写入单个范围响应体
pub type WriteDataToSingleRangeResponse = SheetDataUpdates;

impl SpreadsheetService {
    /// 写入单个范围
    pub async fn write_data_to_single_range(
        &self,
        request: WriteDataToSingleRangeRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<WriteDataToSingleRangeResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path = format!(
            "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values",
            spreadsheet_token = request.spreadsheet_token
        );
        api_req.http_method = reqwest::Method::PUT;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}
