//! EHR人力资源服务数据模型
//!
//! 定义人力资源管理的核心数据结构，包括：
//! - 员工信息管理
//! - 组织架构管理
//! - 职位管理
//! - 薪酬福利
//! - 考勤记录
//! - 绩效管理

use serde::{Deserialize, Serialize};

// ==================== 基础数据类型 ====================

/// 性别
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    /// 男
    Male,
    /// 女
    Female,
    /// 其他
    Other,
}

/// 婚姻状况
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaritalStatus {
    /// 未婚
    Single,
    /// 已婚
    Married,
    /// 离异
    Divorced,
    /// 丧偶
    Widowed,
}

/// 员工状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmployeeStatus {
    /// 在职
    Active,
    /// 离职
    Inactive,
    /// 试用期
    Probation,
    /// 休假
    Leave,
    /// 停薪留职
    Suspended,
}

/// 雇佣类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmploymentType {
    /// 全职
    FullTime,
    /// 兼职
    PartTime,
    /// 实习
    Intern,
    /// 外包
    Contractor,
    /// 顾问
    Consultant,
}

/// 教育程度
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EducationLevel {
    /// 高中及以下
    HighSchool,
    /// 大专
    College,
    /// 本科
    Bachelor,
    /// 硕士
    Master,
    /// 博士
    Doctor,
}

// ==================== 员工信息 ====================

/// 员工基本信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Employee {
    /// 员工ID
    pub employee_id: String,
    /// 工号
    pub employee_number: Option<String>,
    /// 姓名
    pub name: String,
    /// 英文名
    pub english_name: Option<String>,
    /// 性别
    pub gender: Option<Gender>,
    /// 手机号
    pub mobile: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 身份证号
    pub id_card_number: Option<String>,
    /// 出生日期
    pub birth_date: Option<String>,
    /// 婚姻状况
    pub marital_status: Option<MaritalStatus>,
    /// 教育程度
    pub education_level: Option<EducationLevel>,
    /// 毕业院校
    pub graduate_school: Option<String>,
    /// 专业
    pub major: Option<String>,
    /// 入职日期
    pub hire_date: Option<String>,
    /// 转正日期
    pub regular_date: Option<String>,
    /// 合同开始日期
    pub contract_start_date: Option<String>,
    /// 合同结束日期
    pub contract_end_date: Option<String>,
    /// 员工状态
    pub status: Option<EmployeeStatus>,
    /// 雇佣类型
    pub employment_type: Option<EmploymentType>,
    /// 工作地点
    pub work_location: Option<String>,
    /// 居住地址
    pub address: Option<String>,
    /// 紧急联系人
    pub emergency_contact: Option<EmergencyContact>,
    /// 银行账户信息
    pub bank_account: Option<BankAccount>,
    /// 社保公积金信息
    pub social_insurance: Option<SocialInsurance>,
    /// 头像URL
    pub avatar_url: Option<String>,
    /// 所属部门ID
    pub department_id: Option<String>,
    /// 职位ID
    pub position_id: Option<String>,
    /// 直属上级ID
    pub supervisor_id: Option<String>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
}

/// 紧急联系人
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmergencyContact {
    /// 联系人姓名
    pub name: String,
    /// 联系人电话
    pub phone: String,
    /// 与员工关系
    pub relationship: String,
}

/// 银行账户信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankAccount {
    /// 银行名称
    pub bank_name: String,
    /// 账户持有人
    pub account_holder: String,
    /// 账户号码
    pub account_number: String,
}

/// 社保公积金信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SocialInsurance {
    /// 社保缴纳地
    pub payment_location: String,
    /// 社保账号
    pub social_insurance_number: Option<String>,
    /// 公积金账号
    pub housing_fund_number: Option<String>,
    /// 缴纳基数
    pub payment_base: Option<f64>,
}

// ==================== 组织架构 ====================

/// 部门信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Department {
    /// 部门ID
    pub department_id: String,
    /// 部门名称
    pub name: String,
    /// 父部门ID
    pub parent_department_id: Option<String>,
    /// 部门负责人ID
    pub manager_id: Option<String>,
    /// 部门描述
    pub description: Option<String>,
    /// 部门层级
    pub level: Option<i32>,
    /// 排序
    pub order: Option<i32>,
    /// 状态
    pub status: Option<String>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
}

/// 职位信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Position {
    /// 职位ID
    pub position_id: String,
    /// 职位名称
    pub name: String,
    /// 职位编码
    pub code: Option<String>,
    /// 职位级别
    pub level: Option<String>,
    /// 职位序列
    pub sequence: Option<String>,
    /// 所属部门ID
    pub department_id: Option<String>,
    /// 职位描述
    pub description: Option<String>,
    /// 任职要求
    pub requirements: Option<String>,
    /// 薪资范围
    pub salary_range: Option<SalaryRange>,
    /// 状态
    pub status: Option<String>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
}

/// 薪资范围
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SalaryRange {
    /// 最低薪资
    pub min_salary: f64,
    /// 最高薪资
    pub max_salary: f64,
    /// 货币单位
    pub currency: Option<String>,
}

/// 员工职位关系
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmployeePosition {
    /// 员工ID
    pub employee_id: String,
    /// 职位ID
    pub position_id: String,
    /// 部门ID
    pub department_id: String,
    /// 直属上级ID
    pub supervisor_id: Option<String>,
    /// 任职开始时间
    pub start_date: String,
    /// 任职结束时间
    pub end_date: Option<String>,
    /// 是否主职位
    pub is_primary: bool,
    /// 创建时间
    pub create_time: Option<String>,
}

// ==================== 薪酬管理 ====================

/// 薪资信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Salary {
    /// 薪资ID
    pub salary_id: String,
    /// 员工ID
    pub employee_id: String,
    /// 基本工资
    pub base_salary: f64,
    /// 岗位工资
    pub position_salary: Option<f64>,
    /// 绩效工资
    pub performance_salary: Option<f64>,
    /// 津贴补贴
    pub allowances: Option<Vec<Allowance>>,
    /// 扣款项
    pub deductions: Option<Vec<Deduction>>,
    /// 总薪资
    pub total_salary: f64,
    /// 计算周期
    pub pay_period: String,
    /// 生效日期
    pub effective_date: String,
    /// 失效日期
    pub expiry_date: Option<String>,
    /// 货币单位
    pub currency: Option<String>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
}

/// 津贴补贴
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Allowance {
    /// 津贴类型
    pub allowance_type: String,
    /// 津贴名称
    pub name: String,
    /// 金额
    pub amount: f64,
    /// 计算方式
    pub calculation_method: Option<String>,
}

/// 扣款项
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Deduction {
    /// 扣款类型
    pub deduction_type: String,
    /// 扣款名称
    pub name: String,
    /// 金额
    pub amount: f64,
    /// 计算方式
    pub calculation_method: Option<String>,
}

/// 薪资调整记录
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SalaryAdjustment {
    /// 调整ID
    pub adjustment_id: String,
    /// 员工ID
    pub employee_id: String,
    /// 调整类型
    pub adjustment_type: String,
    /// 调整前薪资
    pub previous_salary: f64,
    /// 调整后薪资
    pub new_salary: f64,
    /// 调整金额
    pub adjustment_amount: f64,
    /// 调整原因
    pub reason: String,
    /// 生效日期
    pub effective_date: String,
    /// 审批人ID
    pub approver_id: Option<String>,
    /// 创建时间
    pub create_time: Option<String>,
}

// ==================== 考勤管理 ====================

/// 考勤记录
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AttendanceRecord {
    /// 记录ID
    pub record_id: String,
    /// 员工ID
    pub employee_id: String,
    /// 考勤日期
    pub attendance_date: String,
    /// 上班打卡时间
    pub check_in_time: Option<String>,
    /// 下班打卡时间
    pub check_out_time: Option<String>,
    /// 工作时长（小时）
    pub work_hours: Option<f64>,
    /// 考勤状态
    pub status: AttendanceStatus,
    /// 迟到分钟数
    pub late_minutes: Option<i32>,
    /// 早退分钟数
    pub early_leave_minutes: Option<i32>,
    /// 请假类型
    pub leave_type: Option<String>,
    /// 备注
    pub remarks: Option<String>,
    /// 创建时间
    pub create_time: Option<String>,
}

/// 考勤状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttendanceStatus {
    /// 正常
    Normal,
    /// 迟到
    Late,
    /// 早退
    Early,
    /// 缺卡
    Absent,
    /// 请假
    Leave,
    /// 外勤
    FieldWork,
}

// ==================== 绩效管理 ====================

/// 绩效评估
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceEvaluation {
    /// 评估ID
    pub evaluation_id: String,
    /// 员工ID
    pub employee_id: String,
    /// 评估周期
    pub evaluation_period: String,
    /// 评估类型
    pub evaluation_type: String,
    /// 评估人ID
    pub evaluator_id: String,
    /// 总体评分
    pub overall_score: Option<f64>,
    /// 评估维度
    pub dimensions: Option<Vec<PerformanceDimension>>,
    /// 评语
    pub comments: Option<String>,
    /// 改进建议
    pub improvement_suggestions: Option<String>,
    /// 评估状态
    pub status: EvaluationStatus,
    /// 评估日期
    pub evaluation_date: String,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
}

/// 绩效维度
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceDimension {
    /// 维度名称
    pub name: String,
    /// 维度描述
    pub description: Option<String>,
    /// 权重
    pub weight: f64,
    /// 评分
    pub score: f64,
    /// 评语
    pub comments: Option<String>,
}

/// 评估状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvaluationStatus {
    /// 草稿
    Draft,
    /// 进行中
    InProgress,
    /// 已完成
    Completed,
    /// 已审核
    Reviewed,
}

// ==================== 请求响应模型 ====================

/// 创建员工请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEmployeeRequest {
    /// 员工基本信息
    pub employee: Employee,
    /// 职位信息
    pub position: EmployeePosition,
    /// 薪资信息
    pub salary: Option<Salary>,
}

/// 更新员工请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEmployeeRequest {
    /// 员工ID
    pub employee_id: String,
    /// 更新字段
    pub update_fields: EmployeeUpdateFields,
}

/// 员工更新字段
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmployeeUpdateFields {
    /// 姓名
    pub name: Option<String>,
    /// 手机号
    pub mobile: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 员工状态
    pub status: Option<EmployeeStatus>,
    /// 工作地点
    pub work_location: Option<String>,
    /// 居住地址
    pub address: Option<String>,
    /// 紧急联系人
    pub emergency_contact: Option<EmergencyContact>,
    /// 银行账户
    pub bank_account: Option<BankAccount>,
}

/// 查询员工请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryEmployeeRequest {
    /// 部门ID筛选
    pub department_id: Option<String>,
    /// 职位ID筛选
    pub position_id: Option<String>,
    /// 员工状态筛选
    pub status: Option<EmployeeStatus>,
    /// 雇佣类型筛选
    pub employment_type: Option<EmploymentType>,
    /// 关键词搜索
    pub keyword: Option<String>,
    /// 分页大小
    pub page_size: Option<i32>,
    /// 页面标记
    pub page_token: Option<String>,
}

/// 分页响应基础结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResponse<T> {
    /// 数据项列表
    pub items: Vec<T>,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 总数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

/// 基础响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseResponse<T> {
    /// 状态码
    pub code: i32,
    /// 消息
    pub msg: String,
    /// 数据
    pub data: Option<T>,
}

/// 员工列表响应
pub type EmployeeListResponse = BaseResponse<PageResponse<Employee>>;

/// 员工详情响应
pub type EmployeeResponse = BaseResponse<Employee>;

/// 部门列表响应
pub type DepartmentListResponse = BaseResponse<PageResponse<Department>>;

/// 职位列表响应
pub type PositionListResponse = BaseResponse<PageResponse<Position>>;

/// 薪资列表响应
pub type SalaryListResponse = BaseResponse<PageResponse<Salary>>;

/// 考勤记录列表响应
pub type AttendanceListResponse = BaseResponse<PageResponse<AttendanceRecord>>;

/// 绩效评估列表响应
pub type PerformanceListResponse = BaseResponse<PageResponse<PerformanceEvaluation>>;

/// 空响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyResponse {
    pub code: i32,
    pub msg: String,
}

// 实现Default trait
impl Default for Gender {
    fn default() -> Self {
        Gender::Male
    }
}

impl Default for MaritalStatus {
    fn default() -> Self {
        MaritalStatus::Single
    }
}

impl Default for EmployeeStatus {
    fn default() -> Self {
        EmployeeStatus::Active
    }
}

impl Default for EmploymentType {
    fn default() -> Self {
        EmploymentType::FullTime
    }
}

impl Default for EducationLevel {
    fn default() -> Self {
        EducationLevel::Bachelor
    }
}

impl Default for AttendanceStatus {
    fn default() -> Self {
        AttendanceStatus::Normal
    }
}

impl Default for EvaluationStatus {
    fn default() -> Self {
        EvaluationStatus::Draft
    }
}

impl Default for CreateEmployeeRequest {
    fn default() -> Self {
        Self {
            employee: Employee::default(),
            position: EmployeePosition::default(),
            salary: None,
        }
    }
}

impl Default for UpdateEmployeeRequest {
    fn default() -> Self {
        Self {
            employee_id: String::new(),
            update_fields: EmployeeUpdateFields::default(),
        }
    }
}

impl Default for QueryEmployeeRequest {
    fn default() -> Self {
        Self {
            department_id: None,
            position_id: None,
            status: None,
            employment_type: None,
            keyword: None,
            page_size: None,
            page_token: None,
        }
    }
}

impl Default for PageResponse<Employee> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            page_token: None,
            has_more: Some(false),
            total: Some(0),
        }
    }
}

impl Default for PageResponse<Department> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            page_token: None,
            has_more: Some(false),
            total: Some(0),
        }
    }
}

impl Default for PageResponse<Position> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            page_token: None,
            has_more: Some(false),
            total: Some(0),
        }
    }
}

impl Default for PageResponse<Salary> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            page_token: None,
            has_more: Some(false),
            total: Some(0),
        }
    }
}

impl Default for PageResponse<AttendanceRecord> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            page_token: None,
            has_more: Some(false),
            total: Some(0),
        }
    }
}

impl Default for PageResponse<PerformanceEvaluation> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            page_token: None,
            has_more: Some(false),
            total: Some(0),
        }
    }
}

impl Default for BaseResponse<Employee> {
    fn default() -> Self {
        Self {
            code: 0,
            msg: String::new(),
            data: None,
        }
    }
}

impl Default for EmptyResponse {
    fn default() -> Self {
        Self {
            code: 0,
            msg: String::new(),
        }
    }
}