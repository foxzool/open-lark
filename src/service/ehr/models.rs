use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// ============ 员工花名册相关结构 ============

/// 批量获取员工花名册信息请求
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeListRequest {
    /// 分页大小，最大值100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记，当has_more为true时，会同时返回新的page_token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 员工状态筛选，不传则返回所有状态的员工
    /// 可选值：active(在职)、inactive(离职)、trial(试用期)等
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 部门ID，用于筛选特定部门的员工
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 用户ID类型，支持open_id、union_id、user_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门ID类型，支持open_department_id、department_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
    /// 是否包含离职员工，默认false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_resigned: Option<bool>,
    /// 查询的字段列表，不传则返回所有字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
}

/// 员工信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    /// 员工ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    /// 用户ID（根据user_id_type返回对应类型）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 员工工号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_number: Option<String>,
    /// 员工姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 英文姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    /// 员工邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 手机号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// 性别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// 生日
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,
    /// 身份证号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,
    /// 员工状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<EmployeeStatus>,
    /// 部门信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_info: Option<EmployeeDepartment>,
    /// 职位信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_info: Option<EmployeeJob>,
    /// 入职信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hire_info: Option<EmployeeHire>,
    /// 个人信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_info: Option<EmployeePersonal>,
    /// 教育经历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub education_info: Option<Vec<EmployeeEducation>>,
    /// 工作经历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_experience: Option<Vec<EmployeeWorkExperience>>,
    /// 紧急联系人信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emergency_contact: Option<Vec<EmergencyContact>>,
    /// 银行卡信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<Vec<BankAccount>>,
    /// 社保信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_security: Option<SocialSecurity>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<HashMap<String, Value>>,
}

/// 员工状态信息
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeStatus {
    /// 员工状态：active(在职)、inactive(离职)、trial(试用期)等
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 状态生效日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// 离职日期（如果是离职状态）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resignation_date: Option<String>,
    /// 离职原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resignation_reason: Option<String>,
}

/// 员工部门信息
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeDepartment {
    /// 部门ID（根据department_id_type返回对应类型）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 部门名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_name: Option<String>,
    /// 上级部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    /// 部门路径
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_path: Option<String>,
}

/// 员工职位信息
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeJob {
    /// 职位名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    /// 职级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_level: Option<String>,
    /// 职位序列
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family: Option<String>,
    /// 直属上级用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supervisor_id: Option<String>,
    /// 直属上级姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supervisor_name: Option<String>,
    /// 员工类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_type: Option<String>,
    /// 工作地点
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_location: Option<String>,
}

/// 员工入职信息
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeHire {
    /// 入职日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hire_date: Option<String>,
    /// 试用期结束日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_end_date: Option<String>,
    /// 合同类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_type: Option<String>,
    /// 合同开始日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_start_date: Option<String>,
    /// 合同结束日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_end_date: Option<String>,
}

/// 员工个人信息
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeePersonal {
    /// 国籍
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    /// 民族
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethnicity: Option<String>,
    /// 婚姻状况
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<String>,
    /// 政治面貌
    #[serde(skip_serializing_if = "Option::is_none")]
    pub political_status: Option<String>,
    /// 户籍地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_address: Option<String>,
    /// 现居住地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_address: Option<String>,
    /// 毕业院校
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graduate_school: Option<String>,
    /// 最高学历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highest_education: Option<String>,
}

/// 教育经历
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeEducation {
    /// 学校名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub school_name: Option<String>,
    /// 专业
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<String>,
    /// 学历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub degree: Option<String>,
    /// 开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 结束时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

/// 工作经历
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeWorkExperience {
    /// 公司名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// 职位
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// 开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 结束时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 工作描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 紧急联系人
#[derive(Debug, Serialize, Deserialize)]
pub struct EmergencyContact {
    /// 姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 关系
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<String>,
    /// 电话
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

/// 银行卡信息
#[derive(Debug, Serialize, Deserialize)]
pub struct BankAccount {
    /// 银行名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    /// 银行卡号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// 开户行
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    /// 账户类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
}

/// 社保信息
#[derive(Debug, Serialize, Deserialize)]
pub struct SocialSecurity {
    /// 社保号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_security_number: Option<String>,
    /// 公积金号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub housing_fund_number: Option<String>,
    /// 社保缴纳地
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_security_location: Option<String>,
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

// ============ 附件下载相关结构 ============

/// 下载人员附件请求
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeAttachmentRequest {
    /// 员工ID
    pub employee_id: String,
    /// 附件ID
    pub attachment_id: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

/// 附件信息
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeAttachment {
    /// 附件ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    /// 附件名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// 附件类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    /// 附件大小（字节）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// 附件分类
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// 上传时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_time: Option<String>,
    /// 附件描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 附件下载响应
#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentDownloadResponse {
    /// 文件内容（base64编码）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_content: Option<String>,
    /// 文件名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// 文件类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// 文件大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}

// ============ 通用响应结构 ============

/// EHR响应包装器
#[derive(Debug, Serialize, Deserialize)]
pub struct EhrResponse<T> {
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
