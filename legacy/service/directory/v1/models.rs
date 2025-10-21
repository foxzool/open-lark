use serde::{Deserialize, Serialize};

/// 员工信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    /// 员工ID
    pub employee_id: Option<String>,
    /// 员工工号
    pub employee_no: Option<String>,
    /// 姓名
    pub name: Option<String>,
    /// 英文名
    pub en_name: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 手机号
    pub mobile: Option<String>,
    /// 部门ID列表
    pub department_ids: Option<Vec<String>>,
    /// 员工状态
    pub status: Option<EmployeeStatus>,
    /// 入职时间
    pub join_time: Option<String>,
    /// 离职时间
    pub leave_time: Option<String>,
    /// 工作地点
    pub work_location: Option<String>,
    /// 职级
    pub job_level: Option<String>,
    /// 职位
    pub job_title: Option<String>,
    /// 上级ID
    pub leader_id: Option<String>,
}

/// 员工状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmployeeStatus {
    /// 在职
    Active,
    /// 离职
    Inactive,
    /// 待离职
    ToBeResigned,
}

/// 部门信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Department {
    /// 部门ID
    pub department_id: Option<String>,
    /// 部门名称
    pub name: Option<String>,
    /// 英文名称
    pub en_name: Option<String>,
    /// 父部门ID
    pub parent_department_id: Option<String>,
    /// 部门主管ID
    pub leader_id: Option<String>,
    /// 部门状态
    pub status: Option<DepartmentStatus>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
    /// 部门顺序
    pub order: Option<i32>,
}

/// 部门状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DepartmentStatus {
    /// 正常
    Normal,
    /// 删除
    Deleted,
}

/// 用户ID类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserIdType {
    /// Open ID
    OpenId,
    /// Union ID
    UnionId,
    /// User ID
    UserId,
}

impl std::fmt::Display for UserIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserIdType::OpenId => write!(f, "open_id"),
            UserIdType::UnionId => write!(f, "union_id"),
            UserIdType::UserId => write!(f, "user_id"),
        }
    }
}

/// 部门ID类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DepartmentIdType {
    /// Open Department ID
    OpenDepartmentId,
    /// Department ID
    DepartmentId,
}

impl std::fmt::Display for DepartmentIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DepartmentIdType::OpenDepartmentId => write!(f, "open_department_id"),
            DepartmentIdType::DepartmentId => write!(f, "department_id"),
        }
    }
}

/// 分页参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    /// 分页大小
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
}

/// 搜索参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    /// 搜索关键词
    pub query: String,
    /// 搜索范围
    pub scope: Option<String>,
}
