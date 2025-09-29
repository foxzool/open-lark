use serde::{Deserialize, Serialize};

use crate::event::{context::EventHeader, dispatcher::EventHandler};

#[derive(Debug, Serialize, Deserialize)]
pub struct P2ContactDepartmentCreatedV3 {
    pub schema: String,
    pub header: EventHeader,
    pub event: P2ContactDepartmentCreatedV3Data,
}

pub(crate) struct P2ContactDepartmentCreatedV3ProcessorImpl<F>
where
    F: Fn(P2ContactDepartmentCreatedV3) + 'static,
{
    f: F,
}

impl<F> EventHandler for P2ContactDepartmentCreatedV3ProcessorImpl<F>
where
    F: Fn(P2ContactDepartmentCreatedV3) + 'static + Sync + Send,
{
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
        let event: P2ContactDepartmentCreatedV3 = serde_json::from_slice(payload)?;
        (self.f)(event);
        Ok(())
    }
}

impl<F> P2ContactDepartmentCreatedV3ProcessorImpl<F>
where
    F: Fn(P2ContactDepartmentCreatedV3) + 'static,
{
    pub(crate) fn new(f: F) -> Self {
        P2ContactDepartmentCreatedV3ProcessorImpl { f }
    }
}

/// 部门创建事件数据
#[derive(Debug, Serialize, Deserialize)]
pub struct P2ContactDepartmentCreatedV3Data {
    /// 事件对象
    pub object: ContactDepartmentEventObject,
}

/// 通讯录部门事件对象
#[derive(Debug, Serialize, Deserialize)]
pub struct ContactDepartmentEventObject {
    /// 对象类型 (department)
    pub object_type: String,
    /// 部门信息
    pub department: ContactDepartment,
}

/// 通讯录部门信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ContactDepartment {
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
}
