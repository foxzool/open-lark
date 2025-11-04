// feishu_people模块的数据模型定义
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// 人员基本信息
#[derive(Debug, Clone)]
pub struct PersonInfo {
    /// 人员ID
#[serde(rename = "person_id")]
    pub person_id: String,
    /// 人员姓名
    pub name: String,
    /// 人员工号
#[serde(rename = "employee_no")]
    pub employee_no: Option<String>,
    /// 手机号
#[serde(rename = "mobile")]
    pub mobile: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 性别
    pub gender: Option<Gender>,
    /// 生日
    pub birthday: Option<String>,
    /// 入职时间
#[serde(rename = "hire_date")]
    pub hire_date: Option<String>,
    /// 离职时间
#[serde(rename = "termination_date")]
    pub termination_date: Option<String>,
    /// 人员状态
#[serde(rename = "employment_status")]
    pub employment_status: Option<EmploymentStatus>,
    /// 人员类型
#[serde(rename = "employee_type")]
    pub employee_type: Option<EmployeeTypeInfo>,
    /// 所属部门
    pub departments: Option<Vec<DepartmentInfo>>,
    /// 职位信息
    pub positions: Option<Vec<PositionInfo>>,
    /// 创建时间
#[serde(rename = "create_time")]
    pub create_time: Option<String>,
    /// 更新时间
#[serde(rename = "update_time")]
    pub update_time: Option<String>,
/// 性别枚举
#[derive(Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Gender {
/// 男性
    Male,
    /// 女性
    Female,
    /// 未知
    Unknown,
/// 雇佣状态枚举
#[derive(Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum EmploymentStatus {
/// 在职
    Active,
    /// 已离职
    Terminated,
    /// 待入职
    Pending,
    /// 休假中
    OnLeave,
/// 部门信息
#[derive(Debug, Clone)]
pub struct DepartmentInfo {
    /// 部门ID
#[serde(rename = "department_id")]
    pub department_id: String,
    /// 部门名称
    pub name: String,
    /// 父部门ID
#[serde(rename = "parent_department_id")]
    pub parent_department_id: Option<String>,
    /// 部门路径
#[serde(rename = "department_path")]
    pub department_path: Option<String>,
    /// 是否为主部门
#[serde(rename = "is_primary")]
    pub is_primary: Option<bool>,
/// 职位信息
#[derive(Debug, Clone)]
pub struct PositionInfo {
    /// 职位ID
#[serde(rename = "position_id")]
    pub position_id: String,
    /// 职位名称
    pub name: String,
    /// 职务序列
#[serde(rename = "job_family")]
    pub job_family: Option<JobFamilyInfo>,
    /// 职务级别
#[serde(rename = "job_level")]
    pub job_level: Option<JobLevelInfo>,
    /// 部门信息
    pub department: Option<DepartmentInfo>,
    /// 汇报对象
#[serde(rename = "reports_to")]
    pub reports_to: Option<PersonInfo>,
    /// 是否为主职位
#[serde(rename = "is_primary")]
    pub is_primary: Option<bool>,
/// 人员类型信息
#[derive(Debug, Clone)]
pub struct EmployeeTypeInfo {
    /// 人员类型ID
#[serde(rename = "employee_type_id")]
    pub employee_type_id: String,
    /// 人员类型名称
    pub name: String,
    /// 人员类型描述
    pub description: Option<String>,
/// 职务序列信息
#[derive(Debug, Clone)]
pub struct JobFamilyInfo {
    /// 职务序列ID
#[serde(rename = "job_family_id")]
    pub job_family_id: String,
    /// 职务序列名称
    pub name: String,
    /// 职务序列描述
    pub description: Option<String>,
/// 职务级别信息
#[derive(Debug, Clone)]
pub struct JobLevelInfo {
    /// 职务级别ID
#[serde(rename = "job_level_id")]
    pub job_level_id: String,
    /// 职务级别名称
    pub name: String,
    /// 级别代码
    pub code: Option<String>,
    /// 职务级别描述
    pub description: Option<String>,
/// 合同信息
#[derive(Debug, Clone)]
pub struct ContractInfo {
    /// 合同ID
#[serde(rename = "contract_id")]
    pub contract_id: String,
    /// 合同编号
#[serde(rename = "contract_no")]
    pub contract_no: Option<String>,
    /// 合同类型
#[serde(rename = "contract_type")]
    pub contract_type: Option<String>,
    /// 合同开始时间
#[serde(rename = "start_date")]
    pub start_date: Option<String>,
    /// 合同结束时间
#[serde(rename = "end_date")]
    pub end_date: Option<String>,
    /// 合同状态
#[serde(rename = "contract_status")]
    pub contract_status: Option<ContractStatus>,
    /// 试用期开始时间
#[serde(rename = "probation_start_date")]
    pub probation_start_date: Option<String>,
    /// 试用期结束时间
#[serde(rename = "probation_end_date")]
    pub probation_end_date: Option<String>,
    /// 薪资标准
#[serde(rename = "salary_standard")]
    pub salary_standard: Option<SalaryStandardInfo>,
/// 合同状态
#[derive(Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ContractStatus {
/// 有效
    Active,
    /// 已过期
    Expired,
    /// 已终止
    Terminated,
    /// 待生效
    Pending,
/// 薪资标准信息
#[derive(Debug, Clone)]
pub struct SalaryStandardInfo {
    /// 薪资标准ID
#[serde(rename = "salary_standard_id")]
    pub salary_standard_id: String,
    /// 基本薪资
#[serde(rename = "base_salary")]
    pub base_salary: Option<f64>,
    /// 薪资标准名称
    pub name: Option<String>,
/// 假期类型
#[derive(Debug, Clone)]
pub struct LeaveType {
    /// 假期类型ID
#[serde(rename = "leave_type_id")]
    pub leave_type_id: String,
    /// 假期类型名称
    pub name: String,
    /// 假期类型代码
    pub code: Option<String>,
    /// 是否需要审批
#[serde(rename = "requires_approval")]
    pub requires_approval: Option<bool>,
    /// 是否可带薪
#[serde(rename = "is_paid")]
    pub is_paid: Option<bool>,
/// 假期余额
#[derive(Debug, Clone)]
pub struct LeaveBalance {
    /// 人员ID
#[serde(rename = "person_id")]
    pub person_id: String,
    /// 假期类型
#[serde(rename = "leave_type")]
    pub leave_type: LeaveType,
    /// 总额度
#[serde(rename = "total_quota")]
    pub total_quota: f64,
    /// 已使用额度
#[serde(rename = "used_quota")]
    pub used_quota: f64,
    /// 剩余额度
#[serde(rename = "remaining_quota")]
    pub remaining_quota: f64,
    /// 有效期开始时间
#[serde(rename = "valid_from")]
    pub valid_from: Option<String>,
    /// 有效期结束时间
#[serde(rename = "valid_to")]
    pub valid_to: Option<String>,
/// 休假申请
#[derive(Debug, Clone)]
pub struct LeaveApplication {
    /// 申请ID
#[serde(rename = "application_id")]
    pub application_id: String,
    /// 申请人
#[serde(rename = "applicant")]
    pub applicant: PersonInfo,
    /// 假期类型
#[serde(rename = "leave_type")]
    pub leave_type: LeaveType,
    /// 开始时间
#[serde(rename = "start_time")]
    pub start_time: String,
    /// 结束时间
#[serde(rename = "end_time")]
    pub end_time: String,
    /// 请假时长（天）
#[serde(rename = "duration_days")]
    pub duration_days: f64,
    /// 请假原因
#[serde(rename = "reason")]
    pub reason: Option<String>,
    /// 申请状态
#[serde(rename = "status")]
    pub status: LeaveApplicationStatus,
    /// 审批人
#[serde(rename = "approvers")]
    pub approvers: Option<Vec<PersonInfo>>,
    /// 申请时间
#[serde(rename = "application_time")]
    pub application_time: String,
    /// 更新时间
#[serde(rename = "update_time")]
    pub update_time: String,
/// 休假申请状态
#[derive(Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum LeaveApplicationStatus {
/// 待审批
    Pending,
    /// 已批准
    Approved,
    /// 已拒绝
    Rejected,
    /// 已撤回
    Withdrawn,
    /// 已完成
    Completed,
/// 用户授权信息
#[derive(Debug, Clone)]
pub struct UserAuthorization {
    /// 用户ID
#[serde(rename = "user_id")]
    pub user_id: String,
    /// 用户姓名
    pub name: String,
    /// 授权的角色列表
    pub roles: Option<Vec<RoleInfo>>,
    /// 直接权限列表
    pub permissions: Option<Vec<PermissionInfo>>,
    /// 授权范围
#[serde(rename = "authorization_scope")]
    pub authorization_scope: Option<AuthorizationScope>,
/// 角色信息
#[derive(Debug, Clone)]
pub struct RoleInfo {
    /// 角色ID
#[serde(rename = "role_id")]
    pub role_id: String,
    /// 角色名称
    pub name: String,
    /// 角色描述
    pub description: Option<String>,
    /// 角色类型
#[serde(rename = "role_type")]
    pub role_type: Option<String>,
    /// 是否启用
#[serde(rename = "enabled")]
    pub enabled: Option<bool>,
/// 权限信息
#[derive(Debug, Clone)]
pub struct PermissionInfo {
    /// 权限ID
#[serde(rename = "permission_id")]
    pub permission_id: String,
    /// 权限名称
    pub name: String,
    /// 权限描述
    pub description: Option<String>,
    /// 权限类型
#[serde(rename = "permission_type")]
    pub permission_type: Option<String>,
/// 授权范围
#[derive(Debug, Clone)]
pub struct AuthorizationScope {
    /// 部门范围
    pub departments: Option<Vec<String>>,
    /// 人员范围
    pub persons: Option<Vec<String>>,
    /// 数据权限范围
#[serde(rename = "data_scope")]
    pub data_scope: Option<String>,
/// 通用响应结构
#[derive(Debug, Clone)]
pub struct FeishuPeopleResponse<T> {,
    /// 响应码
    pub code: i32,
    /// 响应消息
    pub msg: String,
    /// 响应数据
    pub data: Option<T>,
/// 分页响应结构
#[derive(Debug, Clone)]
pub struct PaginatedFeishuPeopleResponse<T> {,
    /// 响应码
    pub code: i32,
    /// 响应消息
    pub msg: String,
    /// 响应数据
    pub data: Option<PaginatedFeishuPeopleData<T>>,
/// 分页数据
#[derive(Debug, Clone)]
pub struct PaginatedFeishuPeopleData<T> {,
    /// 数据列表
    pub items: Option<Vec<T>>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 总数
    pub total: Option<i32>,
}}
}}}}}}}}}}}}}}}}}}}}