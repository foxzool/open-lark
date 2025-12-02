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
    standard_response::Response,
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
