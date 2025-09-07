use open_lark::{
    core::config::Config,
    service::drive::v1::{
        GetFileMetaReq, GetFileMetaReqBuilder,
        CreateFolderReq, CreateFolderReqBuilder,
    },
    service::sheets::v2::{
        ReadSingleRangeReq, ReadSingleRangeReqBuilder,
        WriteSingleRangeReq, WriteSingleRangeReqBuilder,
    },
    service::bitable::v1::{
        CreateAppReq, CreateAppReqBuilder,
        CreateTableReq, CreateTableReqBuilder,
        CreateRecordReq, CreateRecordReqBuilder,
    },
    service::permission::v1::{
        CreateMemberPermissionReq, CreateMemberPermissionReqBuilder,
    },
    service::docx::v1::{
        CreateDocumentReq, CreateDocumentReqBuilder,
        CreateDocumentBlockReq, CreateDocumentBlockReqBuilder,
    },
    service::comments::v1::{
        CreateCommentReq, CreateCommentReqBuilder,
    },
    LarkClient,
};
use rstest::*;
use serde_json::{json, Value};
use std::collections::HashMap;
use tokio;
use wiremock::{
    matchers::{header, method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

/// 测试配置夹具
#[fixture]
fn test_config() -> Config {
    Config::builder()
        .app_id("test_app_id".to_string())
        .app_secret("test_app_secret".to_string())
        .api_base_url("http://127.0.0.1:8080".to_string())
        .build()
}

/// Mock服务器夹具
#[fixture]
async fn mock_server() -> MockServer {
    MockServer::start().await
}

/// 创建LarkClient的辅助函数
fn create_test_client(base_url: &str) -> LarkClient {
    let config = Config::builder()
        .app_id("test_app_id".to_string())
        .app_secret("test_app_secret".to_string())
        .api_base_url(base_url.to_string())
        .build();
    
    LarkClient::new(config)
}

/// 集成测试：从创建文件夹到添加权限的完整工作流
#[rstest]
#[tokio::test]
async fn test_complete_file_workflow(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let client = create_test_client(&mock_server.uri());
    
    // 步骤1: 创建文件夹
    let folder_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "token": "fold_test123",
            "name": "测试文件夹",
            "type": "folder",
            "parent_token": "root",
            "url": "https://example.feishu.cn/drive/folder/fold_test123"
        }
    });
    
    Mock::given(method("POST"))
        .and(path("/open-apis/drive/v1/files/create_folder"))
        .and(header("authorization", "Bearer test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&folder_response))
        .mount(&mock_server)
        .await;
    
    // 步骤2: 获取文件夹元数据
    let meta_response = json!({
        "code": 0,
        "msg": "success", 
        "data": {
            "token": "fold_test123",
            "name": "测试文件夹",
            "type": "folder",
            "parent_token": "root",
            "owner_id": "user123",
            "created_time": "1640995200",
            "modified_time": "1640995200"
        }
    });
    
    Mock::given(method("GET"))
        .and(path("/open-apis/drive/v1/files/fold_test123/meta"))
        .and(header("authorization", "Bearer test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&meta_response))
        .mount(&mock_server)
        .await;
    
    // 步骤3: 为文件夹设置权限
    let permission_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "member_id": "user456",
            "perm": "edit"
        }
    });
    
    Mock::given(method("POST"))
        .and(path("/open-apis/drive/v1/permissions/fold_test123/members"))
        .and(header("authorization", "Bearer test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&permission_response))
        .mount(&mock_server)
        .await;
    
    // 执行完整工作流
    let create_folder_req = CreateFolderReqBuilder::default()
        .name("测试文件夹".to_string())
        .folder_token("root".to_string())
        .build()
        .unwrap();
    
    let folder_result = client.drive.create_folder(&create_folder_req).await;
    assert!(folder_result.is_ok());
    let folder_data = folder_result.unwrap();
    assert_eq!(folder_data.data.name, Some("测试文件夹".to_string()));
    
    let get_meta_req = GetFileMetaReqBuilder::default()
        .file_token("fold_test123".to_string())
        .build()
        .unwrap();
    
    let meta_result = client.drive.get_file_meta(&get_meta_req).await;
    assert!(meta_result.is_ok());
    let meta_data = meta_result.unwrap();
    assert_eq!(meta_data.data.name, Some("测试文件夹".to_string()));
    
    let permission_req = CreateMemberPermissionReqBuilder::default()
        .token("fold_test123".to_string())
        .member_type("user".to_string())
        .member_id("user456".to_string())
        .perm("edit".to_string())
        .build()
        .unwrap();
    
    let permission_result = client.permission.create_member_permission(&permission_req).await;
    assert!(permission_result.is_ok());
}

/// 集成测试：多维表格完整工作流
#[rstest]
#[tokio::test]
async fn test_bitable_complete_workflow(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let client = create_test_client(&mock_server.uri());
    
    // 步骤1: 创建多维表格应用
    let app_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "app": {
                "app_token": "bascn123456",
                "name": "测试应用",
                "description": "集成测试应用",
                "is_advanced": false
            }
        }
    });
    
    Mock::given(method("POST"))
        .and(path("/open-apis/bitable/v1/apps"))
        .and(header("authorization", "Bearer test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&app_response))
        .mount(&mock_server)
        .await;
    
    // 步骤2: 创建数据表
    let table_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "table_id": "tbltest123",
            "name": "测试表格",
            "description": "集成测试表格"
        }
    });
    
    Mock::given(method("POST"))
        .and(path("/open-apis/bitable/v1/apps/bascn123456/tables"))
        .and(header("authorization", "Bearer test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&table_response))
        .mount(&mock_server)
        .await;
    
    // 步骤3: 添加记录
    let record_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "record": {
                "record_id": "rectest123",
                "fields": {
                    "名称": "测试记录",
                    "数量": 100
                },
                "created_by": {
                    "id": "user123",
                    "name": "测试用户"
                },
                "created_time": 1640995200
            }
        }
    });
    
    Mock::given(method("POST"))
        .and(path("/open-apis/bitable/v1/apps/bascn123456/tables/tbltest123/records"))
        .and(header("authorization", "Bearer test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&record_response))
        .mount(&mock_server)
        .await;
    
    // 执行完整工作流
    let create_app_req = CreateAppReqBuilder::default()
        .name("测试应用".to_string())
        .build()
        .unwrap();
    
    let app_result = client.bitable.create_app(&create_app_req).await;
    assert!(app_result.is_ok());
    
    let create_table_req = CreateTableReqBuilder::default()
        .app_token("bascn123456".to_string())
        .name("测试表格".to_string())
        .build()
        .unwrap();
    
    let table_result = client.bitable.create_table(&create_table_req).await;
    assert!(table_result.is_ok());
    
    let mut fields = HashMap::new();
    fields.insert("名称".to_string(), json!("测试记录"));
    fields.insert("数量".to_string(), json!(100));
    
    let create_record_req = CreateRecordReqBuilder::default()
        .app_token("bascn123456".to_string())
        .table_id("tbltest123".to_string())
        .fields(fields)
        .build()
        .unwrap();
    
    let record_result = client.bitable.create_record(&create_record_req).await;
    assert!(record_result.is_ok());
}

/// 集成测试：电子表格与评论系统集成
#[rstest]
#[tokio::test]
async fn test_sheets_with_comments_workflow(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let client = create_test_client(&mock_server.uri());
    
    // 步骤1: 写入电子表格数据
    let write_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "spreadsheet_token": "shtcn123456",
            "updates": {
                "updated_cells": 4,
                "updated_columns": 2,
                "updated_rows": 2
            }
        }
    });
    
    Mock::given(method("PUT"))
        .and(path("/open-apis/sheets/v2/spreadsheets/shtcn123456/values/A1:B2"))
        .and(header("authorization", "Bearer test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&write_response))
        .mount(&mock_server)
        .await;
    
    // 步骤2: 读取电子表格数据
    let read_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "values": [
                ["姓名", "年龄"],
                ["张三", "25"]
            ],
            "major_dimension": "ROWS"
        }
    });
    
    Mock::given(method("GET"))
        .and(path("/open-apis/sheets/v2/spreadsheets/shtcn123456/values/A1:B2"))
        .and(header("authorization", "Bearer test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&read_response))
        .mount(&mock_server)
        .await;
    
    // 步骤3: 为表格添加评论
    let comment_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "comment_id": "com123456",
            "user_id": "user123",
            "create_time": 1640995200,
            "update_time": 1640995200,
            "is_solved": false,
            "solved_time": 0,
            "solver_user_id": "",
            "has_more": false,
            "page_token": "",
            "is_whole": true,
            "quote": "A1:B2区域的数据",
            "reply_list": {
                "replies": [],
                "has_more": false,
                "page_token": ""
            }
        }
    });
    
    Mock::given(method("POST"))
        .and(path("/open-apis/drive/v1/files/shtcn123456/comments"))
        .and(header("authorization", "Bearer test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&comment_response))
        .mount(&mock_server)
        .await;
    
    // 执行完整工作流
    let values = vec![
        vec!["姓名".to_string(), "年龄".to_string()],
        vec!["张三".to_string(), "25".to_string()],
    ];
    
    let write_req = WriteSingleRangeReqBuilder::default()
        .spreadsheet_token("shtcn123456".to_string())
        .range("A1:B2".to_string())
        .values(values)
        .build()
        .unwrap();
    
    let write_result = client.sheets.write_single_range(&write_req).await;
    assert!(write_result.is_ok());
    
    let read_req = ReadSingleRangeReqBuilder::default()
        .spreadsheet_token("shtcn123456".to_string())
        .range("A1:B2".to_string())
        .build()
        .unwrap();
    
    let read_result = client.sheets.read_single_range(&read_req).await;
    assert!(read_result.is_ok());
    let read_data = read_result.unwrap();
    assert!(read_data.data.values.is_some());
    
    let mut reply_list = HashMap::new();
    reply_list.insert("content".to_string(), json!("请确认这些数据的准确性"));
    
    let comment_req = CreateCommentReqBuilder::default()
        .file_token("shtcn123456".to_string())
        .reply_list(reply_list)
        .build()
        .unwrap();
    
    let comment_result = client.comments.create_comment(&comment_req).await;
    assert!(comment_result.is_ok());
}

/// 集成测试：文档创建与权限管理
#[rstest]
#[tokio::test]
async fn test_document_with_permissions_workflow(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let client = create_test_client(&mock_server.uri());
    
    // 步骤1: 创建文档
    let doc_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "document": {
                "document_id": "doctest123",
                "title": "集成测试文档"
            }
        }
    });
    
    Mock::given(method("POST"))
        .and(path("/open-apis/docx/v1/documents"))
        .and(header("authorization", "Bearer test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&doc_response))
        .mount(&mock_server)
        .await;
    
    // 步骤2: 添加文档内容块
    let block_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "children": [
                {
                    "block_id": "block123",
                    "block_type": 2,
                    "text_elements": [
                        {
                            "text_run": {
                                "content": "这是集成测试文档的内容"
                            }
                        }
                    ]
                }
            ]
        }
    });
    
    Mock::given(method("POST"))
        .and(path("/open-apis/docx/v1/documents/doctest123/blocks/block456/children"))
        .and(header("authorization", "Bearer test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&block_response))
        .mount(&mock_server)
        .await;
    
    // 步骤3: 设置文档权限
    let permission_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "member_id": "user789",
            "perm": "view"
        }
    });
    
    Mock::given(method("POST"))
        .and(path("/open-apis/drive/v1/permissions/doctest123/members"))
        .and(header("authorization", "Bearer test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&permission_response))
        .mount(&mock_server)
        .await;
    
    // 执行完整工作流
    let create_doc_req = CreateDocumentReqBuilder::default()
        .title("集成测试文档".to_string())
        .build()
        .unwrap();
    
    let doc_result = client.docx.create_document(&create_doc_req).await;
    assert!(doc_result.is_ok());
    
    let mut text_elements = HashMap::new();
    text_elements.insert("content".to_string(), json!("这是集成测试文档的内容"));
    
    let mut children = HashMap::new();
    children.insert("block_type".to_string(), json!(2)); // 文本块
    children.insert("text_elements".to_string(), json!([{
        "text_run": {
            "content": "这是集成测试文档的内容"
        }
    }]));
    
    let block_req = CreateDocumentBlockReqBuilder::default()
        .document_id("doctest123".to_string())
        .block_id("block456".to_string())
        .children(vec![children])
        .build()
        .unwrap();
    
    let block_result = client.docx.create_document_block(&block_req).await;
    assert!(block_result.is_ok());
    
    let permission_req = CreateMemberPermissionReqBuilder::default()
        .token("doctest123".to_string())
        .member_type("user".to_string())
        .member_id("user789".to_string())
        .perm("view".to_string())
        .build()
        .unwrap();
    
    let permission_result = client.permission.create_member_permission(&permission_req).await;
    assert!(permission_result.is_ok());
}

/// 性能测试：并发操作
#[rstest]
#[tokio::test]
async fn test_concurrent_operations(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let client = create_test_client(&mock_server.uri());
    
    // 设置多个并发请求的Mock
    let response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "token": "concurrent_test",
            "name": "并发测试文件",
            "type": "file"
        }
    });
    
    // 为多个并发请求设置相同的Mock响应
    for i in 0..10 {
        Mock::given(method("GET"))
            .and(path(format!("/open-apis/drive/v1/files/concurrent_test_{}/meta", i)))
            .and(header("authorization", "Bearer test_token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response))
            .mount(&mock_server)
            .await;
    }
    
    // 创建并发任务
    let mut tasks = Vec::new();
    
    for i in 0..10 {
        let client = client.clone();
        let task = tokio::spawn(async move {
            let req = GetFileMetaReqBuilder::default()
                .file_token(format!("concurrent_test_{}", i))
                .build()
                .unwrap();
            
            client.drive.get_file_meta(&req).await
        });
        tasks.push(task);
    }
    
    // 等待所有任务完成
    let results = futures::future::join_all(tasks).await;
    
    // 验证所有请求都成功
    for result in results {
        assert!(result.is_ok());
        let api_result = result.unwrap();
        assert!(api_result.is_ok());
    }
}

/// 错误处理集成测试
#[rstest]
#[tokio::test]
async fn test_error_handling_workflow(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let client = create_test_client(&mock_server.uri());
    
    // 设置返回错误的Mock
    let error_response = json!({
        "code": 403,
        "msg": "permission denied",
        "data": {}
    });
    
    Mock::given(method("GET"))
        .and(path("/open-apis/drive/v1/files/nonexistent/meta"))
        .and(header("authorization", "Bearer test_token"))
        .respond_with(ResponseTemplate::new(403).set_body_json(&error_response))
        .mount(&mock_server)
        .await;
    
    // 测试错误处理
    let req = GetFileMetaReqBuilder::default()
        .file_token("nonexistent".to_string())
        .build()
        .unwrap();
    
    let result = client.drive.get_file_meta(&req).await;
    assert!(result.is_err());
    
    // 可以进一步验证错误类型和消息
    if let Err(error) = result {
        // 验证错误包含期望的信息
        let error_str = format!("{:?}", error);
        assert!(error_str.contains("403") || error_str.contains("permission denied"));
    }
}

#[cfg(test)]
mod test_utilities {
    use super::*;
    
    /// 创建测试用的文件元数据
    pub fn create_test_file_meta() -> Value {
        json!({
            "token": "test_token_123",
            "name": "测试文件.txt",
            "type": "file",
            "parent_token": "folder_123",
            "owner_id": "user_123",
            "created_time": "1640995200",
            "modified_time": "1640995200",
            "size": 1024,
            "url": "https://example.feishu.cn/file/test_token_123"
        })
    }
    
    /// 创建测试用的表格数据
    pub fn create_test_sheet_values() -> Vec<Vec<String>> {
        vec![
            vec!["ID".to_string(), "姓名".to_string(), "部门".to_string()],
            vec!["1".to_string(), "张三".to_string(), "技术部".to_string()],
            vec!["2".to_string(), "李四".to_string(), "产品部".to_string()],
        ]
    }
    
    /// 创建测试用的多维表格字段
    pub fn create_test_bitable_fields() -> HashMap<String, Value> {
        let mut fields = HashMap::new();
        fields.insert("姓名".to_string(), json!("测试用户"));
        fields.insert("年龄".to_string(), json!(30));
        fields.insert("部门".to_string(), json!("技术部"));
        fields.insert("入职日期".to_string(), json!("2023-01-01"));
        fields
    }
    
    /// 验证响应数据的基础结构
    pub fn validate_basic_response_structure(response: &Value) -> bool {
        response.get("code").is_some() 
            && response.get("msg").is_some() 
            && response.get("data").is_some()
    }
}