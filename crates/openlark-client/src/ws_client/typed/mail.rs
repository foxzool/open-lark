//! mail 域 typed 事件（`mail.*`）。
//!
//! 字段权威：Go SDK v3.12.2 `service/mail/v1/model.go` 对应 `P2*` 结构的 json tag；
//! 事件清单对齐 `service/mail/v1/dispatcher.go`。Go 指针字段映射为 `Option<T>`，
//! serde 采用 `#[serde(default)]` 宽松风格，与 [`super::im::ImMessageReceiveV1`] 一致。

use serde::{Deserialize, Serialize};

use super::EventHeader;

/// `mail.user_mailbox.event.message_received_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MailUserMailboxEventMessageReceivedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: MailUserMailboxEventMessageReceivedV1Event,
}

/// `mail.user_mailbox.event.message_received_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MailUserMailboxEventMessageReceivedV1Event {
    /// 收信的邮箱。
    #[serde(default)]
    pub mail_address: Option<String>,
    /// 邮件 ID。
    #[serde(default)]
    pub message_id: Option<String>,
    /// 收到邮件的邮箱类型。
    #[serde(default)]
    pub mailbox_type: Option<i64>,
    /// 订阅者。
    #[serde(default)]
    pub subscriber: Option<Subscriber>,
}

/// 订阅者（Go `Subscriber`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Subscriber {
    /// 收到邮件的用户 ID 列表。
    #[serde(default)]
    pub user_ids: Vec<UserId>,
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
    /// `mail.user_mailbox.event.message_received_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "mail.user_mailbox.event.message_received_v1" | MAIL_USER_MAILBOX_EVENT_MESSAGE_RECEIVED_V1 | MailUserMailboxEventMessageReceivedV1 | MailUserMailboxEventMessageReceivedV1Handler | register_mail_user_mailbox_event_message_received_v1;
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::ws_client::EventDispatcherHandler;

    const MESSAGE_RECEIVED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"m1","event_type":"mail.user_mailbox.event.message_received_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "mail_address":"dev@example.com",
        "message_id":"msg_1",
        "mailbox_type":1,
        "subscriber":{"user_ids":[{"open_id":"on_sub1"},{"open_id":"on_sub2"}]}
      }
    }"#;

    #[test]
    fn typed_mail_message_received_deserializes_subscriber() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_mail_user_mailbox_event_message_received_v1(
                move |event: MailUserMailboxEventMessageReceivedV1| {
                    assert_eq!(
                        event.header.event_type,
                        "mail.user_mailbox.event.message_received_v1"
                    );
                    assert_eq!(event.event.mail_address.as_deref(), Some("dev@example.com"));
                    assert_eq!(event.event.mailbox_type, Some(1));
                    let subscriber = event.event.subscriber.expect("subscriber");
                    assert_eq!(subscriber.user_ids.len(), 2);
                    assert_eq!(subscriber.user_ids[1].open_id.as_deref(), Some("on_sub2"));
                    *seen_h.lock().expect("lock") = event.event.message_id.clone();
                    Ok(())
                },
            )
            .expect("register")
            .build();
        handler
            .do_without_validation(MESSAGE_RECEIVED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("msg_1"));
    }

    /// 事件串常量与票面清单逐字一致。
    #[test]
    fn typed_mail_event_constants_match_go_dispatcher() {
        assert_eq!(
            EventDispatcherHandler::MAIL_USER_MAILBOX_EVENT_MESSAGE_RECEIVED_V1,
            "mail.user_mailbox.event.message_received_v1"
        );
    }

    /// typed 注册后再 `register_raw` 同 key 返回 Err；反方向同理。
    #[test]
    fn typed_mail_duplicate_register_fails() {
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
            .register_mail_user_mailbox_event_message_received_v1(
                |_event: MailUserMailboxEventMessageReceivedV1| Ok(()),
            )
            .expect("first")
            .register_raw("mail.user_mailbox.event.message_received_v1", Noop)
            .expect_err("dup typed-then-raw");
        assert!(err.contains("already registered"));
        let err = EventDispatcherHandler::builder()
            .register_raw("mail.user_mailbox.event.message_received_v1", Noop)
            .expect("raw first")
            .register_mail_user_mailbox_event_message_received_v1(
                |_event: MailUserMailboxEventMessageReceivedV1| Ok(()),
            )
            .expect_err("dup raw-then-typed");
        assert!(err.contains("already registered"));
    }
}
