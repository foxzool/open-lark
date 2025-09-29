use serde::{Deserialize, Serialize};

use crate::event::{context::EventHeader, dispatcher::EventHandler};

#[derive(Debug, Serialize, Deserialize)]
pub struct P2ContactUserDeletedV3 {
    pub schema: String,
    pub header: EventHeader,
    pub event: P2ContactUserDeletedV3Data,
}

pub(crate) struct P2ContactUserDeletedV3ProcessorImpl<F>
where
    F: Fn(P2ContactUserDeletedV3) + 'static,
{
    f: F,
}

impl<F> EventHandler for P2ContactUserDeletedV3ProcessorImpl<F>
where
    F: Fn(P2ContactUserDeletedV3) + 'static + Sync + Send,
{
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
        let event: P2ContactUserDeletedV3 = serde_json::from_slice(payload)?;
        (self.f)(event);
        Ok(())
    }
}

impl<F> P2ContactUserDeletedV3ProcessorImpl<F>
where
    F: Fn(P2ContactUserDeletedV3) + 'static,
{
    pub(crate) fn new(f: F) -> Self {
        P2ContactUserDeletedV3ProcessorImpl { f }
    }
}

/// 用户删除事件数据
#[derive(Debug, Serialize, Deserialize)]
pub struct P2ContactUserDeletedV3Data {
    /// 事件对象
    pub object: ContactEventObject,
    /// 删除操作的相关信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_object: Option<ContactEventObject>,
}

/// 通讯录事件对象
#[derive(Debug, Serialize, Deserialize)]
pub struct ContactEventObject {
    /// 对象类型 (user)
    pub object_type: String,
    /// 用户信息
    pub user: DeletedContactUser,
}

/// 被删除的用户信息（简化版本）
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedContactUser {
    /// 用户 ID
    pub user_id: String,
    /// 用户的 union id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    /// 用户的 open id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    /// 用户名（在删除时可能保留）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 邮箱（在删除时可能保留）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 工号（在删除时可能保留）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_no: Option<String>,
    /// 删除时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_time: Option<String>,
    /// 删除原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_reason: Option<String>,
}
