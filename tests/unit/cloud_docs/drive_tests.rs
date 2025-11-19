//! Drive 服务单元测试
//!
//! 本模块测试飞书云盘 Drive API 的各种功能，包括：
//! - 文件元数据获取
//! - 文件统计信息查询
//! - 文件访问记录
//! - 文件上传下载
//! - 文件夹管理
//! - 权限控制
//! - 导入任务管理

use wiremock::{
    matchers::{method, path, path_regex},
    Mock, MockServer, ResponseTemplate,
};
use serde_json::json;
use rstest::{fixture, rstest};
use mockall::predicate::*;
use proptest::prelude::*;

use open_lark::{
    core::{config::Config, constants::AccessTokenType, req_option::RequestOption},
    service::cloud_docs::drive::{
        v1::file::{
            FileService, GetFileMetaRequest, GetFileStatisticsRequest, 
            ListFileViewRecordsRequest, RequestDoc, FileMeta, 
            FileViewRecord, GetFileMetaRespData
        },
        v1::files::{
            FilesService, UploadAllRequest
        },
        v1::folder::{
            FolderService, CreateFolderRequest, GetFolderMetaRequest,
            ListFolderChildrenRequest
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

// ===================== 文件元数据测试 =====================

#[rstest]
#[tokio::test]
async fn test_get_file_meta_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    // 模拟成功响应
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "metas": [
                {
                    "doc_token": "test_doc_token_1",
                    "doc_type": "docx",
                    "title": "测试文档1",
                    "owner_id": "ou_123456",
                    "create_time": "2024-01-01T10:00:00Z",
                    "update_time": "2024-01-02T11:00:00Z",
                    "url": "https://example.com/doc1"
                },
                {
                    "doc_token": "test_doc_token_2", 
                    "doc_type": "sheet",
                    "title": "测试表格1",
                    "owner_id": "ou_789012",
                    "create_time": "2024-01-01T12:00:00Z",
                    "update_time": "2024-01-02T13:00:00Z",
                    "url": "https://example.com/sheet1"
                }
            ]
        }
    });

    Mock::given(method("POST"))
        .and(path("/open-apis/drive/v1/metas/batch_query"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let file_service = FileService::new(config);
    let request = GetFileMetaRequest::new(vec![
        ("test_doc_token_1".to_string(), "docx".to_string()),
        ("test_doc_token_2".to_string(), "sheet".to_string()),
    ]);

    let result = file_service.get_file_meta(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.code, 0);
    assert_eq!(response.msg, "success");
    
    let data = response.data.unwrap();
    assert_eq!(data.metas.len(), 2);
    
    let first_meta = &data.metas[0];
    assert_eq!(first_meta.doc_token, "test_doc_token_1");
    assert_eq!(first_meta.doc_type, "docx");
    assert_eq!(first_meta.title, "测试文档1");
    assert_eq!(first_meta.owner_id, "ou_123456");
    assert_eq!(first_meta.url.as_ref().unwrap(), "https://example.com/doc1");
}

#[rstest]
#[tokio::test]
async fn test_get_file_meta_empty_request(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let mock_response = json!({
        "code": 0,
        "msg": "success", 
        "data": {
            "metas": []
        }
    });

    Mock::given(method("POST"))
        .and(path("/open-apis/drive/v1/metas/batch_query"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let file_service = FileService::new(config);
    let request = GetFileMetaRequest::new(vec![]);

    let result = file_service.get_file_meta(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    assert_eq!(data.metas.len(), 0);
}

#[rstest]
#[tokio::test]
async fn test_get_file_meta_error_response(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let mock_response = json!({
        "code": 1248006,
        "msg": "文档不存在或无权限访问"
    });

    Mock::given(method("POST"))
        .and(path("/open-apis/drive/v1/metas/batch_query"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let file_service = FileService::new(config);
    let request = GetFileMetaRequest::new(vec![
        ("invalid_token".to_string(), "docx".to_string()),
    ]);

    let result = file_service.get_file_meta(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.code, 1248006);
    assert_eq!(response.msg, "文档不存在或无权限访问");
    assert!(response.data.is_none());
}

// ===================== 文件统计信息测试 =====================

#[rstest]
#[tokio::test]
async fn test_get_file_statistics_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let file_token = "test_file_token";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "uv": 150,
            "pv": 300,
            "like_count": 25,
            "comment_count": 8
        }
    });

    Mock::given(method("GET"))
        .and(path(format!("/open-apis/drive/v1/files/{}/statistics", file_token)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let file_service = FileService::new(config);
    let request = GetFileStatisticsRequest::new(file_token);

    let result = file_service.get_file_statistics(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.code, 0);
    
    let data = response.data.unwrap();
    assert_eq!(data.uv, 150);
    assert_eq!(data.pv, 300);
    assert_eq!(data.like_count, 25);
    assert_eq!(data.comment_count, 8);
}

// ===================== 文件访问记录测试 =====================

#[rstest]
#[tokio::test]
async fn test_list_file_view_records_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let file_token = "test_file_token";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "has_more": true,
            "page_token": "next_page_token",
            "items": [
                {
                    "viewer_id": "ou_123456",
                    "viewer_name": "张三",
                    "view_time": "2024-01-01T10:30:00Z"
                },
                {
                    "viewer_id": "ou_789012",
                    "viewer_name": "李四", 
                    "view_time": "2024-01-01T11:00:00Z"
                }
            ]
        }
    });

    Mock::given(method("GET"))
        .and(path(format!("/open-apis/drive/v1/files/{}/view_records", file_token)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let file_service = FileService::new(config);
    let request = ListFileViewRecordsRequest::new(file_token)
        .with_page_size(10);

    let result = file_service.list_file_view_records(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.code, 0);
    
    let data = response.data.unwrap();
    assert_eq!(data.has_more, true);
    assert_eq!(data.page_token.as_ref().unwrap(), "next_page_token");
    assert_eq!(data.items.len(), 2);
    
    let first_record = &data.items[0];
    assert_eq!(first_record.viewer_id, "ou_123456");
    assert_eq!(first_record.viewer_name, "张三");
    assert_eq!(first_record.view_time, "2024-01-01T10:30:00Z");
}

#[rstest]
#[tokio::test]
async fn test_list_file_view_records_with_pagination(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let file_token = "test_file_token";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "has_more": false,
            "page_token": null,
            "items": [
                {
                    "viewer_id": "ou_345678",
                    "viewer_name": "王五",
                    "view_time": "2024-01-01T12:00:00Z"
                }
            ]
        }
    });

    Mock::given(method("GET"))
        .and(path(format!("/open-apis/drive/v1/files/{}/view_records", file_token)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let file_service = FileService::new(config);
    let request = ListFileViewRecordsRequest::new(file_token)
        .with_page_token("existing_token")
        .with_page_size(5);

    let result = file_service.list_file_view_records(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    assert_eq!(data.has_more, false);
    assert!(data.page_token.is_none());
    assert_eq!(data.items.len(), 1);
}

// ===================== 请求构建器测试 =====================

#[test]
fn test_get_file_meta_request_builder() {
    let request = GetFileMetaRequest::new(vec![
        ("token1".to_string(), "docx".to_string()),
        ("token2".to_string(), "sheet".to_string()),
    ]);

    assert_eq!(request.request_docs.len(), 2);
    assert_eq!(request.with_url, Some(true));
    
    assert_eq!(request.request_docs[0].doc_token, "token1");
    assert_eq!(request.request_docs[0].doc_type, "docx");
    assert_eq!(request.request_docs[1].doc_token, "token2");
    assert_eq!(request.request_docs[1].doc_type, "sheet");
}

#[test]
fn test_get_file_statistics_request_builder() {
    let request = GetFileStatisticsRequest::new("test_token");
    assert_eq!(request.file_token, "test_token");
    
    // 测试字符串引用
    let request2 = GetFileStatisticsRequest::new(&"ref_token".to_string());
    assert_eq!(request2.file_token, "ref_token");
}

#[test]
fn test_list_file_view_records_request_builder() {
    let request = ListFileViewRecordsRequest::new("file_token")
        .with_page_token("page_token")
        .with_page_size(20);

    assert_eq!(request.file_token, "file_token");
    assert_eq!(request.page_token.as_ref().unwrap(), "page_token");
    assert_eq!(request.page_size.unwrap(), 20);
}

// ===================== 属性测试 =====================

proptest! {
    #[test]
    fn test_file_meta_request_invariants(
        tokens in prop::collection::vec(
            (prop::string::string_regex("[a-zA-Z0-9_]{10,50}").unwrap(),
             prop::sample::select(vec!["docx", "sheet", "bitable", "wiki", "mindnote"])),
            0..10
        )
    ) {
        let request = GetFileMetaRequest::new(tokens.clone());
        
        // 验证基本属性
        assert_eq!(request.request_docs.len(), tokens.len());
        assert_eq!(request.with_url, Some(true));
        
        // 验证数据一致性
        for (i, (original_token, original_type)) in tokens.iter().enumerate() {
            assert_eq!(request.request_docs[i].doc_token, *original_token);
            assert_eq!(request.request_docs[i].doc_type, *original_type);
        }
    }
}

proptest! {
    #[test]
    fn test_file_statistics_request_invariants(
        token in prop::string::string_regex("[a-zA-Z0-9_]{10,50}").unwrap()
    ) {
        let request = GetFileStatisticsRequest::new(&token);
        assert_eq!(request.file_token, token);
        
        // 测试不同的输入方式
        let request2 = GetFileStatisticsRequest::new(token.clone());
        assert_eq!(request2.file_token, token);
    }
}

proptest! {
    #[test]
    fn test_view_records_request_pagination_invariants(
        file_token in prop::string::string_regex("[a-zA-Z0-9_]{10,50}").unwrap(),
        page_size in 1i32..=100,
        page_token in prop::option::of(prop::string::string_regex("[a-zA-Z0-9_]{20,100}").unwrap())
    ) {
        let mut request = ListFileViewRecordsRequest::new(&file_token)
            .with_page_size(page_size);
            
        if let Some(token) = page_token {
            request = request.with_page_token(&token);
            assert_eq!(request.page_token.as_ref().unwrap(), &token);
        }
        
        assert_eq!(request.file_token, file_token);
        assert_eq!(request.page_size.unwrap(), page_size);
    }
}

// ===================== 错误处理测试 =====================

#[rstest]
#[tokio::test]
async fn test_network_error_handling(test_config: Config) {
    let mut config = test_config;
    config.lark_host = "http://invalid-host-that-does-not-exist.com".to_string();

    let file_service = FileService::new(config);
    let request = GetFileMetaRequest::new(vec![
        ("token".to_string(), "docx".to_string()),
    ]);

    let result = file_service.get_file_meta(request, None).await;
    
    // 应该是网络错误
    assert!(result.is_err());
}

#[rstest]
#[tokio::test]
async fn test_malformed_response_handling(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    // 返回格式错误的JSON
    let malformed_response = "invalid json response";

    Mock::given(method("POST"))
        .and(path("/open-apis/drive/v1/metas/batch_query"))
        .respond_with(ResponseTemplate::new(200).set_body_string(malformed_response))
        .mount(&mock_server)
        .await;

    let file_service = FileService::new(config);
    let request = GetFileMetaRequest::new(vec![
        ("token".to_string(), "docx".to_string()),
    ]);

    let result = file_service.get_file_meta(request, None).await;
    
    // 应该是解析错误
    assert!(result.is_err());
}

// ===================== 集成场景测试 =====================

#[rstest]
#[tokio::test]
async fn test_file_workflow_integration(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let file_token = "integration_test_token";

    // 1. 模拟获取文件元数据
    let meta_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "metas": [{
                "doc_token": file_token,
                "doc_type": "docx",
                "title": "集成测试文档",
                "owner_id": "ou_test_user",
                "create_time": "2024-01-01T10:00:00Z",
                "update_time": "2024-01-01T10:00:00Z",
                "url": "https://example.com/doc"
            }]
        }
    });

    Mock::given(method("POST"))
        .and(path("/open-apis/drive/v1/metas/batch_query"))
        .respond_with(ResponseTemplate::new(200).set_body_json(meta_response))
        .mount(&mock_server)
        .await;

    // 2. 模拟获取统计信息
    let stats_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "uv": 10,
            "pv": 15,
            "like_count": 2,
            "comment_count": 1
        }
    });

    Mock::given(method("GET"))
        .and(path(format!("/open-apis/drive/v1/files/{}/statistics", file_token)))
        .respond_with(ResponseTemplate::new(200).set_body_json(stats_response))
        .mount(&mock_server)
        .await;

    // 3. 模拟获取访问记录
    let view_records_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "has_more": false,
            "page_token": null,
            "items": [{
                "viewer_id": "ou_viewer",
                "viewer_name": "测试用户",
                "view_time": "2024-01-01T11:00:00Z"
            }]
        }
    });

    Mock::given(method("GET"))
        .and(path(format!("/open-apis/drive/v1/files/{}/view_records", file_token)))
        .respond_with(ResponseTemplate::new(200).set_body_json(view_records_response))
        .mount(&mock_server)
        .await;

    // 执行集成测试流程
    let file_service = FileService::new(config);

    // 1. 获取文件元数据
    let meta_request = GetFileMetaRequest::new(vec![
        (file_token.to_string(), "docx".to_string()),
    ]);
    let meta_result = file_service.get_file_meta(meta_request, None).await;
    assert!(meta_result.is_ok());

    let meta_data = meta_result.unwrap().data.unwrap();
    assert_eq!(meta_data.metas.len(), 1);
    assert_eq!(meta_data.metas[0].title, "集成测试文档");

    // 2. 获取统计信息
    let stats_request = GetFileStatisticsRequest::new(file_token);
    let stats_result = file_service.get_file_statistics(stats_request, None).await;
    assert!(stats_result.is_ok());

    let stats_data = stats_result.unwrap().data.unwrap();
    assert_eq!(stats_data.uv, 10);
    assert_eq!(stats_data.like_count, 2);

    // 3. 获取访问记录
    let view_request = ListFileViewRecordsRequest::new(file_token);
    let view_result = file_service.list_file_view_records(view_request, None).await;
    assert!(view_result.is_ok());

    let view_data = view_result.unwrap().data.unwrap();
    assert_eq!(view_data.items.len(), 1);
    assert_eq!(view_data.items[0].viewer_name, "测试用户");
}

// ===================== 性能相关测试 =====================

#[rstest]
#[tokio::test]
async fn test_batch_file_meta_query_performance(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    // 模拟大量文件的元数据查询
    let large_batch_size = 50;
    let mut mock_metas = Vec::new();
    for i in 0..large_batch_size {
        mock_metas.push(json!({
            "doc_token": format!("token_{}", i),
            "doc_type": "docx",
            "title": format!("文档{}", i),
            "owner_id": "ou_test",
            "create_time": "2024-01-01T10:00:00Z",
            "update_time": "2024-01-01T10:00:00Z",
            "url": format!("https://example.com/doc{}", i)
        }));
    }

    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "metas": mock_metas
        }
    });

    Mock::given(method("POST"))
        .and(path("/open-apis/drive/v1/metas/batch_query"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let file_service = FileService::new(config);
    
    // 构建大量文件的请求
    let docs: Vec<(String, String)> = (0..large_batch_size)
        .map(|i| (format!("token_{}", i), "docx".to_string()))
        .collect();

    let request = GetFileMetaRequest::new(docs);

    let start = std::time::Instant::now();
    let result = file_service.get_file_meta(request, None).await;
    let duration = start.elapsed();

    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    assert_eq!(data.metas.len(), large_batch_size);

    // 性能断言 - 批量查询应该在合理时间内完成
    assert!(duration.as_millis() < 5000, "批量查询耗时过长: {:?}", duration);
}

#[cfg(test)]
mod test_utils {
    use super::*;

    /// 创建测试用的文件元数据
    pub fn create_test_file_meta(token: &str, doc_type: &str, title: &str) -> FileMeta {
        FileMeta {
            doc_token: token.to_string(),
            doc_type: doc_type.to_string(),
            title: title.to_string(),
            owner_id: "ou_test_owner".to_string(),
            create_time: "2024-01-01T10:00:00Z".to_string(),
            update_time: "2024-01-01T10:00:00Z".to_string(),
            url: Some(format!("https://example.com/{}", token)),
        }
    }

    /// 创建测试用的访问记录
    pub fn create_test_view_record(viewer_id: &str, viewer_name: &str, view_time: &str) -> FileViewRecord {
        FileViewRecord {
            viewer_id: viewer_id.to_string(),
            viewer_name: viewer_name.to_string(),
            view_time: view_time.to_string(),
        }
    }
}