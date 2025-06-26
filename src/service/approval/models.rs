use serde::{Deserialize, Serialize};

/// 用户ID类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserIdType {
    /// 用户ID
    #[serde(rename = "user_id")]
    UserId,
    /// union_id
    #[serde(rename = "union_id")]
    UnionId,
    /// open_id
    #[serde(rename = "open_id")]
    OpenId,
}

impl UserIdType {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserIdType::UserId => "user_id",
            UserIdType::UnionId => "union_id",
            UserIdType::OpenId => "open_id",
        }
    }
}

/// 部门ID类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DepartmentIdType {
    /// 部门ID
    #[serde(rename = "department_id")]
    DepartmentId,
    /// open_department_id
    #[serde(rename = "open_department_id")]
    OpenDepartmentId,
}

impl DepartmentIdType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DepartmentIdType::DepartmentId => "department_id",
            DepartmentIdType::OpenDepartmentId => "open_department_id",
        }
    }
}

/// 审批状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ApprovalStatus {
    /// 审批中
    #[serde(rename = "PENDING")]
    Pending,
    /// 通过
    #[serde(rename = "APPROVED")]
    Approved,
    /// 拒绝
    #[serde(rename = "REJECTED")]
    Rejected,
    /// 撤回
    #[serde(rename = "CANCELED")]
    Canceled,
    /// 已删除
    #[serde(rename = "DELETED")]
    Deleted,
}

/// 任务状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    /// 待处理
    #[serde(rename = "PENDING")]
    Pending,
    /// 已通过
    #[serde(rename = "APPROVED")]
    Approved,
    /// 已拒绝
    #[serde(rename = "REJECTED")]
    Rejected,
    /// 已转交
    #[serde(rename = "TRANSFERRED")]
    Transferred,
    /// 已完成
    #[serde(rename = "DONE")]
    Done,
}

/// 审批定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Approval {
    /// 审批定义编码
    pub approval_code: String,
    /// 审批定义名称
    pub approval_name: String,
    /// 审批定义描述
    pub description: Option<String>,
    /// 审批定义状态
    pub status: Option<String>,
    /// 创建者
    pub creator: Option<UserInfo>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
    /// 表单字段
    pub form: Option<Vec<FormField>>,
    /// 审批流程
    pub process: Option<ApprovalProcess>,
}

/// 审批实例
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalInstance {
    /// 审批实例编码
    pub instance_code: String,
    /// 审批定义编码
    pub approval_code: String,
    /// 审批定义名称
    pub approval_name: Option<String>,
    /// 发起人
    pub initiator: Option<UserInfo>,
    /// 审批状态
    pub status: ApprovalStatus,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
    /// 表单数据
    pub form: Option<Vec<FormData>>,
    /// 审批流程
    pub timeline: Option<Vec<ApprovalNode>>,
    /// 抄送人
    pub cc_users: Option<Vec<UserInfo>>,
}

/// 审批任务
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalTask {
    /// 任务ID
    pub task_id: String,
    /// 审批实例编码
    pub instance_code: String,
    /// 审批定义编码
    pub approval_code: String,
    /// 审批定义名称
    pub approval_name: Option<String>,
    /// 发起人
    pub initiator: Option<UserInfo>,
    /// 审批人
    pub approver: Option<UserInfo>,
    /// 任务状态
    pub status: TaskStatus,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
    /// 任务链接
    pub task_links: Option<Vec<TaskLink>>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    pub name: Option<String>,
    /// 用户头像
    pub avatar: Option<String>,
    /// 用户邮箱
    pub email: Option<String>,
}

/// 表单字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormField {
    /// 字段ID
    pub id: String,
    /// 字段名称
    pub name: String,
    /// 字段类型
    pub field_type: String,
    /// 是否必填
    pub required: Option<bool>,
    /// 字段属性
    pub properties: Option<serde_json::Value>,
}

/// 表单数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormData {
    /// 字段ID
    pub id: String,
    /// 字段名称
    pub name: Option<String>,
    /// 字段值
    pub value: Option<serde_json::Value>,
}

/// 审批流程
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalProcess {
    /// 流程节点
    pub nodes: Vec<ProcessNode>,
}

/// 流程节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessNode {
    /// 节点ID
    pub node_id: String,
    /// 节点名称
    pub node_name: Option<String>,
    /// 节点类型
    pub node_type: String,
    /// 审批人
    pub approvers: Option<Vec<UserInfo>>,
}

/// 审批节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalNode {
    /// 节点ID
    pub node_id: String,
    /// 节点名称
    pub node_name: Option<String>,
    /// 节点类型
    pub node_type: String,
    /// 审批人
    pub approver: Option<UserInfo>,
    /// 审批状态
    pub status: Option<String>,
    /// 审批时间
    pub approve_time: Option<String>,
    /// 审批意见
    pub comment: Option<String>,
}

/// 任务链接
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskLink {
    /// 链接平台
    pub platform: String,
    /// 链接地址
    pub link: String,
}

/// 审批文件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalFile {
    /// 文件ID
    pub file_id: String,
    /// 文件名
    pub filename: String,
    /// 文件大小
    pub file_size: Option<i64>,
    /// 文件类型
    pub file_type: Option<String>,
    /// 上传时间
    pub upload_time: Option<String>,
}

/// 审批评论
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalComment {
    /// 评论ID
    pub comment_id: String,
    /// 评论内容
    pub content: String,
    /// 评论者
    pub commenter: Option<UserInfo>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
    /// 附件
    pub attachments: Option<Vec<CommentAttachment>>,
}

/// 评论附件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentAttachment {
    /// 附件ID
    pub attachment_id: String,
    /// 附件名称
    pub name: String,
    /// 附件类型
    pub attachment_type: String,
    /// 附件链接
    pub link: Option<String>,
}
