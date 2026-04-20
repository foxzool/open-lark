//! OpenLark Mail 契约测试
//!
//! 测试邮件服务模块的主要数据模型序列化/反序列化契约。

use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_value, json, to_value, Value};

fn assert_json_contract<T>(value: &T, expected: Value)
where
    T: Serialize,
{
    assert_eq!(to_value(value).unwrap(), expected);
}

fn parse_contract<T>(payload: Value) -> T
where
    T: DeserializeOwned,
{
    from_value(payload).unwrap()
}

#[test]
fn test_json_serialization_contracts() {
    let test_data = json!({
        "email_id": "email_123",
        "subject": "Test Subject"
    });

    let serialized = to_value(&test_data).unwrap();
    let deserialized: Value = parse_contract(serialized.clone());
    assert_eq!(serialized, deserialized);
}

#[test]
fn test_email_message_contract() {
    let message = json!({
        "message_id": "msg_123",
        "thread_id": "thread_456",
        "subject": "会议邀请",
        "from": {
            "email": "sender@example.com",
            "name": "张三"
        },
        "to": [
            {"email": "recipient1@example.com", "name": "李四"},
            {"email": "recipient2@example.com", "name": "王五"}
        ],
        "cc": [],
        "bcc": [],
        "body": {
            "content_type": "text/html",
            "content": "<p>会议详情...</p>"
        },
        "attachments": [
            {"file_name": "agenda.pdf", "size": 1024}
        ],
        "sent_time": "2024-01-15T10:00:00Z",
        "labels": ["INBOX", "IMPORTANT"]
    });

    let parsed: Value = parse_contract(message.clone());
    assert_eq!(parsed["message_id"], "msg_123");
    assert_eq!(parsed["subject"], "会议邀请");
    assert_json_contract(&parsed, message);
}

#[test]
fn test_email_thread_contract() {
    let thread = json!({
        "thread_id": "thread_123",
        "subject": "项目讨论",
        "participants": [
            {"email": "user1@example.com"},
            {"email": "user2@example.com"}
        ],
        "message_count": 5,
        "last_message_time": "2024-01-15T14:30:00Z",
        "has_unread": true
    });

    let parsed: Value = parse_contract(thread.clone());
    assert_eq!(parsed["thread_id"], "thread_123");
    assert_eq!(parsed["message_count"], 5);
    assert_json_contract(&parsed, thread);
}

#[test]
fn test_email_address_contract() {
    let address = json!({
        "email": "test@example.com",
        "name": "测试用户",
        "display_name": "测试用户 <test@example.com>"
    });

    let parsed: Value = parse_contract(address.clone());
    assert_eq!(parsed["email"], "test@example.com");
    assert_json_contract(&parsed, address);
}

#[test]
fn test_email_list_response_contract() {
    let list_response = json!({
        "messages": [
            {
                "message_id": "msg_1",
                "subject": "邮件1",
                "snippet": "邮件摘要..."
            },
            {
                "message_id": "msg_2",
                "subject": "邮件2",
                "snippet": "邮件摘要..."
            }
        ],
        "next_page_token": "next_page",
        "result_size_estimate": 100
    });

    let parsed: Value = parse_contract(list_response.clone());
    assert_eq!(parsed["messages"].as_array().unwrap().len(), 2);
    assert_json_contract(&parsed, list_response);
}

#[test]
fn test_draft_contract() {
    let draft = json!({
        "draft_id": "draft_123",
        "message": {
            "subject": "草稿邮件",
            "to": [{"email": "to@example.com"}],
            "body": {
                "content_type": "text/plain",
                "content": "草稿内容"
            }
        },
        "created_time": "2024-01-15T09:00:00Z",
        "updated_time": "2024-01-15T09:30:00Z"
    });

    let parsed: Value = parse_contract(draft.clone());
    assert_eq!(parsed["draft_id"], "draft_123");
    assert_json_contract(&parsed, draft);
}

#[test]
fn test_label_contract() {
    let label = json!({
        "label_id": "label_123",
        "name": "重要",
        "color": "#FF0000",
        "type": "user",
        "message_total": 50,
        "message_unread": 5
    });

    let parsed: Value = parse_contract(label.clone());
    assert_eq!(parsed["label_id"], "label_123");
    assert_eq!(parsed["name"], "重要");
    assert_json_contract(&parsed, label);
}

#[test]
fn test_error_response_contract() {
    let error_response = json!({
        "code": 400,
        "msg": "Invalid email address",
        "error": {
            "log_id": "log_123"
        }
    });

    let parsed: Value = parse_contract(error_response.clone());
    assert_eq!(parsed["code"], 400);
    assert_json_contract(&parsed, error_response);
}
