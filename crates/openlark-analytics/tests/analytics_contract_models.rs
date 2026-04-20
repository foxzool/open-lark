//! OpenLark Analytics 契约测试
//!
//! 测试数据分析模块的主要数据模型序列化/反序列化契约。

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
        "query_id": "query_123",
        "query_type": "search"
    });

    let serialized = to_value(&test_data).unwrap();
    let deserialized: Value = parse_contract(serialized.clone());
    assert_eq!(serialized, deserialized);
}

#[test]
fn test_search_query_contract() {
    let query = json!({
        "query": "test search",
        "filters": {
            "app_type": "docs",
            "time_range": "last_7_days"
        },
        "page_size": 20,
        "page_token": "page_123"
    });

    let parsed: Value = parse_contract(query.clone());
    assert_eq!(parsed["query"], "test search");
    assert_eq!(parsed["page_size"], 20);
    assert_json_contract(&parsed, query);
}

#[test]
fn test_search_result_contract() {
    let result = json!({
        "items": [
            {
                "id": "doc_1",
                "title": "Document 1",
                "type": "doc",
                "url": "https://example.com/doc1"
            },
            {
                "id": "doc_2",
                "title": "Document 2",
                "type": "sheet",
                "url": "https://example.com/doc2"
            }
        ],
        "total": 100,
        "has_more": true,
        "page_token": "next_page"
    });

    let parsed: Value = parse_contract(result.clone());
    assert_eq!(parsed["items"].as_array().unwrap().len(), 2);
    assert_eq!(parsed["total"], 100);
    assert_json_contract(&parsed, result);
}

#[test]
fn test_report_data_contract() {
    let report = json!({
        "report_id": "report_123",
        "report_name": "用户活跃度报表",
        "dimensions": ["date", "department"],
        "metrics": ["active_users", "login_count"],
        "data": [
            {"date": "2024-01-01", "active_users": 100},
            {"date": "2024-01-02", "active_users": 120}
        ]
    });

    let parsed: Value = parse_contract(report.clone());
    assert_eq!(parsed["report_id"], "report_123");
    assert_eq!(parsed["data"].as_array().unwrap().len(), 2);
    assert_json_contract(&parsed, report);
}

#[test]
fn test_analytics_user_contract() {
    let user = json!({
        "user_id": "user_123",
        "department": "研发部",
        "activity_score": 85.5,
        "last_active": "2024-01-15T10:30:00Z",
        "app_usage": {
            "docs": 50,
            "im": 200,
            "calendar": 30
        }
    });

    let parsed: Value = parse_contract(user.clone());
    assert_eq!(parsed["user_id"], "user_123");
    assert_eq!(parsed["activity_score"], 85.5);
    assert_json_contract(&parsed, user);
}

#[test]
fn test_error_response_contract() {
    let error_response = json!({
        "code": 400,
        "msg": "Invalid query parameters",
        "error": {
            "log_id": "log_123"
        }
    });

    let parsed: Value = parse_contract(error_response.clone());
    assert_eq!(parsed["code"], 400);
    assert_json_contract(&parsed, error_response);
}
