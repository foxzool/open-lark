//! 表格元数据服务
//!
//! 提供飞书电子表格v2版本的表格元数据获取功能，包括：
//! - 获取电子表格的基本信息
//! - 查询工作表列表和属性
//! - 获取电子表格权限信息
//! - 查询创建和修改时间
//! - 获取电子表格版本和状态信息

use std::collections::HashMap;
use serde_json::Value;
use serde::{Deserialize, Serialize};

use openlark_core::endpoints::Endpoints;
use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    error::LarkAPIError,
    http::Transport,
    standard_response::StandardResponse,
    trait_system::Service,
    SDKResult,
};

/// 表格元数据信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpreadsheetMetaInfo {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 电子表格标题
    pub title: String,
    /// 所有者信息
    pub owner: OwnerInfo,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
    /// 修订时间
    pub revision_time: String,
    /// 电子表格URL
    pub spreadsheet_url: String,
    /// 工作表数量
    pub sheet_count: i32,
    /// 工作表列表
    pub sheets: Vec<SheetMetaInfo>,
    /// 权限信息
    pub permissions: Option<PermissionInfo>,
    /// 文件夹路径
    pub folder_path: Option<String>,
    /// 电子表格ID
    pub spreadsheet_id: Option<String>,
    /// 应用信息
    pub app: Option<AppInfo>,
    /// 是否为模板
    pub is_template: Option<bool>,
    /// 时区信息
    pub time_zone: Option<String>,
    /// 语言设置
    pub locale: Option<String>,
    /// 自定义属性
    pub custom_properties: Option<HashMap<String, Value>>,
}

/// 所有者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnerInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    pub name: String,
    /// 用户邮箱
    pub email: Option<String>,
    /// 头像URL
    pub avatar_url: Option<String>,
    /// 用户类型
    pub user_type: Option<String>,
}

/// 工作表元数据信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetMetaInfo {
    /// 工作表ID
    pub sheet_id: String,
    /// 工作表标题
    pub title: String,
    /// 工作表索引
    pub index: i32,
    /// 工作表类型
    pub sheet_type: String,
    /// 是否隐藏
    pub hidden: bool,
    /// 行数
    pub row_count: i32,
    /// 列数
    pub column_count: i32,
    /// 工作表URL
    pub sheet_url: String,
    /// 网格属性
    pub grid_properties: Option<GridProperties>,
    /// 工作表颜色
    pub sheet_color: Option<String>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
}

/// 网格属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridProperties {
    /// 冻结行数
    pub frozen_row_count: Option<i32>,
    /// 冻结列数
    pub frozen_column_count: Option<i32>,
    /// 是否隐藏网格线
    pub hide_gridlines: Option<bool>,
    /// 是否打印行列标题
    pub print_headings: Option<bool>,
}

/// 权限信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionInfo {
    /// 是否可编辑
    pub editable: bool,
    /// 是否可评论
    pub commentable: bool,
    /// 是否可分享
    pub shareable: bool,
    /// 权限类型
    pub permission_type: String,
    /// 角色信息
    pub role: Option<String>,
}

/// 应用信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo {
    /// 应用ID
    pub app_id: String,
    /// 应用名称
    pub app_name: String,
    /// 应用类型
    pub app_type: String,
}

/// 获取表格元数据请求
#[derive(Clone, Debug)]
pub struct GetSpreadsheetMetaRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 是否包含权限信息
    pub include_permissions: Option<bool>,
    /// 是否包含自定义属性
    pub include_custom_properties: Option<bool>,
    /// 语言设置
    pub language: Option<String>,
}

impl Default for GetSpreadsheetMetaRequest {
    fn default() -> Self {
        Self {
            spreadsheet_token: String::new(),
            include_permissions: Some(true),
            include_custom_properties: Some(false),
            language: Some("zh_CN".to_string()),
        }
    }
}

impl GetSpreadsheetMetaRequest {
    /// 创建新的获取表格元数据请求
    pub fn new(spreadsheet_token: impl Into<String>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            
        }
    }

    /// 设置是否包含权限信息
    pub fn include_permissions(mut self, include: bool) -> Self {
        self.include_permissions = Some(include);
        self
    }

    /// 设置是否包含自定义属性
    pub fn include_custom_properties(mut self, include: bool) -> Self {
        self.include_custom_properties = Some(include);
        self
    }

    /// 设置语言
    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> SDKResult<()> {
        // 验证电子表格token
        if self.spreadsheet_token.trim().is_empty() {
            return Err(LarkAPIError::InvalidParameter(
                "电子表格token不能为空".to_string(),
            ));
        }

        // 验证token长度（飞书token通常在10-100字符之间）
        let token_len = self.spreadsheet_token.len();
        if token_len < 10 || token_len > 100 {
            return Err(LarkAPIError::InvalidParameter(format!(
                "电子表格token长度无效，应在10-100字符之间，当前长度: {}",
                token_len
            )));
        }

        Ok(())
    }

    /// 构建查询参数
    pub fn build_query_params(&self) -> String {
        let mut params = vec![];

        if let Some(include_permissions) = self.include_permissions {
            params.push(format!("include_permissions={}", include_permissions));
        }

        if let Some(include_custom_properties) = self.include_custom_properties {
            params.push(format!(
                "include_custom_properties={}",
                include_custom_properties
            ));
        }

        if let Some(language) = &self.language {
            params.push(format!("language={}", urlencoding::encode(language)));
        }

        params.join("&")
    }
}

/// API响应体结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpreadsheetMetaResponseBody {
    /// 表格元数据
    pub data: SpreadsheetMetaInfo,
}

impl ApiResponseTrait for SpreadsheetMetaInfo {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for SpreadsheetMetaResponseBody {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 表格元数据服务
#[derive(Clone, Debug)]
pub struct SpreadsheetMetaService {
    config: Config,
}

impl SpreadsheetMetaService {
    /// 创建表格元数据服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取电子表格元数据
    ///
    /// # 参数
    /// - `request`: 获取表格元数据请求
    ///
    /// # 返回
    /// 表格元数据信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v2::metainfo::*;
    /// use open_lark::core::config::Config;
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = SpreadsheetMetaService::new(config);
    ///
    /// // 获取表格基本信息
    /// let request = GetSpreadsheetMetaRequest::new("spreadsheet_token")
    ///     .include_permissions(true)
    ///     .include_custom_properties(false)
    ///     .language("zh_CN");
    ///
    /// let meta_info = service.get_spreadsheet_meta(request).await?;
    /// println!("表格标题: {}", meta_info.title);
    /// println!("工作表数量: {}", meta_info.sheet_count);
    /// ```
    pub async fn get_spreadsheet_meta(
        &self,
        request: GetSpreadsheetMetaRequest,
    ) -> SDKResult<Response<SpreadsheetMetaInfo>> {
        // 验证请求参数
        request.validate()?;

        // 构建API请求
        let endpoint = format!(
            "{}/open-apis/sheets/v2/spreadsheets/{}/metainfo",
            self.config.base_url, request.spreadsheet_token
        );

        let mut api_req = ApiRequest::with_method(openlark_core::api::HttpMethod::Get);
        api_req.set_api_path(endpoint);
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

        // 添加查询参数
        let query_params = request.build_query_params();
        if !query_params.is_empty() {
            for param in query_params.split('&') {
                if let Some((key, value)) = param.split_once('=') {
                    api_req.query_params.insert(key, value);
                }
            }
        }

        // 发送请求
        Transport::<SpreadsheetMetaInfo>::request(api_req, &self.config, None).await
    }

    /// 创建表格元数据获取构建器
    pub fn get_spreadsheet_meta_builder(
        &self,
        spreadsheet_token: impl Into<String>,
    ) -> SpreadsheetMetaBuilder {
        SpreadsheetMetaBuilder::new(self.clone() spreadsheet_token)
    }

    /// 快速获取表格基本信息
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格token
    ///
    /// # 返回
    /// 表格元数据信息
    pub async fn get_basic_meta(
        &self,
        spreadsheet_token: impl Into<String>,
    ) -> SDKResult<Response<SpreadsheetMetaInfo>> {
        let request = GetSpreadsheetMetaRequest::new(spreadsheet_token)
            .include_permissions(false)
            .include_custom_properties(false);

        self.get_spreadsheet_meta(request).await
    }

    /// 快速获取表格完整信息
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格token
    ///
    /// # 返回
    /// 包含权限和自定义属性的表格元数据信息
    pub async fn get_full_meta(
        &self,
        spreadsheet_token: impl Into<String>,
    ) -> SDKResult<Response<SpreadsheetMetaInfo>> {
        let request = GetSpreadsheetMetaRequest::new(spreadsheet_token)
            .include_permissions(true)
            .include_custom_properties(true);

        self.get_spreadsheet_meta(request).await
    }
}

impl Service for SpreadsheetMetaService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "SpreadsheetMetaService"
    }
}

/// 表格元数据获取构建器
pub struct SpreadsheetMetaBuilder {
    service: SpreadsheetMetaService,
    spreadsheet_token: String,
    include_permissions: Option<bool>,
    include_custom_properties: Option<bool>,
    language: Option<String>,
}

impl SpreadsheetMetaBuilder {
    /// 创建新的构建器
    pub fn new(service: SpreadsheetMetaService, spreadsheet_token: impl Into<String>) -> Self {
        Self {
            service,
            spreadsheet_token: spreadsheet_token.into(),
            include_permissions: Some(true),
            include_custom_properties: Some(false),
            language: Some("zh_CN".to_string()),
        }
    }

    /// 设置是否包含权限信息
    pub fn include_permissions(mut self, include: bool) -> Self {
        self.include_permissions = Some(include);
        self
    }

    /// 设置是否包含自定义属性
    pub fn include_custom_properties(mut self, include: bool) -> Self {
        self.include_custom_properties = Some(include);
        self
    }

    /// 设置语言
    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// 执行获取操作
    pub async fn execute(self) -> SDKResult<Response<SpreadsheetMetaInfo>> {
        let mut request = GetSpreadsheetMetaRequest::new(self.spreadsheet_token);

        if let Some(include_permissions) = self.include_permissions {
            request = request.include_permissions(include_permissions);
        }

        if let Some(include_custom_properties) = self.include_custom_properties {
            request = request.include_custom_properties(include_custom_properties);
        }

        if let Some(language) = self.language {
            request = request.language(language);
        }

        self.service.get_spreadsheet_meta(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_spreadsheet_meta_request_validation() {
        // 测试空token
        let request = GetSpreadsheetMetaRequest::new("");
        assert!(request.validate().is_err());

        // 测试正常请求
        let request = GetSpreadsheetMetaRequest::new("shtcnmBRWQKbsJRHXXXXXXXXXX");
        assert!(request.validate().is_ok());

        // 测试token过短
        let request = GetSpreadsheetMetaRequest::new("short");
        assert!(request.validate().is_err());

        // 测试token过长
        let long_token = "a".repeat(101);
        let request = GetSpreadsheetMetaRequest::new(long_token);
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_build_query_params() {
        let request = GetSpreadsheetMetaRequest::new("token")
            .include_permissions(true)
            .include_custom_properties(false)
            .language("zh_CN");

        let params = request.build_query_params();
        assert!(params.contains("include_permissions=true"));
        assert!(params.contains("include_custom_properties=false"));
        assert!(params.contains("language=zh_CN"));
    }

    #[test]
    fn test_builder_pattern() {
        let config = openlark_core::config::Config::default();
        let service = SpreadsheetMetaService::new(config);

        let builder = service
            .get_spreadsheet_meta_builder("test_token")
            .include_permissions(true)
            .include_custom_properties(true)
            .language("en_US");

        // 验证构建器设置
        assert_eq!(builder.spreadsheet_token, "test_token");
        assert_eq!(builder.include_permissions, Some(true));
        assert_eq!(builder.include_custom_properties, Some(true));
        assert_eq!(builder.language.as_ref().unwrap(), "en_US");
    }

    #[test]
    fn test_spreadsheet_meta_service() {
        let config = openlark_core::config::Config::default();
        let service = SpreadsheetMetaService::new(config);

        assert_eq!(service.service_name(), "SpreadsheetMetaService");
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_sheet_meta_info_structure() {
        let sheet_meta = SheetMetaInfo {
            sheet_id: "sheet1".to_string(),
            title: "工作表1".to_string(),
            index: 0,
            sheet_type: "GRID".to_string(),
            hidden: false,
            row_count: 1000,
            column_count: 26,
            sheet_url: "https://example.com".to_string(),
            grid_properties: Some(GridProperties {
                frozen_row_count: Some(1),
                frozen_column_count: Some(0),
                hide_gridlines: Some(false),
                print_headings: Some(true),
            }),
            sheet_color: Some("#FF0000".to_string()),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-01-02T00:00:00Z".to_string()),
        };

        assert_eq!(sheet_meta.sheet_id, "sheet1");
        assert_eq!(sheet_meta.title, "工作表1");
        assert_eq!(sheet_meta.index, 0);
        assert!(!sheet_meta.hidden);
        assert_eq!(sheet_meta.row_count, 1000);
        assert_eq!(sheet_meta.column_count, 26);
    }

    #[test]
    fn test_owner_info_structure() {
        let owner = OwnerInfo {
            user_id: "user_123".to_string(),
            name: "张三".to_string(),
            email: Some("zhangsan@example.com".to_string()),
            avatar_url: Some("https://example.com/avatar.jpg".to_string()),
            user_type: Some("user".to_string()),
        };

        assert_eq!(owner.user_id, "user_123");
        assert_eq!(owner.name, "张三");
        assert_eq!(owner.email.as_ref().unwrap(), "zhangsan@example.com");
    }

    #[test]
    fn test_permission_info_structure() {
        let permission = PermissionInfo {
            editable: true,
            commentable: true,
            shareable: false,
            permission_type: "owner".to_string(),
            role: Some("owner".to_string()),
        };

        assert!(permission.editable);
        assert!(permission.commentable);
        assert!(!permission.shareable);
        assert_eq!(permission.permission_type, "owner");
    }
}
