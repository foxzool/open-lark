//! 电子表格管理服务
//!
//! 提供飞书电子表格的创建、查询、管理等基础功能，包括：
//! - 创建新电子表格
//! - 获取电子表格信息
//! - 删除电子表格
//! - 电子表格权限管理

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

/// 电子表格信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Spreadsheet {
    /// 电子表格ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spreadsheet_id: Option<String>,
    /// 电子表格标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 电子表格URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 创建者信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<Creator>,
    /// 所在文件夹信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
}

impl Default for Spreadsheet {
    fn default() -> Self {
        Self {
            spreadsheet_id: None,
            title: None,
            url: None,
            create_time: None,
            update_time: None,
            creator: None,
            folder_token: None,
        }
    }
}

/// 创建者信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Creator {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 用户头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

impl Default for Creator {
    fn default() -> Self {
        Self {
            user_id: None,
            name: None,
            avatar: None,
        }
    }
}

/// 创建电子表格请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSpreadsheetRequest {
    /// 电子表格标题
    pub title: String,
    /// 所在文件夹token（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
}

impl CreateSpreadsheetRequest {
    /// 创建新的请求实例
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            folder_token: None,
        }
    }

    /// 设置所在文件夹
    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.folder_token = Some(folder_token.into());
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("标题不能为空".to_string());
        }
        if self.title.len() > 100 {
            return Err("标题长度不能超过100个字符".to_string());
        }
        Ok(())
    }
}

/// 创建电子表格响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateSpreadsheetResponse {
    /// 创建的电子表格信息
    pub spreadsheet: Spreadsheet,
}

impl ApiResponseTrait for CreateSpreadsheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 电子表格管理服务
#[derive(Clone, Debug)]
pub struct SpreadsheetService {
    config: Config,
}

impl SpreadsheetService {
    /// 创建电子表格管理服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v3::spreadsheet::SpreadsheetService;
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = SpreadsheetService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建电子表格
    ///
    /// 创建一个新的电子表格，支持指定标题和所在文件夹
    ///
    /// # 参数
    /// * `req` - 创建电子表格请求
    ///
    /// # 返回值
    /// 返回创建的电子表格信息
    pub async fn create(
        &self,
        req: &CreateSpreadsheetRequest,
    ) -> SDKResult<CreateSpreadsheetResponse> {
        req.validate()
            .map_err(|msg| LarkAPIError::IllegalParamError(msg))?;
        log::debug!("开始创建电子表格: title={:?}", req.title);

        let mut api_request = ApiRequest::with_method_and_path(
            Method::POST,
            openlark_core::endpoints::Endpoints::SHEETS_V3_SPREADSHEETS,
        );
        api_request.supported_access_token_types =
            vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_request.body = Some(openlark_core::api::RequestData::Json(req))?;

        let resp = Transport::<CreateSpreadsheetResponse>::request(api_request, &self.config, None)
            .await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "电子表格创建成功: title={}, spreadsheet_id={:?}",
            req.title,
            response.spreadsheet.spreadsheet_id
        );

        Ok(response)
    }
}

// ==================== 构建器模式 ====================

/// 创建电子表格构建器
#[derive(Clone, Debug)]
pub struct CreateSpreadsheetBuilder {
    request: CreateSpreadsheetRequest,
}

impl Default for CreateSpreadsheetBuilder {
    fn default() -> Self {
        Self {
            request: CreateSpreadsheetRequest {
                title: String::new(),
                folder_token: None,
            },
        }
    }
}

impl CreateSpreadsheetBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            request: CreateSpreadsheetRequest {
                title: String::new(),
                folder_token: None,
            },
        }
    }

    /// 设置电子表格标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.request.title = title.into();
        self
    }

    /// 设置所在文件夹token
    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.request.folder_token = Some(folder_token.into());
        self
    }

    /// 执行创建电子表格操作
    pub async fn execute(
        self,
        service: &SpreadsheetService,
    ) -> SDKResult<CreateSpreadsheetResponse> {
        service.create(&self.request).await
    }
}

impl SpreadsheetService {
    /// 创建电子表格构建器
    pub fn create_spreadsheet_builder(&self) -> CreateSpreadsheetBuilder {
        CreateSpreadsheetBuilder::new()
    }

    /// 获取电子表格信息
    ///
    /// 根据电子表格token获取电子表格的基础信息。
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格的token
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    ///
    /// let response = client.sheets.v3.spreadsheet
    ///     .get("spreadsheet_token")
    ///     .await?;
    /// ```
    pub async fn get(
        &self,
        spreadsheet_token: &str,
    ) -> SDKResult<Response<GetSpreadsheetResponse>> {
        let endpoint = format!(
            "{}/{}",
            openlark_core::endpoints::Endpoints::SHEETS_V3_SPREADSHEETS,
            spreadsheet_token
        );

        let mut api_request = ApiRequest::with_method_and_path(Method::GET, &endpoint);
        api_request.supported_access_token_types =
            vec![AccessTokenType::Tenant, AccessTokenType::User];

        Transport::<GetSpreadsheetResponse>::request(api_request, &self.config, None).await
    }

    /// 更新电子表格属性
    ///
    /// 根据电子表格token更新电子表格的属性。
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格的token
    /// - `request`: 更新请求
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    ///
    /// let request = UpdateSpreadsheetRequest::new("新标题")
    ///     .folder_token("new_folder_token");
    ///
    /// let response = client.sheets.v3.spreadsheet
    ///     .update("spreadsheet_token", &request)
    ///     .await?;
    /// ```
    pub async fn update(
        &self,
        spreadsheet_token: &str,
        request: &UpdateSpreadsheetRequest,
    ) -> SDKResult<Response<UpdateSpreadsheetResponse>> {
        let endpoint = format!(
            "{}/{}",
            openlark_core::endpoints::Endpoints::SHEETS_V3_SPREADSHEETS,
            spreadsheet_token
        );

        let mut api_request = ApiRequest::with_method_and_path(Method::PATCH, &endpoint);
        api_request.supported_access_token_types =
            vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_request.body = Some(openlark_core::api::RequestData::Json(request))?;

        Transport::<UpdateSpreadsheetResponse>::request(api_request, &self.config, None).await
    }
}

/// 更新电子表格请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSpreadsheetRequest {
    /// 电子表格标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 所在文件夹token（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
}

impl UpdateSpreadsheetRequest {
    /// 创建新的更新请求实例
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: Some(title.into()),
            folder_token: None,
        }
    }

    /// 设置文件夹token
    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.folder_token = Some(folder_token.into());
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if let Some(ref title) = self.title {
            if title.trim().is_empty() {
                return Err("电子表格标题不能为空".to_string());
            }
            if title.len() > 100 {
                return Err("电子表格标题长度不能超过100个字符".to_string());
            }
        }
        Ok(())
    }
}

/// 获取电子表格响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSpreadsheetResponse {
    /// 电子表格信息
    pub data: GetSpreadsheetData,
}

/// 获取电子表格数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSpreadsheetData {
    /// 电子表格信息
    pub spreadsheet: Spreadsheet,
}

impl Default for GetSpreadsheetResponse {
    fn default() -> Self {
        Self {
            data: GetSpreadsheetData {
                spreadsheet: Spreadsheet::default(),
            },
        }
    }
}

impl ApiResponseTrait for GetSpreadsheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新电子表格响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSpreadsheetResponse {
    /// 电子表格信息
    pub data: UpdateSpreadsheetData,
}

/// 更新电子表格数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSpreadsheetData {
    /// 电子表格信息
    pub spreadsheet: Spreadsheet,
}

impl Default for UpdateSpreadsheetResponse {
    fn default() -> Self {
        Self {
            data: UpdateSpreadsheetData {
                spreadsheet: Spreadsheet::default(),
            },
        }
    }
}

impl ApiResponseTrait for UpdateSpreadsheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新电子表格构建器
pub struct UpdateSpreadsheetBuilder {
    request: UpdateSpreadsheetRequest,
    transport: Transport<()>,
}

impl Clone for UpdateSpreadsheetBuilder {
    fn clone(&self) -> Self {
        Self {
            request: self.request.clone(),
            transport: Transport::new(), // 创建新的Transport实例
        }
    }
}

impl UpdateSpreadsheetBuilder {
    /// 创建新的构建器实例
    pub fn new(transport: Transport<()>) -> Self {
        Self {
            request: UpdateSpreadsheetRequest::new(""),
            transport,
        }
    }

    /// 设置电子表格标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.request.title = Some(title.into());
        self
    }

    /// 设置文件夹token
    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.request.folder_token = Some(folder_token.into());
        self
    }

    /// 执行更新请求
    pub async fn execute(
        self,
        service: &SpreadsheetService,
        spreadsheet_token: &str,
    ) -> SDKResult<Response<UpdateSpreadsheetResponse>> {
        self.request
            .validate()
            .map_err(|msg| LarkAPIError::IllegalParamError(msg))?;
        service.update(spreadsheet_token, &self.request).await
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spreadsheet_service_creation() {
        let config = openlark_core::config::Config::default();
        let service = SpreadsheetService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_spreadsheet_default_creation() {
        let spreadsheet = Spreadsheet::default();
        assert_eq!(spreadsheet.spreadsheet_id, None);
        assert_eq!(spreadsheet.title, None);
        assert_eq!(spreadsheet.url, None);
        assert_eq!(spreadsheet.create_time, None);
        assert_eq!(spreadsheet.update_time, None);
        assert_eq!(spreadsheet.creator, None);
        assert_eq!(spreadsheet.folder_token, None);
    }

    #[test]
    fn test_spreadsheet_with_data() {
        let creator = Creator {
            user_id: Some("user_123".to_string()),
            name: Some("张三".to_string()),
            avatar: Some("avatar_url".to_string()),
        };

        let spreadsheet = Spreadsheet {
            spreadsheet_id: Some("sheet_456".to_string()),
            title: Some("财务报表".to_string()),
            url: Some("https://example.com/sheet".to_string()),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-01-02T00:00:00Z".to_string()),
            creator: Some(creator),
            folder_token: Some("folder_789".to_string()),
        };

        assert_eq!(spreadsheet.spreadsheet_id, Some("sheet_456".to_string()));
        assert_eq!(spreadsheet.title, Some("财务报表".to_string()));
        assert_eq!(
            spreadsheet.url,
            Some("https://example.com/sheet".to_string())
        );
        assert_eq!(
            spreadsheet.creator.as_ref().unwrap().user_id,
            Some("user_123".to_string())
        );
        assert_eq!(
            spreadsheet.creator.as_ref().unwrap().name,
            Some("张三".to_string())
        );
        assert_eq!(spreadsheet.folder_token, Some("folder_789".to_string()));
    }

    #[test]
    fn test_creator_default_creation() {
        let creator = Creator::default();
        assert_eq!(creator.user_id, None);
        assert_eq!(creator.name, None);
        assert_eq!(creator.avatar, None);
    }

    #[test]
    fn test_create_spreadsheet_request() {
        let request = CreateSpreadsheetRequest::new("测试表格").folder_token("folder_token");

        assert_eq!(request.title, "测试表格");
        assert_eq!(request.folder_token, Some("folder_token".to_string()));
    }

    #[test]
    fn test_create_spreadsheet_request_validation() {
        // 测试正常情况
        let valid_request = CreateSpreadsheetRequest::new("有效标题");
        assert!(valid_request.validate().is_ok());

        // 测试空标题
        let empty_title_request = CreateSpreadsheetRequest::new("");
        assert!(empty_title_request.validate().is_err());

        // 测试空白标题
        let whitespace_title_request = CreateSpreadsheetRequest::new("   ");
        assert!(whitespace_title_request.validate().is_err());

        // 测试标题过长
        let long_title_request = CreateSpreadsheetRequest::new(&"a".repeat(101));
        assert!(long_title_request.validate().is_err());

        // 测试标题长度边界
        let boundary_title_request = CreateSpreadsheetRequest::new(&"a".repeat(100));
        assert!(boundary_title_request.validate().is_ok());
    }

    #[test]
    fn test_create_spreadsheet_builder() {
        let builder = CreateSpreadsheetBuilder::new()
            .title("构建器测试")
            .folder_token("test_folder");

        assert_eq!(builder.request.title, "构建器测试");
        assert_eq!(
            builder.request.folder_token,
            Some("test_folder".to_string())
        );
    }

    #[test]
    fn test_create_spreadsheet_builder_default() {
        let builder = CreateSpreadsheetBuilder::default();
        assert_eq!(builder.request.title, "");
        assert_eq!(builder.request.folder_token, None);
    }

    #[test]
    fn test_response_default_creation() {
        let response = CreateSpreadsheetResponse::default();
        assert_eq!(response.spreadsheet.spreadsheet_id, None);
        assert_eq!(response.spreadsheet.title, None);
    }

    #[test]
    fn test_response_with_data() {
        let mut response = CreateSpreadsheetResponse::default();
        response.spreadsheet = Spreadsheet {
            spreadsheet_id: Some("sheet_abc".to_string()),
            title: Some("响应测试".to_string()),
            
        };

        assert_eq!(
            response.spreadsheet.spreadsheet_id,
            Some("sheet_abc".to_string())
        );
        assert_eq!(response.spreadsheet.title, Some("响应测试".to_string()));
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(
            CreateSpreadsheetResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_request_serialization() {
        let request = CreateSpreadsheetRequest::new("序列化测试").folder_token("test_folder");

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: CreateSpreadsheetRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.title, deserialized.title);
        assert_eq!(request.folder_token, deserialized.folder_token);
    }

    #[test]
    fn test_endpoint_constant() {
        // Test that the endpoint constant is properly defined
        assert_eq!(
            openlark_core::endpoints::Endpoints::SHEETS_V3_SPREADSHEETS,
            "/open-apis/sheets/v3/spreadsheets"
        );
    }

    #[test]
    fn test_spreadsheet_title_variations() {
        // Test different spreadsheet titles
        let finance_sheet = Spreadsheet {
            title: Some("财务报表".to_string()),
            
        };

        let hr_sheet = Spreadsheet {
            title: Some("人力资源表".to_string()),
            
        };

        let project_sheet = Spreadsheet {
            title: Some("项目管理表".to_string()),
            
        };

        assert_eq!(finance_sheet.title, Some("财务报表".to_string()));
        assert_eq!(hr_sheet.title, Some("人力资源表".to_string()));
        assert_eq!(project_sheet.title, Some("项目管理表".to_string()));
    }

    #[test]
    fn test_comprehensive_spreadsheet_data() {
        // Test comprehensive spreadsheet data with all fields
        let comprehensive_creator = Creator {
            user_id: Some("creator_001".to_string()),
            name: Some("李四".to_string()),
            avatar: Some("https://example.com/avatar.jpg".to_string()),
        };

        let comprehensive_spreadsheet = Spreadsheet {
            spreadsheet_id: Some("comprehensive_sheet_001".to_string()),
            title: Some("2023年度预算表".to_string()),
            url: Some("https://docs.example.com/sheets/comprehensive_sheet_001".to_string()),
            create_time: Some("2023-01-01T08:00:00Z".to_string()),
            update_time: Some("2023-12-31T16:00:00Z".to_string()),
            creator: Some(comprehensive_creator),
            folder_token: Some("budget_folder_2023".to_string()),
        };

        assert_eq!(
            comprehensive_spreadsheet.spreadsheet_id,
            Some("comprehensive_sheet_001".to_string())
        );
        assert_eq!(
            comprehensive_spreadsheet.title,
            Some("2023年度预算表".to_string())
        );
        assert_eq!(
            comprehensive_spreadsheet.url,
            Some("https://docs.example.com/sheets/comprehensive_sheet_001".to_string())
        );
        assert_eq!(
            comprehensive_spreadsheet.create_time,
            Some("2023-01-01T08:00:00Z".to_string())
        );
        assert_eq!(
            comprehensive_spreadsheet.update_time,
            Some("2023-12-31T16:00:00Z".to_string())
        );
        assert_eq!(
            comprehensive_spreadsheet.creator.as_ref().unwrap().user_id,
            Some("creator_001".to_string())
        );
        assert_eq!(
            comprehensive_spreadsheet.creator.as_ref().unwrap().name,
            Some("李四".to_string())
        );
        assert_eq!(
            comprehensive_spreadsheet.folder_token,
            Some("budget_folder_2023".to_string())
        );
    }

    #[test]
    fn test_request_validation_edge_cases() {
        // Test with whitespace-only title
        let whitespace_request = CreateSpreadsheetRequest::new("  \t\n  ");
        assert!(whitespace_request.validate().is_err());

        // Test with special characters in title
        let special_chars_request = CreateSpreadsheetRequest::new("财务报表-Q1_2023.xlsx");
        assert!(special_chars_request.validate().is_ok());

        // Test with Unicode characters
        let unicode_request = CreateSpreadsheetRequest::new("📊 财务数据 📈");
        assert!(unicode_request.validate().is_ok());
    }
}
