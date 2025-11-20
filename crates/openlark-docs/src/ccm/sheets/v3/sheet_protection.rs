//! 工作表保护服务
//!
//! 提供电子表格工作表的安全保护功能，包括：
//! - 创建和删除工作表保护
//! - 设置保护范围和权限
//! - 管理保护条件和编辑权限
//! - 查询和更新工作表保护状态

use std::collections::HashMap;
use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    http::Transport,
};

use serde_json;

use reqwest::Method;
use serde::{Deserialize, Serialize};

use openlark_core::trait_system::Service;

// v3模块核心类型定义
pub type SpreadsheetToken = String;
pub type SheetId = String;
pub type CellValue = serde_json::Value;
pub type SheetPagedResponse<T> = Vec<T>;

/// 工作表保护服务
///
/// 提供电子表格工作表保护管理功能，支持工作表级别的安全控制。
/// 通过设置保护范围和编辑权限，实现对工作表内容的精细化访问控制。
///
/// # 功能特性
///
/// ## 保护范围管理
/// - **整工作表保护**: 对整个工作表设置保护
/// - **区域保护**: 对指定单元格范围设置保护
/// - **条件保护**: 基于单元格内容条件设置保护
///
/// ## 权限控制
/// - **编辑权限**: 控制特定用户的编辑权限
/// - **只读模式**: 设置工作表为只读状态
/// - **管理员权限**: 设置保护管理员角色
///
/// # 常见应用场景
///
/// ```rust
/// # use open_lark::prelude::*;
/// # use open_lark::service::sheets::v3::SheetProtectionService;
/// # use open_lark::service::sheets::v3::models::spreadsheet::SpreadsheetToken;
/// # use open_lark::service::sheets::v3::models::sheet::SheetId;
///
/// let service = SheetProtectionService::new(client_config);
///
/// // 场景1: 保护整个工作表，只允许特定用户编辑
/// let request = CreateSheetProtectionRequest::builder()
///     .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
///     .sheet_id("0XXXXXXXXXX".to_string())
///     .description("财务数据保护".to_string())
///     .add_editor("ou_1", "user_id")
///     .build()?;
///
/// let protection = service.create(&request).await?;
///
/// // 场景2: 保护重要数据区域
/// let request = CreateSheetProtectionRequest::builder()
///     .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
///     .sheet_id("0XXXXXXXXXX".to_string())
///     .description("关键公式区域保护".to_string())
///     .add_range(SheetProtectionRange::builder()
///         .start_row_index(0)
///         .start_column_index(0)
///         .end_row_index(100)
///         .end_column_index(10)
///         .build()?)
///     .set_warning_only(true)
///     .build()?;
///
/// let protection = service.create(&request).await?;
///
/// // 场景3: 设置条件保护
/// let condition = SheetProtectionCondition::builder()
///     .condition_type("CELL_CONTAINS".to_string())
///     .content("公式".to_string())
///     .build()?;
///
/// let request = CreateSheetProtectionRequest::builder()
///     .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
///     .sheet_id("0XXXXXXXXXX".to_string())
///     .description("公式保护".to_string())
///     .add_condition(condition)
///     .build()?;
///
/// let protection = service.create(&request).await?;
/// ```
#[derive(Clone, Debug)]
pub struct SheetProtectionService {
    config: openlark_core::config::Config,
}

/// 工作表保护信息
///
/// 表示一个工作表保护配置的完整信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetProtection {
    /// 保护ID
    #[serde(rename = "protectionId")]
    pub protection_id: String,
    /// 电子表格Token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: SpreadsheetToken,
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: SheetId,
    /// 创建者信息
    pub creator: SheetProtectionCreator,
    /// 创建时间戳
    #[serde(rename = "createTime")]
    pub create_time: i64,
    /// 保护描述
    pub description: Option<String>,
    /// 保护范围列表
    pub ranges: Vec<SheetProtectionRange>,
    /// 保护条件列表
    pub conditions: Vec<SheetProtectionCondition>,
    /// 权限配置
    pub permissions: SheetProtectionPermissions,
    /// 保护类型
    #[serde(rename = "protectionType")]
    pub protection_type: String,
    /// 是否仅警告
    #[serde(rename = "warningOnly")]
    pub warning_only: bool,
    /// 保护状态
    pub status: String,
}

/// 保护创建者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetProtectionCreator {
    /// 用户ID
    #[serde(rename = "userId")]
    pub user_id: String,
    /// 用户类型
    #[serde(rename = "userType")]
    pub user_type: String,
    /// 用户名
    pub name: Option<String>,
    /// 用户邮箱
    pub email: Option<String>,
}

/// 保护范围
///
/// 定义保护生效的单元格范围。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetProtectionRange {
    /// 起始行索引（从0开始）
    #[serde(rename = "startRowIndex", skip_serializing_if = "Option::is_none")]
    pub start_row_index: Option<i32>,
    /// 结束行索引（从0开始）
    #[serde(rename = "endRowIndex", skip_serializing_if = "Option::is_none")]
    pub end_row_index: Option<i32>,
    /// 起始列索引（从0开始）
    #[serde(rename = "startColumnIndex", skip_serializing_if = "Option::is_none")]
    pub start_column_index: Option<i32>,
    /// 结束列索引（从0开始）
    #[serde(rename = "endColumnIndex", skip_serializing_if = "Option::is_none")]
    pub end_column_index: Option<i32>,
    /// 范围表示（如"A1:D10"）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
}

impl SheetProtectionRange {
    /// 创建保护范围构建器
    pub fn builder() -> SheetProtectionRangeBuilder {
        SheetProtectionRangeBuilder::new()
    }
}

/// 保护范围构建器
pub struct SheetProtectionRangeBuilder {
    start_row_index: Option<i32>,
    end_row_index: Option<i32>,
    start_column_index: Option<i32>,
    end_column_index: Option<i32>,
    range: Option<String>,
}

impl SheetProtectionRangeBuilder {
    /// 创建新的保护范围构建器
    pub fn new() -> Self {
        Self {
            start_row_index: None,
            end_row_index: None,
            start_column_index: None,
            end_column_index: None,
            range: None,
        }
    }

    /// 设置起始行索引
    pub fn start_row_index(mut self, start_row_index: i32) -> Self {
        self.start_row_index = Some(start_row_index);
        self
    }

    /// 设置结束行索引
    pub fn end_row_index(mut self, end_row_index: i32) -> Self {
        self.end_row_index = Some(end_row_index);
        self
    }

    /// 设置起始列索引
    pub fn start_column_index(mut self, start_column_index: i32) -> Self {
        self.start_column_index = Some(start_column_index);
        self
    }

    /// 设置结束列索引
    pub fn end_column_index(mut self, end_column_index: i32) -> Self {
        self.end_column_index = Some(end_column_index);
        self
    }

    /// 设置范围表示
    pub fn range(mut self, range: String) -> Self {
        self.range = Some(range);
        self
    }

    /// 构建保护范围对象
    pub fn build(self) -> openlark_core::error::SDKResult<SheetProtectionRange> {
        // 验证范围参数
        if let (Some(start_row), Some(end_row)) = (self.start_row_index, self.end_row_index) {
            if start_row > end_row {
                return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                    "起始行索引不能大于结束行索引".to_string(),
                ));
            }
        }

        if let (Some(start_col), Some(end_col)) = (self.start_column_index, self.end_column_index) {
            if start_col > end_col {
                return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                    "起始列索引不能大于结束列索引".to_string(),
                ));
            }
        }

        Ok(SheetProtectionRange {
            start_row_index: self.start_row_index,
            end_row_index: self.end_row_index,
            start_column_index: self.start_column_index,
            end_column_index: self.end_column_index,
            range: self.range,
        })
    }
}

/// 保护条件
///
///定义基于单元格内容的保护条件。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetProtectionCondition {
    /// 条件类型
    #[serde(rename = "conditionType")]
    pub condition_type: String,
    /// 条件内容
    pub content: String,
    /// 条件参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, serde_json::Value>>,
    /// 条件描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl SheetProtectionCondition {
    /// 创建保护条件构建器
    pub fn builder() -> SheetProtectionConditionBuilder {
        SheetProtectionConditionBuilder::new()
    }
}

/// 保护条件构建器
pub struct SheetProtectionConditionBuilder {
    condition_type: Option<String>,
    content: Option<String>,
    parameters: Option<HashMap<String, serde_json::Value>>,
    description: Option<String>,
}

impl SheetProtectionConditionBuilder {
    /// 创建新的保护条件构建器
    pub fn new() -> Self {
        Self {
            condition_type: None,
            content: None,
            parameters: None,
            description: None,
        }
    }

    /// 设置条件类型
    pub fn condition_type(mut self, condition_type: String) -> Self {
        self.condition_type = Some(condition_type);
        self
    }

    /// 设置条件内容
    pub fn content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    /// 设置条件参数
    pub fn parameters(mut self, parameters: HashMap<String, serde_json::Value>) -> Self {
        self.parameters = Some(parameters);
        self
    }

    /// 添加条件参数
    pub fn add_parameter(mut self, key: String, value: serde_json::Value) -> Self {
        let mut params = self.parameters.unwrap_or_default();
        params.insert(key, value);
        self.parameters = Some(params);
        self
    }

    /// 设置条件描述
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 构建保护条件对象
    pub fn build(self) -> openlark_core::error::SDKResult<SheetProtectionCondition> {
        match (self.condition_type, self.content) {
            (Some(condition_type), Some(content)) => {
                if condition_type.is_empty() {
                    return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                        "条件类型不能为空".to_string(),
                    ));
                }
                if content.is_empty() {
                    return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                        "条件内容不能为空".to_string(),
                    ));
                }

                Ok(SheetProtectionCondition {
                    condition_type,
                    content,
                    parameters: self.parameters,
                    description: self.description,
                })
            }
            _ => Err(openlark_core::error::LarkAPIError::IllegalParamError(
                "条件类型和内容都是必需的".to_string(),
            )),
        }
    }
}

/// 保护权限配置
///
/// 定义工作表保护的权限规则。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetProtectionPermissions {
    /// 编辑者列表
    pub editors: Vec<SheetProtectionEditor>,
    /// 是否允许所有用户编辑
    #[serde(rename = "allowAllUsers")]
    pub allow_all_users: bool,
    /// 是否允许评论
    #[serde(rename = "allowComments")]
    pub allow_comments: bool,
}

/// 编辑者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetProtectionEditor {
    /// 用户ID
    #[serde(rename = "userId")]
    pub user_id: String,
    /// 用户类型
    #[serde(rename = "userType")]
    pub user_type: String,
    /// 用户名
    pub name: Option<String>,
    /// 用户邮箱
    pub email: Option<String>,
}

/// 创建工作表保护请求
///
/// 用于创建新的工作表保护配置。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSheetProtectionRequest {
    /// 电子表格Token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: SpreadsheetToken,
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: SheetId,
    /// 保护描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 保护范围列表
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ranges: Vec<SheetProtectionRange>,
    /// 保护条件列表
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<SheetProtectionCondition>,
    /// 权限配置
    pub permissions: SheetProtectionPermissions,
    /// 保护类型
    #[serde(rename = "protectionType", skip_serializing_if = "Option::is_none")]
    pub protection_type: Option<String>,
    /// 是否仅警告
    #[serde(rename = "warningOnly", skip_serializing_if = "Option::is_none")]
    pub warning_only: Option<bool>,
}

impl CreateSheetProtectionRequest {
    /// 创建请求构建器
    pub fn builder() -> CreateSheetProtectionRequestBuilder {
        CreateSheetProtectionRequestBuilder::new()
    }
}

/// 创建工作表保护请求构建器
pub struct CreateSheetProtectionRequestBuilder {
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
    description: Option<String>,
    ranges: Vec<SheetProtectionRange>,
    conditions: Vec<SheetProtectionCondition>,
    permissions: Option<SheetProtectionPermissions>,
    protection_type: Option<String>,
    warning_only: Option<bool>,
}

impl CreateSheetProtectionRequestBuilder {
    /// 创建新的请求构建器
    pub fn new() -> Self {
        Self {
            spreadsheet_token: None,
            sheet_id: None,
            description: None,
            ranges: vec![],
            conditions: vec![],
            permissions: None,
            protection_type: None,
            warning_only: None,
        }
    }

    /// 设置电子表格Token
    pub fn spreadsheet_token(mut self, spreadsheet_token: String) -> Self {
        self.spreadsheet_token = Some(SpreadsheetToken::from(spreadsheet_token));
        self
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, sheet_id: String) -> Self {
        self.sheet_id = Some(SheetId::from(sheet_id));
        self
    }

    /// 设置保护描述
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 添加保护范围
    pub fn add_range(mut self, range: SheetProtectionRange) -> Self {
        self.ranges.push(range);
        self
    }

    /// 添加保护条件
    pub fn add_condition(mut self, condition: SheetProtectionCondition) -> Self {
        self.conditions.push(condition);
        self
    }

    /// 设置权限配置
    pub fn permissions(mut self, permissions: SheetProtectionPermissions) -> Self {
        self.permissions = Some(permissions);
        self
    }

    /// 设置保护类型
    pub fn protection_type(mut self, protection_type: String) -> Self {
        self.protection_type = Some(protection_type);
        self
    }

    /// 设置是否仅警告
    pub fn warning_only(mut self, warning_only: bool) -> Self {
        self.warning_only = Some(warning_only);
        self
    }

    /// 添加编辑者
    pub fn add_editor(mut self, user_id: &str, user_type: &str) -> Self {
        let permissions = self
            .permissions
            .get_or_insert_with(|| SheetProtectionPermissions {
                editors: vec![],
                allow_all_users: false,
                allow_comments: false,
            });

        permissions.editors.push(SheetProtectionEditor {
            user_id: user_id.to_string(),
            user_type: user_type.to_string(),
            name: None,
            email: None,
        });

        self
    }

    /// 构建请求对象
    pub fn build(self) -> openlark_core::error::SDKResult<CreateSheetProtectionRequest> {
        match (self.spreadsheet_token, self.sheet_id) {
            (Some(spreadsheet_token), Some(sheet_id)) => {
                let permissions = self
                    .permissions
                    .unwrap_or_else(|| SheetProtectionPermissions {
                        editors: vec![],
                        allow_all_users: false,
                        allow_comments: false,
                    });

                Ok(CreateSheetProtectionRequest {
                    spreadsheet_token,
                    sheet_id,
                    description: self.description,
                    ranges: self.ranges,
                    conditions: self.conditions,
                    permissions,
                    protection_type: self.protection_type,
                    warning_only: self.warning_only,
                })
            }
            _ => Err(openlark_core::error::LarkAPIError::IllegalParamError(
                "电子表格Token和工作表ID都是必需的".to_string(),
            )),
        }
    }
}

/// 创建工作表保护响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSheetProtectionResponse {
    /// 保护信息
    pub data: SheetProtection,
}

/// 查询工作表保护请求
///
/// 用于查询工作表的保护配置列表。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuerySheetProtectionRequest {
    /// 电子表格Token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: SpreadsheetToken,
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: SheetId,
    /// 保护ID列表（可选，用于查询特定保护）
    #[serde(rename = "protectionIds", skip_serializing_if = "Vec::is_empty")]
    pub protection_ids: Vec<String>,
    /// 页大小
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页令牌
    #[serde(rename = "pageToken", skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl QuerySheetProtectionRequest {
    /// 创建查询请求构建器
    pub fn builder() -> QuerySheetProtectionRequestBuilder {
        QuerySheetProtectionRequestBuilder::new()
    }
}

/// 查询工作表保护请求构建器
pub struct QuerySheetProtectionRequestBuilder {
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
    protection_ids: Vec<String>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl QuerySheetProtectionRequestBuilder {
    /// 创建新的查询请求构建器
    pub fn new() -> Self {
        Self {
            spreadsheet_token: None,
            sheet_id: None,
            protection_ids: vec![],
            page_size: None,
            page_token: None,
        }
    }

    /// 设置电子表格Token
    pub fn spreadsheet_token(mut self, spreadsheet_token: String) -> Self {
        self.spreadsheet_token = Some(SpreadsheetToken::from(spreadsheet_token));
        self
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, sheet_id: String) -> Self {
        self.sheet_id = Some(SheetId::from(sheet_id));
        self
    }

    /// 添加保护ID
    pub fn add_protection_id(mut self, protection_id: String) -> Self {
        self.protection_ids.push(protection_id);
        self
    }

    /// 设置保护ID列表
    pub fn protection_ids(mut self, protection_ids: Vec<String>) -> Self {
        self.protection_ids = protection_ids;
        self
    }

    /// 设置页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置页令牌
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 构建查询请求
    pub fn build(self) -> openlark_core::error::SDKResult<QuerySheetProtectionRequest> {
        match (self.spreadsheet_token, self.sheet_id) {
            (Some(spreadsheet_token), Some(sheet_id)) => Ok(QuerySheetProtectionRequest {
                spreadsheet_token,
                sheet_id,
                protection_ids: self.protection_ids,
                page_size: self.page_size,
                page_token: self.page_token,
            }),
            _ => Err(openlark_core::error::LarkAPIError::IllegalParamError(
                "电子表格Token和工作表ID都是必需的".to_string(),
            )),
        }
    }
}

/// 查询工作表保护响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuerySheetProtectionResponse {
    /// 保护列表数据
    pub data: SheetPagedResponse<SheetProtection>,
}

/// 更新工作表保护请求
///
/// 用于更新现有的工作表保护配置。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSheetProtectionRequest {
    /// 电子表格Token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: SpreadsheetToken,
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: SheetId,
    /// 保护ID
    #[serde(rename = "protectionId")]
    pub protection_id: String,
    /// 更新字段
    pub fields: Vec<String>,
    /// 保护描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 保护范围列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranges: Option<Vec<SheetProtectionRange>>,
    /// 保护条件列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<SheetProtectionCondition>>,
    /// 权限配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<SheetProtectionPermissions>,
    /// 是否仅警告
    #[serde(rename = "warningOnly", skip_serializing_if = "Option::is_none")]
    pub warning_only: Option<bool>,
}

impl UpdateSheetProtectionRequest {
    /// 创建更新请求构建器
    pub fn builder() -> UpdateSheetProtectionRequestBuilder {
        UpdateSheetProtectionRequestBuilder::new()
    }
}

/// 更新工作表保护请求构建器
pub struct UpdateSheetProtectionRequestBuilder {
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
    protection_id: Option<String>,
    fields: Vec<String>,
    description: Option<String>,
    ranges: Option<Vec<SheetProtectionRange>>,
    conditions: Option<Vec<SheetProtectionCondition>>,
    permissions: Option<SheetProtectionPermissions>,
    warning_only: Option<bool>,
}

impl UpdateSheetProtectionRequestBuilder {
    /// 创建新的更新请求构建器
    pub fn new() -> Self {
        Self {
            spreadsheet_token: None,
            sheet_id: None,
            protection_id: None,
            fields: vec![],
            description: None,
            ranges: None,
            conditions: None,
            permissions: None,
            warning_only: None,
        }
    }

    /// 设置电子表格Token
    pub fn spreadsheet_token(mut self, spreadsheet_token: String) -> Self {
        self.spreadsheet_token = Some(SpreadsheetToken::from(spreadsheet_token));
        self
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, sheet_id: String) -> Self {
        self.sheet_id = Some(SheetId::from(sheet_id));
        self
    }

    /// 设置保护ID
    pub fn protection_id(mut self, protection_id: String) -> Self {
        self.protection_id = Some(protection_id);
        self
    }

    /// 添加更新字段
    pub fn add_field(mut self, field: String) -> Self {
        self.fields.push(field);
        self
    }

    /// 设置更新字段列表
    pub fn fields(mut self, fields: Vec<String>) -> Self {
        self.fields = fields;
        self
    }

    /// 设置保护描述
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 设置保护范围列表
    pub fn ranges(mut self, ranges: Vec<SheetProtectionRange>) -> Self {
        self.ranges = Some(ranges);
        self
    }

    /// 设置保护条件列表
    pub fn conditions(mut self, conditions: Vec<SheetProtectionCondition>) -> Self {
        self.conditions = Some(conditions);
        self
    }

    /// 设置权限配置
    pub fn permissions(mut self, permissions: SheetProtectionPermissions) -> Self {
        self.permissions = Some(permissions);
        self
    }

    /// 设置是否仅警告
    pub fn warning_only(mut self, warning_only: bool) -> Self {
        self.warning_only = Some(warning_only);
        self
    }

    /// 构建更新请求
    pub fn build(self) -> openlark_core::error::SDKResult<UpdateSheetProtectionRequest> {
        match (self.spreadsheet_token, self.sheet_id, self.protection_id) {
            (Some(spreadsheet_token), Some(sheet_id), Some(protection_id)) => {
                if self.fields.is_empty() {
                    return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                        "至少需要指定一个更新字段".to_string(),
                    ));
                }

                Ok(UpdateSheetProtectionRequest {
                    spreadsheet_token,
                    sheet_id,
                    protection_id,
                    fields: self.fields,
                    description: self.description,
                    ranges: self.ranges,
                    conditions: self.conditions,
                    permissions: self.permissions,
                    warning_only: self.warning_only,
                })
            }
            _ => Err(openlark_core::error::LarkAPIError::IllegalParamError(
                "电子表格Token、工作表ID和保护ID都是必需的".to_string(),
            )),
        }
    }
}

/// 更新工作表保护响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSheetProtectionResponse {
    /// 保护信息
    pub data: SheetProtection,
}

/// 删除工作表保护请求
///
/// 用于删除指定的工作表保护配置。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSheetProtectionRequest {
    /// 电子表格Token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: SpreadsheetToken,
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: SheetId,
    /// 保护ID
    #[serde(rename = "protectionId")]
    pub protection_id: String,
}

impl DeleteSheetProtectionRequest {
    /// 创建删除请求构建器
    pub fn builder() -> DeleteSheetProtectionRequestBuilder {
        DeleteSheetProtectionRequestBuilder::new()
    }
}

/// 删除工作表保护请求构建器
pub struct DeleteSheetProtectionRequestBuilder {
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
    protection_id: Option<String>,
}

impl DeleteSheetProtectionRequestBuilder {
    /// 创建新的删除请求构建器
    pub fn new() -> Self {
        Self {
            spreadsheet_token: None,
            sheet_id: None,
            protection_id: None,
        }
    }

    /// 设置电子表格Token
    pub fn spreadsheet_token(mut self, spreadsheet_token: String) -> Self {
        self.spreadsheet_token = Some(SpreadsheetToken::from(spreadsheet_token));
        self
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, sheet_id: String) -> Self {
        self.sheet_id = Some(SheetId::from(sheet_id));
        self
    }

    /// 设置保护ID
    pub fn protection_id(mut self, protection_id: String) -> Self {
        self.protection_id = Some(protection_id);
        self
    }

    /// 构建删除请求
    pub fn build(self) -> openlark_core::error::SDKResult<DeleteSheetProtectionRequest> {
        match (self.spreadsheet_token, self.sheet_id, self.protection_id) {
            (Some(spreadsheet_token), Some(sheet_id), Some(protection_id)) => {
                Ok(DeleteSheetProtectionRequest {
                    spreadsheet_token,
                    sheet_id,
                    protection_id,
                })
            }
            _ => Err(openlark_core::error::LarkAPIError::IllegalParamError(
                "电子表格Token、工作表ID和保护ID都是必需的".to_string(),
            )),
        }
    }
}

/// 删除工作表保护响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSheetProtectionResponse {
    /// 是否成功
    pub data: SheetPagedResponse<serde_json::Value>,
}

impl SheetProtectionService {
    /// 创建工作表保护服务实例
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self { config }
    }

    /// 创建工作表保护
    ///
    /// 在指定的工作表上创建保护配置，实现对工作表内容的访问控制。
    ///
    /// # 参数
    /// - `request`: 创建工作表保护请求，包含保护配置信息
    ///
    /// # 返回
    /// 返回创建成功的保护配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v3::{SheetProtectionService, CreateSheetProtectionRequest};
    ///
    /// let service = SheetProtectionService::new(client_config);
    ///
    /// // 创建整工作表保护
    /// let request = CreateSheetProtectionRequest::builder()
    ///     .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
    ///     .sheet_id("0XXXXXXXXXX".to_string())
    ///     .description("整个工作表保护".to_string())
    ///     .add_editor("user_id_123", "user_id")
    ///     .warning_only(false)
    ///     .build()?;
    ///
    /// let protection = service.create(&request).await?;
    /// println!("创建保护成功: {}", protection.data.protection_id);
    /// ```
    ///
    /// # 错误处理
    /// - 如果电子表格不存在，返回相应错误
    /// - 如果工作表不存在，返回相应错误
    /// - 如果权限不足，返回授权错误
    /// - 如果保护配置冲突，返回相应错误
    pub async fn create(
        &self,
        request: &CreateSheetProtectionRequest,
    ) -> openlark_core::error::SDKResult<Response<CreateSheetProtectionResponse>> {
        let url = format!(
            "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/protections",
            self.config.base_url,
            request.spreadsheet_token.as_str(),
            request.sheet_id.as_str()
        );

        let mut api_request = ApiRequest::with_method_and_path(Method::POST, &url);
        api_request.body = Some(openlark_core::api::RequestData::Json(request))?;

        let base_resp =
            Transport::<CreateSheetProtectionResponse>::request(api_request, &self.config, None)
                .await?;

        Ok(base_resp)
    }

    /// 查询工作表保护列表
    ///
    /// 查询指定工作表的所有保护配置列表。
    ///
    /// # 参数
    /// - `request`: 查询工作表保护请求，包含筛选条件和分页参数
    ///
    /// # 返回
    /// 返回工作表保护配置列表和分页信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v3::{SheetProtectionService, QuerySheetProtectionRequest};
    ///
    /// let service = SheetProtectionService::new(client_config);
    ///
    /// // 查询所有保护
    /// let request = QuerySheetProtectionRequest::builder()
    ///     .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
    ///     .sheet_id("0XXXXXXXXXX".to_string())
    ///     .page_size(20)
    ///     .build()?;
    ///
    /// let response = service.query(&request).await?;
    ///
    /// for protection in &response.data.items {
    ///     println!("保护ID: {}, 描述: {:?}", protection.protection_id, protection.description);
    /// }
    /// ```
    ///
    /// # 分页
    /// - 支持`page_size`和`page_token`参数进行分页查询
    /// - 默认`page_size`为20，最大为100
    pub async fn query(
        &self,
        request: &QuerySheetProtectionRequest,
    ) -> openlark_core::error::SDKResult<Response<QuerySheetProtectionResponse>> {
        let mut url = format!(
            "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/protections/query",
            self.config.base_url,
            request.spreadsheet_token.as_str(),
            request.sheet_id.as_str()
        );

        // 构建查询参数
        let mut params = vec![];

        if !request.protection_ids.is_empty() {
            params.push(format!(
                "protection_ids={}",
                request.protection_ids.join(",")
            ));
        }

        if let Some(page_size) = request.page_size {
            params.push(format!("page_size={}", page_size));
        }

        if let Some(page_token) = &request.page_token {
            params.push(format!("page_token={}", page_token));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let api_request = ApiRequest::with_method_and_path(Method::GET, &url);

        let base_resp =
            Transport::<QuerySheetProtectionResponse>::request(api_request, &self.config, None)
                .await?;

        Ok(base_resp)
    }

    /// 更新工作表保护
    ///
    /// 更新现有的工作表保护配置。
    ///
    /// # 参数
    /// - `request`: 更新工作表保护请求，包含保护ID和更新字段
    ///
    /// # 返回
    /// 返回更新后的保护配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v3::{SheetProtectionService, UpdateSheetProtectionRequest};
    ///
    /// let service = SheetProtectionService::new(client_config);
    ///
    /// // 更新保护描述和权限
    /// let request = UpdateSheetProtectionRequest::builder()
    ///     .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
    ///     .sheet_id("0XXXXXXXXXX".to_string())
    ///     .protection_id("protection_123".to_string())
    ///     .add_field("description".to_string())
    ///     .add_field("permissions".to_string())
    ///     .description("更新后的保护描述".to_string())
    ///     .add_editor("new_user_id", "user_id")
    ///     .build()?;
    ///
    /// let protection = service.update(&request).await?;
    /// println!("更新保护成功: {}", protection.data.protection_id);
    /// ```
    ///
    /// # 更新字段
    /// - `description`: 保护描述
    /// - `ranges`: 保护范围
    /// - `conditions`: 保护条件
    /// - `permissions`: 权限配置
    /// - `warningOnly`: 是否仅警告
    pub async fn update(
        &self,
        request: &UpdateSheetProtectionRequest,
    ) -> openlark_core::error::SDKResult<Response<UpdateSheetProtectionResponse>> {
        let url = format!(
            "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/protections/{}",
            self.config.base_url,
            request.spreadsheet_token.as_str(),
            request.sheet_id.as_str(),
            request.protection_id
        );

        let mut api_request = ApiRequest::with_method_and_path(Method::PATCH, &url);
        api_request.body = Some(openlark_core::api::RequestData::Json(request))?;

        let base_resp =
            Transport::<UpdateSheetProtectionResponse>::request(api_request, &self.config, None)
                .await?;

        Ok(base_resp)
    }

    /// 删除工作表保护
    ///
    /// 删除指定的工作表保护配置。
    ///
    /// # 参数
    /// - `request`: 删除工作表保护请求，包含保护ID
    ///
    /// # 返回
    /// 返回删除操作结果
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v3::{SheetProtectionService, DeleteSheetProtectionRequest};
    ///
    /// let service = SheetProtectionService::new(client_config);
    ///
    /// let request = DeleteSheetProtectionRequest::builder()
    ///     .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
    ///     .sheet_id("0XXXXXXXXXX".to_string())
    ///     .protection_id("protection_123".to_string())
    ///     .build()?;
    ///
    /// let response = service.delete(&request).await?;
    /// println!("删除保护成功");
    /// ```
    ///
    /// # 注意事项
    /// - 删除后保护配置将立即失效
    /// - 删除操作不可撤销
    /// - 需要相应的管理权限
    pub async fn delete(
        &self,
        request: &DeleteSheetProtectionRequest,
    ) -> openlark_core::error::SDKResult<Response<DeleteSheetProtectionResponse>> {
        let url = format!(
            "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/protections/{}",
            self.config.base_url,
            request.spreadsheet_token.as_str(),
            request.sheet_id.as_str(),
            request.protection_id
        );

        let api_request = ApiRequest::with_method_and_path(Method::DELETE, &url);

        let base_resp =
            Transport::<DeleteSheetProtectionResponse>::request(api_request, &self.config, None)
                .await?;

        Ok(base_resp)
    }

    /// 创建工作表保护构建器
    ///
    /// 提供链式调用的构建器模式，便于快速创建工作表保护请求。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v3::SheetProtectionService;
    ///
    /// let service = SheetProtectionService::new(client_config);
    ///
    /// let protection = service
    ///     .create_protection_builder()
    ///     .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
    ///     .sheet_id("0XXXXXXXXXX".to_string())
    ///     .description("财务数据保护".to_string())
    ///     .add_editor("user_123", "user_id")
    ///     .execute()
    ///     .await?;
    /// ```
    pub fn create_protection_builder(&self) -> SheetProtectionServiceBuilder<'_> {
        SheetProtectionServiceBuilder {
            service: self,
            spreadsheet_token: None,
            sheet_id: None,
            description: None,
            ranges: vec![],
            conditions: vec![],
            editors: vec![],
            protection_type: None,
            warning_only: None,
        }
    }
}

/// 工作表保护服务构建器
///
/// 提供流畅的API用于构建工作表保护请求。
pub struct SheetProtectionServiceBuilder<'a> {
    service: &'a SheetProtectionService,
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
    description: Option<String>,
    ranges: Vec<SheetProtectionRange>,
    conditions: Vec<SheetProtectionCondition>,
    editors: Vec<(String, String)>, // (user_id, user_type)
    protection_type: Option<String>,
    warning_only: Option<bool>,
}

impl<'a> SheetProtectionServiceBuilder<'a> {
    /// 设置电子表格Token
    pub fn spreadsheet_token(mut self, spreadsheet_token: String) -> Self {
        self.spreadsheet_token = Some(SpreadsheetToken::from(spreadsheet_token));
        self
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, sheet_id: String) -> Self {
        self.sheet_id = Some(SheetId::from(sheet_id));
        self
    }

    /// 设置保护描述
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 添加保护范围
    pub fn add_range(mut self, range: SheetProtectionRange) -> Self {
        self.ranges.push(range);
        self
    }

    /// 添加保护条件
    pub fn add_condition(mut self, condition: SheetProtectionCondition) -> Self {
        self.conditions.push(condition);
        self
    }

    /// 添加编辑者
    pub fn add_editor(mut self, user_id: &str, user_type: &str) -> Self {
        self.editors
            .push((user_id.to_string(), user_type.to_string()));
        self
    }

    /// 设置保护类型
    pub fn protection_type(mut self, protection_type: String) -> Self {
        self.protection_type = Some(protection_type);
        self
    }

    /// 设置是否仅警告
    pub fn warning_only(mut self, warning_only: bool) -> Self {
        self.warning_only = Some(warning_only);
        self
    }

    /// 执行创建操作
    pub async fn execute(
        self,
    ) -> openlark_core::error::SDKResult<Response<CreateSheetProtectionResponse>> {
        match (self.spreadsheet_token, self.sheet_id) {
            (Some(spreadsheet_token), Some(sheet_id)) => {
                let mut editors = vec![];
                for (user_id, user_type) in self.editors {
                    editors.push(SheetProtectionEditor {
                        user_id,
                        user_type,
                        name: None,
                        email: None,
                    });
                }

                let permissions = SheetProtectionPermissions {
                    editors,
                    allow_all_users: false,
                    allow_comments: false,
                };

                let request = CreateSheetProtectionRequest {
                    spreadsheet_token,
                    sheet_id,
                    description: self.description,
                    ranges: self.ranges,
                    conditions: self.conditions,
                    permissions,
                    protection_type: self.protection_type,
                    warning_only: self.warning_only,
                };

                self.service.create(&request).await
            }
            _ => Err(openlark_core::error::LarkAPIError::IllegalParamError(
                "电子表格Token和工作表ID都是必需的".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::trait_system::Service;

    #[test]
    fn test_sheet_protection_service_creation() {
        let config = openlark_core::config::Config::default();
        let service = SheetProtectionService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_create_sheet_protection_request_builder() {
        let request = CreateSheetProtectionRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .description("测试保护".to_string())
            .add_editor("user_123", "user_id")
            .warning_only(false)
            .build();

        assert!(request.is_ok());
        let req = request.unwrap();
        assert_eq!(req.spreadsheet_token.as_str(), "shtcnmBRWQKbsJRHXXXXXXXXXX");
        assert_eq!(req.sheet_id.as_str(), "0XXXXXXXXXX");
        assert_eq!(req.description, Some("测试保护".to_string()));
        assert_eq!(req.permissions.editors.len(), 1);
        assert_eq!(req.warning_only, Some(false));
    }

    #[test]
    fn test_query_sheet_protection_request_builder() {
        let request = QuerySheetProtectionRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .add_protection_id("protection_123".to_string())
            .page_size(20)
            .build();

        assert!(request.is_ok());
        let req = request.unwrap();
        assert_eq!(req.spreadsheet_token.as_str(), "shtcnmBRWQKbsJRHXXXXXXXXXX");
        assert_eq!(req.sheet_id.as_str(), "0XXXXXXXXXX");
        assert_eq!(req.protection_ids.len(), 1);
        assert_eq!(req.page_size, Some(20));
    }

    #[test]
    fn test_update_sheet_protection_request_builder() {
        let request = UpdateSheetProtectionRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .protection_id("protection_123".to_string())
            .add_field("description".to_string())
            .description("更新描述".to_string())
            .build();

        assert!(request.is_ok());
        let req = request.unwrap();
        assert_eq!(req.spreadsheet_token.as_str(), "shtcnmBRWQKbsJRHXXXXXXXXXX");
        assert_eq!(req.sheet_id.as_str(), "0XXXXXXXXXX");
        assert_eq!(req.protection_id, "protection_123");
        assert_eq!(req.fields.len(), 1);
        assert_eq!(req.description, Some("更新描述".to_string()));
    }

    #[test]
    fn test_delete_sheet_protection_request_builder() {
        let request = DeleteSheetProtectionRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .protection_id("protection_123".to_string())
            .build();

        assert!(request.is_ok());
        let req = request.unwrap();
        assert_eq!(req.spreadsheet_token.as_str(), "shtcnmBRWQKbsJRHXXXXXXXXXX");
        assert_eq!(req.sheet_id.as_str(), "0XXXXXXXXXX");
        assert_eq!(req.protection_id, "protection_123");
    }

    #[test]
    fn test_sheet_protection_range_builder() {
        let range = SheetProtectionRange::builder()
            .start_row_index(0)
            .end_row_index(10)
            .start_column_index(0)
            .end_column_index(5)
            .build();

        assert!(range.is_ok());
        let r = range.unwrap();
        assert_eq!(r.start_row_index, Some(0));
        assert_eq!(r.end_row_index, Some(10));
        assert_eq!(r.start_column_index, Some(0));
        assert_eq!(r.end_column_index, Some(5));
    }

    #[test]
    fn test_sheet_protection_condition_builder() {
        let condition = SheetProtectionCondition::builder()
            .condition_type("CELL_CONTAINS".to_string())
            .content("公式".to_string())
            .description("保护公式".to_string())
            .build();

        assert!(condition.is_ok());
        let c = condition.unwrap();
        assert_eq!(c.condition_type, "CELL_CONTAINS");
        assert_eq!(c.content, "公式");
        assert_eq!(c.description, Some("保护公式".to_string()));
    }

    #[test]
    fn test_validation_errors() {
        // 测试无效范围
        let invalid_range = SheetProtectionRange::builder()
            .start_row_index(10)
            .end_row_index(0) // 起始行大于结束行
            .build();
        assert!(invalid_range.is_err());

        // 测试空条件类型
        let invalid_condition = SheetProtectionCondition::builder()
            .condition_type("".to_string())
            .content("内容".to_string())
            .build();
        assert!(invalid_condition.is_err());

        // 测试空条件内容
        let invalid_condition2 = SheetProtectionCondition::builder()
            .condition_type("CELL_CONTAINS".to_string())
            .content("".to_string())
            .build();
        assert!(invalid_condition2.is_err());

        // 测试缺少必需参数
        let invalid_request = CreateSheetProtectionRequest::builder()
            .description("测试".to_string())
            .build();
        assert!(invalid_request.is_err());
    }

    #[test]
    fn test_sheet_protection_service_builder() {
        let config = openlark_core::config::Config::default();
        let service = SheetProtectionService::new(config);

        let builder = service
            .create_protection_builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .description("测试保护".to_string())
            .add_editor("user_123", "user_id")
            .warning_only(false);

        // 注意：这里只测试构建器设置，不实际执行API调用
        assert_eq!(
            builder.spreadsheet_token.unwrap().as_str(),
            "shtcnmBRWQKbsJRHXXXXXXXXXX"
        );
        assert_eq!(builder.sheet_id.unwrap().as_str(), "0XXXXXXXXXX");
        assert_eq!(builder.description, Some("测试保护".to_string()));
        assert_eq!(builder.editors.len(), 1);
        assert_eq!(builder.warning_only, Some(false));
    }

    #[test]
    fn test_complete_protection_workflow() {
        let config = openlark_core::config::Config::default();
        let service = SheetProtectionService::new(config);

        // 1. 创建保护范围
        let range1 = SheetProtectionRange::builder()
            .start_row_index(0)
            .end_row_index(10)
            .start_column_index(0)
            .end_column_index(5)
            .build()
            .unwrap();

        let range2 = SheetProtectionRange::builder()
            .range("A1:D10".to_string())
            .build()
            .unwrap();

        // 2. 创建保护条件
        let condition = SheetProtectionCondition::builder()
            .condition_type("CELL_CONTAINS".to_string())
            .content("重要数据".to_string())
            .description("保护包含重要数据的单元格".to_string())
            .build()
            .unwrap();

        // 3. 创建保护请求
        let create_request = CreateSheetProtectionRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .description("完整保护测试".to_string())
            .add_range(range1)
            .add_range(range2)
            .add_condition(condition)
            .add_editor("admin_001", "user_id")
            .add_editor("user_002", "user_id")
            .warning_only(false)
            .build()
            .unwrap();

        assert_eq!(create_request.ranges.len(), 2);
        assert_eq!(create_request.conditions.len(), 1);
        assert_eq!(create_request.permissions.editors.len(), 2);
        assert_eq!(create_request.warning_only, Some(false));

        // 4. 创建查询请求
        let query_request = QuerySheetProtectionRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .page_size(10)
            .build()
            .unwrap();

        assert_eq!(query_request.page_size, Some(10));

        // 5. 创建更新请求
        let update_request = UpdateSheetProtectionRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .protection_id("protection_test_001".to_string())
            .add_field("description".to_string())
            .add_field("warningOnly".to_string())
            .description("更新后的保护描述".to_string())
            .warning_only(true)
            .build()
            .unwrap();

        assert_eq!(update_request.fields.len(), 2);
        assert_eq!(update_request.warning_only, Some(true));

        // 6. 创建删除请求
        let delete_request = DeleteSheetProtectionRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .protection_id("protection_test_001".to_string())
            .build()
            .unwrap();

        assert_eq!(delete_request.protection_id, "protection_test_001");
    }
}

// ApiResponseTrait implementations for Transport compatibility
impl ApiResponseTrait for CreateSheetProtectionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for QuerySheetProtectionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for UpdateSheetProtectionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for DeleteSheetProtectionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
