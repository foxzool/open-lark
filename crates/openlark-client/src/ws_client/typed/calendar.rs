//! calendar 域 typed 事件（`calendar.*`）。
//!
//! 字段权威：Go SDK v3.12.2 `service/calendar/v4/model.go` 对应 `P2*` 结构的 json tag；
//! 事件清单对齐 `service/calendar/v4/dispatcher.go`。Go 指针字段映射为 `Option<T>`，
//! serde 采用 `#[serde(default)]` 宽松风格，与 [`super::im::ImMessageReceiveV1`] 一致。

use serde::{Deserialize, Serialize};

use super::EventHeader;

/// `calendar.calendar.changed_v4` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CalendarCalendarChangedV4 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CalendarCalendarChangedV4Event,
}

/// `calendar.calendar.changed_v4` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CalendarCalendarChangedV4Event {
    /// 需要推送事件的用户列表。
    #[serde(default)]
    pub user_id_list: Vec<UserId>,
}

/// `calendar.calendar.acl.created_v4` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CalendarCalendarAclCreatedV4 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CalendarCalendarAclCreatedV4Event,
}

/// `calendar.calendar.acl.created_v4` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CalendarCalendarAclCreatedV4Event {
    /// 访问控制 ID。该 ID 在单个日历实体内唯一。
    #[serde(default)]
    pub acl_id: Option<String>,
    /// 对日历的访问权限。
    #[serde(default)]
    pub role: Option<String>,
    /// 权限生效范围。
    #[serde(default)]
    pub scope: Option<AclScopeEvent>,
    /// 需要推送事件的用户列表。
    #[serde(default)]
    pub user_id_list: Vec<UserId>,
}

/// `calendar.calendar.acl.deleted_v4` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CalendarCalendarAclDeletedV4 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CalendarCalendarAclDeletedV4Event,
}

/// `calendar.calendar.acl.deleted_v4` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CalendarCalendarAclDeletedV4Event {
    /// 访问控制 ID。该 ID 在单个日历实体内唯一。
    #[serde(default)]
    pub acl_id: Option<String>,
    /// 对日历的访问权限。
    #[serde(default)]
    pub role: Option<String>,
    /// 权限生效范围。
    #[serde(default)]
    pub scope: Option<AclScopeEvent>,
    /// 需要推送事件的用户列表。
    #[serde(default)]
    pub user_id_list: Vec<UserId>,
}

/// `calendar.calendar.event.changed_v4` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CalendarCalendarEventChangedV4 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CalendarCalendarEventChangedV4Event,
}

/// `calendar.calendar.event.changed_v4` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CalendarCalendarEventChangedV4Event {
    /// 日程所在的日历 ID。
    #[serde(default)]
    pub calendar_id: Option<String>,
    /// 需要推送事件的用户列表。
    #[serde(default)]
    pub user_id_list: Vec<UserId>,
    /// 发生变更的日程 ID。
    #[serde(default)]
    pub calendar_event_id: Option<String>,
    /// 日程变更类型：`create` / `update` / `delete` / `rsvp`。
    #[serde(default)]
    pub change_type: Option<String>,
    /// RSVP 变更详情，即日程参与人的回复状态。
    #[serde(default)]
    pub rsvp_infos: Vec<OpenEventRsvpInfo>,
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

/// 权限生效范围（Go `AclScopeEvent`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct AclScopeEvent {
    /// 权限生效范围的类型。
    #[serde(default)]
    pub r#type: Option<String>,
    /// 用户 ID。
    #[serde(default)]
    pub user_id: Option<UserId>,
}

/// RSVP 变更详情（Go `OpenEventRsvpInfo`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct OpenEventRsvpInfo {
    /// 用户类型参与人的用户 ID。
    #[serde(default)]
    pub from_user_id: Option<UserId>,
    /// RSVP 操作状态：`accept` / `decline` / `tentative`。
    #[serde(default)]
    pub rsvp_status: Option<String>,
}

typed_events! {
    /// `calendar.calendar.changed_v4` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "calendar.calendar.changed_v4" | CALENDAR_CALENDAR_CHANGED_V4 | CalendarCalendarChangedV4 | CalendarCalendarChangedV4Handler | register_calendar_calendar_changed_v4;

    /// `calendar.calendar.acl.created_v4` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "calendar.calendar.acl.created_v4" | CALENDAR_CALENDAR_ACL_CREATED_V4 | CalendarCalendarAclCreatedV4 | CalendarCalendarAclCreatedV4Handler | register_calendar_calendar_acl_created_v4;

    /// `calendar.calendar.acl.deleted_v4` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "calendar.calendar.acl.deleted_v4" | CALENDAR_CALENDAR_ACL_DELETED_V4 | CalendarCalendarAclDeletedV4 | CalendarCalendarAclDeletedV4Handler | register_calendar_calendar_acl_deleted_v4;

    /// `calendar.calendar.event.changed_v4` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "calendar.calendar.event.changed_v4" | CALENDAR_CALENDAR_EVENT_CHANGED_V4 | CalendarCalendarEventChangedV4 | CalendarCalendarEventChangedV4Handler | register_calendar_calendar_event_changed_v4;
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::ws_client::EventDispatcherHandler;

    const CALENDAR_CHANGED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"c1","event_type":"calendar.calendar.changed_v4","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "user_id_list":[{"open_id":"on_u1","union_id":"un_u1"},{"open_id":"on_u2"}]
      }
    }"#;

    const CALENDAR_ACL_CREATED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"c2","event_type":"calendar.calendar.acl.created_v4","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "acl_id":"acl_1",
        "role":"owner",
        "scope":{"type":"user","user_id":{"open_id":"on_scope"}},
        "user_id_list":[{"open_id":"on_u1"}]
      }
    }"#;

    const CALENDAR_ACL_DELETED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"c3","event_type":"calendar.calendar.acl.deleted_v4","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "acl_id":"acl_2",
        "role":"reader",
        "scope":{"type":"user","user_id":{"open_id":"on_scope2"}},
        "user_id_list":[{"open_id":"on_u2"}]
      }
    }"#;

    const CALENDAR_EVENT_CHANGED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"c4","event_type":"calendar.calendar.event.changed_v4","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "calendar_id":"cal_1",
        "user_id_list":[{"open_id":"on_u1"}],
        "calendar_event_id":"ev_9",
        "change_type":"rsvp",
        "rsvp_infos":[{"from_user_id":{"open_id":"on_rsvp"},"rsvp_status":"accept"}]
      }
    }"#;

    #[test]
    fn typed_calendar_calendar_changed_deserializes_user_list() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_calendar_calendar_changed_v4(move |event: CalendarCalendarChangedV4| {
                assert_eq!(event.header.event_type, "calendar.calendar.changed_v4");
                assert_eq!(event.event.user_id_list.len(), 2);
                assert_eq!(
                    event.event.user_id_list[0].union_id.as_deref(),
                    Some("un_u1")
                );
                *seen_h.lock().expect("lock") = event.event.user_id_list[1].open_id.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(CALENDAR_CHANGED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("on_u2"));
    }

    #[test]
    fn typed_calendar_acl_created_deserializes_scope() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_calendar_calendar_acl_created_v4(
                move |event: CalendarCalendarAclCreatedV4| {
                    assert_eq!(event.event.role.as_deref(), Some("owner"));
                    let scope = event.event.scope.expect("scope");
                    assert_eq!(scope.r#type.as_deref(), Some("user"));
                    assert_eq!(
                        scope.user_id.as_ref().and_then(|u| u.open_id.as_deref()),
                        Some("on_scope")
                    );
                    *seen_h.lock().expect("lock") = event.event.acl_id.clone();
                    Ok(())
                },
            )
            .expect("register")
            .build();
        handler
            .do_without_validation(CALENDAR_ACL_CREATED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("acl_1"));
    }

    #[test]
    fn typed_calendar_acl_deleted_deserializes_scope() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_calendar_calendar_acl_deleted_v4(
                move |event: CalendarCalendarAclDeletedV4| {
                    assert_eq!(event.event.role.as_deref(), Some("reader"));
                    let scope = event.event.scope.expect("scope");
                    assert_eq!(
                        scope.user_id.as_ref().and_then(|u| u.open_id.as_deref()),
                        Some("on_scope2")
                    );
                    *seen_h.lock().expect("lock") = event.event.acl_id.clone();
                    Ok(())
                },
            )
            .expect("register")
            .build();
        handler
            .do_without_validation(CALENDAR_ACL_DELETED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("acl_2"));
    }

    #[test]
    fn typed_calendar_event_changed_deserializes_rsvp() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_calendar_calendar_event_changed_v4(
                move |event: CalendarCalendarEventChangedV4| {
                    assert_eq!(
                        event.header.event_type,
                        "calendar.calendar.event.changed_v4"
                    );
                    assert_eq!(event.event.change_type.as_deref(), Some("rsvp"));
                    let rsvp = event.event.rsvp_infos.first().expect("rsvp_infos");
                    assert_eq!(rsvp.rsvp_status.as_deref(), Some("accept"));
                    assert_eq!(
                        rsvp.from_user_id
                            .as_ref()
                            .and_then(|u| u.open_id.as_deref()),
                        Some("on_rsvp")
                    );
                    *seen_h.lock().expect("lock") = event.event.calendar_event_id.clone();
                    Ok(())
                },
            )
            .expect("register")
            .build();
        handler
            .do_without_validation(CALENDAR_EVENT_CHANGED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("ev_9"));
    }

    /// 事件串常量与票面清单逐字一致。
    #[test]
    fn typed_calendar_event_constants_match_go_dispatcher() {
        assert_eq!(
            EventDispatcherHandler::CALENDAR_CALENDAR_CHANGED_V4,
            "calendar.calendar.changed_v4"
        );
        assert_eq!(
            EventDispatcherHandler::CALENDAR_CALENDAR_ACL_CREATED_V4,
            "calendar.calendar.acl.created_v4"
        );
        assert_eq!(
            EventDispatcherHandler::CALENDAR_CALENDAR_ACL_DELETED_V4,
            "calendar.calendar.acl.deleted_v4"
        );
        assert_eq!(
            EventDispatcherHandler::CALENDAR_CALENDAR_EVENT_CHANGED_V4,
            "calendar.calendar.event.changed_v4"
        );
    }

    /// typed 注册后再 `register_raw` 同 key 返回 Err；反方向同理。
    #[test]
    fn typed_calendar_duplicate_register_fails() {
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
            .register_calendar_calendar_changed_v4(|_event: CalendarCalendarChangedV4| Ok(()))
            .expect("first")
            .register_raw("calendar.calendar.changed_v4", Noop)
            .expect_err("dup typed-then-raw");
        assert!(err.contains("already registered"));
        let err = EventDispatcherHandler::builder()
            .register_raw("calendar.calendar.event.changed_v4", Noop)
            .expect("raw first")
            .register_calendar_calendar_event_changed_v4(
                |_event: CalendarCalendarEventChangedV4| Ok(()),
            )
            .expect_err("dup raw-then-typed");
        assert!(err.contains("already registered"));
    }
}
