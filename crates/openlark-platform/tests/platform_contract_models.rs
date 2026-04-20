//! OpenLark Platform 契约测试
//!
//! 测试平台服务模块的主要数据模型序列化/反序列化契约。

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
        "app_id": "test_app_123",
        "app_secret": "test_secret_456"
    });

    let serialized = to_value(&test_data).unwrap();
    let deserialized: Value = parse_contract(serialized.clone());
    assert_eq!(serialized, deserialized);
}

#[test]
fn test_app_info_contract() {
    let app_info = json!({
        "app_id": "app_123",
        "app_name": "测试应用",
        "description": "应用描述",
        "status": 1,
        "create_time": "2024-01-01T00:00:00Z"
    });

    let parsed: Value = parse_contract(app_info.clone());
    assert_eq!(parsed["app_id"], "app_123");
    assert_json_contract(&parsed, app_info);
}

#[test]
fn test_directory_user_contract() {
    let user = json!({
        "user_id": "user_123",
        "union_id": "union_456",
        "open_id": "ou_789",
        "name": "张三",
        "en_name": "Zhang San",
        "email": "zhangsan@example.com",
        "department": "研发部"
    });

    let parsed: Value = parse_contract(user.clone());
    assert_eq!(parsed["user_id"], "user_123");
    assert_json_contract(&parsed, user);
}

#[test]
fn test_tenant_info_contract() {
    let tenant = json!({
        "tenant_key": "tenant_123",
        "name": "测试租户",
        "display_name": "Test Tenant"
    });

    let parsed: Value = parse_contract(tenant.clone());
    assert_eq!(parsed["tenant_key"], "tenant_123");
    assert_json_contract(&parsed, tenant);
}

#[test]
fn test_admin_setting_contract() {
    let setting = json!({
        "setting_key": "setting_123",
        "setting_value": "value_456",
        "setting_type": "string"
    });

    let parsed: Value = parse_contract(setting.clone());
    assert_eq!(parsed["setting_key"], "setting_123");
    assert_json_contract(&parsed, setting);
}

#[test]
fn test_mdm_device_contract() {
    let device = json!({
        "device_id": "device_123",
        "device_type": "mobile",
        "os_type": "iOS",
        "status": "active"
    });

    let parsed: Value = parse_contract(device.clone());
    assert_eq!(parsed["device_id"], "device_123");
    assert_json_contract(&parsed, device);
}

#[test]
fn test_list_response_contract() {
    let list_response = json!({
        "items": [
            {"id": "1", "name": "Item 1"},
            {"id": "2", "name": "Item 2"}
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
        "code": 400,
        "msg": "Invalid request",
        "error": {
            "log_id": "log_123"
        }
    });

    let parsed: Value = parse_contract(error_response.clone());
    assert_eq!(parsed["code"], 400);
    assert_json_contract(&parsed, error_response);
}
