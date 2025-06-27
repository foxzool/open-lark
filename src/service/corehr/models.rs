use serde::{Deserialize, Serialize};
use serde_json::Value;

// ============ 通用结构 ============

/// 多语言文本
#[derive(Debug, Serialize, Deserialize)]
pub struct I18nText {
    /// 中文
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 英文
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

/// 分页响应
#[derive(Debug, Serialize, Deserialize)]
pub struct PageResponse<T> {
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 数据列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<T>>,
}

/// 自定义字段
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomField {
    /// 字段ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    /// 字段值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}

// ============ 基础数据相关结构 ============

/// 查询枚举信息请求
#[derive(Debug, Serialize, Deserialize)]
pub struct EnumSearchRequest {
    /// 枚举类型
    pub enum_type: String,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 枚举信息
#[derive(Debug, Serialize, Deserialize)]
pub struct EnumInfo {
    /// 枚举值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_value: Option<String>,
    /// 枚举名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<I18nText>,
    /// 枚举类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_type: Option<String>,
    /// 是否启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_status: Option<i32>,
}

/// 查询国家/地区信息请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CountryRegionSearchRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 国家/地区信息
#[derive(Debug, Serialize, Deserialize)]
pub struct CountryRegion {
    /// 国家/地区ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    /// 国家/地区名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 全称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<I18nText>,
    /// 国家代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 时区
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

/// 查询国籍信息请求
#[derive(Debug, Serialize, Deserialize)]
pub struct NationalitySearchRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 国籍信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Nationality {
    /// 国籍ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality_id: Option<String>,
    /// 国籍名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 国籍代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

// ============ 员工信息相关结构 ============

/// 批量查询员工信息请求
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeBatchGetRequest {
    /// 员工ID列表
    pub employee_ids: Vec<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 员工ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id_type: Option<String>,
    /// 部门ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
    /// 查询字段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
}

/// 搜索员工信息请求
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeSearchRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 员工ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id_type: Option<String>,
    /// 部门ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
    /// 查询字段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
    /// 员工状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_status: Option<Vec<String>>,
    /// 部门ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 工号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_number: Option<String>,
}

/// 员工基本信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    /// 员工ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 员工工号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_number: Option<String>,
    /// 员工状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_status: Option<String>,
    /// 个人信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person: Option<Person>,
    /// 雇佣信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment: Option<Employment>,
    /// 任职信息列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_datas: Option<Vec<JobData>>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
}

/// 个人信息
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Person {
    /// 个人信息ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_id: Option<String>,
    /// 姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 英文姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub english_name: Option<String>,
    /// 姓名拼音
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_pinyin: Option<String>,
    /// 性别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// 出生日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// 国籍ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality_id: Option<String>,
    /// 电话号码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 身份证件信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_ids: Option<Vec<NationalId>>,
    /// 地址信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<Address>>,
    /// 婚姻状况
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<String>,
    /// 政治面貌
    #[serde(skip_serializing_if = "Option::is_none")]
    pub political_status: Option<String>,
    /// 民族
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethnicity: Option<String>,
    /// 户口性质
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hukou_type: Option<String>,
    /// 户口所在地
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hukou_location: Option<String>,
}

/// 身份证件信息
#[derive(Debug, Serialize, Deserialize)]
pub struct NationalId {
    /// 国家证件类型ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_id_type_id: Option<String>,
    /// 证件号码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_id_number: Option<String>,
    /// 证件有效期开始日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 证件有效期结束日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 证件签发地
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_location: Option<String>,
}

/// 地址信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    /// 国家/地区ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    /// 省份/州ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 城市ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city_id: Option<String>,
    /// 区/县ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district_id: Option<String>,
    /// 详细地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line_1: Option<String>,
    /// 地址补充信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line_2: Option<String>,
    /// 邮政编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// 地址类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_type: Option<String>,
}

/// 雇佣信息
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Employment {
    /// 雇佣ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 雇佣类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_type: Option<String>,
    /// 人员类型ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_type_id: Option<String>,
    /// 入职日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hire_date: Option<String>,
    /// 离职日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 雇佣状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 工时制度ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_hours_type_id: Option<String>,
    /// 邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_email: Option<String>,
    /// 工作电话
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_phone: Option<String>,
    /// 试用期开始日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_start_date: Option<String>,
    /// 试用期结束日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_end_date: Option<String>,
    /// 试用期状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_status: Option<String>,
}

/// 任职信息
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JobData {
    /// 任职记录ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_data_id: Option<String>,
    /// 版本ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// 职务ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// 职级ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    /// 职等ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_grade_id: Option<String>,
    /// 序列ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    /// 部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 地点ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    /// 公司ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    /// 成本中心ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    /// 工作地点
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_location: Option<String>,
    /// 汇报关系
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_manager_id: Option<String>,
    /// 虚线汇报关系
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dotted_line_manager_id: Option<String>,
    /// 生效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 失效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// 任职原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_start_reason: Option<String>,
}

// ============ 组织管理相关结构 ============

/// 创建部门请求
#[derive(Debug, Serialize, Deserialize)]
pub struct DepartmentCreateRequest {
    /// 部门名称
    pub name: I18nText,
    /// 上级部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    /// 部门负责人ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager: Option<String>,
    /// 部门编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 部门描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<I18nText>,
    /// 生效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
}

/// 部门信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Department {
    /// 部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 部门名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 上级部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    /// 部门负责人ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager: Option<String>,
    /// 部门编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 部门描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<I18nText>,
    /// 是否启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 生效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 失效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// 子部门列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Department>>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
}

/// 创建公司请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyCreateRequest {
    /// 公司名称
    pub name: I18nText,
    /// 公司类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_type: Option<String>,
    /// 法定公司名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<I18nText>,
    /// 公司编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 生效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 主要地点ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_location_id: Option<String>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
}

/// 公司信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Company {
    /// 公司ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    /// 公司名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 公司类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_type: Option<String>,
    /// 法定公司名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<I18nText>,
    /// 公司编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 生效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 失效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// 主要地点ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_location_id: Option<String>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
}

// ============ 通用响应结构 ============

/// CoreHR响应包装器
#[derive(Debug, Serialize, Deserialize)]
pub struct CoreHRResponse<T> {
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    /// 错误码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    /// 错误消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
}

// ============ ID转换相关结构 ============

/// ID转换请求
#[derive(Debug, Serialize, Deserialize)]
pub struct IdConvertRequest {
    /// 源ID类型
    pub source_id_type: String,
    /// 目标ID类型
    pub target_id_type: String,
    /// ID列表
    pub ids: Vec<String>,
}

/// ID转换结果
#[derive(Debug, Serialize, Deserialize)]
pub struct IdConvertResult {
    /// 源ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// 目标ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
}

// ============ 岗职务管理相关结构 ============

/// 创建序列请求
#[derive(Debug, Serialize, Deserialize)]
pub struct JobFamilyCreateRequest {
    /// 序列名称
    pub name: I18nText,
    /// 序列编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 生效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
}

/// 序列信息
#[derive(Debug, Serialize, Deserialize)]
pub struct JobFamily {
    /// 序列ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    /// 序列名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 序列编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 生效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 失效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
}

/// 创建职级请求
#[derive(Debug, Serialize, Deserialize)]
pub struct JobLevelCreateRequest {
    /// 职级名称
    pub name: I18nText,
    /// 职级编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 职级描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<I18nText>,
    /// 职级顺序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    /// 生效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
}

/// 职级信息
#[derive(Debug, Serialize, Deserialize)]
pub struct JobLevel {
    /// 职级ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    /// 职级名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 职级编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 职级描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<I18nText>,
    /// 职级顺序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    /// 是否启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 生效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 失效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
}

/// 创建职等请求
#[derive(Debug, Serialize, Deserialize)]
pub struct JobGradeCreateRequest {
    /// 职等名称
    pub name: I18nText,
    /// 职等编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 职等描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<I18nText>,
    /// 所属序列ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    /// 生效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
}

/// 职等信息
#[derive(Debug, Serialize, Deserialize)]
pub struct JobGrade {
    /// 职等ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_grade_id: Option<String>,
    /// 职等名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 职等编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 职等描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<I18nText>,
    /// 所属序列ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    /// 是否启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 生效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 失效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
}

/// 创建职务请求
#[derive(Debug, Serialize, Deserialize)]
pub struct JobCreateRequest {
    /// 职务名称
    pub name: I18nText,
    /// 职务编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 职务描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<I18nText>,
    /// 所属序列ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    /// 生效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
}

/// 职务信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    /// 职务ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// 职务名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 职务编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 职务描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<I18nText>,
    /// 所属序列ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    /// 是否启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 生效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 失效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
}

// ============ 入职/离职/异动相关结构 ============

/// 待入职信息创建请求
#[derive(Debug, Serialize, Deserialize)]
pub struct PreHireCreateRequest {
    /// 员工基本信息
    pub person: Person,
    /// 任职信息
    pub employment: Employment,
    /// 任职信息列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_datas: Option<Vec<JobData>>,
    /// 入职流程ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onboarding_flow_id: Option<String>,
    /// 预期入职日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_hire_date: Option<String>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
}

/// 待入职信息搜索请求
#[derive(Debug, Serialize, Deserialize)]
pub struct PreHireSearchRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 员工ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id_type: Option<String>,
    /// 部门ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
    /// 入职状态列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onboarding_status: Option<Vec<String>>,
    /// 部门ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
}

/// 待入职信息
#[derive(Debug, Serialize, Deserialize)]
pub struct PreHire {
    /// 待入职ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_hire_id: Option<String>,
    /// 员工基本信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person: Option<Person>,
    /// 雇佣信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment: Option<Employment>,
    /// 任职信息列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_datas: Option<Vec<JobData>>,
    /// 入职状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onboarding_status: Option<String>,
    /// 入职流程ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onboarding_flow_id: Option<String>,
    /// 预期入职日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_hire_date: Option<String>,
    /// 实际入职日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_hire_date: Option<String>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
}

/// 异动发起请求
#[derive(Debug, Serialize, Deserialize)]
pub struct JobChangeCreateRequest {
    /// 员工ID
    pub employee_id: String,
    /// 异动类型ID
    pub job_change_type_id: String,
    /// 异动原因ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_change_reason_id: Option<String>,
    /// 生效日期
    pub effective_date: String,
    /// 新的任职信息
    pub job_data: JobData,
    /// 异动说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
}

/// 异动搜索请求
#[derive(Debug, Serialize, Deserialize)]
pub struct JobChangeSearchRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 员工ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id_type: Option<String>,
    /// 部门ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
    /// 员工ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_ids: Option<Vec<String>>,
    /// 异动状态列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_change_status: Option<Vec<String>>,
    /// 异动类型ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_change_type_ids: Option<Vec<String>>,
}

/// 异动信息
#[derive(Debug, Serialize, Deserialize)]
pub struct JobChange {
    /// 异动ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_change_id: Option<String>,
    /// 员工ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    /// 异动类型ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_change_type_id: Option<String>,
    /// 异动原因ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_change_reason_id: Option<String>,
    /// 异动状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_change_status: Option<String>,
    /// 生效日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// 原任职信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_job_data: Option<JobData>,
    /// 新任职信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_job_data: Option<JobData>,
    /// 异动说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
}

/// 离职发起请求
#[derive(Debug, Serialize, Deserialize)]
pub struct OffboardingCreateRequest {
    /// 员工ID
    pub employee_id: String,
    /// 离职原因ID
    pub offboarding_reason_id: String,
    /// 离职日期
    pub offboarding_date: String,
    /// 离职类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offboarding_type: Option<String>,
    /// 离职说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
}

/// 离职搜索请求
#[derive(Debug, Serialize, Deserialize)]
pub struct OffboardingSearchRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 员工ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id_type: Option<String>,
    /// 部门ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
    /// 员工ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_ids: Option<Vec<String>>,
    /// 离职状态列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offboarding_status: Option<Vec<String>>,
    /// 离职原因ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offboarding_reason_ids: Option<Vec<String>>,
}

/// 离职信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Offboarding {
    /// 离职ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offboarding_id: Option<String>,
    /// 员工ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    /// 离职原因ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offboarding_reason_id: Option<String>,
    /// 离职状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offboarding_status: Option<String>,
    /// 离职日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offboarding_date: Option<String>,
    /// 离职类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offboarding_type: Option<String>,
    /// 离职说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
}
