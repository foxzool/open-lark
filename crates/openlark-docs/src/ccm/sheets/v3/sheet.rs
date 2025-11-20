//! 工作表管理服务 v3
//!
//! 提供飞书电子表格v3版本的工作表管理功能，包括：
//! - 获取电子表格下的所有工作表
//! - 查询特定工作表的属性信息
//! - 工作表属性管理和操作

use serde_json::Value;
use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    error::LarkAPIError,
    http::Transport,
    SDKResult,
};
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// 工作表信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Sheet {
    /// 工作表ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<String>,
    /// 工作表标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 工作表索引
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// 工作表类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_type: Option<String>,
    /// 工作表是否隐藏
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    /// 行数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_count: Option<i32>,
    /// 列数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_count: Option<i32>,
    /// 工作表URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_url: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

impl Default for Sheet {
    fn default() -> Self {
        Self {
            sheet_id: None,
            title: None,
            index: None,
            sheet_type: None,
            hidden: None,
            row_count: None,
            column_count: None,
            sheet_url: None,
            create_time: None,
            update_time: None,
        }
    }
}

/// 获取工作表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuerySheetsResponse {
    /// 工作表数据
    pub data: QuerySheetsData,
}

/// 获取工作表数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuerySheetsData {
    /// 工作表列表
    pub sheet_items: Vec<Sheet>,
}

impl Default for QuerySheetsResponse {
    fn default() -> Self {
        Self {
            data: QuerySheetsData {
                sheet_items: vec![],
            },
        }
    }
}

impl ApiResponseTrait for QuerySheetsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取单个工作表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSheetResponse {
    /// 工作表数据
    pub data: GetSheetData,
}

/// 获取单个工作表数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSheetData {
    /// 工作表信息
    pub sheet: Sheet,
}

impl Default for GetSheetResponse {
    fn default() -> Self {
        Self {
            data: GetSheetData {
                sheet: Sheet::default(),
            },
        }
    }
}

impl ApiResponseTrait for GetSheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 工作表管理服务
#[derive(Clone, Debug)]
pub struct SheetService {
    config: Config,
}

impl SheetService {
    /// 创建工作表服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取电子表格下的所有工作表
    ///
    /// 根据电子表格token获取电子表格下所有工作表及其属性。
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格的token
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    ///
    /// let response = client.sheets.v3.sheet
    ///     .query_sheets("spreadsheet_token")
    ///     .await?;
    /// ```
    pub async fn query_sheets(
        &self,
        spreadsheet_token: &str,
    ) -> SDKResult<Response<QuerySheetsResponse>> {
        let endpoint = format!(
            "{}/{}/sheets/query",
            openlark_core::endpoints::Endpoints::SHEETS_V3_SPREADSHEETS,
            spreadsheet_token
        );

        let mut api_request = ApiRequest::with_method_and_path(Method::GET, &endpoint);
        api_request.supported_access_token_types =
            vec![AccessTokenType::Tenant, AccessTokenType::User];

        Transport::<QuerySheetsResponse>::request(api_request, &self.config, None).await
    }

    /// 获取单个工作表信息
    ///
    /// 根据工作表ID查询工作表属性信息。
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格的token
    /// - `sheet_id`: 工作表的ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    ///
    /// let response = client.sheets.v3.sheet
    ///     .get_sheet("spreadsheet_token", "sheet_id")
    ///     .await?;
    /// ```
    pub async fn get_sheet(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
    ) -> openlark_core::SDKResult<Response<GetSheetResponse>> {
        let endpoint = format!(
            "{}/{}/sheets/{}",
            openlark_core::endpoints::Endpoints::SHEETS_V3_SPREADSHEETS,
            spreadsheet_token,
            sheet_id
        );

        let mut api_request = ApiRequest::with_method_and_path(Method::GET, &endpoint);
        api_request.supported_access_token_types =
            vec![AccessTokenType::Tenant, AccessTokenType::User];

        let response =
            Transport::<GetSheetResponse>::request(api_request, &self.config, None).await?;

        if response.code() != 0 {
            return Err(LarkAPIError::APIError {
                code: response.code(),
                msg: response.msg().to_string(),
                error: None,
            });
        }

        Ok(response)
    }
}

/// 工作表查询构建器
#[derive(Clone, Debug)]
pub struct QuerySheetsBuilder {
    service: SheetService,
    spreadsheet_token: String,
}

impl QuerySheetsBuilder {
    /// 创建新的查询构建器实例
    pub fn new(service: SheetService, spreadsheet_token: &str) -> Self {
        Self {
            service,
            spreadsheet_token: spreadsheet_token.to_string(),
        }
    }

    /// 执行查询请求
    pub async fn execute(self) -> SDKResult<Response<QuerySheetsResponse>> {
        self.service.query_sheets(&self.spreadsheet_token).await
    }
}

/// 工作表获取构建器
#[derive(Clone, Debug)]
pub struct GetSheetBuilder {
    service: SheetService,
    spreadsheet_token: String,
    sheet_id: String,
}

impl GetSheetBuilder {
    /// 执行获取请求
    pub async fn execute(self) -> openlark_core::SDKResult<Response<GetSheetResponse>> {
        Ok(self
            .service
            .get_sheet(&self.spreadsheet_token, &self.sheet_id)
            .await?)
    }
}

impl SheetService {
    /// 创建工作表查询构建器
    pub fn query_sheets_builder(&self, spreadsheet_token: &str) -> QuerySheetsBuilder {
        QuerySheetsBuilder {
            service: SheetService {
                config: self.config.clone()
            },
            spreadsheet_token: spreadsheet_token.to_string(),
        }
    }

    /// 创建工作表获取构建器
    pub fn get_sheet_builder(&self, spreadsheet_token: &str, sheet_id: &str) -> GetSheetBuilder {
        GetSheetBuilder {
            service: SheetService {
                config: self.config.clone()
            },
            spreadsheet_token: spreadsheet_token.to_string(),
            sheet_id: sheet_id.to_string(),
        }
    }

    /// 查找单元格
    ///
    /// 在指定范围内查找符合查找条件的单元格。
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格的token
    /// - `sheet_id`: 工作表的ID
    /// - `request`: 查找请求
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    ///
    /// let request = FindCellsRequest::new("查找内容")
    ///     .range("A1:C10")
    ///     .match_type("exact");
    ///
    /// let response = client.sheets.v3.sheet
    ///     .find_cells("spreadsheet_token", "sheet_id", &request)
    ///     .await?;
    /// ```
    pub async fn find_cells(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        request: &FindCellsRequest,
    ) -> openlark_core::SDKResult<Response<FindCellsResponse>> {
        let endpoint = format!(
            "{}/{}/sheets/{}/find",
            openlark_core::endpoints::Endpoints::SHEETS_V3_SPREADSHEETS,
            spreadsheet_token,
            sheet_id
        );

        let mut api_request = ApiRequest::with_method_and_path(Method::POST, &endpoint);
        api_request.body = Some(openlark_core::api::RequestData::Json(request))?;

        let response =
            Transport::<FindCellsResponse>::request(api_request, &self.config, None).await?;

        if response.code() != 0 {
            return Err(LarkAPIError::APIError {
                code: response.code(),
                msg: response.msg().to_string(),
                error: None,
            });
        }

        Ok(response)
    }

    /// 创建查找单元格构建器
    pub fn find_cells_builder(&self, spreadsheet_token: &str, sheet_id: &str) -> FindCellsBuilder {
        FindCellsBuilder {
            service: SheetService {
                config: self.config.clone()
            },
            spreadsheet_token: spreadsheet_token.to_string(),
            sheet_id: sheet_id.to_string(),
            request: FindCellsRequest::new(""),
        }
    }
}

/// 查找单元格请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindCellsRequest {
    /// 查找内容
    pub find: String,
    /// 查找范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    /// 匹配类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_type: Option<String>,
    /// 是否区分大小写
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_sensitive: Option<bool>,
    /// 是否整词匹配
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_whole_word: Option<bool>,
}

impl FindCellsRequest {
    /// 创建新的查找请求
    pub fn new(find: impl Into<String>) -> Self {
        Self {
            find: find.into(),
            range: None,
            match_type: None,
            case_sensitive: None,
            match_whole_word: None,
        }
    }

    /// 设置查找范围
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.range = Some(range.into());
        self
    }

    /// 设置匹配类型
    pub fn match_type(mut self, match_type: impl Into<String>) -> Self {
        self.match_type = Some(match_type.into());
        self
    }

    /// 设置是否区分大小写
    pub fn case_sensitive(mut self, case_sensitive: bool) -> Self {
        self.case_sensitive = Some(case_sensitive);
        self
    }

    /// 设置是否整词匹配
    pub fn match_whole_word(mut self, match_whole_word: bool) -> Self {
        self.match_whole_word = Some(match_whole_word);
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.find.trim().is_empty() {
            return Err("查找内容不能为空".to_string());
        }
        if self.find.len() > 1000 {
            return Err("查找内容长度不能超过1000个字符".to_string());
        }
        Ok(())
    }
}

/// 查找单元格响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindCellsResponse {
    /// 查找结果数据
    pub data: FindCellsData,
}

/// 查找单元格数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindCellsData {
    /// 查找结果
    pub value_range: ValueRange,
}

/// 查找结果值范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueRange {
    /// 电子表格token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spreadsheet_token: Option<String>,
    /// 工作表ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<String>,
    /// 查找范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    /// 查找到的值
    pub values: Vec<Vec<String>>,
}

impl Default for FindCellsResponse {
    fn default() -> Self {
        Self {
            data: FindCellsData {
                value_range: ValueRange {
                    spreadsheet_token: None,
                    sheet_id: None,
                    range: None,
                    values: vec![],
                },
            },
        }
    }
}

impl ApiResponseTrait for FindCellsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查找单元格构建器
#[derive(Clone, Debug)]
pub struct FindCellsBuilder {
    request: FindCellsRequest,
    service: SheetService,
    spreadsheet_token: String,
    sheet_id: String,
}

impl FindCellsBuilder {
    /// 创建新的查找构建器实例
    pub fn new(service: SheetService, spreadsheet_token: &str, sheet_id: &str) -> Self {
        Self {
            request: FindCellsRequest::new(""),
            service,
            spreadsheet_token: spreadsheet_token.to_string(),
            sheet_id: sheet_id.to_string(),
        }
    }

    /// 设置查找内容
    pub fn find(mut self, find: impl Into<String>) -> Self {
        self.request.find = find.into();
        self
    }

    /// 设置查找范围
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.request.range = Some(range.into());
        self
    }

    /// 设置匹配类型
    pub fn match_type(mut self, match_type: impl Into<String>) -> Self {
        self.request.match_type = Some(match_type.into());
        self
    }

    /// 设置是否区分大小写
    pub fn case_sensitive(mut self, case_sensitive: bool) -> Self {
        self.request.case_sensitive = Some(case_sensitive);
        self
    }

    /// 设置是否整词匹配
    pub fn match_whole_word(mut self, match_whole_word: bool) -> Self {
        self.request.match_whole_word = Some(match_whole_word);
        self
    }

    /// 执行查找请求
    pub async fn execute(self) -> openlark_core::SDKResult<Response<FindCellsResponse>> {
        self.request
            .validate()
            .map_err(|msg| LarkAPIError::IllegalParamError(msg))?;
        Ok(self
            .service
            .find_cells(&self.spreadsheet_token, &self.sheet_id, &self.request)
            .await?)
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sheet_service_creation() {
        let config = openlark_core::config::openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = SheetService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_sheet_default_creation() {
        let sheet = Sheet::default();
        assert_eq!(sheet.sheet_id, None);
        assert_eq!(sheet.title, None);
        assert_eq!(sheet.index, None);
    }

    #[test]
    fn test_sheet_with_data() {
        let sheet = Sheet {
            sheet_id: Some("sheet_001".to_string()),
            title: Some("工作表1".to_string()),
            index: Some(0),
            sheet_type: Some("grid".to_string()),
            hidden: Some(false),
            row_count: Some(1000),
            column_count: Some(26),
            
        };

        assert_eq!(sheet.sheet_id, Some("sheet_001".to_string()));
        assert_eq!(sheet.title, Some("工作表1".to_string()));
        assert_eq!(sheet.index, Some(0));
        assert_eq!(sheet.sheet_type, Some("grid".to_string()));
        assert_eq!(sheet.hidden, Some(false));
        assert_eq!(sheet.row_count, Some(1000));
        assert_eq!(sheet.column_count, Some(26));
    }

    #[test]
    fn test_query_sheets_response_default() {
        let response = QuerySheetsResponse::default();
        assert!(response.data.sheet_items.is_empty());
    }

    #[test]
    fn test_query_sheets_response_with_data() {
        let sheet = Sheet {
            sheet_id: Some("sheet_002".to_string()),
            title: Some("财务数据".to_string()),
            index: Some(1),
            
        };

        let response = QuerySheetsResponse {
            data: QuerySheetsData {
                sheet_items: vec![sheet],
            },
        };

        assert_eq!(response.data.sheet_items.len(), 1);
        assert_eq!(
            response.data.sheet_items[0].title,
            Some("财务数据".to_string())
        );
    }

    #[test]
    fn test_get_sheet_response_default() {
        let response = GetSheetResponse::default();
        assert_eq!(response.data.sheet.sheet_id, None);
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(QuerySheetsResponse::data_format(), ResponseFormat::Data);
        assert_eq!(GetSheetResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_sheet = Sheet {
            sheet_id: Some("test_sheet".to_string()),
            title: Some("测试工作表".to_string()),
            index: Some(0),
            row_count: Some(500),
            column_count: Some(20),
            
        };

        let serialized = serde_json::to_string(&original_sheet).unwrap();
        let deserialized: Sheet = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_sheet.sheet_id, deserialized.sheet_id);
        assert_eq!(original_sheet.title, deserialized.title);
        assert_eq!(original_sheet.index, deserialized.index);
        assert_eq!(original_sheet.row_count, deserialized.row_count);
        assert_eq!(original_sheet.column_count, deserialized.column_count);
    }

    #[test]
    fn test_endpoint_url_construction() {
        // 测试查询工作表的URL构建
        let spreadsheet_token = "test_spreadsheet_123";
        let expected_query_url = format!(
            "{}/{}/sheets/query",
            openlark_core::endpoints::Endpoints::SHEETS_V3_SPREADSHEETS,
            spreadsheet_token
        );

        // 这里我们只验证格式，不实际调用API
        assert!(expected_query_url.contains("/sheets/query"));
        assert!(expected_query_url.contains(spreadsheet_token));

        // 测试获取单个工作表的URL构建
        let sheet_id = "test_sheet_456";
        let expected_get_url = format!(
            "{}/{}/sheets/{}",
            openlark_core::endpoints::Endpoints::SHEETS_V3_SPREADSHEETS,
            spreadsheet_token,
            sheet_id
        );

        assert!(expected_get_url.contains("/sheets/"));
        assert!(expected_get_url.contains(spreadsheet_token));
        assert!(expected_get_url.contains(sheet_id));
    }

    #[test]
    fn test_builder_patterns() {
        let config = openlark_core::config::openlark_core::config::Config::default();
        let service = SheetService::new(config);

        let query_builder = service.query_sheets_builder("test_token");
        assert!(!format!("{:?}", query_builder).is_empty());

        let get_builder = service.get_sheet_builder("test_token", "test_sheet");
        assert!(!format!("{:?}", get_builder).is_empty());
    }

    #[test]
    fn test_find_cells_request_creation() {
        let request = FindCellsRequest::new("查找内容");
        assert_eq!(request.find, "查找内容");
        assert_eq!(request.range, None);
        assert_eq!(request.match_type, None);
    }

    #[test]
    fn test_find_cells_request_builder() {
        let request = FindCellsRequest::new("测试查找")
            .range("A1:C10")
            .match_type("exact")
            .case_sensitive(true)
            .match_whole_word(false);

        assert_eq!(request.find, "测试查找");
        assert_eq!(request.range, Some("A1:C10".to_string()));
        assert_eq!(request.match_type, Some("exact".to_string()));
        assert_eq!(request.case_sensitive, Some(true));
        assert_eq!(request.match_whole_word, Some(false));
    }

    #[test]
    fn test_find_cells_request_validation() {
        // 测试正常内容
        let valid_request = FindCellsRequest::new("正常查找内容");
        assert!(valid_request.validate().is_ok());

        // 测试空内容
        let empty_request = FindCellsRequest::new("  ");
        assert!(empty_request.validate().is_err());

        // 测试过长内容
        let long_request = FindCellsRequest::new(&"a".repeat(1001));
        assert!(long_request.validate().is_err());
    }

    #[test]
    fn test_find_cells_response_default() {
        let response = FindCellsResponse::default();
        assert!(response.data.value_range.values.is_empty());
        assert_eq!(response.data.value_range.spreadsheet_token, None);
        assert_eq!(response.data.value_range.sheet_id, None);
        assert_eq!(response.data.value_range.range, None);
    }

    #[test]
    fn test_find_cells_response_with_data() {
        let value_range = ValueRange {
            spreadsheet_token: Some("test_spreadsheet".to_string()),
            sheet_id: Some("test_sheet".to_string()),
            range: Some("A1:B2".to_string()),
            values: vec![
                vec!["结果1".to_string(), "结果2".to_string()],
                vec!["结果3".to_string(), "结果4".to_string()],
            ],
        };

        let response = FindCellsResponse {
            data: FindCellsData { value_range },
        };

        assert_eq!(
            response.data.value_range.spreadsheet_token,
            Some("test_spreadsheet".to_string())
        );
        assert_eq!(
            response.data.value_range.sheet_id,
            Some("test_sheet".to_string())
        );
        assert_eq!(response.data.value_range.range, Some("A1:B2".to_string()));
        assert_eq!(response.data.value_range.values.len(), 2);
        assert_eq!(response.data.value_range.values[0][0], "结果1");
    }

    #[test]
    fn test_find_cells_builder() {
        let config = openlark_core::config::openlark_core::config::Config::default();
        let service = SheetService::new(config);

        let find_builder = service
            .find_cells_builder("test_token", "test_sheet")
            .find("查找文本")
            .range("A1:D20")
            .match_type("contains")
            .case_sensitive(true);

        assert_eq!(find_builder.request.find, "查找文本");
        assert_eq!(find_builder.request.range, Some("A1:D20".to_string()));
        assert_eq!(
            find_builder.request.match_type,
            Some("contains".to_string())
        );
        assert_eq!(find_builder.request.case_sensitive, Some(true));
        assert!(!format!("{:?}", find_builder).is_empty());
    }

    #[test]
    fn test_find_cells_serialization() {
        let request = FindCellsRequest::new("序列化测试")
            .range("B2:C15")
            .match_type("exact");

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: FindCellsRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.find, deserialized.find);
        assert_eq!(request.range, deserialized.range);
        assert_eq!(request.match_type, deserialized.match_type);
    }

    #[test]
    fn test_value_range_creation() {
        let value_range = ValueRange {
            spreadsheet_token: Some("sheet_token_123".to_string()),
            sheet_id: Some("sheet_456".to_string()),
            range: Some("Sheet1!A1:Z100".to_string()),
            values: vec![
                vec!["A1".to_string(), "B1".to_string()],
                vec!["A2".to_string(), "B2".to_string()],
            ],
        };

        assert_eq!(
            value_range.spreadsheet_token,
            Some("sheet_token_123".to_string())
        );
        assert_eq!(value_range.sheet_id, Some("sheet_456".to_string()));
        assert_eq!(value_range.range, Some("Sheet1!A1:Z100".to_string()));
        assert_eq!(value_range.values.len(), 2);
        assert_eq!(value_range.values[0].len(), 2);
    }

    #[test]
    fn test_endpoint_construction() {
        let spreadsheet_token = "test_spreadsheet_789";
        let sheet_id = "test_sheet_012";

        let expected_endpoint = format!(
            "{}/{}/sheets/{}/find",
            openlark_core::endpoints::Endpoints::SHEETS_V3_SPREADSHEETS,
            spreadsheet_token,
            sheet_id
        );

        assert!(expected_endpoint.contains("/sheets/"));
        assert!(expected_endpoint.contains("/find"));
        assert!(expected_endpoint.contains(spreadsheet_token));
        assert!(expected_endpoint.contains(sheet_id));
    }

    #[test]
    fn test_sheet_property_validation() {
        // 测试工作表属性的各种组合
        let mut sheet = Sheet::default();

        // 设置基本属性
        sheet.title = Some("新工作表".to_string());
        sheet.index = Some(1);
        sheet.row_count = Some(100);
        sheet.column_count = Some(10);

        assert_eq!(sheet.title, Some("新工作表".to_string()));
        assert_eq!(sheet.index, Some(1));
        assert_eq!(sheet.row_count, Some(100));
        assert_eq!(sheet.column_count, Some(10));

        // 测试隐藏属性
        sheet.hidden = Some(true);
        assert_eq!(sheet.hidden, Some(true));

        // 测试工作表类型
        sheet.sheet_type = Some("object".to_string());
        assert_eq!(sheet.sheet_type, Some("object".to_string()));
    }
}
