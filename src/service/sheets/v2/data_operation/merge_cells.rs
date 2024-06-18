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

/// 合并单元格请求
#[derive(Serialize, Debug, Default)]
pub struct MergeCellsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 查询范围，包含 sheetId 与单元格范围两部分，目前支持四种索引方式，详见 在线表格开发指南
    range: String,
    /// 可选三个类型，"MERGE_ALL" 将所选区域直接合并、"MERGE_ROWS"
    /// 将所选区域按行合并、"MERGE_COLUMNS" 将所选区域按列合并响应
    #[serde(rename = "mergeType")]
    merge_type: String,
}

impl MergeCellsRequest {
    pub fn builder() -> MergeCellsRequestBuilder {
        MergeCellsRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct MergeCellsRequestBuilder {
    request: MergeCellsRequest,
}

impl MergeCellsRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    /// 	查询范围，包含 sheetId 与单元格范围两部分，目前支持四种索引方式，详见 在线表格开发指南
    pub fn range(mut self, range: impl ToString) -> Self {
        self.request.range = range.to_string();
        self
    }

    /// 可选三个类型，"MERGE_ALL" 将所选区域直接合并、"MERGE_ROWS"
    /// 将所选区域按行合并、"MERGE_COLUMNS" 将所选区域按列合并响应
    pub fn merge_type(mut self, merge_type: impl ToString) -> Self {
        self.request.merge_type = merge_type.to_string();
        self
    }

    pub fn build(mut self) -> MergeCellsRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

#[derive(Deserialize, Debug)]
pub struct MergeCellsResponse {
    /// spreadsheet 的 token
    #[serde(rename = "spreadsheetToken")]
    pub spread_sheet_token: String,
}

impl ApiResponseTrait for MergeCellsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SpreadsheetService {
    /// 合并单元格
    pub async fn merge_cells(
        &self,
        request: MergeCellsRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<MergeCellsResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path = format!(
            "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/merge_cells",
            spreadsheet_token = request.spreadsheet_token
        );
        api_req.http_method = reqwest::Method::POST;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}
