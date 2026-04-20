//! OpenLark Protocol 契约测试
//!
//! 测试 WebSocket 协议模块的主要数据模型序列化/反序列化契约。

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
        "msg_type": "text",
        "content": "Hello"
    });

    let serialized = to_value(&test_data).unwrap();
    let deserialized: Value = parse_contract(serialized.clone());
    assert_eq!(serialized, deserialized);
}

#[test]
fn test_websocket_frame_contract() {
    let frame = json!({
        "frame_type": "message",
        "version": 1,
        "payload": {
            "event_type": "message.receive_v1",
            "token": "token_123",
            "app_id": "app_456",
            "tenant_key": "tenant_789"
        }
    });

    let parsed: Value = parse_contract(frame.clone());
    assert_eq!(parsed["frame_type"], "message");
    assert_eq!(parsed["version"], 1);
    assert_json_contract(&parsed, frame);
}

#[test]
fn test_event_message_contract() {
    let event = json!({
        "schema": "2.0",
        "header": {
            "event_id": "event_123",
            "event_type": "im.message.receive_v1",
            "token": "token_456",
            "app_id": "app_789",
            "tenant_key": "tenant_abc",
            "create_time": "1234567890000"
        },
        "event": {
            "message": {
                "message_id": "msg_123",
                "root_id": "root_456",
                "parent_id": "parent_789",
                "create_time": "1234567890000",
                "chat_id": "chat_abc",
                "chat_type": "p2p",
                "message_type": "text",
                "content": "{\"text\":\"Hello\"}",
                "mentions": []
            },
            "sender": {
                "sender_id": {
                    "union_id": "union_123",
                    "user_id": "user_456",
                    "open_id": "ou_789"
                },
                "sender_type": "user",
                "tenant_key": "tenant_abc"
            }
        }
    });

    let parsed: Value = parse_contract(event.clone());
    assert_eq!(parsed["schema"], "2.0");
    assert_eq!(parsed["header"]["event_type"], "im.message.receive_v1");
    assert_json_contract(&parsed, event);
}

#[test]
fn test_challenge_request_contract() {
    let challenge = json!({
        "challenge": "challenge_token_123",
        "token": "verification_token_456",
        "type": "url_verification"
    });

    let parsed: Value = parse_contract(challenge.clone());
    assert_eq!(parsed["type"], "url_verification");
    assert_json_contract(&parsed, challenge);
}

#[test]
fn test_challenge_response_contract() {
    let response = json!({
        "challenge": "challenge_token_123"
    });

    let parsed: Value = parse_contract(response.clone());
    assert_eq!(parsed["challenge"], "challenge_token_123");
    assert_json_contract(&parsed, response);
}

#[test]
fn test_pong_response_contract() {
    let pong = json!({
        "type": "pong",
        "timestamp": 1234567890
    });

    let parsed: Value = parse_contract(pong.clone());
    assert_eq!(parsed["type"], "pong");
    assert_json_contract(&parsed, pong);
}

#[test]
fn test_ping_request_contract() {
    let ping = json!({
        "type": "ping",
        "timestamp": 1234567890
    });

    let parsed: Value = parse_contract(ping.clone());
    assert_eq!(parsed["type"], "ping");
    assert_json_contract(&parsed, ping);
}

#[test]
fn test_message_content_text_contract() {
    let content = json!({
        "text": "Hello World"
    });

    let parsed: Value = parse_contract(content.clone());
    assert_eq!(parsed["text"], "Hello World");
    assert_json_contract(&parsed, content);
}

#[test]
fn test_message_content_image_contract() {
    let content = json!({
        "image_key": "img_123"
    });

    let parsed: Value = parse_contract(content.clone());
    assert_eq!(parsed["image_key"], "img_123");
    assert_json_contract(&parsed, content);
}

#[test]
fn test_error_response_contract() {
    let error_response = json!({
        "code": 400,
        "msg": "Invalid WebSocket frame",
        "error": {
            "log_id": "log_123"
        }
    });

    let parsed: Value = parse_contract(error_response.clone());
    assert_eq!(parsed["code"], 400);
    assert_json_contract(&parsed, error_response);
}
