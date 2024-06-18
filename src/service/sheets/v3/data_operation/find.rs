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

#[derive(Serialize, Debug, Default)]
struct FindCondition {
    /// 查找范围，参考 名词解释 Range
    range: String,
    /// 是否忽略大小写，默认为 false
    ///
    /// - true：表示忽略字符串中字母大小写差异
    /// - false：表示区分字符串中字母大小写
    match_case: Option<bool>,
    /// 是否完全匹配整个单元格，默认值为 false
    ///
    /// - true：表示完全匹配单元格，比如 find 取值为 "hello"，则单元格中的内容必须为 "hello"
    /// - false：表示允许部分匹配单元格，比如 find 取值为 "hello"，则单元格中的内容包含 "hello"
    ///   即可
    match_entire_cell: Option<bool>,
    /// 是否为正则匹配，默认值为 false
    ///
    /// - true：表示使用正则匹配
    /// - false：表示不使用正则匹配
    search_by_regex: Option<bool>,
    ///
    // 是否仅搜索单元格公式，默认值为 false
    /// - true：表示仅搜索单元格公式
    /// - false：表示仅搜索单元格内容
    include_formulas: Option<bool>,
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

/// 符合条件的信息
#[derive(Deserialize, Debug)]
pub struct FindReplaceResult {
    /// 符合查找条件的单元格数组，不包含公式
    pub matched_cells: Vec<String>,
    /// 符合查找条件的含有公式的单元格数组
    pub matched_formula_cells: Vec<String>,
    /// 符合查找条件的总行数
    pub rows_count: i32,
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
