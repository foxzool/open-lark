use serde::{Deserialize, Serialize};

use crate::event::{context::EventHeader, dispatcher::EventHandler};

#[derive(Debug, Serialize, Deserialize)]
pub struct P2ContactDepartmentUpdatedV3 {
    pub schema: String,
    pub header: EventHeader,
    pub event: P2ContactDepartmentUpdatedV3Data,
}

pub(crate) struct P2ContactDepartmentUpdatedV3ProcessorImpl<F>
where
    F: Fn(P2ContactDepartmentUpdatedV3) + 'static,
{
    f: F,
}

impl<F> EventHandler for P2ContactDepartmentUpdatedV3ProcessorImpl<F>
where
    F: Fn(P2ContactDepartmentUpdatedV3) + 'static + Sync + Send,
{
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
        let event: P2ContactDepartmentUpdatedV3 = serde_json::from_slice(payload)?;
        (self.f)(event);
        Ok(())
    }
}

impl<F> P2ContactDepartmentUpdatedV3ProcessorImpl<F>
where
    F: Fn(P2ContactDepartmentUpdatedV3) + 'static,
{
    pub(crate) fn new(f: F) -> Self {
        P2ContactDepartmentUpdatedV3ProcessorImpl { f }
    }
}

/// 部门更新事件数据
#[derive(Debug, Serialize, Deserialize)]
pub struct P2ContactDepartmentUpdatedV3Data {
    /// 事件对象
    pub object: ContactDepartmentEventObject,
    /// 更新前的部门信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_object: Option<ContactDepartmentEventObject>,
    /// 更新类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_type: Option<DepartmentUpdateType>,
}

/// 通讯录部门事件对象
#[derive(Debug, Serialize, Deserialize)]
pub struct ContactDepartmentEventObject {
    /// 对象类型 (department)
    pub object_type: String,
    /// 部门信息
    pub department: UpdatedContactDepartment,
}

/// 更新的部门信息
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatedContactDepartment {
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
    /// 部门状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DepartmentStatus>,
    /// 是否同步创建部门群
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_group_chat: Option<bool>,
    /// 部门群ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    /// 在父部门中的次序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    /// 部门绑定的单位自定义ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_ids: Option<Vec<String>>,
    /// 当前部门及其下属部门的用户总数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i32>,
    /// 创建时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 更新详情
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_info: Option<DepartmentUpdateInfo>,
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

/// 部门状态
#[derive(Debug, Serialize, Deserialize)]
pub struct DepartmentStatus {
    /// 是否被删除
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    /// 是否被启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
}

/// 部门更新类型
#[derive(Debug, Serialize, Deserialize)]
pub struct DepartmentUpdateType {
    /// 更新类型 (info, structure, leader, status)
    pub update_category: String,
    /// 具体更新字段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_fields: Option<Vec<String>>,
}

/// 部门更新信息
#[derive(Debug, Serialize, Deserialize)]
pub struct DepartmentUpdateInfo {
    /// 更新操作者用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    /// 更新时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
    /// 更新描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_description: Option<String>,
    /// 层级变化信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_change: Option<DepartmentHierarchyChange>,
}

/// 部门层级变化
#[derive(Debug, Serialize, Deserialize)]
pub struct DepartmentHierarchyChange {
    /// 旧的父部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_parent_id: Option<String>,
    /// 新的父部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_parent_id: Option<String>,
    /// 层级深度变化
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth_change: Option<i32>,
}
