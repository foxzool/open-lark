// base模块的数据模型定义

use serde::{Deserialize, Serialize};
use crate::core::api_resp::{ApiResponseTrait, ResponseFormat};
/// 多维表格应用信息
#[derive(Debug, Deserialize, Serialize)]
pub struct BitableApp {
    /// 应用访问令牌
#[serde(rename = "app_token")]
    pub app_token: String,
    /// 应用名称
#[serde(rename = "name")]
    pub name: String,
    /// 应用链接
#[serde(rename = "link")]
    pub link: String,
    /// 应用是否开启高级权限
#[serde(rename = "advanced_permission")]
    pub advanced_permission: bool,
    /// 应用时间戳
#[serde(rename = "time_zone")]
    pub time_zone: String,
    /// 应用创建时间
#[serde(rename = "create_time")]
    pub create_time: String,
    /// 应用更新时间
#[serde(rename = "update_time")]
    pub update_time: String,
    /// 应用是否归档
#[serde(rename = "is_archived")]
    pub is_archived: bool,
}
/// 创建多维表格请求
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAppRequest {
    /// 应用名称
#[serde(rename = "name")]
    pub name: String,
    /// 文件夹token（可选）
#[serde(rename = "folder_token")]
    pub folder_token: Option<String>,
    /// 应用图标（可选）
#[serde(rename = "icon")]
    pub icon: Option<String>,
}
/// 复制多维表格请求
#[derive(Debug, Deserialize, Serialize)]
pub struct CopyAppRequest {
    /// 复制后的应用名称
#[serde(rename = "name")]
    pub name: String,
    /// 目标文件夹token（可选）
#[serde(rename = "folder_token")]
    pub folder_token: Option<String>,
    /// 是否复制数据表数据
#[serde(rename = "with_data")]
    pub with_data: Option<bool>,
}
/// 更新多维表格请求
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateAppRequest {
    /// 应用名称
#[serde(rename = "name")]
    pub name: Option<String>,
    /// 应用图标
#[serde(rename = "icon")]
    pub icon: Option<String>,
    /// 是否归档
#[serde(rename = "is_archived")]
    pub is_archived: Option<bool>,
}
/// 数据表信息
#[derive(Debug, Deserialize, Serialize)]
pub struct BitableTable {
    /// 数据表ID
#[serde(rename = "table_id")]
    pub table_id: String,
    /// 数据表名称
#[serde(rename = "name")]
    pub name: String,
    /// 数据表创建时间
#[serde(rename = "create_time")]
    pub create_time: String,
    /// 数据表更新时间
#[serde(rename = "update_time")]
    pub update_time: String,
    /// 数据表是否归档
#[serde(rename = "is_archived")]
    pub is_archived: bool,
    /// 数据表字段数量
#[serde(rename = "field_count")]
    pub field_count: i32,
    /// 数据表记录数量
#[serde(rename = "record_count")]
    pub record_count: i32,
}
/// 创建数据表请求
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTableRequest {
    /// 数据表名称
#[serde(rename = "table")]
    pub table: TableInfo,
}
/// 数据表信息
#[derive(Debug, Deserialize, Serialize)]
pub struct TableInfo {
    /// 数据表名称
#[serde(rename = "name")]
    pub name: String,
    /// 数据表默认视图名称
#[serde(rename = "default_view_name")]
    pub default_view_name: Option<String>,
    /// 数据表字段列表
#[serde(rename = "fields")]
    pub fields: Option<Vec<FieldInfo>>,
}
/// 批量创建数据表请求
#[derive(Debug, Deserialize, Serialize)]
pub struct BatchCreateTablesRequest {
    /// 数据表列表
#[serde(rename = "tables")]
    pub tables: Vec<TableInfo>,
}
/// 更新数据表请求
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTableRequest {
    /// 数据表名称
#[serde(rename = "name")]
    pub name: Option<String>,
}
/// 视图信息
#[derive(Debug, Deserialize, Serialize)]
pub struct BitableView {
    /// 视图ID
#[serde(rename = "view_id")]
    pub view_id: String,
    /// 视图名称
#[serde(rename = "name")]
    pub name: String,
    /// 视图类型
#[serde(rename = "view_type")]
    pub view_type: String,
    /// 视图创建时间
#[serde(rename = "create_time")]
    pub create_time: String,
    /// 视图更新时间
#[serde(rename = "update_time")]
    pub update_time: String,
}
/// 创建视图请求
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateViewRequest {
    /// 视图名称
#[serde(rename = "name")]
    pub name: String,
    /// 视图类型
#[serde(rename = "view_type")]
    pub view_type: String,
}
/// 更新视图请求
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateViewRequest {
    /// 视图名称
#[serde(rename = "name")]
    pub name: Option<String>,
}
/// 记录信息
#[derive(Debug, Deserialize, Serialize)]
pub struct BitableRecord {
    /// 记录ID
#[serde(rename = "record_id")]
    pub record_id: String,
    /// 记录字段
#[serde(rename = "fields")]
    pub fields: serde_json::Value,
    /// 记录创建时间
#[serde(rename = "create_time")]
    pub create_time: String,
    /// 记录更新时间
#[serde(rename = "update_time")]
    pub update_time: String,
}
/// 创建记录请求
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateRecordRequest {
    /// 记录字段
#[serde(rename = "fields")]
    pub fields: serde_json::Value,
}
/// 更新记录请求
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateRecordRequest {
    /// 记录字段
#[serde(rename = "fields")]
    pub fields: serde_json::Value,
}
/// 查询记录请求
#[derive(Debug, Deserialize, Serialize)]
pub struct SearchRecordsRequest {
    /// 分页信息
#[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    /// 分页令牌
#[serde(rename = "page_token")]
    pub page_token: Option<String>,
    /// 排序信息
#[serde(rename = "sort")]
    pub sort: Option<Vec<SortInfo>>,
    /// 过滤条件
#[serde(rename = "filter")]
    pub filter: Option<FilterInfo>,
    /// 字段列表
#[serde(rename = "field_names")]
    pub field_names: Option<Vec<String>>,
}
/// 排序信息
#[derive(Debug, Deserialize, Serialize)]
pub struct SortInfo {
    /// 字段名
#[serde(rename = "field_name")]
    pub field_name: String,
    /// 排序方向
#[serde(rename = "desc")]
    pub desc: Option<bool>,
}
/// 过滤条件
#[derive(Debug, Deserialize, Serialize)]
pub struct FilterInfo {
    /// 过滤条件组
#[serde(rename = "conjunction")]
    pub conjunction: String,
    /// 条件列表
#[serde(rename = "conditions")]
    pub conditions: Vec<FilterCondition>,
}
/// 过滤条件
#[derive(Debug, Deserialize, Serialize)]
pub struct FilterCondition {
    /// 字段名
#[serde(rename = "field_name")]
    pub field_name: String,
    /// 操作符
#[serde(rename = "operator")]
    pub operator: String,
    /// 值
#[serde(rename = "value")]
    pub value: serde_json::Value,
}
/// 批量操作记录请求
#[derive(Debug, Deserialize, Serialize)]
pub struct BatchRecordsRequest {
    /// 记录列表
#[serde(rename = "records")]
    pub records: Vec<RecordData>,
}
/// 记录数据
#[derive(Debug, Deserialize, Serialize)]
pub struct RecordData {
    /// 记录ID（更新时需要）
#[serde(rename = "record_id")]
    pub record_id: Option<String>,
    /// 记录字段
#[serde(rename = "fields")]
    pub fields: serde_json::Value,
}
/// 批量获取记录请求
#[derive(Debug, Deserialize, Serialize)]
pub struct BatchGetRecordsRequest {
    /// 记录ID列表
#[serde(rename = "record_ids")]
    pub record_ids: Vec<String>,
    /// 字段列表
#[serde(rename = "field_names")]
    pub field_names: Option<Vec<String>>,
}
/// 字段信息
#[derive(Debug, Deserialize, Serialize)]
pub struct FieldInfo {
    /// 字段名称
#[serde(rename = "name")]
    pub name: String,
    /// 字段类型
#[serde(rename = "type")]
    pub field_type: String,
    /// 字段属性
#[serde(rename = "property")]
    pub property: Option<serde_json::Value>,
    /// 字段是否必填
#[serde(rename = "is_required")]
    pub is_required: Option<bool>,
}
/// 创建字段请求
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateFieldRequest {
    /// 字段名称
#[serde(rename = "field_name")]
    pub field_name: String,
    /// 字段类型
#[serde(rename = "type")]
    pub field_type: String,
    /// 字段属性
#[serde(rename = "property")]
    pub property: Option<serde_json::Value>,
}
/// 更新字段请求
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateFieldRequest {
    /// 字段名称
#[serde(rename = "field_name")]
    pub field_name: Option<String>,
    /// 字段属性
#[serde(rename = "property")]
    pub property: Option<serde_json::Value>,
}
/// 仪表盘信息
#[derive(Debug, Deserialize, Serialize)]
pub struct BitableDashboard {
    /// 仪表盘ID
#[serde(rename = "dashboard_id")]
    pub dashboard_id: String,
    /// 仪表盘名称
#[serde(rename = "name")]
    pub name: String,
    /// 仪表盘创建时间
#[serde(rename = "create_time")]
    pub create_time: String,
    /// 仪表盘更新时间
#[serde(rename = "update_time")]
    pub update_time: String,
}
/// 复制仪表盘请求
#[derive(Debug, Deserialize, Serialize)]
pub struct CopyDashboardRequest {
    /// 仪表盘名称
#[serde(rename = "name")]
    pub name: Option<String>,
}
/// 表单信息
#[derive(Debug, Deserialize, Serialize)]
pub struct BitableForm {
    /// 表单ID
#[serde(rename = "form_id")]
    pub form_id: String,
    /// 表单名称
#[serde(rename = "name")]
    pub name: String,
    /// 表单创建时间
#[serde(rename = "create_time")]
    pub create_time: String,
    /// 表单更新时间
#[serde(rename = "update_time")]
    pub update_time: String,
    /// 表单问题列表
#[serde(rename = "questions")]
    pub questions: Vec<FormQuestion>,
}
/// 表单问题
#[derive(Debug, Deserialize, Serialize)]
pub struct FormQuestion {
    /// 问题ID
#[serde(rename = "question_id")]
    pub question_id: String,
    /// 问题名称
#[serde(rename = "name")]
    pub name: String,
    /// 问题类型
#[serde(rename = "type")]
    pub question_type: String,
    /// 问题属性
#[serde(rename = "property")]
    pub property: serde_json::Value,
    /// 是否必填
#[serde(rename = "required")]
    pub required: bool,
}
/// 更新表单请求
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateFormRequest {
    /// 表单名称
#[serde(rename = "name")]
    pub name: Option<String>,
    /// 表单问题列表
#[serde(rename = "questions")]
    pub questions: Option<Vec<FormQuestion>>,
}
/// 更新表单问题请求
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateFormQuestionRequest {
    /// 问题名称
#[serde(rename = "name")]
    pub name: Option<String>,
    /// 问题属性
#[serde(rename = "property")]
    pub property: Option<serde_json::Value>,
    /// 是否必填
#[serde(rename = "required")]
    pub required: Option<bool>,
}
/// 角色信息
#[derive(Debug, Deserialize, Serialize)]
pub struct BitableRole {
    /// 角色ID
#[serde(rename = "role_id")]
    pub role_id: String,
    /// 角色名称
#[serde(rename = "name")]
    pub name: String,
    /// 角色描述
#[serde(rename = "description")]
    pub description: Option<String>,
    /// 角色权限
#[serde(rename = "permissions")]
    pub permissions: Vec<String>,
    /// 角色创建时间
#[serde(rename = "create_time")]
    pub create_time: String,
    /// 角色更新时间
#[serde(rename = "update_time")]
    pub update_time: String,
}
/// 创建角色请求
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateRoleRequest {
    /// 角色名称
#[serde(rename = "name")]
    pub name: String,
    /// 角色描述
#[serde(rename = "description")]
    pub description: Option<String>,
    /// 角色权限
#[serde(rename = "permissions")]
    pub permissions: Option<Vec<String>>,
}
/// 更新角色请求
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateRoleRequest {
    /// 角色名称
#[serde(rename = "name")]
    pub name: Option<String>,
    /// 角色描述
#[serde(rename = "description")]
    pub description: Option<String>,
    /// 角色权限
#[serde(rename = "permissions")]
    pub permissions: Option<Vec<String>>,
}
/// 角色成员信息
#[derive(Debug, Deserialize, Serialize)]
pub struct RoleMember {
    /// 成员ID
#[serde(rename = "member_id")]
    pub member_id: String,
    /// 成员类型
#[serde(rename = "member_type")]
    pub member_type: String,
    /// 成员名称
#[serde(rename = "name")]
    pub name: String,
    /// 添加时间
#[serde(rename = "add_time")]
    pub add_time: String,
}
/// 添加角色成员请求
#[derive(Debug, Deserialize, Serialize)]
pub struct AddRoleMemberRequest {
    /// 成员ID列表
#[serde(rename = "member_ids")]
    pub member_ids: Vec<String>,
    /// 成员类型
#[serde(rename = "member_type")]
    pub member_type: String,
}
/// 批量添加角色成员请求
#[derive(Debug, Deserialize, Serialize)]
pub struct BatchAddRoleMembersRequest {
    /// 成员列表
#[serde(rename = "members")]
    pub members: Vec<RoleMemberInfo>,
}
/// 角色成员信息
#[derive(Debug, Deserialize, Serialize)]
pub struct RoleMemberInfo {
    /// 成员ID
#[serde(rename = "member_id")]
    pub member_id: String,
    /// 成员类型
#[serde(rename = "member_type")]
    pub member_type: String,
}
/// 自动化流程信息
#[derive(Debug, Deserialize, Serialize)]
pub struct BitableWorkflow {
    /// 流程ID
#[serde(rename = "workflow_id")]
    pub workflow_id: String,
    /// 流程名称
#[serde(rename = "name")]
    pub name: String,
    /// 流程状态
#[serde(rename = "status")]
    pub status: String,
    /// 流程创建时间
#[serde(rename = "create_time")]
    pub create_time: String,
    /// 流程更新时间
#[serde(rename = "update_time")]
    pub update_time: String,
}
/// 更新自动化流程请求
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateWorkflowRequest {
    /// 流程状态
#[serde(rename = "status")]
    pub status: String,
}
/// 通用响应结构
#[derive(Debug, Deserialize, Serialize)]
pub struct pub struct BaseResponse<T> {, {
    /// 响应码
    pub code: i32,
    /// 响应消息
    pub msg: String,
    /// 响应数据
    pub data: Option<T>,
}
/// 分页响应结构
#[derive(Debug, Deserialize, Serialize)]
pub struct pub struct PaginatedResponse<T> {, {
    /// 响应码
    pub code: i32,
    /// 响应消息
    pub msg: String,
    /// 响应数据
    pub data: Option<PaginatedData<T>>,
}
/// 分页数据
#[derive(Debug, Deserialize, Serialize)]
pub struct pub struct PaginatedData<T> {, {
    /// 数据项
    pub items: Option<Vec<T>>,
    /// 分页令牌
    pub page_token: Option<String>,
    /// 是否有更多数据
    pub has_more: Option<bool>,
}
// 为主要模型实现ApiResponseTrait
impl ApiResponseTrait for SimpleResponse {

        ResponseFormat::Data
}
}
impl ApiResponseTrait for SimpleResponse {
    
ResponseFormat::Data
    }
}
impl ApiResponseTrait for SimpleResponse {
    
ResponseFormat::Data
    }
}
impl ApiResponseTrait for SimpleResponse {
    
ResponseFormat::Data
    }
}
impl ApiResponseTrait for SimpleResponse {
    
ResponseFormat::Data
    }
}
impl ApiResponseTrait for SimpleResponse {
    
ResponseFormat::Data
    }
}
impl ApiResponseTrait for SimpleResponse {
    
ResponseFormat::Data
    }
}
impl ApiResponseTrait for SimpleResponse {
    
ResponseFormat::Data
    }
}
impl ApiResponseTrait for SimpleResponse {
    
ResponseFormat::Data
    }
}
impl ApiResponseTrait for SimpleResponse {
    
ResponseFormat::Data
    }
}
impl ApiResponseTrait for SimpleResponse {
    
ResponseFormat::Data
    }
}
// 为集合类型实现
impl<T: ApiResponseTrait> ApiResponseTrait for Vec<T> {

        ResponseFormat::Data
}
}
impl<T: ApiResponseTrait> ApiResponseTrait for PaginatedData<T> {
    
ResponseFormat::Data
    }
}
// 为空类型实现
impl ApiResponseTrait for SimpleResponse {

        ResponseFormat::Data
}
}