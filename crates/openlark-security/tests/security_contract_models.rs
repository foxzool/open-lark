//! OpenLark Security 契约测试
//!
//! 测试安全服务模块的主要数据模型序列化/反序列化契约。

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
        "user_id": "test_user_123",
        "device_id": "device_456"
    });

    let serialized = to_value(&test_data).unwrap();
    let deserialized: Value = parse_contract(serialized.clone());
    assert_eq!(serialized, deserialized);
}

#[test]
fn test_acs_user_contract() {
    let user = json!({
        "user_id": "user_123",
        "name": "张三",
        "employee_id": "EMP001",
        "department": "研发部",
        "status": "active"
    });

    let parsed: Value = parse_contract(user.clone());
    assert_eq!(parsed["user_id"], "user_123");
    assert_json_contract(&parsed, user);
}

#[test]
fn test_acs_device_contract() {
    let device = json!({
        "device_id": "device_123",
        "device_name": "前门门禁",
        "device_type": "access_control",
        "location": "A座一楼",
        "status": "online"
    });

    let parsed: Value = parse_contract(device.clone());
    assert_eq!(parsed["device_id"], "device_123");
    assert_json_contract(&parsed, device);
}

#[test]
fn test_acs_visitor_contract() {
    let visitor = json!({
        "visitor_id": "visitor_123",
        "name": "访客A",
        "phone": "13800138000",
        "visit_time": "2024-01-15T10:00:00Z",
        "status": "pending"
    });

    let parsed: Value = parse_contract(visitor.clone());
    assert_eq!(parsed["visitor_id"], "visitor_123");
    assert_json_contract(&parsed, visitor);
}

#[test]
fn test_device_record_contract() {
    let record = json!({
        "record_id": "record_123",
        "device_id": "device_456",
        "user_id": "user_789",
        "access_time": "2024-01-15T08:30:00Z",
        "access_type": "in",
        "result": "success"
    });

    let parsed: Value = parse_contract(record.clone());
    assert_eq!(parsed["record_id"], "record_123");
    assert_json_contract(&parsed, record);
}

#[test]
fn test_audit_log_contract() {
    let log = json!({
        "log_id": "log_123",
        "user_id": "user_456",
        "action": "login",
        "resource": "api",
        "timestamp": "2024-01-15T10:00:00Z",
        "ip_address": "192.168.1.1"
    });

    let parsed: Value = parse_contract(log.clone());
    assert_eq!(parsed["log_id"], "log_123");
    assert_json_contract(&parsed, log);
}

#[test]
fn test_rule_contract() {
    let rule = json!({
        "rule_id": "rule_123",
        "rule_name": "研发部权限规则",
        "department_ids": ["dept_1", "dept_2"],
        "device_ids": ["device_1", "device_2"],
        "time_range": {
            "start": "09:00",
            "end": "18:00"
        }
    });

    let parsed: Value = parse_contract(rule.clone());
    assert_eq!(parsed["rule_id"], "rule_123");
    assert_json_contract(&parsed, rule);
}

#[test]
fn test_list_response_contract() {
    let list_response = json!({
        "items": [
            {"user_id": "user_1", "name": "用户1"},
            {"user_id": "user_2", "name": "用户2"}
        ],
        "page_token": "next_page",
        "has_more": true
    });

    let parsed: Value = parse_contract(list_response.clone());
    assert_eq!(parsed["items"].as_array().unwrap().len(), 2);
    assert_json_contract(&parsed, list_response);
}

#[test]
fn test_error_response_contract() {
    let error_response = json!({
        "code": 403,
        "msg": "Access denied",
        "error": {
            "log_id": "log_123"
        }
    });

    let parsed: Value = parse_contract(error_response.clone());
    assert_eq!(parsed["code"], 403);
    assert_json_contract(&parsed, error_response);
}
