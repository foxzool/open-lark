//! approval 域 typed 事件（`approval.*`）。
//!
//! 字段权威：Go SDK v3.12.2 `service/approval/v4/model.go` 对应 `P2*` 结构的 json tag；
//! 事件清单对齐 `service/approval/v4/dispatcher.go`。Go 指针字段映射为 `Option<T>`，
//! serde 采用 `#[serde(default)]` 宽松风格，与 [`super::im::ImMessageReceiveV1`] 一致。

use serde::{Deserialize, Serialize};

use super::EventHeader;

/// `approval.approval.updated_v4` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApprovalApprovalUpdatedV4 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ApprovalApprovalUpdatedV4Event,
}

/// `approval.approval.updated_v4` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApprovalApprovalUpdatedV4Event {
    /// 事件详情数据。
    #[serde(default)]
    pub object: Option<ApprovalEvent>,
}

/// 审批定义变更详情（Go `ApprovalEvent`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct ApprovalEvent {
    /// 审批定义 ID。
    #[serde(default)]
    pub approval_id: Option<String>,
    /// 审批定义 code。
    #[serde(default)]
    pub approval_code: Option<String>,
    /// 版本 ID。
    #[serde(default)]
    pub version_id: Option<String>,
    /// 画布群类型。
    #[serde(default)]
    pub widget_group_type: Option<i64>,
    /// 表单定义 ID。
    #[serde(default)]
    pub form_definition_id: Option<String>,
    /// 流程对象（JSON 序列化后的字符串）。
    #[serde(default)]
    pub process_obj: Option<String>,
    /// 时间戳（毫秒）。官方类型为 int；JSON 数字与十进制字符串都接受，存成 `i64`。
    #[serde(default, deserialize_with = "super::flex_time::de_opt_i64")]
    pub timestamp: Option<i64>,
    /// 扩展信息。
    #[serde(default)]
    pub extra: Option<String>,
}

/// `approval.instance.status_changed_v4` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApprovalInstanceStatusChangedV4 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ApprovalInstanceStatusChangedV4Event,
}

/// `approval.instance.status_changed_v4` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApprovalInstanceStatusChangedV4Event {
    /// 审批定义 code。
    #[serde(default)]
    pub approval_code: Option<String>,
    /// 审批实例 code。
    #[serde(default)]
    pub instance_code: Option<String>,
    /// 审批实例三方 ID（仅为三方审批时有值）。
    #[serde(default)]
    pub external_id: Option<String>,
    /// 审批实例状态。
    #[serde(default)]
    pub status: Option<String>,
    /// 事件发生时间，毫秒时间戳。
    #[serde(default, deserialize_with = "super::flex_time::de_opt_string")]
    pub operate_time: Option<String>,
    /// 审批实例发起人。
    #[serde(default)]
    pub start_user: Option<UserId>,
    /// 订阅用户 ID 列表。
    #[serde(default)]
    pub subscriber_id_list: Vec<UserId>,
}

/// `approval.task.status_changed_v4` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApprovalTaskStatusChangedV4 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ApprovalTaskStatusChangedV4Event,
}

/// `approval.task.status_changed_v4` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApprovalTaskStatusChangedV4Event {
    /// 审批定义 code。
    #[serde(default)]
    pub approval_code: Option<String>,
    /// 审批实例 code。
    #[serde(default)]
    pub instance_code: Option<String>,
    /// 审批任务 ID。
    #[serde(default)]
    pub task_id: Option<String>,
    /// 三方审批任务 ID（仅为三方审批时有值）。
    #[serde(default)]
    pub task_external_id: Option<String>,
    /// 任务分配人。
    #[serde(default)]
    pub assigned_user: Option<UserId>,
    /// 任务状态。
    #[serde(default)]
    pub status: Option<String>,
    /// 事件触发时间，毫秒时间戳。
    #[serde(default, deserialize_with = "super::flex_time::de_opt_string")]
    pub operate_time: Option<String>,
    /// 订阅用户 ID 列表。
    #[serde(default)]
    pub subscriber_id_list: Vec<UserId>,
}

/// 事件中的用户 ID 集合（Go `UserId`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct UserId {
    /// user_id。
    #[serde(default)]
    pub user_id: Option<String>,
    /// open_id。
    #[serde(default)]
    pub open_id: Option<String>,
    /// union_id。
    #[serde(default)]
    pub union_id: Option<String>,
}

typed_events! {
    /// `approval.approval.updated_v4` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "approval.approval.updated_v4" | APPROVAL_APPROVAL_UPDATED_V4 | ApprovalApprovalUpdatedV4 | ApprovalApprovalUpdatedV4Handler | register_approval_approval_updated_v4;

    /// `approval.instance.status_changed_v4` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "approval.instance.status_changed_v4" | APPROVAL_INSTANCE_STATUS_CHANGED_V4 | ApprovalInstanceStatusChangedV4 | ApprovalInstanceStatusChangedV4Handler | register_approval_instance_status_changed_v4;

    /// `approval.task.status_changed_v4` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "approval.task.status_changed_v4" | APPROVAL_TASK_STATUS_CHANGED_V4 | ApprovalTaskStatusChangedV4 | ApprovalTaskStatusChangedV4Handler | register_approval_task_status_changed_v4;
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::ws_client::EventDispatcherHandler;

    const APPROVAL_UPDATED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"a1","event_type":"approval.approval.updated_v4","create_time":"1700000000000","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "object":{"approval_id":"apv_1","approval_code":"7a57e3","version_id":"ver_2","widget_group_type":1,"form_definition_id":"fd_1","process_obj":"{\"nodes\":[]}","timestamp":"1700000000000","extra":"sync"}
      }
    }"#;

    const INSTANCE_STATUS_CHANGED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"a2","event_type":"approval.instance.status_changed_v4","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "approval_code":"7a57e3",
        "instance_code":"inst_9",
        "external_id":"ext_1",
        "status":"APPROVED",
        "operate_time":"1700000000001",
        "start_user":{"user_id":"u_starter","open_id":"on_starter","union_id":"un_starter"},
        "subscriber_id_list":[{"open_id":"on_s1"},{"open_id":"on_s2"}]
      }
    }"#;

    const TASK_STATUS_CHANGED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"a3","event_type":"approval.task.status_changed_v4","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "approval_code":"7a57e3",
        "instance_code":"inst_9",
        "task_id":"task_1",
        "task_external_id":"t_ext",
        "assigned_user":{"open_id":"on_assignee"},
        "status":"DONE",
        "operate_time":"1700000000002",
        "subscriber_id_list":[{"open_id":"on_s1"}]
      }
    }"#;

    #[test]
    fn typed_approval_approval_updated_deserializes_object() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_approval_approval_updated_v4(move |event: ApprovalApprovalUpdatedV4| {
                assert_eq!(event.header.event_type, "approval.approval.updated_v4");
                let object = event.event.object.expect("object");
                assert_eq!(object.approval_code.as_deref(), Some("7a57e3"));
                assert_eq!(object.version_id.as_deref(), Some("ver_2"));
                assert_eq!(object.widget_group_type, Some(1));
                *seen_h.lock().expect("lock") = object.approval_id.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(APPROVAL_UPDATED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("apv_1"));
    }

    #[test]
    fn typed_approval_instance_status_changed_deserializes_start_user() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_approval_instance_status_changed_v4(
                move |event: ApprovalInstanceStatusChangedV4| {
                    assert_eq!(event.event.status.as_deref(), Some("APPROVED"));
                    assert_eq!(
                        event
                            .event
                            .start_user
                            .as_ref()
                            .and_then(|u| u.open_id.as_deref()),
                        Some("on_starter")
                    );
                    assert_eq!(event.event.subscriber_id_list.len(), 2);
                    assert_eq!(
                        event.event.subscriber_id_list[1].open_id.as_deref(),
                        Some("on_s2")
                    );
                    *seen_h.lock().expect("lock") = event.event.instance_code.clone();
                    Ok(())
                },
            )
            .expect("register")
            .build();
        handler
            .do_without_validation(INSTANCE_STATUS_CHANGED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("inst_9"));
    }

    #[test]
    fn typed_approval_task_status_changed_deserializes_task() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_approval_task_status_changed_v4(move |event: ApprovalTaskStatusChangedV4| {
                assert_eq!(event.event.status.as_deref(), Some("DONE"));
                assert_eq!(event.event.task_external_id.as_deref(), Some("t_ext"));
                assert_eq!(
                    event
                        .event
                        .assigned_user
                        .as_ref()
                        .and_then(|u| u.open_id.as_deref()),
                    Some("on_assignee")
                );
                *seen_h.lock().expect("lock") = event.event.task_id.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(TASK_STATUS_CHANGED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("task_1"));
    }

    /// 事件串常量与票面清单逐字一致。
    #[test]
    fn typed_approval_event_constants_match_go_dispatcher() {
        assert_eq!(
            EventDispatcherHandler::APPROVAL_APPROVAL_UPDATED_V4,
            "approval.approval.updated_v4"
        );
        assert_eq!(
            EventDispatcherHandler::APPROVAL_INSTANCE_STATUS_CHANGED_V4,
            "approval.instance.status_changed_v4"
        );
        assert_eq!(
            EventDispatcherHandler::APPROVAL_TASK_STATUS_CHANGED_V4,
            "approval.task.status_changed_v4"
        );
    }

    /// typed 注册后再 `register_raw` 同 key 返回 Err；反方向同理。
    #[test]
    fn typed_approval_duplicate_register_fails() {
        struct Noop;
        impl crate::ws_client::EventHandler for Noop {
            fn handle(
                &self,
                _payload: &[u8],
            ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                Ok(())
            }
        }
        let err = EventDispatcherHandler::builder()
            .register_approval_approval_updated_v4(|_event: ApprovalApprovalUpdatedV4| Ok(()))
            .expect("first")
            .register_raw("approval.approval.updated_v4", Noop)
            .expect_err("dup typed-then-raw");
        assert!(err.contains("already registered"));
        let err = EventDispatcherHandler::builder()
            .register_raw("approval.task.status_changed_v4", Noop)
            .expect("raw first")
            .register_approval_task_status_changed_v4(|_event: ApprovalTaskStatusChangedV4| Ok(()))
            .expect_err("dup raw-then-typed");
        assert!(err.contains("already registered"));
    }

    #[test]
    fn numeric_timestamp_and_operate_time_reach_typed_handlers() {
        let seen_ts = Arc::new(Mutex::new(None));
        let seen_ts_h = Arc::clone(&seen_ts);
        let seen_op = Arc::new(Mutex::new(None));
        let seen_op_h = Arc::clone(&seen_op);
        let handler = EventDispatcherHandler::builder()
            .register_approval_approval_updated_v4(move |event: ApprovalApprovalUpdatedV4| {
                *seen_ts_h.lock().expect("lock") = event
                    .event
                    .object
                    .as_ref()
                    .and_then(|object| object.timestamp);
                Ok(())
            })
            .expect("register updated")
            .register_approval_instance_status_changed_v4(
                move |event: ApprovalInstanceStatusChangedV4| {
                    *seen_op_h.lock().expect("lock") = event.event.operate_time.clone();
                    Ok(())
                },
            )
            .expect("register instance")
            .build();

        handler
            .do_without_validation(
                br#"{
                  "header":{"event_type":"approval.approval.updated_v4","create_time":1700000000000},
                  "event":{"object":{"timestamp":1700000000000}}
                }"#,
            )
            .expect("dispatch timestamp number");
        handler
            .do_without_validation(
                br#"{
                  "header":{"event_type":"approval.instance.status_changed_v4"},
                  "event":{"operate_time":1700000000001}
                }"#,
            )
            .expect("dispatch operate_time number");

        assert_eq!(*seen_ts.lock().expect("lock"), Some(1_700_000_000_000));
        assert_eq!(
            seen_op.lock().expect("lock").as_deref(),
            Some("1700000000001")
        );
    }
}
