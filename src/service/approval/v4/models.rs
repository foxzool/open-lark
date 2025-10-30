//! Approval服务数据模型
//!
//! 定义审批相关的数据结构，包括审批定义、实例、任务等核心实体。

use serde::{Deserialize, Serialize};

/// 基础审批响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalBaseResponse<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    pub name: Option<String>,
    /// 用户邮箱
    pub email: Option<String>,
    /// 用户头像
    pub avatar: Option<String>,
}

/// 表单字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormField {
    /// 字段键
    pub key: String,
    /// 字段名称
    pub name: String,
    /// 字段类型
    pub field_type: String,
    /// 字段值
    pub value: Option<serde_json::Value>,
    /// 是否必填
    pub required: Option<bool>,
}

/// 审批节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalNode {
    /// 节点ID
    pub node_id: String,
    /// 节点名称
    pub node_name: String,
    /// 节点类型
    pub node_type: String,
    /// 处理人列表
    pub approvers: Vec<UserInfo>,
    /// 是否必须全部同意
    pub require_all_approve: Option<bool>,
}

/// 审批流程
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalProcess {
    /// 流程ID
    pub process_id: String,
    /// 流程名称
    pub process_name: String,
    /// 节点列表
    pub nodes: Vec<ApprovalNode>,
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
    pub form_data: Option<serde_json::Value>,
    /// 审批任务列表
    pub tasks: Option<Vec<ApprovalTask>>,
    /// 当前节点
    pub current_node: Option<String>,
    /// 备注
    pub comment: Option<String>,
}

/// 审批任务
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalTask {
    /// 任务ID
    pub task_id: String,
    /// 实例编码
    pub instance_code: String,
    /// 任务名称
    pub task_name: String,
    /// 处理人
    pub approver: Option<UserInfo>,
    /// 任务状态
    pub status: TaskStatus,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
    /// 截止时间
    pub due_time: Option<String>,
    /// 处理意见
    pub comment: Option<String>,
}

/// 审批状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ApprovalStatus {
    /// 审批中
    InProgress,
    /// 通过
    Approved,
    /// 拒绝
    Rejected,
    /// 撤回
    Canceled,
    /// 已删除
    Deleted,
}

/// 任务状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TaskStatus {
    /// 待处理
    Pending,
    /// 已通过
    Approved,
    /// 已拒绝
    Rejected,
    /// 已转交
    Transferred,
    /// 已完成
    Done,
}

/// 创建审批实例请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInstanceRequest {
    /// 审批定义编码
    pub approval_code: String,
    /// 表单数据
    pub form: Option<serde_json::Value>,
    /// 发起人用户ID
    pub user_id: Option<String>,
    /// 发起人部门ID
    pub department_id: Option<String>,
    /// 审批实例UUID
    pub uuid: Option<String>,
    /// 用户ID类型
    pub user_id_type: Option<String>,
    /// 部门ID类型
    pub department_id_type: Option<String>,
}

/// 创建审批实例响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInstanceResponse {
    /// 审批实例编码
    pub instance_code: String,
    /// 审批实例UUID
    pub uuid: String,
}

/// 处理审批任务请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessTaskRequest {
    /// 任务ID
    pub task_id: String,
    /// 处理动作
    pub action: TaskAction,
    /// 处理意见
    pub comment: Option<String>,
    /// 转交人ID（转交时使用）
    pub transfer_to_user_id: Option<String>,
    /// 加签人ID列表（加签时使用）
    pub add_approvers: Option<Vec<String>>,
}

/// 任务处理动作
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskAction {
    /// 同意
    Approve,
    /// 拒绝
    Reject,
    /// 转交
    Transfer,
    /// 退回
    Rollback,
    /// 加签
    AddApprover,
}

/// 处理审批任务响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessTaskResponse {
    /// 任务ID
    pub task_id: String,
    /// 处理结果
    pub success: bool,
    /// 消息
    pub message: Option<String>,
}

/// 查询审批实例请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryInstanceRequest {
    /// 审批定义编码（可选）
    pub approval_code: Option<String>,
    /// 发起人ID（可选）
    pub initiator_id: Option<String>,
    /// 审批状态（可选）
    pub status: Option<ApprovalStatus>,
    /// 开始时间（可选）
    pub start_time: Option<String>,
    /// 结束时间（可选）
    pub end_time: Option<String>,
    /// 分页大小
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 用户ID类型
    pub user_id_type: Option<String>,
}

/// 查询审批实例响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryInstanceResponse {
    /// 审批实例列表
    pub instances: Vec<ApprovalInstance>,
    /// 总数
    pub total: i32,
    /// 是否有更多
    pub has_more: bool,
    /// 下一页标记
    pub next_page_token: Option<String>,
}

/// 获取审批任务请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTaskRequest {
    /// 任务ID
    pub task_id: String,
    /// 用户ID类型
    pub user_id_type: Option<String>,
}

/// 获取审批任务响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTaskResponse {
    /// 审批任务
    pub task: ApprovalTask,
}

/// 查询审批任务请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryTaskRequest {
    /// 处理人ID（可选）
    pub approver_id: Option<String>,
    /// 任务状态（可选）
    pub status: Option<TaskStatus>,
    /// 实例编码（可选）
    pub instance_code: Option<String>,
    /// 分页大小
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 用户ID类型
    pub user_id_type: Option<String>,
}

/// 查询审批任务响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryTaskResponse {
    /// 审批任务列表
    pub tasks: Vec<ApprovalTask>,
    /// 总数
    pub total: i32,
    /// 是否有更多
    pub has_more: bool,
    /// 下一页标记
    pub next_page_token: Option<String>,
}

// 为所有结构体实现Default trait
impl Default for UserInfo {
    fn default() -> Self {
        Self {
            user_id: String::new(),
            name: None,
            email: None,
            avatar: None,
        }
    }
}

impl Default for FormField {
    fn default() -> Self {
        Self {
            key: String::new(),
            name: String::new(),
            field_type: String::new(),
            value: None,
            required: None,
        }
    }
}

impl Default for ApprovalNode {
    fn default() -> Self {
        Self {
            node_id: String::new(),
            node_name: String::new(),
            node_type: String::new(),
            approvers: Vec::new(),
            require_all_approve: None,
        }
    }
}

impl Default for ApprovalProcess {
    fn default() -> Self {
        Self {
            process_id: String::new(),
            process_name: String::new(),
            nodes: Vec::new(),
        }
    }
}

impl Default for Approval {
    fn default() -> Self {
        Self {
            approval_code: String::new(),
            approval_name: String::new(),
            description: None,
            status: None,
            creator: None,
            create_time: None,
            update_time: None,
            form: None,
            process: None,
        }
    }
}

impl Default for ApprovalInstance {
    fn default() -> Self {
        Self {
            instance_code: String::new(),
            approval_code: String::new(),
            approval_name: None,
            initiator: None,
            status: ApprovalStatus::InProgress,
            create_time: None,
            update_time: None,
            form_data: None,
            tasks: None,
            current_node: None,
            comment: None,
        }
    }
}

impl Default for ApprovalTask {
    fn default() -> Self {
        Self {
            task_id: String::new(),
            instance_code: String::new(),
            task_name: String::new(),
            approver: None,
            status: TaskStatus::Pending,
            create_time: None,
            update_time: None,
            due_time: None,
            comment: None,
        }
    }
}

impl Default for CreateInstanceRequest {
    fn default() -> Self {
        Self {
            approval_code: String::new(),
            form: None,
            user_id: None,
            department_id: None,
            uuid: None,
            user_id_type: None,
            department_id_type: None,
        }
    }
}

impl Default for CreateInstanceResponse {
    fn default() -> Self {
        Self {
            instance_code: String::new(),
            uuid: String::new(),
        }
    }
}

impl Default for ProcessTaskRequest {
    fn default() -> Self {
        Self {
            task_id: String::new(),
            action: TaskAction::Approve,
            comment: None,
            transfer_to_user_id: None,
            add_approvers: None,
        }
    }
}

impl Default for ProcessTaskResponse {
    fn default() -> Self {
        Self {
            task_id: String::new(),
            success: false,
            message: None,
        }
    }
}

impl Default for QueryInstanceRequest {
    fn default() -> Self {
        Self {
            approval_code: None,
            initiator_id: None,
            status: None,
            start_time: None,
            end_time: None,
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }
}

impl Default for QueryInstanceResponse {
    fn default() -> Self {
        Self {
            instances: Vec::new(),
            total: 0,
            has_more: false,
            next_page_token: None,
        }
    }
}

impl Default for GetTaskRequest {
    fn default() -> Self {
        Self {
            task_id: String::new(),
            user_id_type: None,
        }
    }
}

impl Default for GetTaskResponse {
    fn default() -> Self {
        Self {
            task: ApprovalTask::default(),
        }
    }
}

impl Default for QueryTaskRequest {
    fn default() -> Self {
        Self {
            approver_id: None,
            status: None,
            instance_code: None,
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }
}

impl Default for QueryTaskResponse {
    fn default() -> Self {
        Self {
            tasks: Vec::new(),
            total: 0,
            has_more: false,
            next_page_token: None,
        }
    }
}