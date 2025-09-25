use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        req_option,
        standard_response::StandardResponse,
        SDKResult,
    },
    impl_executable_builder_owned,
    service::sheets::v3::{
        data_operation::{FindCondition, FindReplaceResult},
        SpreadsheetSheetService,
    },
};

#[derive(Serialize, Debug, Default)]
pub struct ReplaceCellsRequest {
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
    /// 替换的字符串
    replacement: String,
}

impl ReplaceCellsRequest {
    pub fn builder() -> ReplaceCellsRequestBuilder {
        ReplaceCellsRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct ReplaceCellsRequestBuilder {
    request: ReplaceCellsRequest,
}

impl ReplaceCellsRequestBuilder {
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

    pub fn replacement(mut self, replacement: impl ToString) -> Self {
        self.request.replacement = replacement.to_string();
        self
    }

    pub fn build(mut self) -> ReplaceCellsRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

// Trait implementation
impl_executable_builder_owned!(
    ReplaceCellsRequestBuilder,
    SpreadsheetSheetService,
    ReplaceCellsRequest,
    ReplaceCellsResponse,
    replace_cells
);

/// 替换单元格响应
#[derive(Deserialize, Debug)]
pub struct ReplaceCellsResponse {
    /// 符合查找条件并替换的单元格信息
    pub replace_result: FindReplaceResult,
}

impl ApiResponseTrait for ReplaceCellsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SpreadsheetSheetService {
    /// 替换单元格
    ///
    /// 按照指定的条件查找子表的某个范围内的数据符合条件的单元格并替换值，返回替换成功的单元格位置。
    /// 一次请求最多允许替换5000个单元格，如果超过请将range缩小范围再操作。请求体中的
    /// range、find、replacement 字段必填。
    pub async fn replace_cells(
        &self,
        request: ReplaceCellsRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<ReplaceCellsResponse> {
        let mut api_req = request.api_request;
        api_req.api_path = SHEETS_V3_SPREADSHEET_SHEET_REPLACE
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id);
        api_req.http_method = reqwest::Method::POST;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp: BaseResponse<ReplaceCellsResponse> =
            crate::core::http::Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }
}
