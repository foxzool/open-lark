use serde::{Deserialize, Serialize};

use crate::event::{context::EventHeader, dispatcher::EventHandler};

#[derive(Debug, Serialize, Deserialize)]
pub struct P2ApprovalInstanceApprovedV4 {
    pub schema: String,
    pub header: EventHeader,
    pub event: P2ApprovalInstanceApprovedV4Data,
}

pub(crate) struct P2ApprovalInstanceApprovedV4ProcessorImpl<F>
where
    F: Fn(P2ApprovalInstanceApprovedV4) + 'static,
{
    f: F,
}

impl<F> EventHandler for P2ApprovalInstanceApprovedV4ProcessorImpl<F>
where
    F: Fn(P2ApprovalInstanceApprovedV4) + 'static + Sync + Send,
{
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
        let event: P2ApprovalInstanceApprovedV4 = serde_json::from_slice(payload)?;
        (self.f)(event);
        Ok(())
    }
}

impl<F> P2ApprovalInstanceApprovedV4ProcessorImpl<F>
where
    F: Fn(P2ApprovalInstanceApprovedV4) + 'static,
{
    pub(crate) fn new(f: F) -> Self {
        P2ApprovalInstanceApprovedV4ProcessorImpl { f }
    }
}

/// 审批实例通过事件数据
#[derive(Debug, Serialize, Deserialize)]
pub struct P2ApprovalInstanceApprovedV4Data {
    /// 事件对象
    pub object: ApprovalInstanceEventObject,
    /// 审批前状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_status: Option<String>,
}

/// 审批实例事件对象
#[derive(Debug, Serialize, Deserialize)]
pub struct ApprovalInstanceEventObject {
    /// 对象类型 (approval_instance)
    pub object_type: String,
    /// 审批实例信息
    pub instance: ApprovedApprovalInstance,
}

/// 已通过的审批实例信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ApprovedApprovalInstance {
    /// 审批实例ID
    pub instance_id: String,
    /// 审批定义ID
    pub approval_id: String,
    /// 审批定义名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_name: Option<String>,
    /// 审批实例状态 (approved)
    pub status: String,
    /// 申请人用户ID
    pub user_id: String,
    /// 申请人信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_info: Option<ApprovalUser>,
    /// 审批实例表单内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<ApprovalForm>,
    /// 审批流程信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<ApprovalProcess>,
    /// 审批结果信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_result: Option<ApprovalResult>,
    /// 创建时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 提交时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<String>,
    /// 完成时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_time: Option<String>,
    /// 审批实例序列号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// 审批实例链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_url: Option<String>,
    /// 审批总耗时（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_duration: Option<i64>,
}

/// 审批用户信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ApprovalUser {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 用户邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 用户头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// 部门ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
}

/// 审批表单信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ApprovalForm {
    /// 表单字段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_fields: Option<Vec<ApprovalFormField>>,
}

/// 审批表单字段
#[derive(Debug, Serialize, Deserialize)]
pub struct ApprovalFormField {
    /// 字段ID
    pub field_id: String,
    /// 字段名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    /// 字段类型 (text, textarea, number, date, select, multi_select, attachment)
    pub field_type: String,
    /// 字段值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_value: Option<String>,
    /// 是否必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

/// 审批流程信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ApprovalProcess {
    /// 审批流程节点列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<ApprovalNode>>,
    /// 当前处理节点ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_node_id: Option<String>,
    /// 审批路径
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_path: Option<Vec<String>>,
}

/// 审批流程节点
#[derive(Debug, Serialize, Deserialize)]
pub struct ApprovalNode {
    /// 节点ID
    pub node_id: String,
    /// 节点名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    /// 节点类型 (approval, cc, condition, start, end)
    pub node_type: String,
    /// 节点状态 (pending, approved, rejected, skipped)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 审批人列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approvers: Option<Vec<ApprovalUser>>,
    /// 处理时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_time: Option<String>,
    /// 审批意见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// 节点处理耗时（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
}

/// 审批结果信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ApprovalResult {
    /// 最终审批结果 (approved)
    pub final_result: String,
    /// 审批意见汇总
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_comment: Option<String>,
    /// 各节点审批详情
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_results: Option<Vec<NodeResult>>,
    /// 通过率
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rate: Option<f64>,
}

/// 节点审批结果
#[derive(Debug, Serialize, Deserialize)]
pub struct NodeResult {
    /// 节点ID
    pub node_id: String,
    /// 节点结果 (approved, rejected, skipped)
    pub result: String,
    /// 审批人ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approver_id: Option<String>,
    /// 审批意见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// 审批时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approve_time: Option<String>,
}
