/// 查找替换
///
/// 在电子表格中查找并替换指定内容，支持多种查找选项。
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/findReplace
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::common::{api_endpoints::CcmSheetApiV3, api_utils::*};

/// 查找替换请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindReplaceRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表ID
    pub sheet_id: String,
    /// 查找内容
    pub find: String,
    /// 替换内容
    pub replacement: String,
    /// 查找范围
    pub range: Option<String>,
    /// 是否区分大小写
    pub case_sensitive: Option<bool>,
    /// 是否全词匹配
    pub match_entire_cell: Option<bool>,
    /// 是否搜索公式
    pub search_formulas: Option<bool>,
    /// 是否包含正则表达式
    pub include_regex: Option<bool>,
}

impl FindReplaceRequest {
    /// 创建查找替换请求
    ///
    /// # 参数
    /// * `spreadsheet_token` - 电子表格token
    /// * `sheet_id` - 工作表ID
    /// * `find` - 查找内容
    /// * `replacement` - 替换内容
    pub fn new(
        spreadsheet_token: impl Into<String>,
        sheet_id: impl Into<String>,
        find: impl Into<String>,
        replacement: impl Into<String>,
    ) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            sheet_id: sheet_id.into(),
            find: find.into(),
            replacement: replacement.into(),
            range: None,
            case_sensitive: None,
            match_entire_cell: None,
            search_formulas: None,
            include_regex: None,
        }
    }

    /// 设置查找范围
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.range = Some(range.into());
        self
    }

    /// 设置是否区分大小写
    pub fn case_sensitive(mut self, case_sensitive: bool) -> Self {
        self.case_sensitive = Some(case_sensitive);
        self
    }

    /// 设置是否全词匹配
    pub fn match_entire_cell(mut self, match_entire_cell: bool) -> Self {
        self.match_entire_cell = Some(match_entire_cell);
        self
    }

    /// 设置是否搜索公式
    pub fn search_formulas(mut self, search_formulas: bool) -> Self {
        self.search_formulas = Some(search_formulas);
        self
    }

    /// 设置是否包含正则表达式
    pub fn include_regex(mut self, include_regex: bool) -> Self {
        self.include_regex = Some(include_regex);
        self
    }
}

/// 查找替换响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindReplaceResponse {
    /// 查找替换结果
    pub data: Option<FindReplaceResult>,
}

impl ApiResponseTrait for FindReplaceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查找替换结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindReplaceResult {
    /// 找到的单元格数
    pub num_found: i32,
    /// 替换的单元格数
    pub num_changed: i32,
    /// 公式修改的单元格数
    pub formulas_changed: i32,
    /// 查找的单元格列表
    pub values: Vec<FoundValue>,
}

/// 找到的值
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoundValue {
    /// 工作表ID
    pub sheet_id: String,
    /// 行号
    pub row_index: i32,
    /// 列号
    pub column_index: i32,
    /// 找到的值
    pub value: serde_json::Value,
}

/// 查找替换
///
/// 在电子表格中查找并替换指定内容，支持多种查找选项。
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/findReplace
pub async fn find_replace(
    request: FindReplaceRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<FindReplaceResponse>> {
    // 构建请求体
    let mut body = json!({
        "sheetId": request.sheet_id,
        "find": request.find,
        "replacement": request.replacement
    });

    if let Some(range) = &request.range {
        body["range"] = json!(range);
    }
    if let Some(case_sensitive) = request.case_sensitive {
        body["caseSensitive"] = json!(case_sensitive);
    }
    if let Some(match_entire_cell) = request.match_entire_cell {
        body["matchEntireCell"] = json!(match_entire_cell);
    }
    if let Some(search_formulas) = request.search_formulas {
        body["searchFormulas"] = json!(search_formulas);
    }
    if let Some(include_regex) = request.include_regex {
        body["includeRegex"] = json!(include_regex);
    }

    // 创建API请求
    let mut api_request: ApiRequest<FindReplaceResponse> =
        ApiRequest::post(&format!("{}/spreadsheets/{}/findReplace", CcmSheetApiV3, request.spreadsheet_token))
            .body(body);

    // 如果有请求选项，应用它们
    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    // 发送请求
    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_replace_request_builder() {
        let request = FindReplaceRequest::new(
            "spreadsheet_token",
            "sheet_id",
            "find_text",
            "replace_text"
        );

        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert_eq!(request.sheet_id, "sheet_id");
        assert_eq!(request.find, "find_text");
        assert_eq!(request.replacement, "replace_text");
        assert!(request.range.is_none());
    }

    #[test]
    fn test_find_replace_request_builder_chain() {
        let request = FindReplaceRequest::new(
            "spreadsheet_token",
            "sheet_id",
            "find_text",
            "replace_text"
        )
        .range("A1:C10")
        .case_sensitive(true)
        .match_entire_cell(false)
        .search_formulas(false);

        assert_eq!(request.range, Some("A1:C10".to_string()));
        assert_eq!(request.case_sensitive, Some(true));
        assert_eq!(request.match_entire_cell, Some(false));
        assert_eq!(request.search_formulas, Some(false));
    }

    #[test]
    fn test_found_value_structure() {
        let found_value = FoundValue {
            sheet_id: "sheet_123".to_string(),
            row_index: 5,
            column_index: 3,
            value: serde_json::json!("测试值"),
        };

        assert_eq!(found_value.sheet_id, "sheet_123");
        assert_eq!(found_value.row_index, 5);
        assert_eq!(found_value.column_index, 3);
        assert_eq!(found_value.value, serde_json::json!("测试值"));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(FindReplaceResponse::data_format(), ResponseFormat::Data);
    }
}