//! 测试数据工厂
//!
//! 提供各种测试场景所需的数据生成器，包括：
//! - 基础测试数据（表格、记录、角色等）
//! - 边界条件数据
//! - 错误测试数据
//! - 性能测试数据

use serde_json::{json, Value};

/// 测试数据工厂
pub struct TestDataFactory;

impl TestDataFactory {
    /// 创建测试用的多维表格数据
    pub fn create_test_app() -> Value {
        json!({
            "app": {
                "app_token": "test_app_token_123456789",
                "name": "测试多维表格",
                "avatar": {
                    "token": "test_avatar_token",
                    "uri": "https://example.com/avatar.png"
                },
                "time_zone": "Asia/Shanghai",
                "status": "published",
                "folder_id": "test_folder_id",
                "revision": 1,
                "is_advanced": true,
                "platform": {
                    "name": "web",
                    "version": "1.0.0"
                },
                "created_at": "2023-01-01T00:00:00Z",
                "updated_at": "2023-01-01T00:00:00Z"
            }
        })
    }

    /// 创建测试用的数据表数据
    pub fn create_test_table() -> Value {
        json!({
            "table": {
                "table_id": "test_table_id_123456789",
                "name": "测试数据表",
                "revision": 1,
                "default_view_id": "test_view_id_123456789",
                "field_list": [
                    {
                        "field_id": "field_name",
                        "field_name": "姓名",
                        "field_type": 1,
                        "property": {
                            "default_value": ""
                        }
                    },
                    {
                        "field_id": "field_age",
                        "field_name": "年龄",
                        "field_type": 2,
                        "property": {
                            "default_value": 0,
                            "number_format": {
                                "precision": 0
                            }
                        }
                    }
                ],
                "created_at": "2023-01-01T00:00:00Z",
                "updated_at": "2023-01-01T00:00:00Z"
            }
        })
    }

    /// 创建测试用的记录数据
    pub fn create_test_record() -> Value {
        json!({
            "record": {
                "record_id": "test_record_id_123456789",
                "fields": {
                    "field_name": "测试用户",
                    "field_age": 25
                },
                "created_at": "2023-01-01T00:00:00Z",
                "updated_at": "2023-01-01T00:00:00Z"
            }
        })
    }

    /// 创建测试用的角色数据
    pub fn create_test_role() -> Value {
        json!({
            "role": {
                "role_id": "test_role_id_123456789",
                "name": "测试角色",
                "description": "这是一个测试角色",
                "permission_scopes": ["read", "write"],
                "is_system": false,
                "created_at": "2023-01-01T00:00:00Z",
                "updated_at": "2023-01-01T00:00:00Z"
            }
        })
    }

    /// 创建批量测试记录数据
    pub fn create_batch_test_records(count: usize) -> Vec<Value> {
        (0..count).map(|i| {
            json!({
                "record": {
                    "record_id": format!("test_record_id_{:09}", i),
                    "fields": {
                        "field_name": format!("测试用户{}", i),
                        "field_age": 20 + (i % 50)
                    },
                    "created_at": "2023-01-01T00:00:00Z",
                    "updated_at": "2023-01-01T00:00:00Z"
                }
            })
        }).collect()
    }

    /// 创建错误测试数据（无效字段）
    pub fn create_invalid_record_data() -> Value {
        json!({
            "fields": {
                "non_existent_field": "无效数据",
                "field_age": "invalid_number" // 应该是数字
            }
        })
    }

    /// 创建边界测试数据（长字符串）
    pub fn create_long_text_field(length: usize) -> String {
        "A".repeat(length)
    }

    /// 创建特殊字符测试数据
    pub fn create_special_char_data() -> Value {
        json!({
            "fields": {
                "field_name": "测试\"用户'<>\\/&%$#@!",
                "field_description": "包含\n\t\r特殊字符的\n\n多行文本"
            }
        })
    }

    /// 创建大数据量测试数据
    pub fn create_large_dataset(size: usize) -> Vec<Value> {
        (0..size).map(|i| {
            json!({
                "record": {
                    "record_id": format!("large_record_{:09}", i),
                    "fields": {
                        "field_name": format!("用户{}", i),
                        "field_email": format!("user{}@example.com", i),
                        "field_phone": format!("138{:08}", i % 100000000),
                        "field_address": format!("测试地址{}号，这是一个很长的地址文本用于测试数据存储和传输的性能", i),
                        "field_notes": "这是用户的备注信息，包含详细的描述和一些额外信息"
                    }
                }
            })
        }).collect()
    }

    /// 创建当前时间戳
    pub fn current_timestamp() -> String {
        use chrono::Utc;
        Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
    }

    /// 创建分页测试数据
    pub fn create_paginated_response(total: i32, page_size: i32, _page_token: Option<&str>) -> Value {
        let has_more = total > page_size;
        let next_page_token = if has_more {
            Some(format!("next_token_{}", chrono::Utc::now().timestamp()))
        } else {
            None
        };

        json!({
            "items": TestDataFactory::create_batch_test_records(page_size as usize),
            "page_token": next_page_token,
            "has_more": has_more,
            "total": total
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_test_app() {
        let app = TestDataFactory::create_test_app();
        assert_eq!(app["app"]["app_token"], "test_app_token_123456789");
        assert_eq!(app["app"]["name"], "测试多维表格");
        assert_eq!(app["app"]["status"], "published");
    }

    #[test]
    fn test_create_test_table() {
        let table = TestDataFactory::create_test_table();
        assert_eq!(table["table"]["table_id"], "test_table_id_123456789");
        assert_eq!(table["table"]["name"], "测试数据表");
        assert_eq!(table["table"]["field_list"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_create_test_record() {
        let record = TestDataFactory::create_test_record();
        assert_eq!(record["record"]["record_id"], "test_record_id_123456789");
        assert_eq!(record["record"]["fields"]["field_name"], "测试用户");
        assert_eq!(record["record"]["fields"]["field_age"], 25);
    }

    #[test]
    fn test_create_test_role() {
        let role = TestDataFactory::create_test_role();
        assert_eq!(role["role"]["role_id"], "test_role_id_123456789");
        assert_eq!(role["role"]["name"], "测试角色");
        assert_eq!(role["role"]["is_system"], false);
    }

    #[test]
    fn test_create_batch_test_records() {
        let records = TestDataFactory::create_batch_test_records(5);
        assert_eq!(records.len(), 5);

        for (i, record) in records.iter().enumerate() {
            assert_eq!(record["record"]["record_id"], format!("test_record_id_{:09}", i));
            assert_eq!(record["record"]["fields"]["field_name"], format!("测试用户{}", i));
        }
    }

    #[test]
    fn test_create_long_text_field() {
        let long_text = TestDataFactory::create_long_text_field(100);
        assert_eq!(long_text.len(), 100);
        assert!(long_text.chars().all(|c| c == 'A'));
    }

    #[test]
    fn test_create_special_char_data() {
        let data = TestDataFactory::create_special_char_data();
        assert!(data["fields"]["field_name"].as_str().unwrap().contains("\""));
        assert!(data["fields"]["field_name"].as_str().unwrap().contains("'"));
    }

    #[test]
    fn test_create_paginated_response() {
        let response = TestDataFactory::create_paginated_response(50, 20, None);
        assert_eq!(response["total"], 50);
        assert_eq!(response["items"].as_array().unwrap().len(), 20);
        assert_eq!(response["has_more"], true);
        assert!(!response["page_token"].is_null());
    }

    #[test]
    fn test_current_timestamp() {
        let timestamp = TestDataFactory::current_timestamp();
        assert!(timestamp.len() > 0);
        assert!(timestamp.contains('T'));
        assert!(timestamp.contains('Z'));
    }
}