/// 获取工作表列表
///
/// 此接口用于获取指定电子表格中所有工作表的基本信息，包括工作表ID、标题、索引等。
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/sheets/meta
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 获取工作表列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSheetsRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
}

impl ListSheetsRequest {
    /// 创建获取工作表列表请求
    ///
    /// # 参数
    /// * `spreadsheet_token` - 电子表格token
    pub fn new(spreadsheet_token: impl Into<String>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
        }
    }

    /// 设置电子表格token
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl Into<String>) -> Self {
        self.spreadsheet_token = spreadsheet_token.into();
        self
    }
}

/// 获取工作表列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSheetsResponse {
    /// 电子表格属性
    pub data: Option<SpreadsheetProperties>,
}

impl ApiResponseTrait for ListSheetsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 电子表格属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpreadsheetProperties {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表列表
    pub sheets: Vec<SheetInfo>,
}

/// 工作表信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetInfo {
    /// 工作表属性
    pub properties: SheetPropertiesInfo,
    /// 数据区域
    pub data: Option<Vec<GridData>>,
}

/// 工作表属性信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetPropertiesInfo {
    /// 工作表ID
    pub sheet_id: String,
    /// 工作表标题
    pub title: String,
    /// 工作表索引
    pub index: i32,
    /// 工作表类型
    pub sheet_type: String,
    /// 网格属性
    pub grid_properties: Option<GridPropertiesInfo>,
}

/// 网格属性信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridPropertiesInfo {
    /// 行数
    pub row_count: i32,
    /// 列数
    pub column_count: i32,
    /// 冻结行数
    pub frozen_row_count: i32,
    /// 冻结列数
    pub frozen_column_count: i32,
}

/// 网格数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridData {
    /// 起始行
    pub start_row: i32,
    /// 起始列
    pub start_column: i32,
    /// 行数据
    pub row_data: Vec<RowData>,
}

/// 行数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RowData {
    /// 行号
    pub row_number: i32,
    /// 单元格数据
    pub values: Vec<CellData>,
}

/// 单元格数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellData {
    /// 行号
    pub row_index: i32,
    /// 列号
    pub column_index: i32,
    /// 单元格值
    pub value: Option<serde_json::Value>,
    /// 格式
    pub format: Option<String>,
}

/// 获取工作表列表
///
/// 获取指定电子表格中所有工作表的基本信息。
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/sheets/meta
pub async fn list_sheets(
    request: ListSheetsRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<ListSheetsResponse>> {
    // 使用SheetsApiV3枚举生成API端点
    let api_endpoint = SheetsApiV3::SheetsMeta(request.spreadsheet_token);

    // 创建API请求
    let mut api_request: ApiRequest<ListSheetsResponse> = ApiRequest::get(&api_endpoint.to_url());

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
    fn test_list_sheets_request_builder() {
        let request = ListSheetsRequest::new("spreadsheet_token");

        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
    }

    #[test]
    fn test_list_sheets_request_with_token() {
        let request = ListSheetsRequest::new("initial_token").spreadsheet_token("new_token");

        assert_eq!(request.spreadsheet_token, "new_token");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(ListSheetsResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_api_endpoint() {
        let endpoint = SheetsApiV3::SheetsMeta("test_token".to_string());
        assert_eq!(endpoint.to_url(), "/open-apis/sheets/v3/spreadsheets/test_token/sheets/meta");
    }
}