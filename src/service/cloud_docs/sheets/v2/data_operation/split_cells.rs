use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        req_option, SDKResult,
    },
    service::sheets::v2::SpreadsheetService,
};

/// 拆分单元格请求
#[derive(Serialize, Debug, Default)]
pub struct SplitCellsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 查询范围，包含 sheetId 与单元格范围两部分，目前支持四种索引方式，详见 在线表格开发指南
    range: String,
}

impl SplitCellsRequest {
    pub fn builder() -> SplitCellsRequestBuilder {
        SplitCellsRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct SplitCellsRequestBuilder {
    request: SplitCellsRequest,
}

impl SplitCellsRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    /// 查询范围，包含 sheetId 与单元格范围两部分，目前支持四种索引方式，详见 在线表格开发指南
    pub fn range(mut self, range: impl ToString) -> Self {
        self.request.range = range.to_string();
        self
    }

    pub fn build(mut self) -> SplitCellsRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

#[derive(Deserialize, Debug)]
pub struct SplitCellsResponse {
    /// spreadsheet 的 token
    #[serde(rename = "spreadsheetToken")]
    pub spread_sheet_token: String,
}

impl ApiResponseTrait for SplitCellsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SpreadsheetService {
    /// 拆分单元格
    pub async fn split_cells(
        &self,
        request: SplitCellsRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<SplitCellsResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path = format!(
            "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/unmerge_cells",
            spreadsheet_token = request.spreadsheet_token
        );
        api_req.http_method = reqwest::Method::POST;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}
