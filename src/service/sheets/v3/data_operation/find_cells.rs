use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        req_option, SDKResult,
    },
    service::sheets::v3::SpreadsheetSheetService,
};
use crate::service::sheets::v3::data_operation::{FindCondition, FindReplaceResult};

#[derive(Serialize, Debug, Default)]
pub struct FindCellsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 工作表的id
    #[serde(skip)]
    sheet_id: String,
    /// 查找条件
    find_condition: FindCondition,
    /// 查找的字符串，当search_by_regex字段为 true 时，该字段为正则表达式
    ///
    /// 示例值："如下
    ///
    /// - 普通查找示例: "hello"
    /// - 正则查找示例: "[A-Z]\w+""
    find: String,
}


impl FindCellsRequest {
    pub fn builder() -> FindCellsRequestBuilder {
        FindCellsRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct FindCellsRequestBuilder {
    request: FindCellsRequest,
}

impl FindCellsRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn sheet_id(mut self, sheet_id: impl ToString) -> Self {
        self.request.sheet_id = sheet_id.to_string();
        self
    }

    pub fn find(mut self, find: impl ToString) -> Self {
        self.request.find = find.to_string();
        self
    }

    pub fn range(mut self, range: impl ToString) -> Self {
        self.request.find_condition.range = range.to_string();
        self
    }

    pub fn match_case(mut self, match_case: bool) -> Self {
        self.request.find_condition.match_case = Some(match_case);
        self
    }

    pub fn match_entire_cell(mut self, match_entire_cell: bool) -> Self {
        self.request.find_condition.match_entire_cell = Some(match_entire_cell);
        self
    }

    pub fn search_by_regex(mut self, search_by_regex: bool) -> Self {
        self.request.find_condition.search_by_regex = Some(search_by_regex);
        self
    }

    pub fn include_formulas(mut self, include_formulas: bool) -> Self {
        self.request.find_condition.include_formulas = Some(include_formulas);
        self
    }

    pub fn build(mut self) -> FindCellsRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 查找单元格响应
#[derive(Deserialize, Debug)]
pub struct FindCellsResponse {
    /// 符合条件的信息
    pub find_result: FindReplaceResult,
}

impl ApiResponseTrait for FindCellsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}


impl SpreadsheetSheetService {
    /// 查找单元格
    pub async fn find_cells(
        &self,
        request: FindCellsRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<FindCellsResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/find",
            spreadsheet_token = request.spreadsheet_token,
            sheet_id = request.sheet_id
        );
        api_req.http_method = reqwest::Method::POST;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}
