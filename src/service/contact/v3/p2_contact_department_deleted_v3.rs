use serde::{Deserialize, Serialize};

use crate::event::{context::EventHeader, dispatcher::EventHandler};

#[derive(Debug, Serialize, Deserialize)]
pub struct P2ContactDepartmentDeletedV3 {
    pub schema: String,
    pub header: EventHeader,
    pub event: P2ContactDepartmentDeletedV3Data,
}

pub(crate) struct P2ContactDepartmentDeletedV3ProcessorImpl<F>
where
    F: Fn(P2ContactDepartmentDeletedV3) + 'static,
{
    f: F,
}

impl<F> EventHandler for P2ContactDepartmentDeletedV3ProcessorImpl<F>
where
    F: Fn(P2ContactDepartmentDeletedV3) + 'static + Sync + Send,
{
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
        let event: P2ContactDepartmentDeletedV3 = serde_json::from_slice(payload)?;
        (self.f)(event);
        Ok(())
    }
}

impl<F> P2ContactDepartmentDeletedV3ProcessorImpl<F>
where
    F: Fn(P2ContactDepartmentDeletedV3) + 'static,
{
    pub(crate) fn new(f: F) -> Self {
        P2ContactDepartmentDeletedV3ProcessorImpl { f }
    }
}

/// 部门删除事件数据
#[derive(Debug, Serialize, Deserialize)]
pub struct P2ContactDepartmentDeletedV3Data {
    /// 事件对象
    pub object: ContactDepartmentEventObject,
    /// 删除前的部门信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_object: Option<ContactDepartmentEventObject>,
}

/// 通讯录部门事件对象
#[derive(Debug, Serialize, Deserialize)]
pub struct ContactDepartmentEventObject {
    /// 对象类型 (department)
    pub object_type: String,
    /// 部门信息
    pub department: DeletedContactDepartment,
}

/// 被删除的部门信息
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedContactDepartment {
    /// 部门ID
    pub department_id: String,
    /// 部门名称
    pub name: String,
    /// 国际化部门名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<DepartmentI18nName>,
    /// 父部门ID，根部门的父部门ID为"0"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    /// 部门主管用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader_user_id: Option<String>,
    /// 子部门列表 (删除时的快照)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_department_ids: Option<Vec<String>>,
    /// 部门成员列表 (删除时的快照)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_user_ids: Option<Vec<String>>,
    /// 部门成员数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i32>,
    /// 删除时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_time: Option<String>,
    /// 删除原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_reason: Option<String>,
    /// 删除操作者信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_by: Option<String>,
    /// 删除方式 (manual, auto, merge)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_type: Option<String>,
}

/// 部门国际化名称
#[derive(Debug, Serialize, Deserialize)]
pub struct DepartmentI18nName {
    /// 中文名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 英文名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
    /// 日文名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ja_jp: Option<String>,
}
