use serde::{Deserialize, Serialize};
use serde_json::Value;

// ============ 通用结构 ============

/// 多语言文本
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_i18n_text_serialization() {
        let text = I18nText {
            zh_cn: Some("中文".to_string()),
            en_us: Some("English".to_string()),
        };

        let json = serde_json::to_string(&text).unwrap();
        assert!(json.contains("zh_cn"));
        assert!(json.contains("en_us"));

        let _deserialized: I18nText = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn test_i18n_text_empty() {
        let text = I18nText {
            zh_cn: None,
            en_us: None,
        };

        let json = serde_json::to_string(&text).unwrap();
        assert_eq!(json, "{}");

        let _deserialized: I18nText = serde_json::from_str("{}").unwrap();
    }

    #[test]
    fn test_page_response_serialization() {
        let response: PageResponse<String> = PageResponse {
            has_more: Some(true),
            page_token: Some("token_123".to_string()),
            items: Some(vec!["item1".to_string(), "item2".to_string()]),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("has_more"));
        assert!(json.contains("page_token"));
        assert!(json.contains("items"));

        let _deserialized: PageResponse<String> = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn test_page_response_empty() {
        let response: PageResponse<String> = PageResponse {
            has_more: None,
            page_token: None,
            items: None,
        };

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(json, "{}");

        let _deserialized: PageResponse<String> = serde_json::from_str("{}").unwrap();
    }

    #[test]
    fn test_custom_field_complete() {
        let field = CustomField {
            field_name: Some("employee_level".to_string()),
            value: Some(serde_json::json!({"level": "senior", "years": 5})),
        };

        let json = serde_json::to_string(&field).unwrap();
        assert!(json.contains("employee_level"));
        assert!(json.contains("level"));

        let deserialized: CustomField = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.field_name, Some("employee_level".to_string()));
    }

    #[test]
    fn test_employee_basic_info() {
        let employee = Employee {
            employee_id: Some("emp_001".to_string()),
            user_id: Some("user_001".to_string()),
            employee_number: Some("E001".to_string()),
            employment_status: Some("active".to_string()),
            person: None,
            employment: None,
            job_datas: None,
            custom_fields: None,
        };

        let json = serde_json::to_string(&employee).unwrap();
        assert!(json.contains("emp_001"));
        assert!(json.contains("E001"));
        assert!(json.contains("active"));

        let _deserialized: Employee = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn test_person_complete_profile() {
        let person = Person {
            person_id: Some("person_001".to_string()),
            name: Some(I18nText {
                zh_cn: Some("张三".to_string()),
                en_us: Some("John Zhang".to_string()),
            }),
            english_name: Some("John Zhang".to_string()),
            name_pinyin: Some("zhang san".to_string()),
            gender: Some("male".to_string()),
            date_of_birth: Some("1990-06-15".to_string()),
            nationality_id: Some("CHN".to_string()),
            phone: Some("13800138000".to_string()),
            email: Some("zhang.san@email.com".to_string()),
            national_ids: Some(vec![NationalId {
                national_id_type_id: Some("id_card".to_string()),
                national_id_number: Some("110101199006150123".to_string()),
                start_date: None,
                end_date: None,
                issued_location: None,
            }]),
            addresses: None,
            marital_status: Some("single".to_string()),
            political_status: Some("member".to_string()),
            ethnicity: Some("han".to_string()),
            hukou_type: Some("urban".to_string()),
            hukou_location: Some("Beijing".to_string()),
        };

        let json = serde_json::to_string(&person).unwrap();
        assert!(json.contains("person_001"));
        assert!(json.contains("张三"));
        assert!(json.contains("13800138000"));

        let _deserialized: Person = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn test_department_creation() {
        let department = Department {
            department_id: Some("dept_001".to_string()),
            name: Some(I18nText {
                zh_cn: Some("技术部".to_string()),
                en_us: Some("Technology Department".to_string()),
            }),
            parent_department_id: Some("root_dept".to_string()),
            manager: Some("mgr_001".to_string()),
            code: Some("TECH".to_string()),
            description: Some(I18nText {
                zh_cn: Some("负责技术研发工作".to_string()),
                en_us: Some("Responsible for technology R&D".to_string()),
            }),
            active: Some(true),
            effective_time: Some("2023-01-01 00:00:00".to_string()),
            expiration_time: Some("2025-12-31 23:59:59".to_string()),
            children: None,
            custom_fields: None,
        };

        let json = serde_json::to_string(&department).unwrap();
        assert!(json.contains("技术部"));
        assert!(json.contains("TECH"));
        assert!(json.contains("Technology Department"));

        let _deserialized: Department = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn test_employment_details() {
        let employment = Employment {
            employment_id: Some("employment_001".to_string()),
            employment_type: Some("full_time".to_string()),
            employee_type_id: Some("regular".to_string()),
            hire_date: Some("2023-01-15".to_string()),
            end_date: None,
            status: Some("active".to_string()),
            working_hours_type_id: Some("standard".to_string()),
            work_email: Some("employee@example.com".to_string()),
            work_phone: Some("010-12345678".to_string()),
            probation_start_date: Some("2023-01-15".to_string()),
            probation_end_date: Some("2023-07-15".to_string()),
            probation_status: Some("in_progress".to_string()),
        };

        let json = serde_json::to_string(&employment).unwrap();
        assert!(json.contains("employment_001"));
        assert!(json.contains("full_time"));
        assert!(json.contains("2023-01-15"));

        let _deserialized: Employment = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn test_job_data_comprehensive() {
        let job = JobData {
            job_data_id: Some("job_data_001".to_string()),
            version_id: Some("v1".to_string()),
            job_id: Some("job_001".to_string()),
            job_level_id: Some("level_p6".to_string()),
            job_grade_id: Some("grade_1".to_string()),
            job_family_id: Some("tech".to_string()),
            department_id: Some("dept_001".to_string()),
            location_id: Some("beijing".to_string()),
            company_id: Some("company_001".to_string()),
            cost_center_id: Some("cost_center_01".to_string()),
            work_location: Some("北京望京".to_string()),
            direct_manager_id: Some("manager_001".to_string()),
            dotted_line_manager_id: None,
            effective_time: Some("2023-01-01 00:00:00".to_string()),
            expiration_time: Some("2024-12-31 23:59:59".to_string()),
            assignment_start_reason: Some("new_hire".to_string()),
        };

        let json = serde_json::to_string(&job).unwrap();
        assert!(json.contains("job_data_001"));
        assert!(json.contains("level_p6"));
        assert!(json.contains("北京望京"));

        let _deserialized: JobData = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn test_national_id_validation() {
        let national_id = NationalId {
            national_id_type_id: Some("passport".to_string()),
            national_id_number: Some("G12345678".to_string()),
            start_date: Some("2023-01-01".to_string()),
            end_date: Some("2033-01-01".to_string()),
            issued_location: Some("Beijing".to_string()),
        };

        let json = serde_json::to_string(&national_id).unwrap();
        assert!(json.contains("passport"));
        assert!(json.contains("G12345678"));

        let _deserialized: NationalId = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn test_address_complete() {
        let address = Address {
            country_region_id: Some("CHN".to_string()),
            region_id: Some("beijing".to_string()),
            city_id: Some("beijing_city".to_string()),
            district_id: Some("chaoyang".to_string()),
            address_line_1: Some("北京市朝阳区xxx街道xxx号".to_string()),
            address_line_2: Some("软件园2号楼".to_string()),
            postal_code: Some("100000".to_string()),
            address_type: Some("home".to_string()),
        };

        let json = serde_json::to_string(&address).unwrap();
        assert!(json.contains("北京市"));
        assert!(json.contains("100000"));
        assert!(json.contains("chaoyang"));

        let _deserialized: Address = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn test_country_region_info() {
        let country = CountryRegion {
            country_region_id: Some("CHN".to_string()),
            name: Some(I18nText {
                zh_cn: Some("中国".to_string()),
                en_us: Some("China".to_string()),
            }),
            full_name: Some(I18nText {
                zh_cn: Some("中华人民共和国".to_string()),
                en_us: Some("People's Republic of China".to_string()),
            }),
            code: Some("CN".to_string()),
            time_zone: Some("Asia/Shanghai".to_string()),
        };

        let json = serde_json::to_string(&country).unwrap();
        assert!(json.contains("中国"));
        assert!(json.contains("中华人民共和国"));
        assert!(json.contains("\"CN\""));
        assert!(json.contains("Asia/Shanghai"));

        let _deserialized: CountryRegion = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn test_nationality_info() {
        let nationality = Nationality {
            nationality_id: Some("CHN".to_string()),
            name: Some(I18nText {
                zh_cn: Some("中国".to_string()),
                en_us: Some("Chinese".to_string()),
            }),
            code: Some("CN".to_string()),
        };

        let json = serde_json::to_string(&nationality).unwrap();
        assert!(json.contains("中国"));
        assert!(json.contains("Chinese"));
        assert!(json.contains("CHN"));

        let _deserialized: Nationality = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn test_enum_info_structure() {
        let enum_info = EnumInfo {
            enum_value: Some("employment_type".to_string()),
            content: Some(I18nText {
                zh_cn: Some("雇佣类型".to_string()),
                en_us: Some("Employment Type".to_string()),
            }),
            enum_type: Some("hr".to_string()),
            enum_status: Some(1),
        };

        let json = serde_json::to_string(&enum_info).unwrap();
        assert!(json.contains("雇佣类型"));
        assert!(json.contains("Employment Type"));

        let _deserialized: EnumInfo = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn test_page_response_generic_type() {
        let employee_response: PageResponse<Employee> = PageResponse {
            has_more: Some(false),
            page_token: None,
            items: Some(vec![Employee {
                employee_id: Some("emp_test".to_string()),
                user_id: Some("user_test".to_string()),
                employee_number: Some("E999".to_string()),
                employment_status: Some("active".to_string()),
                person: None,
                employment: None,
                job_datas: None,
                custom_fields: None,
            }]),
        };

        let json = serde_json::to_string(&employee_response).unwrap();
        assert!(json.contains("emp_test"));
        assert!(json.contains("user_test"));
        assert!(json.contains("E999"));

        let _deserialized: PageResponse<Employee> = serde_json::from_str(&json).unwrap();
    }
}
