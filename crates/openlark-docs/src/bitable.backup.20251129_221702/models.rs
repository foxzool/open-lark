//! 多维表格 (Bitable) 数据模型

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 多维表格应用信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App {
    /// 应用令牌
    pub app_token: String,
    /// 应用名称
    pub name: String,
    /// 应用链接
    pub url: String,
    /// 创建时间
    pub create_time: DateTime<Utc>,
    /// 更新时间
    pub update_time: DateTime<Utc>,
    /// 是否已归档
    pub is_archived: bool,
    /// 归档时间
    pub archive_time: Option<DateTime<Utc>>,
    /// 是否启用高级权限
    pub enable_advanced_permissions: bool,
    /// 预览统计信息
    pub preview_stats: Option<PreviewStats>,
    /// 删除时间
    pub delete_time: Option<DateTime<Utc>>,
}

/// 预览统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewStats {
    /// 表格数量
    pub table_count: i32,
    /// 记录数量
    pub record_count: i32,
    /// 字段数量
    pub field_count: i32,
}

/// 数据表信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    /// 表格ID
    pub table_id: String,
    /// 表格名称
    pub name: String,
    /// 表格版本
    pub revision: i32,
    /// 创建时间
    pub create_time: DateTime<Utc>,
    /// 更新时间
    pub update_time: DateTime<Utc>,
    /// 是否已归档
    pub is_archived: bool,
    /// 归档时间
    pub archive_time: Option<DateTime<Utc>>,
    /// 字段列表
    pub fields: Vec<Field>,
    /// 默认视图ID
    pub default_view_id: Option<String>,
}

/// 字段信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    /// 字段ID
    pub field_id: String,
    /// 字段名称
    pub field_name: String,
    /// 字段类型
    pub field_type: FieldType,
    /// 是否为主键
    pub is_primary: bool,
    /// 是否必填
    pub is_required: bool,
    /// 字段配置
    pub property: FieldProperty,
    /// 创建时间
    pub create_time: DateTime<Utc>,
    /// 更新时间
    pub update_time: DateTime<Utc>,
}

/// 字段类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FieldType {
    /// 文本
    Text,
    /// 数字
    Number,
    /// 单选
    SingleSelect,
    /// 多选
    MultiSelect,
    /// 日期
    Date,
    /// 人员
    User,
    /// 多人员
    MultipleUser,
    /// 部门
    Department,
    /// 多部门
    MultipleDepartment,
    /// 电话号码
    PhoneNumber,
    /// 邮箱
    Email,
    /// 超链接
    URL,
    /// 复选框
    Checkbox,
    /// 进度
    Progress,
    /// 评分
    Rating,
    /// 货币
    Currency,
    /// 百分比
    Percent,
    /// 自动编号
    AutoNumber,
    /// 创建时间
    CreatedTime,
    /// 最后修改时间
    ModifiedTime,
    /// 创建人
    CreatedBy,
    /// 最后修改人
    ModifiedBy,
    /// 附件
    Attachment,
    /// 公式
    Formula,
    /// 查找引用
    Lookup,
    /// 级联选择
    CascadeSelect,
    /// 位置
    Location,
    /// 其他
    Other(String),
}

/// 字段属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldProperty {
    /// 字段描述
    pub description: Option<String>,
    /// 默认值
    pub default_value: Option<serde_json::Value>,
    /// 选项（用于单选、多选字段）
    pub options: Option<Vec<SelectOption>>,
    /// 格式化配置
    pub format: Option<String>,
    /// 验证规则
    pub validation: Option<ValidationRule>,
    /// 其他属性
    pub extra: HashMap<String, serde_json::Value>,
}

/// 选择选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectOption {
    /// 选项名称
    pub name: String,
    /// 选项值
    pub value: String,
    /// 选项颜色
    pub color: Option<String>,
}

/// 验证规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    /// 是否必填
    pub required: Option<bool>,
    /// 最小值（用于数字、货币字段）
    pub min: Option<f64>,
    /// 最大值（用于数字、货币字段）
    pub max: Option<f64>,
    /// 最小长度（用于文本字段）
    pub min_length: Option<i32>,
    /// 最大长度（用于文本字段）
    pub max_length: Option<i32>,
    /// 正则表达式
    pub pattern: Option<String>,
}

/// 视图信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct View {
    /// 视图ID
    pub view_id: String,
    /// 视图名称
    pub name: String,
    /// 视图类型
    pub view_type: ViewType,
    /// 创建时间
    pub create_time: DateTime<Utc>,
    /// 更新时间
    pub update_time: DateTime<Utc>,
    /// 排序配置
    pub sorts: Vec<ViewSort>,
    /// 筛选配置
    pub filters: Option<Vec<ViewFilter>>,
    /// 字段配置
    pub field_setting: Option<HashMap<String, FieldSetting>>,
}

/// 视图类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ViewType {
    /// 表格视图
    Grid,
    /// 看板视图
    Kanban,
    /// 日历视图
    Calendar,
    /// 甘特图视图
    Gantt,
    /// 图表视图
    Chart,
}

/// 视图排序配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewSort {
    /// 字段ID
    pub field_id: String,
    /// 排序方向
    pub direction: SortDirection,
    /// 排序优先级
    pub priority: i32,
}

/// 排序方向
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SortDirection {
    /// 升序
    Asc,
    /// 降序
    Desc,
}

/// 视图筛选配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewFilter {
    /// 字段ID
    pub field_id: String,
    /// 操作符
    pub operator: FilterOperator,
    /// 筛选值
    pub value: serde_json::Value,
    /// 筛选条件连接符
    pub conjunction: Option<Conjunction>,
}

/// 筛选操作符
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FilterOperator {
    /// 等于
    Equal,
    /// 不等于
    NotEqual,
    /// 包含
    Contains,
    /// 不包含
    DoesNotContain,
    /// 为空
    IsEmpty,
    /// 不为空
    IsNotEmpty,
    /// 大于
    GreaterThan,
    /// 大于等于
    GreaterThanOrEqual,
    /// 小于
    LessThan,
    /// 小于等于
    LessThanOrEqual,
    /// 在范围内
    Within,
    /// 不在范围内
    NotWithin,
    /// 其他
    Other(String),
}

/// 条件连接符
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Conjunction {
    /// 并且
    And,
    /// 或者
    Or,
}

/// 字段设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldSetting {
    /// 字段宽度
    pub width: Option<i32>,
    /// 是否隐藏
    pub hidden: Option<bool>,
    /// 是否固定
    pub fixed: Option<bool>,
    /// 对齐方式
    pub align: Option<TextAlign>,
    /// 其他设置
    pub extra: HashMap<String, serde_json::Value>,
}

/// 文本对齐方式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TextAlign {
    /// 左对齐
    Left,
    /// 居中
    Center,
    /// 右对齐
    Right,
}

/// 记录信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    /// 记录ID
    pub record_id: String,
    /// 字段值
    pub fields: HashMap<String, serde_json::Value>,
    /// 创建时间
    pub create_time: DateTime<Utc>,
    /// 更新时间
    pub update_time: DateTime<Utc>,
    /// 创建人
    pub created_by: Option<UserInfo>,
    /// 最后修改人
    pub modified_by: Option<UserInfo>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub name: String,
    /// 邮箱
    pub email: Option<String>,
    /// 头像URL
    pub avatar_url: Option<String>,
}

/// 请求结构体

/// 创建应用请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAppRequest {
    /// 应用名称
    pub name: String,
    /// 文件夹ID
    pub folder_token: Option<String>,
}

/// 复制应用请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyAppRequest {
    /// 应用名称
    pub name: Option<String>,
    /// 文件夹ID
    pub folder_token: Option<String>,
    /// 是否复制链接
    pub link: Option<bool>,
}

/// 更新应用请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAppRequest {
    /// 应用名称
    pub name: Option<String>,
    /// 是否启用高级权限
    pub enable_advanced_permissions: Option<bool>,
    /// 是否归档
    pub is_archived: Option<bool>,
}

/// 创建数据表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTableRequest {
    /// 表格名称
    pub table: String,
    /// 字段列表
    pub fields: Vec<CreateFieldRequest>,
    /// 默认视图配置
    pub default_view: Option<CreateViewRequest>,
}

/// 创建字段请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFieldRequest {
    /// 字段名称
    pub field_name: String,
    /// 字段类型
    pub field_type: String,
    /// 是否必填
    pub is_required: Option<bool>,
    /// 字段描述
    pub description: Option<String>,
    /// 默认值
    pub default_value: Option<serde_json::Value>,
    /// 字段属性
    pub property: Option<FieldProperty>,
}

/// 更新数据表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTableRequest {
    /// 表格名称
    pub table: Option<String>,
    /// 是否归档
    pub is_archived: Option<bool>,
}

/// 创建视图请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateViewRequest {
    /// 视图名称
    pub name: String,
    /// 视图类型
    pub view_type: String,
    /// 排序配置
    pub sorts: Option<Vec<ViewSort>>,
    /// 筛选配置
    pub filters: Option<Vec<ViewFilter>>,
    /// 字段配置
    pub field_setting: Option<HashMap<String, FieldSetting>>,
}

/// 更新视图请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateViewRequest {
    /// 视图名称
    pub name: Option<String>,
    /// 排序配置
    pub sorts: Option<Vec<ViewSort>>,
    /// 筛选配置
    pub filters: Option<Vec<ViewFilter>>,
    /// 字段配置
    pub field_setting: Option<HashMap<String, FieldSetting>>,
}

/// 创建记录请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRecordRequest {
    /// 字段值
    pub fields: HashMap<String, serde_json::Value>,
}

/// 更新记录请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRecordRequest {
    /// 字段值
    pub fields: HashMap<String, serde_json::Value>,
}

/// 批量创建记录请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateRecordsRequest {
    /// 记录列表
    pub records: Vec<CreateRecordRequest>,
}

/// 搜索记录请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRecordsRequest {
    /// 视图ID
    pub view_id: Option<String>,
    /// 记录ID列表
    pub record_ids: Option<Vec<String>>,
    /// 排序配置
    pub sort: Option<Vec<ViewSort>>,
    /// 筛选条件
    pub filter: Option<ViewFilter>,
    /// 字段列表
    pub field_names: Option<Vec<String>>,
    /// 分页配置
    pub page_token: Option<String>,
    /// 页面大小
    pub page_size: Option<i32>,
}

/// 响应结构体

/// 应用列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppListResponse {
    /// 应用列表
    pub items: Vec<App>,
    /// 是否有更多
    pub has_more: bool,
    /// 页面令牌
    pub page_token: Option<String>,
}

/// 数据表列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableListResponse {
    /// 数据表列表
    pub items: Vec<Table>,
    /// 是否有更多
    pub has_more: bool,
    /// 页面令牌
    pub page_token: Option<String>,
}

/// 视图列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewListResponse {
    /// 视图列表
    pub items: Vec<View>,
    /// 是否有更多
    pub has_more: bool,
    /// 页面令牌
    pub page_token: Option<String>,
}

/// 搜索记录响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRecordsResponse {
    /// 记录列表
    pub records: Vec<Record>,
    /// 是否有更多
    pub has_more: bool,
    /// 页面令牌
    pub page_token: Option<String>,
    /// 总记录数
    pub total: Option<i32>,
}

/// 批量创建记录响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateRecordsResponse {
    /// 创建的记录列表
    pub records: Vec<Record>,
    /// 错误信息列表
    pub errors: Vec<BatchError>,
}

/// 批量操作错误信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchError {
    /// 行号
    pub row: i32,
    /// 字段ID
    pub field_id: String,
    /// 错误消息
    pub message: String,
    /// 错误代码
    pub code: i32,
}

impl Default for CreateAppRequest {
    fn default() -> Self {
        Self {
            name: String::new(),
            folder_token: None,
        }
    }
}

impl Default for CreateTableRequest {
    fn default() -> Self {
        Self {
            table: String::new(),
            fields: Vec::new(),
            default_view: None,
        }
    }
}

impl Default for SearchRecordsRequest {
    fn default() -> Self {
        Self {
            view_id: None,
            record_ids: None,
            sort: None,
            filter: None,
            field_names: None,
            page_token: None,
            page_size: None,
        }
    }
}