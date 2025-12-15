use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 合并单元格请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MergeCellsRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表ID
    pub sheet_id: String,
    /// 合并类型
    pub merge_type: String,
    /// 起始行
    pub start_row: i32,
    /// 起始列
    pub start_column: i32,
    /// 结束行
    pub end_row: i32,
    /// 结束列
    pub end_column: i32,
}

/// 合并单元格响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct MergeCellsResponse {
    /// 电子表格属性
    pub spreadsheet: SpreadsheetProperties,
    /// 操作结果
    pub result: String,
}

/// 电子表格属性
#[derive(Debug, Clone, Deserialize, Default)]
pub struct SpreadsheetProperties {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表列表
    pub sheets: Vec<SheetInfo>,
}

/// 工作表信息
#[derive(Debug, Clone, Deserialize, Default)]
pub struct SheetInfo {
    /// 工作表属性
    pub properties: SheetPropertiesInfo,
}

/// 工作表属性信息
#[derive(Debug, Clone, Deserialize, Default)]
pub struct SheetPropertiesInfo {
    /// 工作表ID
    pub sheet_id: String,
    /// 工作表标题
    pub title: String,
    /// 工作表索引
    pub index: i32,
}

impl ApiResponseTrait for MergeCellsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl MergeCellsRequest {
    /// 创建新的合并单元格请求构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置电子表格token
    pub fn spreadsheet_token(mut self, token: impl Into<String>) -> Self {
        self.spreadsheet_token = token.into();
        self
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, id: impl Into<String>) -> Self {
        self.sheet_id = id.into();
        self
    }

    /// 设置合并类型
    pub fn merge_type(mut self, merge_type: impl Into<String>) -> Self {
        self.merge_type = merge_type.into();
        self
    }

    /// 设置起始行
    pub fn start_row(mut self, row: i32) -> Self {
        self.start_row = row;
        self
    }

    /// 设置起始列
    pub fn start_column(mut self, column: i32) -> Self {
        self.start_column = column;
        self
    }

    /// 设置结束行
    pub fn end_row(mut self, row: i32) -> Self {
        self.end_row = row;
        self
    }

    /// 设置结束列
    pub fn end_column(mut self, column: i32) -> Self {
        self.end_column = column;
        self
    }

    /// 设置范围（便捷方法）
    pub fn range(mut self, start_row: i32, start_column: i32, end_row: i32, end_column: i32) -> Self {
        self.start_row = start_row;
        self.start_column = start_column;
        self.end_row = end_row;
        self.end_column = end_column;
        self
    }
}

/// 合并单元格
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/merge
pub async fn merge_cells(
    request: MergeCellsRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<MergeCellsResponse>> {
    // 构建请求体
    let body = json!(request);

    // 创建API请求
    let mut api_request: ApiRequest<MergeCellsResponse> =
        ApiRequest::post(&format!("{}/spreadsheets/{}/merge", SheetsApiV3, request.spreadsheet_token))
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
    fn test_merge_cells_request_builder() {
        let request = MergeCellsRequest::new()
            .spreadsheet_token("test_token")
            .sheet_id("test_sheet")
            .merge_type("MERGE_ALL")
            .range(0, 0, 1, 2);

        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.sheet_id, "test_sheet");
        assert_eq!(request.merge_type, "MERGE_ALL");
        assert_eq!(request.start_row, 0);
        assert_eq!(request.start_column, 0);
        assert_eq!(request.end_row, 1);
        assert_eq!(request.end_column, 2);
    }
}