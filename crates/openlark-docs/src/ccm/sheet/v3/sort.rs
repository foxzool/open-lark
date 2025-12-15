use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 排序请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SortRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表ID
    pub sheet_id: String,
    /// 排序范围
    pub range: String,
    /// 排序规则
    pub sort_specs: Vec<SortSpec>,
}

/// 排序规则
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SortSpec {
    /// 排序顺序
    pub sort_order: String,
    /// 排序键索引
    pub sort_key: i32,
    /// 排序类型
    pub sort_type: Option<String>,
}

/// 排序响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct SortResponse {
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
    /// 数据区域
    pub data: Option<Vec<GridData>>,
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
    /// 工作表类型
    pub sheet_type: String,
}

/// 网格数据
#[derive(Debug, Clone, Deserialize, Default)]
pub struct GridData {
    /// 起始行
    pub start_row: i32,
    /// 起始列
    pub start_column: i32,
    /// 行数据
    pub row_data: Vec<RowData>,
}

/// 行数据
#[derive(Debug, Clone, Deserialize, Default)]
pub struct RowData {
    /// 行号
    pub row_number: i32,
    /// 单元格数据
    pub values: Vec<CellData>,
}

/// 单元格数据
#[derive(Debug, Clone, Deserialize, Default)]
pub struct CellData {
    /// 行号
    pub row_index: i32,
    /// 列号
    pub column_index: i32,
    /// 单元格值
    pub value: Option<serde_json::Value>,
}

impl ApiResponseTrait for SortResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SortRequest {
    /// 创建新的排序请求构建器
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

    /// 设置排序范围
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.range = range.into();
        self
    }

    /// 添加排序规则
    pub fn add_sort_spec(mut self, spec: SortSpec) -> Self {
        self.sort_specs.push(spec);
        self
    }

    /// 设置排序规则
    pub fn sort_specs(mut self, specs: Vec<SortSpec>) -> Self {
        self.sort_specs = specs;
        self
    }
}

impl SortSpec {
    /// 创建新的排序规则构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置排序顺序
    pub fn sort_order(mut self, order: impl Into<String>) -> Self {
        self.sort_order = order.into();
        self
    }

    /// 设置排序键索引
    pub fn sort_key(mut self, key: i32) -> Self {
        self.sort_key = key;
        self
    }

    /// 设置排序类型
    pub fn sort_type(mut self, sort_type: impl Into<String>) -> Self {
        self.sort_type = Some(sort_type.into());
        self
    }

    /// 创建升序排序
    pub fn ascending(key: i32) -> Self {
        Self {
            sort_order: "ASCENDING".to_string(),
            sort_key: key,
            sort_type: None,
        }
    }

    /// 创建降序排序
    pub fn descending(key: i32) -> Self {
        Self {
            sort_order: "DESCENDING".to_string(),
            sort_key: key,
            sort_type: None,
        }
    }

    /// 创建文本升序排序
    pub fn text_ascending(key: i32) -> Self {
        Self {
            sort_order: "ASCENDING".to_string(),
            sort_key: key,
            sort_type: Some("TEXT".to_string()),
        }
    }

    /// 创建文本降序排序
    pub fn text_descending(key: i32) -> Self {
        Self {
            sort_order: "DESCENDING".to_string(),
            sort_key: key,
            sort_type: Some("TEXT".to_string()),
        }
    }

    /// 创建数字升序排序
    pub fn number_ascending(key: i32) -> Self {
        Self {
            sort_order: "ASCENDING".to_string(),
            sort_key: key,
            sort_type: Some("NUMBER".to_string()),
        }
    }

    /// 创建数字降序排序
    pub fn number_descending(key: i32) -> Self {
        Self {
            sort_order: "DESCENDING".to_string(),
            sort_key: key,
            sort_type: Some("NUMBER".to_string()),
        }
    }
}

/// 排序
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/sort
pub async fn sort_range(
    request: SortRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<SortResponse>> {
    // 构建请求体
    let body = json!(request);

    // 创建API请求
    let mut api_request: ApiRequest<SortResponse> =
        ApiRequest::post(&format!("{}/spreadsheets/{}/sort", SheetsApiV3, request.spreadsheet_token))
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
    fn test_sort_request_builder() {
        let request = SortRequest::new()
            .spreadsheet_token("test_token")
            .sheet_id("test_sheet")
            .range("A1:D10")
            .add_sort_spec(SortSpec::text_ascending(0))
            .add_sort_spec(SortSpec::number_descending(1));

        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.sheet_id, "test_sheet");
        assert_eq!(request.range, "A1:D10");
        assert_eq!(request.sort_specs.len(), 2);
        assert_eq!(request.sort_specs[0].sort_order, "ASCENDING");
        assert_eq!(request.sort_specs[0].sort_type, Some("TEXT".to_string()));
        assert_eq!(request.sort_specs[1].sort_order, "DESCENDING");
        assert_eq!(request.sort_specs[1].sort_type, Some("NUMBER".to_string()));
    }

    #[test]
    fn test_sort_spec_convenience_methods() {
        let spec1 = SortSpec::ascending(0);
        assert_eq!(spec1.sort_order, "ASCENDING");
        assert_eq!(spec1.sort_key, 0);
        assert_eq!(spec1.sort_type, None);

        let spec2 = SortSpec::descending(1);
        assert_eq!(spec2.sort_order, "DESCENDING");
        assert_eq!(spec2.sort_key, 1);
        assert_eq!(spec2.sort_type, None);

        let spec3 = SortSpec::text_ascending(0);
        assert_eq!(spec3.sort_order, "ASCENDING");
        assert_eq!(spec3.sort_key, 0);
        assert_eq!(spec3.sort_type, Some("TEXT".to_string()));

        let spec4 = SortSpec::number_descending(1);
        assert_eq!(spec4.sort_order, "DESCENDING");
        assert_eq!(spec4.sort_key, 1);
        assert_eq!(spec4.sort_type, Some("NUMBER".to_string()));
    }

    #[test]
    fn test_sort_spec_builder() {
        let spec = SortSpec::new()
            .sort_order("ASCENDING")
            .sort_key(0)
            .sort_type("TEXT");

        assert_eq!(spec.sort_order, "ASCENDING");
        assert_eq!(spec.sort_key, 0);
        assert_eq!(spec.sort_type, Some("TEXT".to_string()));
    }
}