use serde::{Deserialize, Serialize};

use crate::{
    event::{context::EventHeader, dispatcher::EventHandler},
    service::im::v1::p2_im_message_receive_v1::UserId,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct P2ImMessageReadV1 {
    pub schema: String,
    pub header: EventHeader,
    pub event: P2ImMessageMessageReadV1Data,
}

/// 消息已读事件数据
#[derive(Debug, Serialize, Deserialize)]
pub struct P2ImMessageMessageReadV1Data {
    /// 消息阅读者信息
    pub reader: EventReader,
    /// 已读消息ID列表
    pub message_id_list: Vec<String>,
}

/// 消息阅读者信息
#[derive(Debug, Serialize, Deserialize)]
pub struct EventReader {
    /// 阅读时间戳（毫秒）
    pub read_time: String,
    /// 阅读者ID信息
    pub reader_id: UserId,
    /// tenant key，为租户在飞书上的唯一标识
    pub tenant_key: String,
}

pub struct P2ImMessageReadV1ProcessorImpl<F>
where
    F: Fn(P2ImMessageReadV1) + 'static,
{
    f: F,
}

impl<F> P2ImMessageReadV1ProcessorImpl<F>
where
    F: Fn(P2ImMessageReadV1) + 'static,
{
    pub fn new(f: F) -> Self {
        P2ImMessageReadV1ProcessorImpl { f }
    }
}

impl<F> EventHandler for P2ImMessageReadV1ProcessorImpl<F>
where
    F: Fn(P2ImMessageReadV1) + 'static + Sync + Send,
{
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
        let message: P2ImMessageReadV1 = serde_json::from_slice(payload)?;
        (self.f)(message);
        Ok(())
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use crate::event::context::EventHeader;
    use crate::service::im::v1::p2_im_message_receive_v1::UserId;
    use serde_json;
    use std::sync::{Arc, Mutex};

    fn create_test_user_id() -> UserId {
        UserId {
            union_id: "test_union_id".to_string(),
            user_id: "test_user_id".to_string(),
            open_id: "test_open_id".to_string(),
        }
    }

    fn create_test_event_header() -> EventHeader {
        EventHeader {
            event_id: Some("test_event_id".to_string()),
            event_type: Some("im.message.read_v1".to_string()),
            create_time: Some("1640995200000".to_string()),
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            tenant_key: Some("test_tenant_key".to_string()),
        }
    }

    fn create_test_event_reader() -> EventReader {
        EventReader {
            read_time: "1640995200000".to_string(),
            reader_id: create_test_user_id(),
            tenant_key: "test_tenant_key".to_string(),
        }
    }

    fn create_test_message_read_data() -> P2ImMessageMessageReadV1Data {
        P2ImMessageMessageReadV1Data {
            reader: create_test_event_reader(),
            message_id_list: vec![
                "msg_id_1".to_string(),
                "msg_id_2".to_string(),
                "msg_id_3".to_string(),
            ],
        }
    }

    fn create_test_p2_im_message_read() -> P2ImMessageReadV1 {
        P2ImMessageReadV1 {
            schema: "2.0".to_string(),
            header: create_test_event_header(),
            event: create_test_message_read_data(),
        }
    }

    #[test]
    fn test_event_reader_creation() {
        let reader = create_test_event_reader();

        assert_eq!(reader.read_time, "1640995200000");
        assert_eq!(reader.tenant_key, "test_tenant_key");
        assert_eq!(reader.reader_id.user_id, "test_user_id");
        assert_eq!(reader.reader_id.union_id, "test_union_id");
        assert_eq!(reader.reader_id.open_id, "test_open_id");
    }

    #[test]
    fn test_event_reader_serialization() {
        let reader = create_test_event_reader();
        let json = serde_json::to_string(&reader).unwrap();

        assert!(json.contains("1640995200000"));
        assert!(json.contains("test_tenant_key"));
        assert!(json.contains("test_user_id"));
        assert!(json.contains("test_union_id"));
        assert!(json.contains("test_open_id"));
    }

    #[test]
    fn test_event_reader_deserialization() {
        let json = r#"{
            "read_time": "1640995200000",
            "reader_id": {
                "union_id": "test_union_id",
                "user_id": "test_user_id",
                "open_id": "test_open_id"
            },
            "tenant_key": "test_tenant_key"
        }"#;

        let reader: EventReader = serde_json::from_str(json).unwrap();
        assert_eq!(reader.read_time, "1640995200000");
        assert_eq!(reader.tenant_key, "test_tenant_key");
        assert_eq!(reader.reader_id.user_id, "test_user_id");
        assert_eq!(reader.reader_id.union_id, "test_union_id");
        assert_eq!(reader.reader_id.open_id, "test_open_id");
    }

    #[test]
    fn test_p2_im_message_read_v1_data_creation() {
        let data = create_test_message_read_data();

        assert_eq!(data.message_id_list.len(), 3);
        assert_eq!(data.message_id_list[0], "msg_id_1");
        assert_eq!(data.message_id_list[1], "msg_id_2");
        assert_eq!(data.message_id_list[2], "msg_id_3");
        assert_eq!(data.reader.read_time, "1640995200000");
    }

    #[test]
    fn test_p2_im_message_read_v1_data_serialization() {
        let data = create_test_message_read_data();
        let json = serde_json::to_string(&data).unwrap();

        assert!(json.contains("msg_id_1"));
        assert!(json.contains("msg_id_2"));
        assert!(json.contains("msg_id_3"));
        assert!(json.contains("1640995200000"));
        assert!(json.contains("test_tenant_key"));
    }

    #[test]
    fn test_p2_im_message_read_v1_data_deserialization() {
        let json = r#"{
            "reader": {
                "read_time": "1640995200000",
                "reader_id": {
                    "union_id": "test_union_id",
                    "user_id": "test_user_id",
                    "open_id": "test_open_id"
                },
                "tenant_key": "test_tenant_key"
            },
            "message_id_list": ["msg_1", "msg_2"]
        }"#;

        let data: P2ImMessageMessageReadV1Data = serde_json::from_str(json).unwrap();
        assert_eq!(data.message_id_list.len(), 2);
        assert_eq!(data.message_id_list[0], "msg_1");
        assert_eq!(data.message_id_list[1], "msg_2");
        assert_eq!(data.reader.read_time, "1640995200000");
    }

    #[test]
    fn test_p2_im_message_read_v1_creation() {
        let message = create_test_p2_im_message_read();

        assert_eq!(message.schema, "2.0");
        assert_eq!(
            message.header.event_type,
            Some("im.message.read_v1".to_string())
        );
        assert_eq!(message.header.app_id, Some("test_app_id".to_string()));
        assert_eq!(message.event.message_id_list.len(), 3);
    }

    #[test]
    fn test_p2_im_message_read_v1_serialization() {
        let message = create_test_p2_im_message_read();
        let json = serde_json::to_string(&message).unwrap();

        assert!(json.contains("2.0"));
        assert!(json.contains("im.message.read_v1"));
        assert!(json.contains("test_app_id"));
        assert!(json.contains("msg_id_1"));
        assert!(json.contains("msg_id_2"));
        assert!(json.contains("msg_id_3"));
    }

    #[test]
    fn test_p2_im_message_read_v1_deserialization() {
        let json = r#"{
            "schema": "2.0",
            "header": {
                "event_id": "test_event_id",
                "event_type": "im.message.read_v1",
                "create_time": "1640995200000",
                "token": "test_token",
                "app_id": "test_app_id",
                "tenant_key": "test_tenant_key"
            },
            "event": {
                "reader": {
                    "read_time": "1640995200000",
                    "reader_id": {
                        "union_id": "test_union_id",
                        "user_id": "test_user_id",
                        "open_id": "test_open_id"
                    },
                    "tenant_key": "test_tenant_key"
                },
                "message_id_list": ["msg_a", "msg_b"]
            }
        }"#;

        let message: P2ImMessageReadV1 = serde_json::from_str(json).unwrap();
        assert_eq!(message.schema, "2.0");
        assert_eq!(
            message.header.event_type,
            Some("im.message.read_v1".to_string())
        );
        assert_eq!(message.header.app_id, Some("test_app_id".to_string()));
        assert_eq!(message.event.message_id_list.len(), 2);
        assert_eq!(message.event.message_id_list[0], "msg_a");
        assert_eq!(message.event.message_id_list[1], "msg_b");
    }

    #[test]
    fn test_processor_impl_creation() {
        let processor = P2ImMessageReadV1ProcessorImpl::new(|_message| {
            // Test callback
        });

        // Processor should be created successfully
        // We can't directly test the function field, but we can test that creation works
        let ptr = std::ptr::addr_of!(processor) as *const u8;
        assert!(!ptr.is_null());
    }

    #[test]
    fn test_processor_impl_handle_success() {
        let called = Arc::new(Mutex::new(false));
        let called_clone = Arc::clone(&called);

        let processor = P2ImMessageReadV1ProcessorImpl::new(move |message| {
            let mut called = called_clone.lock().unwrap();
            *called = true;

            // Verify the message content
            assert_eq!(message.schema, "2.0");
            assert_eq!(
                message.header.event_type,
                Some("im.message.read_v1".to_string())
            );
        });

        let test_message = create_test_p2_im_message_read();
        let payload = serde_json::to_vec(&test_message).unwrap();

        let result = processor.handle(&payload);
        assert!(result.is_ok());

        let was_called = *called.lock().unwrap();
        assert!(was_called);
    }

    #[test]
    fn test_processor_impl_handle_invalid_json() {
        let processor = P2ImMessageReadV1ProcessorImpl::new(|_message| {
            panic!("This should not be called with invalid JSON");
        });

        let invalid_payload = b"invalid json";
        let result = processor.handle(invalid_payload);

        assert!(result.is_err());
    }

    #[test]
    fn test_processor_impl_handle_with_callback_data() {
        let received_messages = Arc::new(Mutex::new(Vec::new()));
        let messages_clone = Arc::clone(&received_messages);

        let processor = P2ImMessageReadV1ProcessorImpl::new(move |message| {
            let mut messages = messages_clone.lock().unwrap();
            messages.push(message.event.message_id_list.clone());
        });

        // Test multiple messages
        let test_cases = vec![
            vec!["msg1".to_string()],
            vec!["msg2".to_string(), "msg3".to_string()],
            vec!["msg4".to_string(), "msg5".to_string(), "msg6".to_string()],
        ];

        for message_ids in &test_cases {
            let mut test_message = create_test_p2_im_message_read();
            test_message.event.message_id_list = message_ids.clone();

            let payload = serde_json::to_vec(&test_message).unwrap();
            let result = processor.handle(&payload);
            assert!(result.is_ok());
        }

        let received = received_messages.lock().unwrap();
        assert_eq!(received.len(), 3);
        assert_eq!(received[0], test_cases[0]);
        assert_eq!(received[1], test_cases[1]);
        assert_eq!(received[2], test_cases[2]);
    }

    #[test]
    fn test_debug_trait_implementations() {
        let reader = create_test_event_reader();
        let data = create_test_message_read_data();
        let message = create_test_p2_im_message_read();

        let reader_debug = format!("{:?}", reader);
        let data_debug = format!("{:?}", data);
        let message_debug = format!("{:?}", message);

        assert!(reader_debug.contains("EventReader"));
        assert!(reader_debug.contains("1640995200000"));

        assert!(data_debug.contains("P2ImMessageMessageReadV1Data"));
        assert!(data_debug.contains("msg_id_1"));

        assert!(message_debug.contains("P2ImMessageReadV1"));
        assert!(message_debug.contains("2.0"));
    }

    #[test]
    fn test_event_reader_with_empty_strings() {
        let reader = EventReader {
            read_time: "".to_string(),
            reader_id: UserId {
                union_id: "".to_string(),
                user_id: "".to_string(),
                open_id: "".to_string(),
            },
            tenant_key: "".to_string(),
        };

        let json = serde_json::to_string(&reader).unwrap();
        let deserialized: EventReader = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.read_time, "");
        assert_eq!(deserialized.tenant_key, "");
        assert_eq!(deserialized.reader_id.user_id, "");
    }

    #[test]
    fn test_message_read_data_with_empty_message_list() {
        let data = P2ImMessageMessageReadV1Data {
            reader: create_test_event_reader(),
            message_id_list: vec![],
        };

        let json = serde_json::to_string(&data).unwrap();
        let deserialized: P2ImMessageMessageReadV1Data = serde_json::from_str(&json).unwrap();

        assert!(deserialized.message_id_list.is_empty());
        assert_eq!(deserialized.reader.read_time, "1640995200000");
    }
}
