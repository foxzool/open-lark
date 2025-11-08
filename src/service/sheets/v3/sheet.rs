//! 工作表管理服务 v3
//!
//! 提供飞书电子表格v3版本的工作表管理功能，包括：
//! - 获取电子表格下的所有工作表
//! - 查询特定工作表的属性信息
//! - 工作表属性管理和操作

use crate::core::{
    api_resp::{ApiResponseTrait, ResponseFormat, BaseResponse},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};
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
#[derive(Debug, Clone)]
pub struct SheetService {
    transport: Transport,
}

impl SheetService {
    /// 创建工作表服务实例
    pub fn new(config: Config) -> Self {
        Self {
            transport: Transport::new(config, AccessTokenType::Tenant),
        }
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
    pub async fn query_sheets(&self, spreadsheet_token: &str) -> SDKResult<BaseResponse<QuerySheetsResponse>> {
        let endpoint = format!(
            "{}/{}/sheets/query",
            crate::core::endpoints_original::Endpoints::SHEETS_V3_SPREADSHEETS,
            spreadsheet_token
        );

        let request = ApiRequest {
            method: "GET".to_string(),
            url: endpoint,
            headers: vec![],
            params: vec![],
            body: None,
        };

        self.transport.request(&request).await
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
    ) -> SDKResult<BaseResponse<GetSheetResponse>> {
        let endpoint = format!(
            "{}/{}/sheets/{}",
            crate::core::endpoints_original::Endpoints::SHEETS_V3_SPREADSHEETS,
            spreadsheet_token,
            sheet_id
        );

        let request = ApiRequest {
            method: "GET".to_string(),
            url: endpoint,
            headers: vec![],
            params: vec![],
            body: None,
        };

        self.transport.request(&request).await
    }
}

/// 工作表查询构建器
#[derive(Debug, Clone)]
pub struct QuerySheetsBuilder {
    transport: Transport,
    spreadsheet_token: String,
}

impl QuerySheetsBuilder {
    /// 创建新的查询构建器实例
    pub fn new(transport: Transport, spreadsheet_token: &str) -> Self {
        Self {
            transport,
            spreadsheet_token: spreadsheet_token.to_string(),
        }
    }

    /// 执行查询请求
    pub async fn execute(self) -> SDKResult<BaseResponse<QuerySheetsResponse>> {
        let service = SheetService {
            transport: self.transport,
        };
        service.query_sheets(&self.spreadsheet_token).await
    }
}

/// 工作表获取构建器
#[derive(Debug, Clone)]
pub struct GetSheetBuilder {
    transport: Transport,
    spreadsheet_token: String,
    sheet_id: String,
}

impl GetSheetBuilder {
    /// 创建新的获取构建器实例
    pub fn new(transport: Transport, spreadsheet_token: &str, sheet_id: &str) -> Self {
        Self {
            transport,
            spreadsheet_token: spreadsheet_token.to_string(),
            sheet_id: sheet_id.to_string(),
        }
    }

    /// 执行获取请求
    pub async fn execute(self) -> SDKResult<BaseResponse<GetSheetResponse>> {
        let service = SheetService {
            transport: self.transport,
        };
        service.get_sheet(&self.spreadsheet_token, &self.sheet_id).await
    }
}

impl SheetService {
    /// 创建工作表查询构建器
    pub fn query_sheets_builder(&self, spreadsheet_token: &str) -> QuerySheetsBuilder {
        QuerySheetsBuilder::new(self.transport.clone(), spreadsheet_token)
    }

    /// 创建工作表获取构建器
    pub fn get_sheet_builder(&self, spreadsheet_token: &str, sheet_id: &str) -> GetSheetBuilder {
        GetSheetBuilder::new(self.transport.clone(), spreadsheet_token, sheet_id)
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sheet_service_creation() {
        let config = crate::core::config::Config::builder()
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
            ..Default::default()
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
            ..Default::default()
        };

        let response = QuerySheetsResponse {
            data: QuerySheetsData {
                sheet_items: vec![sheet],
            },
        };

        assert_eq!(response.data.sheet_items.len(), 1);
        assert_eq!(response.data.sheet_items[0].title, Some("财务数据".to_string()));
    }

    #[test]
    fn test_get_sheet_response_default() {
        let response = GetSheetResponse::default();
        assert_eq!(response.data.sheet.sheet_id, None);
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(
            QuerySheetsResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(
            GetSheetResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_sheet = Sheet {
            sheet_id: Some("test_sheet".to_string()),
            title: Some("测试工作表".to_string()),
            index: Some(0),
            row_count: Some(500),
            column_count: Some(20),
            ..Default::default()
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
            crate::core::endpoints_original::Endpoints::SHEETS_V3_SPREADSHEETS,
            spreadsheet_token
        );

        // 这里我们只验证格式，不实际调用API
        assert!(expected_query_url.contains("/sheets/query"));
        assert!(expected_query_url.contains(spreadsheet_token));

        // 测试获取单个工作表的URL构建
        let sheet_id = "test_sheet_456";
        let expected_get_url = format!(
            "{}/{}/sheets/{}",
            crate::core::endpoints_original::Endpoints::SHEETS_V3_SPREADSHEETS,
            spreadsheet_token,
            sheet_id
        );

        assert!(expected_get_url.contains("/sheets/"));
        assert!(expected_get_url.contains(spreadsheet_token));
        assert!(expected_get_url.contains(sheet_id));
    }

    #[test]
    fn test_builder_patterns() {
        let config = crate::core::config::Config::default();
        let service = SheetService::new(config);

        let query_builder = service.query_sheets_builder("test_token");
        assert!(!format!("{:?}", query_builder).is_empty());

        let get_builder = service.get_sheet_builder("test_token", "test_sheet");
        assert!(!format!("{:?}", get_builder).is_empty());
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