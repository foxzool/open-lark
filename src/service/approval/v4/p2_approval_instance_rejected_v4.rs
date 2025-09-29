use serde::{Deserialize, Serialize};

use crate::event::{context::EventHeader, dispatcher::EventHandler};

#[derive(Debug, Serialize, Deserialize)]
pub struct P2ApprovalInstanceRejectedV4 {
    pub schema: String,
    pub header: EventHeader,
    pub event: P2ApprovalInstanceRejectedV4Data,
}

pub(crate) struct P2ApprovalInstanceRejectedV4ProcessorImpl<F>
where
    F: Fn(P2ApprovalInstanceRejectedV4) + 'static,
{
    f: F,
}

impl<F> EventHandler for P2ApprovalInstanceRejectedV4ProcessorImpl<F>
where
    F: Fn(P2ApprovalInstanceRejectedV4) + 'static + Sync + Send,
{
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
        let event: P2ApprovalInstanceRejectedV4 = serde_json::from_slice(payload)?;
        (self.f)(event);
        Ok(())
    }
}

impl<F> P2ApprovalInstanceRejectedV4ProcessorImpl<F>
where
    F: Fn(P2ApprovalInstanceRejectedV4) + 'static,
{
    pub(crate) fn new(f: F) -> Self {
        P2ApprovalInstanceRejectedV4ProcessorImpl { f }
    }
}

/// 审批实例拒绝事件数据
#[derive(Debug, Serialize, Deserialize)]
pub struct P2ApprovalInstanceRejectedV4Data {
    /// 事件对象
    pub object: ApprovalInstanceEventObject,
    /// 审批前状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_status: Option<String>,
    /// 拒绝信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejection_info: Option<RejectionInfo>,
}

/// 审批实例事件对象
#[derive(Debug, Serialize, Deserialize)]
pub struct ApprovalInstanceEventObject {
    /// 对象类型 (approval_instance)
    pub object_type: String,
    /// 审批实例信息
    pub instance: RejectedApprovalInstance,
}

/// 被拒绝的审批实例信息
#[derive(Debug, Serialize, Deserialize)]
pub struct RejectedApprovalInstance {
    /// 审批实例ID
    pub instance_id: String,
    /// 审批定义ID
    pub approval_id: String,
    /// 审批定义名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_name: Option<String>,
    /// 审批实例状态 (rejected)
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
    /// 拒绝详情
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejection_details: Option<RejectionDetails>,
    /// 创建时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 提交时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<String>,
    /// 拒绝时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_time: Option<String>,
    /// 审批实例序列号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// 审批实例链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_url: Option<String>,
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
    /// 字段类型
    pub field_type: String,
    /// 字段值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_value: Option<String>,
}

/// 审批流程信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ApprovalProcess {
    /// 审批流程节点列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<ApprovalNode>>,
    /// 拒绝节点ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_node_id: Option<String>,
}

/// 审批流程节点
#[derive(Debug, Serialize, Deserialize)]
pub struct ApprovalNode {
    /// 节点ID
    pub node_id: String,
    /// 节点名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    /// 节点类型
    pub node_type: String,
    /// 节点状态
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
}

/// 拒绝信息
#[derive(Debug, Serialize, Deserialize)]
pub struct RejectionInfo {
    /// 拒绝原因类型 (user_reject, system_reject, timeout_reject)
    pub reject_type: String,
    /// 拒绝操作者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejector_id: Option<String>,
    /// 拒绝操作者信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejector_info: Option<ApprovalUser>,
    /// 拒绝时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_time: Option<String>,
}

/// 拒绝详情
#[derive(Debug, Serialize, Deserialize)]
pub struct RejectionDetails {
    /// 拒绝节点ID
    pub rejected_node_id: String,
    /// 拒绝原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 拒绝意见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// 是否可以重新提交
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_resubmit: Option<bool>,
    /// 建议修改内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_changes: Option<Vec<String>>,
}
