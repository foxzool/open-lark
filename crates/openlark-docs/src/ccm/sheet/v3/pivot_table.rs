use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 创建数据透视表请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreatePivotTableRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 源工作表ID
    pub source_sheet_id: String,
    /// 数据范围
    pub source_range: String,
    /// 目标工作表ID
    pub target_sheet_id: String,
    /// 行字段
    pub row_fields: Vec<String>,
    /// 列字段
    pub column_fields: Vec<String>,
    /// 值字段
    pub value_fields: Vec<ValueField>,
}

/// 值字段
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ValueField {
    /// 字段名称
    pub field_name: String,
    /// 汇总函数
    pub summarize_function: String,
}

/// 创建数据透视表响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct CreatePivotTableResponse {
    /// 透视表ID
    pub pivot_table_id: String,
    /// 操作结果
    pub result: String,
}

impl ApiResponseTrait for CreatePivotTableResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreatePivotTableRequest {
    /// 创建新的创建数据透视表请求构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置电子表格token
    pub fn spreadsheet_token(mut self, token: impl Into<String>) -> Self {
        self.spreadsheet_token = token.into();
        self
    }

    /// 设置源工作表ID
    pub fn source_sheet_id(mut self, id: impl Into<String>) -> Self {
        self.source_sheet_id = id.into();
        self
    }

    /// 设置数据范围
    pub fn source_range(mut self, range: impl Into<String>) -> Self {
        self.source_range = range.into();
        self
    }

    /// 设置目标工作表ID
    pub fn target_sheet_id(mut self, id: impl Into<String>) -> Self {
        self.target_sheet_id = id.into();
        self
    }

    /// 添加行字段
    pub fn add_row_field(mut self, field: impl Into<String>) -> Self {
        self.row_fields.push(field.into());
        self
    }

    /// 设置行字段
    pub fn row_fields(mut self, fields: Vec<String>) -> Self {
        self.row_fields = fields;
        self
    }

    /// 添加列字段
    pub fn add_column_field(mut self, field: impl Into<String>) -> Self {
        self.column_fields.push(field.into());
        self
    }

    /// 设置列字段
    pub fn column_fields(mut self, fields: Vec<String>) -> Self {
        self.column_fields = fields;
        self
    }

    /// 添加值字段
    pub fn add_value_field(mut self, field: ValueField) -> Self {
        self.value_fields.push(field);
        self
    }

    /// 设置值字段
    pub fn value_fields(mut self, fields: Vec<ValueField>) -> Self {
        self.value_fields = fields;
        self
    }
}

impl ValueField {
    /// 创建新的值字段构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置字段名称
    pub fn field_name(mut self, name: impl Into<String>) -> Self {
        self.field_name = name.into();
        self
    }

    /// 设置汇总函数
    pub fn summarize_function(mut self, function: impl Into<String>) -> Self {
        self.summarize_function = function.into();
        self
    }
}

/// 创建数据透视表
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/pivotTables
pub async fn create_pivot_table(
    request: CreatePivotTableRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<CreatePivotTableResponse>> {
    // 构建请求体
    let body = json!(request);

    // 创建API请求
    let mut api_request: ApiRequest<CreatePivotTableResponse> =
        ApiRequest::post(&format!("{}/spreadsheets/{}/pivotTables", SheetsApiV3, request.spreadsheet_token))
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
    fn test_create_pivot_table_request_builder() {
        let request = CreatePivotTableRequest::new()
            .spreadsheet_token("test_token")
            .source_sheet_id("source_sheet")
            .source_range("A1:D100")
            .target_sheet_id("target_sheet")
            .add_row_field("category")
            .add_column_field("month")
            .add_value_field(
                ValueField::new()
                    .field_name("amount")
                    .summarize_function("SUM")
            );

        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.source_sheet_id, "source_sheet");
        assert_eq!(request.source_range, "A1:D100");
        assert_eq!(request.target_sheet_id, "target_sheet");
        assert_eq!(request.row_fields, vec!["category"]);
        assert_eq!(request.column_fields, vec!["month"]);
        assert_eq!(request.value_fields.len(), 1);
        assert_eq!(request.value_fields[0].field_name, "amount");
        assert_eq!(request.value_fields[0].summarize_function, "SUM");
    }

    #[test]
    fn test_value_field_builder() {
        let field = ValueField::new()
            .field_name("sales")
            .summarize_function("AVERAGE");

        assert_eq!(field.field_name, "sales");
        assert_eq!(field.summarize_function, "AVERAGE");
    }
}