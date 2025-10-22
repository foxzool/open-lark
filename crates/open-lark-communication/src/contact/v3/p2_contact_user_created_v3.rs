
use serde::{Deserialize, Serialize};

use open_lark_core::event::{context::EventHeader, dispatcher::EventHandler};

#[derive(Debug, Serialize, Deserialize)]
pub struct P2ContactUserCreatedV3 {
    pub schema: String,
    pub header: EventHeader,
    pub event: P2ContactUserCreatedV3Data,
}

pub(crate) struct P2ContactUserCreatedV3ProcessorImpl<F>
where
    F: Fn(P2ContactUserCreatedV3) + 'static,
{
    f: F,
}

impl<F> EventHandler for P2ContactUserCreatedV3ProcessorImpl<F>
where
    F: Fn(P2ContactUserCreatedV3) + 'static + Sync + Send,
{
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
        let event: P2ContactUserCreatedV3 = serde_json::from_slice(payload)?;
        (self.f)(event);
        Ok(())
    }
}

impl<F> P2ContactUserCreatedV3ProcessorImpl<F>
where
    F: Fn(P2ContactUserCreatedV3) + 'static,
{
    pub(crate) fn new(f: F) -> Self {
        P2ContactUserCreatedV3ProcessorImpl { f }
    }
}

/// 用户创建事件数据
#[derive(Debug, Serialize, Deserialize)]
pub struct P2ContactUserCreatedV3Data {
    /// 事件对象
    pub object: ContactEventObject,
}

/// 通讯录事件对象
#[derive(Debug, Serialize, Deserialize)]
pub struct ContactEventObject {
    /// 对象类型 (user)
    pub object_type: String,
    /// 用户信息
    pub user: ContactUser,
}

/// 通讯录用户信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ContactUser {
    /// 用户 ID
    pub user_id: String,
    /// 用户的 union id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    /// 用户的 open id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    /// 用户名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 英文名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    /// 别名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// 邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 手机号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// 性别 (0:保密, 1:男, 2:女)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<i32>,
    /// 头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<UserAvatar>,
    /// 用户状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    /// 部门ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 工号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_no: Option<String>,
    /// 员工类型 (1:正式员工, 2:实习生, 3:外包, 4:劳务, 5:顾问)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_type: Option<i32>,
    /// 入职时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_time: Option<String>,
    /// 职位
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    /// 是否是租户超管
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_tenant_manager: Option<bool>,
    /// 创建时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

/// 用户头像信息
#[derive(Debug, Serialize, Deserialize)]
pub struct UserAvatar {
    /// 72*72像素头像链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_72: Option<String>,
    /// 240*240像素头像链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_240: Option<String>,
    /// 640*640像素头像链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_640: Option<String>,
    /// 原始头像链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_origin: Option<String>,
}

/// 用户状态
#[derive(Debug, Serialize, Deserialize)]
pub struct UserStatus {
    /// 是否冻结
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_frozen: Option<bool>,
    /// 是否离职
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_resigned: Option<bool>,
    /// 是否激活
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_activated: Option<bool>,
    /// 是否主动退出，主动退出的用户不会再加入到相同的租户内
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_exited: Option<bool>,
}
