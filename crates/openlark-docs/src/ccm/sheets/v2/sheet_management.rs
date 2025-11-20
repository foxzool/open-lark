//! 工作表管理服务 v2
//!
//! 提供飞书电子表格v2版本的工作表管理功能，包括：
//! - 批量更新工作表属性
//! - 工作表重命名和属性修改
//! - 工作表隐藏/显示操作
//! - 工作表索引调整

use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    error::LarkAPIError,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// 工作表更新请求项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSheetRequest {
    /// 工作表ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<String>,
    /// 工作表标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 工作表索引
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// 是否隐藏
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    /// 工作表行数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_count: Option<i32>,
    /// 工作表列数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_count: Option<i32>,
}

impl UpdateSheetRequest {
    /// 创建新的工作表更新请求
    pub fn new() -> Self {
        Self {
            sheet_id: None,
            title: None,
            index: None,
            hidden: None,
            row_count: None,
            column_count: None,
        }
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, sheet_id: impl Into<String>) -> Self {
        self.sheet_id = Some(sheet_id.into());
        self
    }

    /// 设置工作表标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 设置工作表索引
    pub fn index(mut self, index: i32) -> Self {
        self.index = Some(index);
        self
    }

    /// 设置是否隐藏
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = Some(hidden);
        self
    }

    /// 设置行数
    pub fn row_count(mut self, row_count: i32) -> Self {
        self.row_count = Some(row_count);
        self
    }

    /// 设置列数
    pub fn column_count(mut self, column_count: i32) -> Self {
        self.column_count = Some(column_count);
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if let Some(ref title) = self.title {
            if title.trim().is_empty() {
                return Err("工作表标题不能为空".to_string());
            }
            if title.len() > 100 {
                return Err("工作表标题长度不能超过100个字符".to_string());
            }
        }

        if let Some(index) = self.index {
            if index < 0 || index > 1000 {
                return Err("工作表索引必须在0-1000之间".to_string());
            }
        }

        if let Some(row_count) = self.row_count {
            if row_count < 1 || row_count > 100000 {
                return Err("工作表行数必须在1-100000之间".to_string());
            }
        }

        if let Some(column_count) = self.column_count {
            if column_count < 1 || column_count > 1000 {
                return Err("工作表列数必须在1-1000之间".to_string());
            }
        }

        Ok(())
    }
}

/// 批量更新工作表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateSheetsRequest {
    /// 工作表更新请求列表
    pub requests: Vec<UpdateSheetRequest>,
}

impl BatchUpdateSheetsRequest {
    /// 创建新的批量更新请求
    pub fn new() -> Self {
        Self { requests: vec![] }
    }

    /// 添加更新请求
    pub fn add_request(mut self, request: UpdateSheetRequest) -> Self {
        self.requests.push(request);
        self
    }

    /// 添加多个更新请求
    pub fn add_requests(mut self, requests: Vec<UpdateSheetRequest>) -> Self {
        self.requests.extend(requests);
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.requests.is_empty() {
            return Err("至少需要一个工作表更新请求".to_string());
        }

        if self.requests.len() > 50 {
            return Err("单次批量更新最多支持50个工作表".to_string());
        }

        for (index, request) in self.requests.iter().enumerate() {
            if let Err(err) = request.validate() {
                return Err(format!("第{}个工作表更新请求验证失败: {}", index + 1, err));
            }
        }

        Ok(())
    }
}

/// 更新工作表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSheetsResponse {
    /// 响应数据
    pub data: UpdateSheetsData,
}

/// 更新工作表数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSheetsData {
    /// 更新结果
    #[serde(rename = "updatedSheets")]
    pub updated_sheets: Vec<UpdatedSheet>,
}

/// 更新后的工作表信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatedSheet {
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: String,
    /// 工作表标题
    #[serde(rename = "title")]
    pub title: String,
    /// 工作表索引
    pub index: i32,
    /// 是否隐藏
    pub hidden: bool,
    /// 行数
    #[serde(rename = "rowCount")]
    pub row_count: i32,
    /// 列数
    #[serde(rename = "columnCount")]
    pub column_count: i32,
    /// 电子表格token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
}

impl Default for UpdateSheetsResponse {
    fn default() -> Self {
        Self {
            data: UpdateSheetsData {
                updated_sheets: vec![],
            },
        }
    }
}

impl ApiResponseTrait for UpdateSheetsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 工作表管理服务
#[derive(Clone, Debug)]
pub struct SheetManagementService {
    config: Config,
}

impl SheetManagementService {
    /// 创建工作表管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 批量更新工作表
    ///
    /// 根据电子表格token批量更新工作表属性。
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格的token
    /// - `request`: 批量更新请求
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    ///
    /// let request1 = UpdateSheetRequest::new()
    ///     .sheet_id("sheet1")
    ///     .title("新标题1")
    ///     .hidden(false);
    ///
    /// let request2 = UpdateSheetRequest::new()
    ///     .sheet_id("sheet2")
    ///     .title("新标题2")
    ///     .index(1);
    ///
    /// let batch_request = BatchUpdateSheetsRequest::new()
    ///     .add_request(request1)
    ///     .add_request(request2);
    ///
    /// let response = client.sheets.v2.sheet_management
    ///     .batch_update_sheets("spreadsheet_token", &batch_request)
    ///     .await?;
    /// ```
    pub async fn batch_update_sheets(
        &self,
        spreadsheet_token: &str,
        request: &BatchUpdateSheetsRequest,
    ) -> SDKResult<Response<UpdateSheetsResponse>> {
        if let Err(err) = request.validate() {
            return Err(LarkAPIError::InvalidParameter(err));
        }

        // 构建API请求
        let mut api_req = ApiRequest::with_method_and_path(
            Method::POST,
            format!(
                "/open-apis/sheets/v2/spreadsheets/{}/sheets_batch_update",
                spreadsheet_token
            ),
        );
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        api_req.body = Some(openlark_core::api::RequestData::Json(request))?;

        // 发送请求
        let api_resp =
            Transport::<UpdateSheetsResponse>::request(api_req, &self.config, None).await?;

        Ok(api_resp)
    }

    /// 创建工作表更新构建器
    pub fn update_sheet_builder(&self) -> UpdateSheetBuilder {
        UpdateSheetBuilder::new(self.config.clone())
    }

    /// 创建批量更新构建器
    pub fn batch_update_builder(&self, spreadsheet_token: &str) -> BatchUpdateSheetsBuilder {
        BatchUpdateSheetsBuilder::new(self.config.clone() spreadsheet_token)
    }
}

/// 工作表更新构建器
#[derive(Clone, Debug)]
pub struct UpdateSheetBuilder {
    request: UpdateSheetRequest,
    config: Config,
}

impl UpdateSheetBuilder {
    /// 创建新的更新构建器实例
    pub fn new(config: Config) -> Self {
        Self {
            request: UpdateSheetRequest::new(),
            config,
        }
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, sheet_id: impl Into<String>) -> Self {
        self.request = self.request.sheet_id(sheet_id);
        self
    }

    /// 设置工作表标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.request = self.request.title(title);
        self
    }

    /// 设置工作表索引
    pub fn index(mut self, index: i32) -> Self {
        self.request = self.request.index(index);
        self
    }

    /// 设置是否隐藏
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.request = self.request.hidden(hidden);
        self
    }

    /// 设置行数
    pub fn row_count(mut self, row_count: i32) -> Self {
        self.request = self.request.row_count(row_count);
        self
    }

    /// 设置列数
    pub fn column_count(mut self, column_count: i32) -> Self {
        self.request = self.request.column_count(column_count);
        self
    }

    /// 执行更新请求
    pub async fn execute(
        self,
        spreadsheet_token: &str,
    ) -> SDKResult<Response<UpdateSheetsResponse>> {
        self.request.validate()?;

        let batch_request = BatchUpdateSheetsRequest::new().add_request(self.request);

        let service = SheetManagementService {
            config: self.config,
        };
        service
            .batch_update_sheets(spreadsheet_token, &batch_request)
            .await
    }
}

/// 批量更新工作表构建器
#[derive(Clone, Debug)]
pub struct BatchUpdateSheetsBuilder {
    batch_request: BatchUpdateSheetsRequest,
    config: Config,
    spreadsheet_token: String,
}

impl BatchUpdateSheetsBuilder {
    /// 创建新的批量更新构建器实例
    pub fn new(config: Config, spreadsheet_token: &str) -> Self {
        Self {
            batch_request: BatchUpdateSheetsRequest::new(),
            config,
            spreadsheet_token: spreadsheet_token.to_string(),
        }
    }

    /// 添加更新请求
    pub fn add_request(mut self, request: UpdateSheetRequest) -> Self {
        self.batch_request = self.batch_request.add_request(request);
        self
    }

    /// 添加多个更新请求
    pub fn add_requests(mut self, requests: Vec<UpdateSheetRequest>) -> Self {
        self.batch_request = self.batch_request.add_requests(requests);
        self
    }

    /// 执行批量更新请求
    pub async fn execute(self) -> SDKResult<Response<UpdateSheetsResponse>> {
        self.batch_request.validate()?;
        let service = SheetManagementService {
            config: self.config,
        };
        service
            .batch_update_sheets(&self.spreadsheet_token, &self.batch_request)
            .await
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sheet_management_service_creation() {
        let config = openlark_core::config::openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = SheetManagementService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_update_sheet_request_creation() {
        let request = UpdateSheetRequest::new();
        assert_eq!(request.sheet_id, None);
        assert_eq!(request.title, None);
        assert_eq!(request.index, None);
    }

    #[test]
    fn test_update_sheet_request_builder() {
        let request = UpdateSheetRequest::new()
            .sheet_id("sheet_123")
            .title("新工作表标题")
            .index(1)
            .hidden(true)
            .row_count(100)
            .column_count(50);

        assert_eq!(request.sheet_id, Some("sheet_123".to_string()));
        assert_eq!(request.title, Some("新工作表标题".to_string()));
        assert_eq!(request.index, Some(1));
        assert_eq!(request.hidden, Some(true));
        assert_eq!(request.row_count, Some(100));
        assert_eq!(request.column_count, Some(50));
    }

    #[test]
    fn test_update_sheet_request_validation() {
        // 测试正常请求
        let valid_request = UpdateSheetRequest::new()
            .title("正常标题")
            .index(5)
            .row_count(1000)
            .column_count(500);

        assert!(valid_request.validate().is_ok());

        // 测试空标题
        let empty_title_request = UpdateSheetRequest::new().title("  ");
        assert!(empty_title_request.validate().is_err());

        // 测试过长标题
        let long_title_request = UpdateSheetRequest::new().title(&"a".repeat(101));
        assert!(long_title_request.validate().is_err());

        // 测试无效索引
        let invalid_index_request = UpdateSheetRequest::new().index(-1);
        assert!(invalid_index_request.validate().is_err());

        // 测试无效行数
        let invalid_rows_request = UpdateSheetRequest::new().row_count(0);
        assert!(invalid_rows_request.validate().is_err());

        // 测试无效列数
        let invalid_cols_request = UpdateSheetRequest::new().column_count(0);
        assert!(invalid_cols_request.validate().is_err());
    }

    #[test]
    fn test_batch_update_sheets_request() {
        let request1 = UpdateSheetRequest::new()
            .sheet_id("sheet1")
            .title("工作表1");

        let request2 = UpdateSheetRequest::new()
            .sheet_id("sheet2")
            .title("工作表2");

        let batch_request = BatchUpdateSheetsRequest::new()
            .add_request(request1)
            .add_request(request2);

        assert_eq!(batch_request.requests.len(), 2);
        assert_eq!(
            batch_request.requests[0].sheet_id,
            Some("sheet1".to_string())
        );
        assert_eq!(
            batch_request.requests[1].sheet_id,
            Some("sheet2".to_string())
        );
    }

    #[test]
    fn test_batch_update_sheets_request_validation() {
        // 测试正常请求
        let valid_request1 = UpdateSheetRequest::new().title("正常标题");
        let valid_request2 = UpdateSheetRequest::new().title("另一个标题");

        let valid_batch = BatchUpdateSheetsRequest::new()
            .add_request(valid_request1)
            .add_request(valid_request2);

        assert!(valid_batch.validate().is_ok());

        // 测试空请求列表
        let empty_batch = BatchUpdateSheetsRequest::new();
        assert!(empty_batch.validate().is_err());

        // 测试过多请求
        let mut too_many_batch = BatchUpdateSheetsRequest::new();
        for i in 0..51 {
            too_many_batch = too_many_batch
                .add_request(UpdateSheetRequest::new().title(&format!("工作表{}", i)));
        }
        assert!(too_many_batch.validate().is_err());

        // 测试包含无效请求
        let invalid_request = UpdateSheetRequest::new().title("  ");
        let invalid_batch = BatchUpdateSheetsRequest::new()
            .add_request(UpdateSheetRequest::new().title("正常标题"))
            .add_request(invalid_request);

        assert!(invalid_batch.validate().is_err());
    }

    #[test]
    fn test_update_sheets_response_default() {
        let response = UpdateSheetsResponse::default();
        assert!(response.data.updated_sheets.is_empty());
    }

    #[test]
    fn test_update_sheets_response_with_data() {
        let updated_sheet = UpdatedSheet {
            sheet_id: "sheet_123".to_string(),
            title: "更新后的工作表".to_string(),
            index: 2,
            hidden: false,
            row_count: 500,
            column_count: 26,
            spreadsheet_token: "token_456".to_string(),
        };

        let response = UpdateSheetsResponse {
            data: UpdateSheetsData {
                updated_sheets: vec![updated_sheet],
            },
        };

        assert_eq!(response.data.updated_sheets.len(), 1);
        assert_eq!(response.data.updated_sheets[0].sheet_id, "sheet_123");
        assert_eq!(response.data.updated_sheets[0].title, "更新后的工作表");
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(UpdateSheetsResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_request = UpdateSheetRequest::new()
            .sheet_id("test_sheet")
            .title("测试标题")
            .index(3)
            .hidden(true);

        let serialized = serde_json::to_string(&original_request).unwrap();
        let deserialized: UpdateSheetRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_request.sheet_id, deserialized.sheet_id);
        assert_eq!(original_request.title, deserialized.title);
        assert_eq!(original_request.index, deserialized.index);
        assert_eq!(original_request.hidden, deserialized.hidden);
    }

    #[test]
    fn test_endpoint_url_construction() {
        let spreadsheet_token = "test_spreadsheet_789";
        let expected_base_url = format!(
            "{}/sheets_batch_update",
            openlark_core::endpoints::Endpoints::SHEETS_V2_SPREADSHEETS
        );

        // 验证URL格式
        assert!(expected_base_url.contains("/sheets_batch_update"));
        assert!(!expected_base_url.contains(spreadsheet_token));

        // 实际请求URL会在执行时添加spreadsheetToken参数
        assert!(!expected_base_url.is_empty());
    }

    #[test]
    fn test_builder_patterns() {
        let config = openlark_core::config::openlark_core::config::Config::default();
        let service = SheetManagementService::new(config);

        // 测试单个更新构建器
        let update_builder = service
            .update_sheet_builder()
            .sheet_id("test_sheet")
            .title("测试标题")
            .index(0);

        assert_eq!(
            update_builder.request.sheet_id,
            Some("test_sheet".to_string())
        );
        assert_eq!(update_builder.request.title, Some("测试标题".to_string()));
        assert_eq!(update_builder.request.index, Some(0));
        assert!(!format!("{:?}", update_builder).is_empty());

        // 测试批量更新构建器
        let batch_builder = service
            .batch_update_builder("test_token")
            .add_request(UpdateSheetRequest::new().title("批量更新1"))
            .add_request(UpdateSheetRequest::new().title("批量更新2"));

        assert_eq!(batch_builder.batch_request.requests.len(), 2);
        assert!(!format!("{:?}", batch_builder).is_empty());
    }

    #[test]
    fn test_updated_sheet_creation() {
        let updated_sheet = UpdatedSheet {
            sheet_id: "sheet_abc".to_string(),
            title: "测试工作表".to_string(),
            index: 0,
            hidden: false,
            row_count: 1000,
            column_count: 50,
            spreadsheet_token: "spreadsheet_xyz".to_string(),
        };

        assert_eq!(updated_sheet.sheet_id, "sheet_abc");
        assert_eq!(updated_sheet.title, "测试工作表");
        assert_eq!(updated_sheet.index, 0);
        assert_eq!(updated_sheet.hidden, false);
        assert_eq!(updated_sheet.row_count, 1000);
        assert_eq!(updated_sheet.column_count, 50);
        assert_eq!(updated_sheet.spreadsheet_token, "spreadsheet_xyz");
    }

    #[test]
    fn test_boundary_values() {
        // 测试边界值
        let boundary_request = UpdateSheetRequest::new()
            .title("边界测试")
            .index(0)
            .row_count(1)
            .column_count(1);

        assert!(boundary_request.validate().is_ok());

        // 测试最大值
        let max_request = UpdateSheetRequest::new()
            .title(&"a".repeat(100)) // 刚好100个字符
            .index(1000)
            .row_count(100000)
            .column_count(1000);

        assert!(max_request.validate().is_ok());
    }
}
