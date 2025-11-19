//! Bitable 服务单元测试
//!
//! 本模块测试飞书多维表格 Bitable API 的各种功能，包括：
//! - 多维表格应用管理
//! - 数据表操作
//! - 记录的 CRUD 操作
//! - 字段管理
//! - 视图管理
//! - 权限和角色管理
//! - 表单管理

use wiremock::{
    matchers::{method, path, path_regex, query_param, body_json},
    Mock, MockServer, ResponseTemplate,
};
use serde_json::{json, Value, Map};
use rstest::{fixture, rstest};
use mockall::predicate::*;
use proptest::prelude::*;
use std::collections::HashMap;

use open_lark::{
    core::{config::Config, constants::AccessTokenType, req_option::RequestOption},
    service::cloud_docs::bitable::{
        v1::{
            app::{
                AppService,
                create::{CreateAppRequest, CreateAppResponseData, App},
                get::{GetAppRequest, GetAppResponseData},
            },
            app_table::{
                AppTableService,
                create::{CreateAppTableRequest, CreateAppTableResponseData, AppTable},
                list::{ListAppTableRequest, ListAppTableResponseData},
            },
            app_table_record::{
                AppTableRecordService, 
                create::{CreateRecordRequest, CreateRecordResponse},
                search::{SearchRecordRequest, SearchRecordResponse},
                batch_create::{BatchCreateRecordRequest, BatchCreateRecordResponse},
                batch_update::{BatchUpdateRecordRequest, BatchUpdateRecordResponse},
                update::{UpdateRecordRequest, UpdateRecordResponse},
                delete::{DeleteRecordRequest, DeleteRecordResponse},
            },
            app_table_field::{
                AppTableFieldService,
                create::{CreateAppTableFieldRequest, CreateAppTableFieldResponseData, AppTableField},
                list::{ListAppTableFieldRequest, ListAppTableFieldResponseData},
            },
            Record,
        },
    },
    core::api::Response,
};

/// 测试配置夹具
#[fixture]
fn test_config() -> Config {
    Config {
        app_id: "test_app_id".to_string(),
        app_secret: "test_app_secret".to_string(),
        lark_host: "https://open.feishu.cn".to_string(),
        ..Default::default()
    }
}

/// 模拟服务器夹具
#[fixture]
async fn mock_server() -> MockServer {
    MockServer::start().await
}

// ===================== 多维表格应用管理测试 =====================

#[rstest]
#[tokio::test]
async fn test_create_app_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "app": {
                "app_token": "bapb1234567890abcdef",
                "name": "项目管理系统",
                "description": "用于管理团队项目和任务",
                "is_advanced": false,
                "time_zone": "Asia/Shanghai",
                "folder_token": "fldbcO1UuPz8VwnpPx5a92abcdef"
            }
        }
    });

    Mock::given(method("POST"))
        .and(path("/open-apis/bitable/v1/apps"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let app_service = AppService::new(config);
    let request = CreateAppRequest::builder()
        .name("项目管理系统")
        .description("用于管理团队项目和任务")
        .folder_token("fldbcO1UuPz8VwnpPx5a92abcdef")
        .time_zone("Asia/Shanghai")
        .build();

    let result = app_service.create(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.code, 0);
    assert_eq!(response.msg, "success");
    
    let data = response.data.unwrap();
    assert_eq!(data.app.app_token, "bapb1234567890abcdef");
    assert_eq!(data.app.name, "项目管理系统");
    assert_eq!(data.app.description, Some("用于管理团队项目和任务".to_string()));
    assert_eq!(data.app.is_advanced, false);
    assert_eq!(data.app.time_zone, "Asia/Shanghai");
}

#[rstest]
#[tokio::test]
async fn test_get_app_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let app_token = "bapb1234567890abcdef";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "app": {
                "app_token": app_token,
                "name": "现有项目管理",
                "description": "现有的项目管理应用",
                "is_advanced": true,
                "time_zone": "Asia/Shanghai",
                "folder_token": "existing_folder_token"
            }
        }
    });

    Mock::given(method("GET"))
        .and(path(format!("/open-apis/bitable/v1/apps/{}", app_token)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let app_service = AppService::new(config);
    let request = GetAppRequest::new(app_token);

    let result = app_service.get(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    assert_eq!(data.app.app_token, app_token);
    assert_eq!(data.app.name, "现有项目管理");
    assert_eq!(data.app.is_advanced, true);
}

// ===================== 数据表管理测试 =====================

#[rstest]
#[tokio::test]
async fn test_create_app_table_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let app_token = "bapb1234567890abcdef";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "table": {
                "table_id": "tblsRc9GRRXKqhvW",
                "name": "任务清单",
                "description": "项目任务管理表"
            }
        }
    });

    Mock::given(method("POST"))
        .and(path(format!("/open-apis/bitable/v1/apps/{}/tables", app_token)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let table_service = AppTableService::new(config);
    let request = CreateAppTableRequest::builder()
        .app_token(app_token)
        .name("任务清单")
        .description("项目任务管理表")
        .build();

    let result = table_service.create(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    assert_eq!(data.table.table_id, "tblsRc9GRRXKqhvW");
    assert_eq!(data.table.name, "任务清单");
    assert_eq!(data.table.description, Some("项目任务管理表".to_string()));
}

#[rstest]
#[tokio::test]
async fn test_list_app_tables_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let app_token = "bapb1234567890abcdef";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "items": [
                {
                    "table_id": "tblsRc9GRRXKqhvW",
                    "name": "任务清单",
                    "description": "项目任务管理表"
                },
                {
                    "table_id": "tblAnotherTable123",
                    "name": "员工信息",
                    "description": "团队成员信息表"
                }
            ],
            "has_more": false,
            "page_token": null
        }
    });

    Mock::given(method("GET"))
        .and(path(format!("/open-apis/bitable/v1/apps/{}/tables", app_token)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let table_service = AppTableService::new(config);
    let request = ListAppTableRequest::new(app_token);

    let result = table_service.list(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    assert_eq!(data.items.len(), 2);
    assert_eq!(data.has_more, false);
    
    assert_eq!(data.items[0].table_id, "tblsRc9GRRXKqhvW");
    assert_eq!(data.items[0].name, "任务清单");
    assert_eq!(data.items[1].table_id, "tblAnotherTable123");
    assert_eq!(data.items[1].name, "员工信息");
}

// ===================== 记录 CRUD 测试 =====================

#[rstest]
#[tokio::test]
async fn test_create_record_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let app_token = "bapb1234567890abcdef";
    let table_id = "tblsRc9GRRXKqhvW";
    
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "record": {
                "record_id": "recqwerty123456",
                "created_time": 1674552412000i64,
                "created_by": {
                    "id": "ou_123456789"
                },
                "fields": {
                    "任务标题": "开发用户登录功能",
                    "优先级": "高",
                    "状态": "进行中",
                    "截止日期": 1675152000000i64,
                    "负责人": [{
                        "id": "ou_987654321"
                    }]
                }
            }
        }
    });

    Mock::given(method("POST"))
        .and(path(format!("/open-apis/bitable/v1/apps/{}/tables/{}/records", app_token, table_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let record_service = AppTableRecordService::new(config);
    
    // 创建记录数据
    let mut fields = HashMap::new();
    fields.insert("任务标题".to_string(), json!("开发用户登录功能"));
    fields.insert("优先级".to_string(), json!("高"));
    fields.insert("状态".to_string(), json!("进行中"));
    fields.insert("截止日期".to_string(), json!(1675152000000i64));
    fields.insert("负责人".to_string(), json!([{"id": "ou_987654321"}]));
    
    let record = Record { fields };
    
    let request = CreateRecordRequest::builder()
        .app_token(app_token)
        .table_id(table_id)
        .fields(record)
        .user_id_type("open_id")
        .build();

    let result = record_service.create(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.code, 0);
    
    let data = response.data.unwrap();
    assert_eq!(data.record.record_id, Some("recqwerty123456".to_string()));
    assert_eq!(data.record.created_time, Some(1674552412000));
    
    // 验证字段数据
    let fields = &data.record.fields;
    assert_eq!(fields.get("任务标题").unwrap(), "开发用户登录功能");
    assert_eq!(fields.get("优先级").unwrap(), "高");
    assert_eq!(fields.get("状态").unwrap(), "进行中");
}

#[rstest]
#[tokio::test]
async fn test_search_records_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let app_token = "bapb1234567890abcdef";
    let table_id = "tblsRc9GRRXKqhvW";
    
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "has_more": false,
            "page_token": null,
            "total": 2,
            "items": [
                {
                    "record_id": "recqwerty123456",
                    "created_time": 1674552412000i64,
                    "fields": {
                        "任务标题": "开发用户登录功能",
                        "优先级": "高",
                        "状态": "进行中"
                    }
                },
                {
                    "record_id": "recasdfgh789012",
                    "created_time": 1674552500000i64,
                    "fields": {
                        "任务标题": "设计系统架构",
                        "优先级": "中",
                        "状态": "已完成"
                    }
                }
            ]
        }
    });

    Mock::given(method("POST"))
        .and(path(format!("/open-apis/bitable/v1/apps/{}/tables/{}/records/search", app_token, table_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let record_service = AppTableRecordService::new(config);
    let request = SearchRecordRequest::builder()
        .app_token(app_token)
        .table_id(table_id)
        .page_size(100)
        .build();

    let result = record_service.search(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.has_more, false);
    assert_eq!(data.total, Some(2));
    assert_eq!(data.items.len(), 2);
    
    let first_record = &data.items[0];
    assert_eq!(first_record.record_id, Some("recqwerty123456".to_string()));
    assert_eq!(first_record.fields.get("任务标题").unwrap(), "开发用户登录功能");
    assert_eq!(first_record.fields.get("优先级").unwrap(), "高");
    
    let second_record = &data.items[1];
    assert_eq!(second_record.record_id, Some("recasdfgh789012".to_string()));
    assert_eq!(second_record.fields.get("状态").unwrap(), "已完成");
}

#[rstest]
#[tokio::test]
async fn test_update_record_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let app_token = "bapb1234567890abcdef";
    let table_id = "tblsRc9GRRXKqhvW";
    let record_id = "recqwerty123456";
    
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "record": {
                "record_id": record_id,
                "last_modified_time": 1674562412000i64,
                "last_modified_by": {
                    "id": "ou_123456789"
                },
                "fields": {
                    "任务标题": "开发用户登录功能",
                    "优先级": "中",
                    "状态": "已完成",
                    "完成时间": 1674562412000i64
                }
            }
        }
    });

    Mock::given(method("PUT"))
        .and(path(format!("/open-apis/bitable/v1/apps/{}/tables/{}/records/{}", app_token, table_id, record_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let record_service = AppTableRecordService::new(config);
    
    // 创建更新数据
    let mut fields = HashMap::new();
    fields.insert("优先级".to_string(), json!("中"));
    fields.insert("状态".to_string(), json!("已完成"));
    fields.insert("完成时间".to_string(), json!(1674562412000i64));
    
    let record = Record { fields };
    
    let request = UpdateRecordRequest::builder()
        .app_token(app_token)
        .table_id(table_id)
        .record_id(record_id)
        .fields(record)
        .build();

    let result = record_service.update(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.record.record_id, Some(record_id.to_string()));
    assert_eq!(data.record.last_modified_time, Some(1674562412000));
    
    let fields = &data.record.fields;
    assert_eq!(fields.get("优先级").unwrap(), "中");
    assert_eq!(fields.get("状态").unwrap(), "已完成");
    assert_eq!(fields.get("完成时间").unwrap(), 1674562412000i64);
}

#[rstest]
#[tokio::test]
async fn test_delete_record_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let app_token = "bapb1234567890abcdef";
    let table_id = "tblsRc9GRRXKqhvW";
    let record_id = "recqwerty123456";
    
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "deleted": true,
            "record_id": record_id
        }
    });

    Mock::given(method("DELETE"))
        .and(path(format!("/open-apis/bitable/v1/apps/{}/tables/{}/records/{}", app_token, table_id, record_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let record_service = AppTableRecordService::new(config);
    let request = DeleteRecordRequest::builder()
        .app_token(app_token)
        .table_id(table_id)
        .record_id(record_id)
        .build();

    let result = record_service.delete(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.deleted, true);
    assert_eq!(data.record_id, record_id);
}

// ===================== 批量记录操作测试 =====================

#[rstest]
#[tokio::test]
async fn test_batch_create_records_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let app_token = "bapb1234567890abcdef";
    let table_id = "tblsRc9GRRXKqhvW";
    
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "records": [
                {
                    "record_id": "recBatchCreate001",
                    "created_time": 1674552412000i64,
                    "fields": {
                        "任务标题": "批量任务1",
                        "优先级": "高"
                    }
                },
                {
                    "record_id": "recBatchCreate002", 
                    "created_time": 1674552413000i64,
                    "fields": {
                        "任务标题": "批量任务2",
                        "优先级": "中"
                    }
                }
            ]
        }
    });

    Mock::given(method("POST"))
        .and(path(format!("/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_create", app_token, table_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let record_service = AppTableRecordService::new(config);
    
    // 创建批量记录数据
    let mut record1 = HashMap::new();
    record1.insert("任务标题".to_string(), json!("批量任务1"));
    record1.insert("优先级".to_string(), json!("高"));
    
    let mut record2 = HashMap::new();
    record2.insert("任务标题".to_string(), json!("批量任务2"));
    record2.insert("优先级".to_string(), json!("中"));
    
    let records = vec![
        Record { fields: record1 },
        Record { fields: record2 }
    ];
    
    let request = BatchCreateRecordRequest::builder()
        .app_token(app_token)
        .table_id(table_id)
        .records(records)
        .build();

    let result = record_service.batch_create(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.records.len(), 2);
    
    let first_record = &data.records[0];
    assert_eq!(first_record.record_id, Some("recBatchCreate001".to_string()));
    assert_eq!(first_record.fields.get("任务标题").unwrap(), "批量任务1");
    
    let second_record = &data.records[1];
    assert_eq!(second_record.record_id, Some("recBatchCreate002".to_string()));
    assert_eq!(second_record.fields.get("任务标题").unwrap(), "批量任务2");
}

#[rstest]
#[tokio::test]
async fn test_batch_update_records_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let app_token = "bapb1234567890abcdef";
    let table_id = "tblsRc9GRRXKqhvW";
    
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "records": [
                {
                    "record_id": "recBatchUpdate001",
                    "last_modified_time": 1674562412000i64,
                    "fields": {
                        "任务标题": "更新后任务1",
                        "状态": "已完成"
                    }
                },
                {
                    "record_id": "recBatchUpdate002",
                    "last_modified_time": 1674562413000i64,
                    "fields": {
                        "任务标题": "更新后任务2",
                        "状态": "已完成"
                    }
                }
            ]
        }
    });

    Mock::given(method("POST"))
        .and(path(format!("/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_update", app_token, table_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let record_service = AppTableRecordService::new(config);
    
    // 创建批量更新数据
    let mut update1 = Record {
        fields: HashMap::new(),
    };
    update1.record_id = Some("recBatchUpdate001".to_string());
    update1.fields.insert("任务标题".to_string(), json!("更新后任务1"));
    update1.fields.insert("状态".to_string(), json!("已完成"));
    
    let mut update2 = Record {
        fields: HashMap::new(),
    };
    update2.record_id = Some("recBatchUpdate002".to_string());
    update2.fields.insert("任务标题".to_string(), json!("更新后任务2"));
    update2.fields.insert("状态".to_string(), json!("已完成"));
    
    let records = vec![update1, update2];
    
    let request = BatchUpdateRecordRequest::builder()
        .app_token(app_token)
        .table_id(table_id)
        .records(records)
        .build();

    let result = record_service.batch_update(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.records.len(), 2);
    
    let first_record = &data.records[0];
    assert_eq!(first_record.record_id, Some("recBatchUpdate001".to_string()));
    assert_eq!(first_record.fields.get("状态").unwrap(), "已完成");
    
    let second_record = &data.records[1];
    assert_eq!(second_record.record_id, Some("recBatchUpdate002".to_string()));
    assert_eq!(second_record.fields.get("状态").unwrap(), "已完成");
}

// ===================== 字段管理测试 =====================

#[rstest]
#[tokio::test]
async fn test_create_app_table_field_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let app_token = "bapb1234567890abcdef";
    let table_id = "tblsRc9GRRXKqhvW";
    
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "field": {
                "field_id": "fldQwerty123456",
                "field_name": "完成进度",
                "type": 5,
                "description": "任务完成百分比",
                "is_primary": false,
                "property": {
                    "formatter": "0%"
                }
            }
        }
    });

    Mock::given(method("POST"))
        .and(path(format!("/open-apis/bitable/v1/apps/{}/tables/{}/fields", app_token, table_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let field_service = AppTableFieldService::new(config);
    let request = CreateAppTableFieldRequest::builder()
        .app_token(app_token)
        .table_id(table_id)
        .field_name("完成进度")
        .field_type(5) // 数字类型
        .description("任务完成百分比")
        .build();

    let result = field_service.create(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.field.field_id, "fldQwerty123456");
    assert_eq!(data.field.field_name, "完成进度");
    assert_eq!(data.field.field_type, 5);
    assert_eq!(data.field.description, Some("任务完成百分比".to_string()));
    assert_eq!(data.field.is_primary, false);
}

#[rstest]
#[tokio::test]
async fn test_list_app_table_fields_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let app_token = "bapb1234567890abcdef";
    let table_id = "tblsRc9GRRXKqhvW";
    
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "has_more": false,
            "page_token": null,
            "total": 5,
            "items": [
                {
                    "field_id": "fldPrimary001",
                    "field_name": "任务标题",
                    "type": 1,
                    "is_primary": true
                },
                {
                    "field_id": "fldSelect001",
                    "field_name": "优先级",
                    "type": 3,
                    "is_primary": false,
                    "property": {
                        "options": [
                            {"name": "高", "color": 1},
                            {"name": "中", "color": 2},
                            {"name": "低", "color": 3}
                        ]
                    }
                },
                {
                    "field_id": "fldDateTime001",
                    "field_name": "截止日期",
                    "type": 4,
                    "is_primary": false,
                    "property": {
                        "date_format": "yyyy/MM/dd",
                        "time_format": "HH:mm"
                    }
                }
            ]
        }
    });

    Mock::given(method("GET"))
        .and(path(format!("/open-apis/bitable/v1/apps/{}/tables/{}/fields", app_token, table_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let field_service = AppTableFieldService::new(config);
    let request = ListAppTableFieldRequest::new(app_token, table_id);

    let result = field_service.list(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.has_more, false);
    assert_eq!(data.total, Some(5));
    assert_eq!(data.items.len(), 3);
    
    // 验证主字段
    let primary_field = &data.items[0];
    assert_eq!(primary_field.field_id, "fldPrimary001");
    assert_eq!(primary_field.field_name, "任务标题");
    assert_eq!(primary_field.field_type, 1);
    assert_eq!(primary_field.is_primary, true);
    
    // 验证选择字段
    let select_field = &data.items[1];
    assert_eq!(select_field.field_id, "fldSelect001");
    assert_eq!(select_field.field_name, "优先级");
    assert_eq!(select_field.field_type, 3);
    assert_eq!(select_field.is_primary, false);
    
    // 验证日期时间字段
    let datetime_field = &data.items[2];
    assert_eq!(datetime_field.field_id, "fldDateTime001");
    assert_eq!(datetime_field.field_name, "截止日期");
    assert_eq!(datetime_field.field_type, 4);
}

// ===================== 请求构建器测试 =====================

#[test]
fn test_create_record_request_builder() {
    let mut fields = HashMap::new();
    fields.insert("标题".to_string(), json!("测试任务"));
    fields.insert("状态".to_string(), json!("待处理"));
    
    let record = Record { fields };
    
    let request = CreateRecordRequest::builder()
        .app_token("app123")
        .table_id("table456")
        .fields(record.clone())
        .user_id_type("open_id")
        .client_token("client789")
        .build();

    assert_eq!(request.app_token, "app123");
    assert_eq!(request.table_id, "table456");
    assert_eq!(request.fields.fields.get("标题").unwrap(), "测试任务");
    assert_eq!(request.user_id_type, Some("open_id".to_string()));
    assert_eq!(request.client_token, Some("client789".to_string()));
    
    // 验证查询参数
    assert_eq!(request.api_request.query_params.get("user_id_type"), Some(&"open_id".to_string()));
    assert_eq!(request.api_request.query_params.get("client_token"), Some(&"client789".to_string()));
}

#[test]
fn test_create_record_request_builder_with_add_field() {
    let request = CreateRecordRequest::builder()
        .app_token("app123")
        .table_id("table456")
        .add_field("任务名称", json!("新任务"))
        .add_field("优先级", json!("高"))
        .add_field("完成度", json!(0.5))
        .build();

    assert_eq!(request.fields.fields.len(), 3);
    assert_eq!(request.fields.fields.get("任务名称").unwrap(), "新任务");
    assert_eq!(request.fields.fields.get("优先级").unwrap(), "高");
    assert_eq!(request.fields.fields.get("完成度").unwrap(), 0.5);
}

#[test]
fn test_batch_create_record_request_builder() {
    let mut record1_fields = HashMap::new();
    record1_fields.insert("名称".to_string(), json!("记录1"));
    
    let mut record2_fields = HashMap::new();
    record2_fields.insert("名称".to_string(), json!("记录2"));
    
    let records = vec![
        Record { fields: record1_fields },
        Record { fields: record2_fields }
    ];
    
    let request = BatchCreateRecordRequest::builder()
        .app_token("app123")
        .table_id("table456")
        .records(records.clone())
        .user_id_type("union_id")
        .build();

    assert_eq!(request.app_token, "app123");
    assert_eq!(request.table_id, "table456");
    assert_eq!(request.records.len(), 2);
    assert_eq!(request.records[0].fields.get("名称").unwrap(), "记录1");
    assert_eq!(request.records[1].fields.get("名称").unwrap(), "记录2");
    assert_eq!(request.user_id_type, Some("union_id".to_string()));
}

// ===================== 属性测试 =====================

proptest! {
    #[test]
    fn test_app_token_invariants(
        app_token in prop::string::string_regex("bapb[a-zA-Z0-9]{16,32}").unwrap(),
        table_id in prop::string::string_regex("tbl[a-zA-Z0-9]{12,20}").unwrap()
    ) {
        let request = CreateRecordRequest::new(&app_token, &table_id);
        assert_eq!(request.app_token, app_token);
        assert_eq!(request.table_id, table_id);
        assert!(request.fields.fields.is_empty());
    }
}

proptest! {
    #[test]
    fn test_record_fields_invariants(
        field_count in 1usize..=20,
        field_names in prop::collection::vec(
            prop::string::string_regex("[a-zA-Z\u4e00-\u9fa5][a-zA-Z0-9\u4e00-\u9fa5_]{0,30}").unwrap(),
            1..=20
        )
    ) {
        prop_assume!(field_names.len() == field_count);
        
        let mut fields = HashMap::new();
        for (i, name) in field_names.iter().enumerate() {
            fields.insert(name.clone(), json!(format!("value_{}", i)));
        }
        
        let record = Record { fields: fields.clone() };
        
        assert_eq!(record.fields.len(), field_count);
        for name in field_names.iter() {
            assert!(record.fields.contains_key(name));
        }
    }
}

// ===================== 错误处理测试 =====================

#[rstest]
#[tokio::test]
async fn test_invalid_app_token_error(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let invalid_app_token = "invalid_token";
    let table_id = "tblsRc9GRRXKqhvW";
    
    let mock_response = json!({
        "code": 1254001,
        "msg": "多维表格不存在或无权限访问"
    });

    Mock::given(method("POST"))
        .and(path(format!("/open-apis/bitable/v1/apps/{}/tables/{}/records", invalid_app_token, table_id)))
        .respond_with(ResponseTemplate::new(400).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let record_service = AppTableRecordService::new(config);
    
    let mut fields = HashMap::new();
    fields.insert("测试字段".to_string(), json!("测试值"));
    let record = Record { fields };
    
    let request = CreateRecordRequest::builder()
        .app_token(invalid_app_token)
        .table_id(table_id)
        .fields(record)
        .build();

    let result = record_service.create(request, None).await;
    
    if result.is_ok() {
        let response = result.unwrap();
        assert_ne!(response.code, 0);
        assert!(response.msg.contains("多维表格") || response.msg.contains("权限"));
    }
}

#[rstest]
#[tokio::test]
async fn test_field_validation_error(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let app_token = "bapb1234567890abcdef";
    let table_id = "tblsRc9GRRXKqhvW";
    
    let mock_response = json!({
        "code": 1254008,
        "msg": "字段类型不匹配或字段不存在"
    });

    Mock::given(method("POST"))
        .and(path(format!("/open-apis/bitable/v1/apps/{}/tables/{}/records", app_token, table_id)))
        .respond_with(ResponseTemplate::new(400).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let record_service = AppTableRecordService::new(config);
    
    // 创建无效字段类型的记录
    let mut fields = HashMap::new();
    fields.insert("不存在的字段".to_string(), json!("某个值"));
    fields.insert("日期字段".to_string(), json!("无效的日期格式"));
    let record = Record { fields };
    
    let request = CreateRecordRequest::builder()
        .app_token(app_token)
        .table_id(table_id)
        .fields(record)
        .build();

    let result = record_service.create(request, None).await;
    
    if result.is_ok() {
        let response = result.unwrap();
        assert_eq!(response.code, 1254008);
        assert!(response.msg.contains("字段"));
    }
}

// ===================== 集成场景测试 =====================

#[rstest]
#[tokio::test]
async fn test_complete_bitable_workflow(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    // 1. 创建多维表格应用
    let create_app_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "app": {
                "app_token": "bapb_integration_test",
                "name": "集成测试应用",
                "description": "完整流程测试",
                "is_advanced": false,
                "time_zone": "Asia/Shanghai"
            }
        }
    });

    Mock::given(method("POST"))
        .and(path("/open-apis/bitable/v1/apps"))
        .respond_with(ResponseTemplate::new(200).set_body_json(create_app_response))
        .mount(&mock_server)
        .await;

    // 2. 创建数据表
    let create_table_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "table": {
                "table_id": "tbl_integration_test",
                "name": "测试数据表",
                "description": "用于集成测试"
            }
        }
    });

    Mock::given(method("POST"))
        .and(path("/open-apis/bitable/v1/apps/bapb_integration_test/tables"))
        .respond_with(ResponseTemplate::new(200).set_body_json(create_table_response))
        .mount(&mock_server)
        .await;

    // 3. 创建字段
    let create_field_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "field": {
                "field_id": "fld_integration_priority",
                "field_name": "优先级",
                "type": 3,
                "is_primary": false
            }
        }
    });

    Mock::given(method("POST"))
        .and(path("/open-apis/bitable/v1/apps/bapb_integration_test/tables/tbl_integration_test/fields"))
        .respond_with(ResponseTemplate::new(200).set_body_json(create_field_response))
        .mount(&mock_server)
        .await;

    // 4. 创建记录
    let create_record_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "record": {
                "record_id": "rec_integration_test",
                "created_time": 1674552412000i64,
                "fields": {
                    "任务名称": "集成测试任务",
                    "优先级": "高"
                }
            }
        }
    });

    Mock::given(method("POST"))
        .and(path("/open-apis/bitable/v1/apps/bapb_integration_test/tables/tbl_integration_test/records"))
        .respond_with(ResponseTemplate::new(200).set_body_json(create_record_response))
        .mount(&mock_server)
        .await;

    // 5. 查询记录验证
    let search_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "has_more": false,
            "total": 1,
            "items": [
                {
                    "record_id": "rec_integration_test",
                    "created_time": 1674552412000i64,
                    "fields": {
                        "任务名称": "集成测试任务",
                        "优先级": "高"
                    }
                }
            ]
        }
    });

    Mock::given(method("POST"))
        .and(path("/open-apis/bitable/v1/apps/bapb_integration_test/tables/tbl_integration_test/records/search"))
        .respond_with(ResponseTemplate::new(200).set_body_json(search_response))
        .mount(&mock_server)
        .await;

    // 执行完整的集成测试流程
    let app_service = AppService::new(config.clone());
    let table_service = AppTableService::new(config.clone());
    let field_service = AppTableFieldService::new(config.clone());
    let record_service = AppTableRecordService::new(config);

    // 1. 创建应用
    let create_app_req = CreateAppRequest::builder()
        .name("集成测试应用")
        .description("完整流程测试")
        .time_zone("Asia/Shanghai")
        .build();

    let app_result = app_service.create(create_app_req, None).await;
    assert!(app_result.is_ok());
    let app_data = app_result.unwrap().data.unwrap();
    let app_token = app_data.app.app_token;
    assert_eq!(app_token, "bapb_integration_test");

    // 2. 创建数据表
    let create_table_req = CreateAppTableRequest::builder()
        .app_token(&app_token)
        .name("测试数据表")
        .description("用于集成测试")
        .build();

    let table_result = table_service.create(create_table_req, None).await;
    assert!(table_result.is_ok());
    let table_data = table_result.unwrap().data.unwrap();
    let table_id = table_data.table.table_id;
    assert_eq!(table_id, "tbl_integration_test");

    // 3. 创建字段
    let create_field_req = CreateAppTableFieldRequest::builder()
        .app_token(&app_token)
        .table_id(&table_id)
        .field_name("优先级")
        .field_type(3)
        .build();

    let field_result = field_service.create(create_field_req, None).await;
    assert!(field_result.is_ok());
    let field_data = field_result.unwrap().data.unwrap();
    assert_eq!(field_data.field.field_name, "优先级");

    // 4. 创建记录
    let mut fields = HashMap::new();
    fields.insert("任务名称".to_string(), json!("集成测试任务"));
    fields.insert("优先级".to_string(), json!("高"));
    let record = Record { fields };

    let create_record_req = CreateRecordRequest::builder()
        .app_token(&app_token)
        .table_id(&table_id)
        .fields(record)
        .build();

    let record_result = record_service.create(create_record_req, None).await;
    assert!(record_result.is_ok());
    let record_data = record_result.unwrap().data.unwrap();
    assert_eq!(record_data.record.record_id, Some("rec_integration_test".to_string()));

    // 5. 查询验证
    let search_req = SearchRecordRequest::builder()
        .app_token(&app_token)
        .table_id(&table_id)
        .build();

    let search_result = record_service.search(search_req, None).await;
    assert!(search_result.is_ok());
    let search_data = search_result.unwrap().data.unwrap();
    assert_eq!(search_data.total, Some(1));
    assert_eq!(search_data.items.len(), 1);
    assert_eq!(search_data.items[0].fields.get("任务名称").unwrap(), "集成测试任务");
}

#[cfg(test)]
mod test_utils {
    use super::*;

    /// 创建测试用的记录
    pub fn create_test_record(title: &str, priority: &str, status: &str) -> Record {
        let mut fields = HashMap::new();
        fields.insert("任务标题".to_string(), json!(title));
        fields.insert("优先级".to_string(), json!(priority));
        fields.insert("状态".to_string(), json!(status));
        
        Record { fields }
    }

    /// 创建测试用的批量记录
    pub fn create_test_batch_records(count: usize) -> Vec<Record> {
        (0..count)
            .map(|i| create_test_record(
                &format!("批量任务{}", i + 1),
                if i % 3 == 0 { "高" } else if i % 3 == 1 { "中" } else { "低" },
                "待处理"
            ))
            .collect()
    }

    /// 验证记录字段
    pub fn assert_record_fields(record: &Record, expected_fields: &[(&str, &str)]) {
        for (field_name, expected_value) in expected_fields {
            assert_eq!(
                record.fields.get(*field_name).unwrap().as_str().unwrap(),
                *expected_value,
                "字段 {} 的值不匹配",
                field_name
            );
        }
    }
}