//! # ACS v1 事件
//!
//! 智能门禁系统的事件定义，包括用户信息变更和门禁访问记录事件。

use crate::service::acs::models::{AccessRecord, AcsUser};
use serde::{Deserialize, Serialize};

/// 用户信息变更事件 (p2_acs_user_updated_v1)
///
/// 当门禁用户信息发生变更时触发此事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2AcsUserUpdatedV1 {
    /// 事件ID
    pub event_id: String,
    /// 事件类型
    pub event_type: String,
    /// 事件时间戳
    pub created_time: String,
    /// 事件数据
    pub event: AcsUserUpdatedEvent,
}

/// 用户信息变更事件数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcsUserUpdatedEvent {
    /// 变更类型 (created, updated, deleted)
    pub change_type: String,
    /// 变更前的用户信息（更新/删除时存在）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_user: Option<AcsUser>,
    /// 变更后的用户信息（创建/更新时存在）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_user: Option<AcsUser>,
}

/// 新增门禁访问记录事件 (p2_acs_access_record_created_v1)
///
/// 当产生新的门禁访问记录时触发此事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2AcsAccessRecordCreatedV1 {
    /// 事件ID
    pub event_id: String,
    /// 事件类型
    pub event_type: String,
    /// 事件时间戳
    pub created_time: String,
    /// 事件数据
    pub event: AccessRecordCreatedEvent,
}

/// 门禁访问记录创建事件数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessRecordCreatedEvent {
    /// 新增的访问记录
    pub access_record: AccessRecord,
    /// 是否为异常访问
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_abnormal: Option<bool>,
    /// 异常原因（如果是异常访问）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abnormal_reason: Option<String>,
}

// 为事件实现 Display trait，便于日志输出
impl std::fmt::Display for P2AcsUserUpdatedV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ACS User Updated Event [{}]: {} at {}",
            self.event_id, self.event.change_type, self.created_time
        )
    }
}

impl std::fmt::Display for P2AcsAccessRecordCreatedV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ACS Access Record Created Event [{}]: user {} accessed device {} at {}",
            self.event_id,
            self.event
                .access_record
                .user_id
                .as_deref()
                .unwrap_or("unknown"),
            self.event.access_record.device_id,
            self.created_time
        )
    }
}
