//! 记录管理API测试
//!
//! 测试覆盖：
//! - 新增记录（单个和批量）
//! - 获取记录（单个和列表）
//! - 更新记录（单个和批量）
//! - 删除记录（单个和批量）
//! - 搜索记录

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{
        MockServerManager, TestConfig, TestHelper, TestDataFactory,
        ResponseValidator, PerformanceTest, BatchTest
    };
    use wiremock::{Mock, ResponseTemplate};
    use wiremock::matchers::{method, path_regex};
    use serde_json::json;
    use std::time::Duration;

    async fn setup_test_server() -> MockServerManager {
        let config = TestConfig::new();
        let server = MockServerManager::new(config).await.unwrap();
        server.register_base_mocks().await.unwrap();
        server
    }

    #[tokio::test]
    async fn test_create_record_request_builder() {
        let config = openlark_core::Config::new(openlark_core::ConfigInner {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            ..Default::default()
        });

        let request = CreateRecordRequest::new(config)
            .builder()
            .app_token("test_app_token")
            .table_id("test_table_id")
            .fields(json!({
                "field_name": "张三",
                "field_age": 25,
                "field_email": "zhangsan@example.com"
            }))
            .build();

        assert_eq!(request.app_token, "test_app_token");
        assert_eq!(request.table_id, "test_table_id");
        assert_eq!(request.fields["field_name"], "张三");
        assert_eq!(request.fields["field_age"], 25);
    }

    #[tokio::test]
    async fn test_create_record_serialization() {
        let fields = json!({
            "field_name": "李四",
            "field_age": 30,
            "field_phone": "13800138000"
        });

        let request_body = CreateRecordRequestBody {
            fields,
        };

        let serialized = serde_json::to_value(&request_body).unwrap();
        let expected = json!({
            "fields": {
                "field_name": "李四",
                "field_age": 30,
                "field_phone": "13800138000"
            }
        });

        assert_eq!(serialized, expected);
    }

    #[tokio::test]
    async fn test_get_record_request_builder() {
        let config = openlark_core::Config::new(openlark_core::ConfigInner {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            ..Default::default()
        });

        let request = GetRecordRequest::new(config)
            .builder()
            .app_token("test_app_token")
            .table_id("test_table_id")
            .record_id("test_record_id")
            .build();

        assert_eq!(request.app_token, "test_app_token");
        assert_eq!(request.table_id, "test_table_id");
        assert_eq!(request.record_id, "test_record_id");
    }

    #[tokio::test]
    async fn test_update_record_request_builder() {
        let config = openlark_core::Config::new(openlark_core::ConfigInner {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            ..Default::default()
        });

        let request = UpdateRecordRequest::new(config)
            .builder()
            .app_token("test_app_token")
            .table_id("test_table_id")
            .record_id("test_record_id")
            .fields(json!({
                "field_name": "王五",
                "field_age": 35
            }))
            .build();

        assert_eq!(request.app_token, "test_app_token");
        assert_eq!(request.table_id, "test_table_id");
        assert_eq!(request.record_id, "test_record_id");
        assert_eq!(request.fields["field_name"], "王五");
        assert_eq!(request.fields["field_age"], 35);
    }

    #[tokio::test]
    async fn test_delete_record_request_builder() {
        let config = openlark_core::Config::new(openlark_core::ConfigInner {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            ..Default::default()
        });

        let request = DeleteRecordRequest::new(config)
            .builder()
            .app_token("test_app_token")
            .table_id("test_table_id")
            .record_id("test_record_id")
            .build();

        assert_eq!(request.app_token, "test_app_token");
        assert_eq!(request.table_id, "test_table_id");
        assert_eq!(request.record_id, "test_record_id");
    }

    #[tokio::test]
    async fn test_batch_create_records_request_builder() {
        let config = openlark_core::Config::new(openlark_core::ConfigInner {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            ..Default::default()
        });

        let records = vec![
            json!({"field_name": "用户1", "field_age": 20}),
            json!({"field_name": "用户2", "field_age": 25}),
            json!({"field_name": "用户3", "field_age": 30}),
        ];

        let request = BatchCreateRecordsRequest::new(config)
            .builder()
            .app_token("test_app_token")
            .table_id("test_table_id")
            .records(records)
            .build();

        assert_eq!(request.app_token, "test_app_token");
        assert_eq!(request.table_id, "test_table_id");
        assert_eq!(request.records.len(), 3);
        assert_eq!(request.records[0]["field_name"], "用户1");
    }

    #[tokio::test]
    async fn test_list_records_request_builder() {
        let config = openlark_core::Config::new(openlark_core::ConfigInner {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            ..Default::default()
        });

        let request = ListRecordsRequest::new(config)
            .builder()
            .app_token("test_app_token")
            .table_id("test_table_id")
            .page_size(50)
            .page_token("page_token_123")
            .sort(json!([{
                "field_id": "field_name",
                "desc": false
            }]))
            .build();

        assert_eq!(request.app_token, "test_app_token");
        assert_eq!(request.table_id, "test_table_id");
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("page_token_123".to_string()));
        assert!(request.sort.is_some());
    }

    #[tokio::test]
    async fn test_search_records_request_builder() {
        let config = openlark_core::Config::new(openlark_core::ConfigInner {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            ..Default::default()
        });

        let request = SearchRecordsRequest::new(config)
            .builder()
            .app_token("test_app_token")
            .table_id("test_table_id")
            .filter(json!({
                "conjunction": [{
                    "field_id": "field_name",
                    "operator": "contains",
                    "value": ["测试"]
                }]
            }))
            .build();

        assert_eq!(request.app_token, "test_app_token");
        assert_eq!(request.table_id, "test_table_id");
        assert!(request.filter.is_some());
    }

    #[tokio::test]
    async fn test_record_response_validation() {
        let response = json!({
            "record": {
                "record_id": "test_record_id_123456789",
                "fields": {
                    "field_name": "测试用户",
                    "field_age": 25,
                    "field_email": "test@example.com"
                },
                "created_at": "2023-01-01T00:00:00Z",
                "updated_at": "2023-01-01T00:00:00Z"
            }
        });

        // 验证响应结构
        assert!(TestHelper::assert_has_field(&response, "record.record_id").is_ok());
        assert!(TestHelper::assert_has_field(&response, "record.fields").is_ok());
        assert!(TestHelper::assert_has_field(&response, "record.created_at").is_ok());

        // 验证字段值
        assert!(TestHelper::assert_field_eq(&response, "record.fields.field_name", &json!("测试用户")).is_ok());
        assert!(TestHelper::assert_field_eq(&response, "record.fields.field_age", &json!(25)).is_ok());

        // 验证元数据
        assert!(TestHelper::assert_response_metadata(&response["record"]).is_ok());
    }

    #[tokio::test]
    async fn test_batch_records_response_validation() {
        let response = json!({
            "records": TestDataFactory::create_batch_test_records(10),
            "total": 10,
            "failed_items": []
        });

        // 验证批量响应结构
        assert!(TestHelper::assert_has_field(&response, "records").is_ok());
        assert!(TestHelper::assert_has_field(&response, "total").is_ok());
        assert!(TestHelper::assert_has_field(&response, "failed_items").is_ok());

        let records = response["records"].as_array().unwrap();
        assert_eq!(records.len(), 10);
        assert_eq!(response["total"], 10);

        // 验证每个记录的结构
        for (i, record) in records.iter().enumerate() {
            let record_id = format!("test_record_id_{:09}", i);
            assert!(TestHelper::assert_field_eq(record, "record.record_id", &json!(record_id)).is_ok());
        }
    }

    #[tokio::test]
    async fn test_pagination_records_response_validation() {
        let response = TestDataFactory::create_paginated_response(100, 20, Some("next_token_123"));

        // 验证分页响应结构
        assert!(ResponseValidator::validate_paginated_response(&response).is_ok());

        let items = response["items"].as_array().unwrap();
        assert_eq!(items.len(), 20);
        assert_eq!(response["total"], 100);
        assert_eq!(response["has_more"], true);
        assert!(response["page_token"].is_null() == false); // next_token_123
    }

    // 性能测试
    #[tokio::test]
    async fn test_record_creation_performance() {
        let config = openlark_core::Config::new(openlark_core::ConfigInner {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            ..Default::default()
        });

        let perf_test = PerformanceTest::start();

        // 模拟创建大量记录请求构建
        for i in 0..1000 {
            let _request = CreateRecordRequest::new(config.clone())
                .builder()
                .app_token("test_app_token")
                .table_id("test_table_id")
                .fields(json!({
                    "field_name": format!("用户{}", i),
                    "field_age": 20 + (i % 50)
                }))
                .build();
        }

        let result = perf_test.assert_duration_less_than(Duration::from_millis(100));
        assert!(result.is_ok(), "批量构建1000个记录请求应该在100ms内完成");
    }

    #[tokio::test]
    async fn test_large_dataset_handling() {
        // 测试大数据量记录处理
        let large_records = TestDataFactory::create_batch_test_records(5000);

        assert_eq!(large_records.len(), 5000);

        // 验证数据结构完整性
        for (i, record) in large_records.iter().enumerate() {
            let record_id = format!("test_record_id_{:09}", i);
            assert_eq!(record["record"]["record_id"], record_id);
            assert!(record["record"]["fields"]["field_name"].as_str().unwrap().contains(&format!("测试用户{}", i)));
        }

        // 内存使用测试
        let memory_before = TestHelper::MemoryMonitor::snapshot();

        // 处理大数据集
        let _total_age: i32 = large_records
            .iter()
            .map(|record| {
                record["record"]["fields"]["field_age"]
                    .as_i64()
                    .unwrap_or(0) as i32
            })
            .sum();

        let memory_after = TestHelper::MemoryMonitor::snapshot();

        assert_eq!(total_age, (0..50).sum::<i32>() * 100); // 5000个记录，每个记录年龄0-49

        // 验证内存使用合理（这里只是占位符验证）
        println!("内存使用变化: {}", memory_after.memory_usage - memory_before.memory_usage);
    }

    #[tokio::test]
    async fn test_concurrent_record_operations() {
        let test_records = vec!["record1", "record2", "record3", "record4", "record5"];

        // 测试并行删除操作
        let batch_test = BatchTest::new(test_records);

        let results = batch_test.execute_parallel(|record_id| {
            async move {
                // 模拟删除操作
                std::thread::sleep(Duration::from_millis(1)).await;
                Ok::<(), Box<dyn std::error::Error>>(())
            }
        }).await;

        // 验证所有操作都成功
        assert_eq!(results.len(), 5);
        for result in results {
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_field_type_handling() {
        // 测试不同字段类型的处理
        let test_fields = json!({
            "text_field": "文本内容",
            "number_field": 12345,
            "boolean_field": true,
            "date_field": "2023-01-01",
            "select_field": "选项1",
            "multi_select_field": ["选项1", "选项2"],
            "attachment_field": json!({
                "file_token": "v1_file_token_123",
                "file_name": "测试文件.pdf"
            }),
            "member_field": json!([{
                "id": "user_123",
                "name": "测试用户"
            }]),
            "url_field": "https://example.com",
            "email_field": "test@example.com",
            "phone_field": "13800138000"
        });

        let config = openlark_core::Config::new(openlark_core::ConfigInner {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            ..Default::default()
        });

        let request = CreateRecordRequest::new(config)
            .builder()
            .app_token("test_app_token")
            .table_id("test_table_id")
            .fields(test_fields)
            .build();

        // 验证不同字段类型的值
        assert_eq!(request.fields["text_field"], "文本内容");
        assert_eq!(request.fields["number_field"], 12345);
        assert_eq!(request.fields["boolean_field"], true);
        assert_eq!(request.fields["date_field"], "2023-01-01");
        assert_eq!(request.fields["select_field"], "选项1");
        assert_eq!(request.fields["multi_select_field"], json!(["选项1", "选项2"]));
    }

    #[tokio::test]
    async fn test_empty_and_null_field_handling() {
        // 测试空值和null字段的处理
        let test_fields = json!({
            "empty_text": "",
            "null_select": serde_json::Value::Null,
            "empty_array": [],
            "empty_object": json!({})
        });

        let config = openlark_core::Config::new(openlark_core::ConfigInner {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            ..Default::default()
        });

        let request = CreateRecordRequest::new(config)
            .builder()
            .app_token("test_app_token")
            .table_id("test_table_id")
            .fields(test_fields)
            .build();

        // 验证空值和null值处理
        assert_eq!(request.fields["empty_text"], "");
        assert!(request.fields["null_select"].is_null());
        assert_eq!(request.fields["empty_array"].as_array().unwrap().len(), 0);
        assert!(request.fields["empty_object"].as_object().unwrap().is_empty());
    }

    // 错误处理测试
    #[tokio::test]
    async fn test_invalid_record_data_handling() {
        let invalid_data = TestDataFactory::create_invalid_record_data();

        // 验证无效数据结构
        assert!(invalid_data["fields"]["non_existent_field"].is_str());
        assert_eq!(invalid_data["fields"]["non_existent_field"], "无效数据");
        assert!(invalid_data["fields"]["field_age"].is_str()); // 应该是数字但被错误地设置为字符串
    }

    #[tokio::test]
    async fn test_search_filter_builder() {
        let config = openlark_core::Config::new(openlark_core::ConfigInner {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            ..Default::default()
        });

        // 测试复杂搜索过滤器
        let complex_filter = json!({
            "conjunction": [
                {
                    "field_id": "field_name",
                    "operator": "contains",
                    "value": ["测试"]
                },
                {
                    "field_id": "field_age",
                    "operator": "is",
                    "value": [25]
                },
                {
                    "field_id": "field_status",
                    "operator": "is",
                    "value": ["active"]
                }
            ]
        });

        let request = SearchRecordsRequest::new(config)
            .builder()
            .app_token("test_app_token")
            .table_id("test_table_id")
            .filter(complex_filter)
            .build();

        assert!(request.filter.is_some());
        let filter = request.filter.as_ref().unwrap();
        assert!(filter["conjunction"].is_array());
        assert_eq!(filter["conjunction"].as_array().unwrap().len(), 3);
    }
}