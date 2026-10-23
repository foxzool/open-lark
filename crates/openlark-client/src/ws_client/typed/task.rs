//! task 域 typed 事件（`task.*`，v1 与 v2）。
//!
//! 字段权威：Go SDK v3.12.2 `service/task/{v1,v2}/model.go` 对应 `P2*` 结构的 json tag；
//! 事件清单对齐 `service/task/{v1,v2}/dispatcher.go`。Go 指针字段映射为 `Option<T>`，
//! serde 采用 `#[serde(default)]` 宽松风格，与 [`super::im::ImMessageReceiveV1`] 一致。

use serde::{Deserialize, Serialize};

use super::EventHeader;

/// `task.task.updated_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskTaskUpdatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: TaskTaskUpdatedV1Event,
}

/// `task.task.updated_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskTaskUpdatedV1Event {
    /// 任务 ID。
    #[serde(default)]
    pub task_id: Option<String>,
    /// 通知类型（1 任务详情、2 协作者、3 关注者、4 提醒时间、5 完成、6 取消完成、7 删除）。
    #[serde(default)]
    pub obj_type: Option<i64>,
}

/// `task.task.update_tenant_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskTaskUpdateTenantV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: TaskTaskUpdateTenantV1Event,
}

/// `task.task.update_tenant_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskTaskUpdateTenantV1Event {
    /// 用户 ID 列表。
    #[serde(default)]
    pub user_id_list: Option<UserIdList>,
    /// 任务的 ID。
    #[serde(default)]
    pub task_id: Option<String>,
    /// 变更的数据类型：`task` / `collaborator` / `follower`。
    #[serde(default)]
    pub object_type: Option<String>,
    /// 事件类型：`create` / `delete` / `update`。
    #[serde(default)]
    pub event_type: Option<String>,
}

/// `task.task.comment.updated_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskTaskCommentUpdatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: TaskTaskCommentUpdatedV1Event,
}

/// `task.task.comment.updated_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskTaskCommentUpdatedV1Event {
    /// 任务 ID。
    #[serde(default)]
    pub task_id: Option<String>,
    /// 任务评论 ID。
    #[serde(default)]
    pub comment_id: Option<String>,
    /// 任务评论父 ID。
    #[serde(default)]
    pub parent_id: Option<String>,
    /// 通知类型（1 创建评论、2 回复评论、3 更新评论、4 删除评论）。
    #[serde(default)]
    pub obj_type: Option<i64>,
}

/// `task.task.update_user_access_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskTaskUpdateUserAccessV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: TaskTaskUpdateUserAccessV2Event,
}

/// `task.task.update_user_access_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskTaskUpdateUserAccessV2Event {
    /// 事件类型列表，如 `task_create` / `task_completed_update` 等。
    #[serde(default)]
    pub event_types: Vec<String>,
    /// 任务 GUID。
    #[serde(default)]
    pub task_guid: Option<String>,
}

/// 用户 ID 列表（Go `UserIdList`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct UserIdList {
    /// 用户 ID 列表。
    #[serde(default)]
    pub user_id_list: Vec<UserId>,
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
    /// `task.task.updated_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "task.task.updated_v1" | TASK_TASK_UPDATED_V1 | TaskTaskUpdatedV1 | TaskTaskUpdatedV1Handler | register_task_task_updated_v1;

    /// `task.task.update_tenant_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "task.task.update_tenant_v1" | TASK_TASK_UPDATE_TENANT_V1 | TaskTaskUpdateTenantV1 | TaskTaskUpdateTenantV1Handler | register_task_task_update_tenant_v1;

    /// `task.task.comment.updated_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "task.task.comment.updated_v1" | TASK_TASK_COMMENT_UPDATED_V1 | TaskTaskCommentUpdatedV1 | TaskTaskCommentUpdatedV1Handler | register_task_task_comment_updated_v1;

    /// `task.task.update_user_access_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "task.task.update_user_access_v2" | TASK_TASK_UPDATE_USER_ACCESS_V2 | TaskTaskUpdateUserAccessV2 | TaskTaskUpdateUserAccessV2Handler | register_task_task_update_user_access_v2;
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::ws_client::EventDispatcherHandler;

    const TASK_UPDATED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"tk1","event_type":"task.task.updated_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{"task_id":"task_1","obj_type":5}
    }"#;

    const TASK_UPDATE_TENANT_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"tk2","event_type":"task.task.update_tenant_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "user_id_list":{"user_id_list":[{"open_id":"on_u1"},{"open_id":"on_u2"}]},
        "task_id":"task_2",
        "object_type":"collaborator",
        "event_type":"create"
      }
    }"#;

    const TASK_COMMENT_UPDATED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"tk3","event_type":"task.task.comment.updated_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{"task_id":"task_1","comment_id":"cm_1","parent_id":"cm_0","obj_type":2}
    }"#;

    const TASK_UPDATE_USER_ACCESS_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"tk4","event_type":"task.task.update_user_access_v2","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{"event_types":["task_create","task_completed_update"],"task_guid":"guid_1"}
    }"#;

    #[test]
    fn typed_task_updated_deserializes_obj_type() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_task_task_updated_v1(move |event: TaskTaskUpdatedV1| {
                assert_eq!(event.header.event_type, "task.task.updated_v1");
                assert_eq!(event.event.obj_type, Some(5));
                *seen_h.lock().expect("lock") = event.event.task_id.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(TASK_UPDATED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("task_1"));
    }

    #[test]
    fn typed_task_update_tenant_deserializes_user_list() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_task_task_update_tenant_v1(move |event: TaskTaskUpdateTenantV1| {
                assert_eq!(event.event.object_type.as_deref(), Some("collaborator"));
                assert_eq!(event.event.event_type.as_deref(), Some("create"));
                let list = event.event.user_id_list.expect("user_id_list");
                assert_eq!(list.user_id_list.len(), 2);
                assert_eq!(list.user_id_list[1].open_id.as_deref(), Some("on_u2"));
                *seen_h.lock().expect("lock") = event.event.task_id.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(TASK_UPDATE_TENANT_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("task_2"));
    }

    #[test]
    fn typed_task_comment_updated_deserializes_comment() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_task_task_comment_updated_v1(move |event: TaskTaskCommentUpdatedV1| {
                assert_eq!(event.event.obj_type, Some(2));
                assert_eq!(event.event.parent_id.as_deref(), Some("cm_0"));
                *seen_h.lock().expect("lock") = event.event.comment_id.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(TASK_COMMENT_UPDATED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("cm_1"));
    }

    #[test]
    fn typed_task_update_user_access_deserializes_event_types() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_task_task_update_user_access_v2(move |event: TaskTaskUpdateUserAccessV2| {
                assert_eq!(event.header.event_type, "task.task.update_user_access_v2");
                assert_eq!(
                    event.event.event_types,
                    vec![
                        "task_create".to_string(),
                        "task_completed_update".to_string()
                    ]
                );
                *seen_h.lock().expect("lock") = event.event.task_guid.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(TASK_UPDATE_USER_ACCESS_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("guid_1"));
    }

    /// 事件串常量与票面清单逐字一致。
    #[test]
    fn typed_task_event_constants_match_go_dispatcher() {
        assert_eq!(
            EventDispatcherHandler::TASK_TASK_UPDATED_V1,
            "task.task.updated_v1"
        );
        assert_eq!(
            EventDispatcherHandler::TASK_TASK_UPDATE_TENANT_V1,
            "task.task.update_tenant_v1"
        );
        assert_eq!(
            EventDispatcherHandler::TASK_TASK_COMMENT_UPDATED_V1,
            "task.task.comment.updated_v1"
        );
        assert_eq!(
            EventDispatcherHandler::TASK_TASK_UPDATE_USER_ACCESS_V2,
            "task.task.update_user_access_v2"
        );
    }

    /// typed 注册后再 `register_raw` 同 key 返回 Err；反方向同理。
    #[test]
    fn typed_task_duplicate_register_fails() {
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
            .register_task_task_updated_v1(|_event: TaskTaskUpdatedV1| Ok(()))
            .expect("first")
            .register_raw("task.task.updated_v1", Noop)
            .expect_err("dup typed-then-raw");
        assert!(err.contains("already registered"));
        let err = EventDispatcherHandler::builder()
            .register_raw("task.task.update_user_access_v2", Noop)
            .expect("raw first")
            .register_task_task_update_user_access_v2(|_event: TaskTaskUpdateUserAccessV2| Ok(()))
            .expect_err("dup raw-then-typed");
        assert!(err.contains("already registered"));
    }
}
