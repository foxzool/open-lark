//! DOCX 服务单元测试
//!
//! 本模块测试飞书文档处理 DOCX API 的各种功能，包括：
//! - 文档创建和管理
//! - 文档内容读写
//! - 文档格式处理
//! - 文档版本管理
//! - 文档导出功能

use wiremock::{
    matchers::{method, path, path_regex, query_param, body_json},
    Mock, MockServer, ResponseTemplate,
};
use serde_json::json;
use rstest::{fixture, rstest};
use proptest::prelude::*;

use open_lark::{
    core::{config::Config, constants::AccessTokenType, req_option::RequestOption},
    service::cloud_docs::docx::{
        v1::{
            DocxService,
            create::{CreateDocxRequest, CreateDocxResponseData, Docx},
            get::{GetDocxRequest, GetDocxResponseData},
            get_raw_content::{GetRawContentRequest, GetRawContentResponseData},
            create_document_block::{
                CreateDocumentBlockRequest, CreateDocumentBlockResponseData, 
                DocumentBlock, BlockType
            },
            get_document_blocks::{
                GetDocumentBlocksRequest, GetDocumentBlocksResponseData
            },
            update_document_block::{
                UpdateDocumentBlockRequest, UpdateDocumentBlockResponseData
            },
            delete_document_block::{
                DeleteDocumentBlockRequest, DeleteDocumentBlockResponseData
            },
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

// ===================== 文档创建和管理测试 =====================

#[rstest]
#[tokio::test]
async fn test_create_docx_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "document": {
                "document_id": "doccnABCDEF1234567890",
                "revision_id": 1,
                "title": "产品需求文档v1.0"
            }
        }
    });

    Mock::given(method("POST"))
        .and(path("/open-apis/docx/v1/documents"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let docx_service = DocxService::new(config);
    let request = CreateDocxRequest::builder()
        .title("产品需求文档v1.0")
        .folder_token("fldbcO1UuPz8VwnpPx5a92abcdef")
        .build();

    let result = docx_service.create(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.code, 0);
    assert_eq!(response.msg, "success");
    
    let data = response.data.unwrap();
    assert_eq!(data.document.document_id, "doccnABCDEF1234567890");
    assert_eq!(data.document.revision_id, 1);
    assert_eq!(data.document.title, "产品需求文档v1.0");
}

#[rstest]
#[tokio::test]
async fn test_get_docx_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let document_id = "doccnABCDEF1234567890";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "document": {
                "document_id": document_id,
                "revision_id": 5,
                "title": "产品需求文档v1.0",
                "owner_id": "ou_123456789abcdef",
                "create_time": "1674552412000",
                "update_time": "1674562412000"
            }
        }
    });

    Mock::given(method("GET"))
        .and(path(format!("/open-apis/docx/v1/documents/{}", document_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let docx_service = DocxService::new(config);
    let request = GetDocxRequest::new(document_id);

    let result = docx_service.get(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.document.document_id, document_id);
    assert_eq!(data.document.revision_id, 5);
    assert_eq!(data.document.title, "产品需求文档v1.0");
    assert_eq!(data.document.owner_id, Some("ou_123456789abcdef".to_string()));
}

#[rstest]
#[tokio::test]
async fn test_get_raw_content_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let document_id = "doccnABCDEF1234567890";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "content": "# 产品需求文档v1.0\n\n## 项目背景\n\n本项目旨在开发一款...\n\n## 功能需求\n\n1. 用户注册和登录\n2. 个人资料管理\n3. 数据展示和分析\n\n## 技术方案\n\n使用React + Node.js技术栈..."
        }
    });

    Mock::given(method("GET"))
        .and(path(format!("/open-apis/docx/v1/documents/{}/raw_content", document_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let docx_service = DocxService::new(config);
    let request = GetRawContentRequest::new(document_id);

    let result = docx_service.get_raw_content(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert!(data.content.contains("产品需求文档v1.0"));
    assert!(data.content.contains("项目背景"));
    assert!(data.content.contains("功能需求"));
    assert!(data.content.contains("技术方案"));
    assert!(data.content.contains("React + Node.js"));
}

// ===================== 文档块操作测试 =====================

#[rstest]
#[tokio::test]
async fn test_create_document_block_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let document_id = "doccnABCDEF1234567890";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "block_id": "doxcnABCDEF1234567890BLOCK001",
            "children": []
        }
    });

    Mock::given(method("POST"))
        .and(path(format!("/open-apis/docx/v1/documents/{}/blocks", document_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let docx_service = DocxService::new(config);
    let block = DocumentBlock {
        block_type: BlockType::Text,
        text: Some("这是新添加的文档内容块，包含重要的项目信息。".to_string()),
        children: vec![],
    };
    
    let request = CreateDocumentBlockRequest::builder()
        .document_id(document_id)
        .index(-1) // 添加到末尾
        .children(vec![block])
        .build();

    let result = docx_service.create_document_block(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.block_id, "doxcnABCDEF1234567890BLOCK001");
    assert_eq!(data.children.len(), 0);
}

#[rstest]
#[tokio::test]
async fn test_get_document_blocks_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let document_id = "doccnABCDEF1234567890";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "blocks": [
                {
                    "block_id": "doxcnBLOCK001",
                    "block_type": "heading1",
                    "heading1": {
                        "elements": [
                            {
                                "type": "text_run",
                                "text_run": {
                                    "content": "产品需求文档",
                                    "text_element_style": {}
                                }
                            }
                        ]
                    },
                    "children": []
                },
                {
                    "block_id": "doxcnBLOCK002",
                    "block_type": "text",
                    "text": {
                        "elements": [
                            {
                                "type": "text_run",
                                "text_run": {
                                    "content": "本文档详细描述了产品的功能需求和技术实现方案。",
                                    "text_element_style": {}
                                }
                            }
                        ]
                    },
                    "children": []
                },
                {
                    "block_id": "doxcnBLOCK003",
                    "block_type": "bullet_list",
                    "bullet_list": {
                        "elements": [
                            {
                                "type": "text_run",
                                "text_run": {
                                    "content": "用户注册功能",
                                    "text_element_style": {}
                                }
                            }
                        ]
                    },
                    "children": [
                        {
                            "block_id": "doxcnBLOCK004",
                            "block_type": "bullet_list",
                            "bullet_list": {
                                "elements": [
                                    {
                                        "type": "text_run", 
                                        "text_run": {
                                            "content": "邮箱注册",
                                            "text_element_style": {}
                                        }
                                    }
                                ]
                            },
                            "children": []
                        }
                    ]
                }
            ],
            "has_more": false,
            "page_token": ""
        }
    });

    Mock::given(method("GET"))
        .and(path(format!("/open-apis/docx/v1/documents/{}/blocks", document_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let docx_service = DocxService::new(config);
    let request = GetDocumentBlocksRequest::builder()
        .document_id(document_id)
        .page_size(100)
        .build();

    let result = docx_service.get_document_blocks(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.blocks.len(), 3);
    assert_eq!(data.has_more, false);
    
    // 验证标题块
    let heading_block = &data.blocks[0];
    assert_eq!(heading_block.block_id, "doxcnBLOCK001");
    assert_eq!(heading_block.block_type, "heading1");
    
    // 验证文本块
    let text_block = &data.blocks[1];
    assert_eq!(text_block.block_id, "doxcnBLOCK002");
    assert_eq!(text_block.block_type, "text");
    
    // 验证列表块及其子块
    let list_block = &data.blocks[2];
    assert_eq!(list_block.block_id, "doxcnBLOCK003");
    assert_eq!(list_block.block_type, "bullet_list");
    assert_eq!(list_block.children.len(), 1);
    
    let sub_list_block = &list_block.children[0];
    assert_eq!(sub_list_block.block_id, "doxcnBLOCK004");
    assert_eq!(sub_list_block.block_type, "bullet_list");
}

#[rstest]
#[tokio::test]
async fn test_update_document_block_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let document_id = "doccnABCDEF1234567890";
    let block_id = "doxcnBLOCK002";
    
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "block_id": block_id,
            "children": []
        }
    });

    Mock::given(method("PATCH"))
        .and(path(format!("/open-apis/docx/v1/documents/{}/blocks/{}", document_id, block_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let docx_service = DocxService::new(config);
    let updated_block = DocumentBlock {
        block_type: BlockType::Text,
        text: Some("这是更新后的文档内容，包含了最新的项目信息和技术细节。".to_string()),
        children: vec![],
    };
    
    let request = UpdateDocumentBlockRequest::builder()
        .document_id(document_id)
        .block_id(block_id)
        .children(vec![updated_block])
        .build();

    let result = docx_service.update_document_block(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.block_id, block_id);
}

#[rstest]
#[tokio::test]
async fn test_delete_document_block_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let document_id = "doccnABCDEF1234567890";
    let block_id = "doxcnBLOCK002";
    
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {}
    });

    Mock::given(method("DELETE"))
        .and(path(format!("/open-apis/docx/v1/documents/{}/blocks/{}", document_id, block_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let docx_service = DocxService::new(config);
    let request = DeleteDocumentBlockRequest::builder()
        .document_id(document_id)
        .block_id(block_id)
        .build();

    let result = docx_service.delete_document_block(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.code, 0);
}

// ===================== 复杂文档结构测试 =====================

#[rstest]
#[tokio::test]
async fn test_create_complex_document_structure(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let document_id = "doccnComplexStructure";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "block_id": "doxcnComplexBlock001",
            "children": [
                {
                    "block_id": "doxcnSubBlock001",
                    "children": []
                },
                {
                    "block_id": "doxcnSubBlock002", 
                    "children": []
                }
            ]
        }
    });

    Mock::given(method("POST"))
        .and(path(format!("/open-apis/docx/v1/documents/{}/blocks", document_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let docx_service = DocxService::new(config);
    
    // 创建复杂的嵌套文档结构
    let sub_block1 = DocumentBlock {
        block_type: BlockType::Text,
        text: Some("第一个子项：用户界面设计".to_string()),
        children: vec![],
    };
    
    let sub_block2 = DocumentBlock {
        block_type: BlockType::Text,
        text: Some("第二个子项：后端API设计".to_string()),
        children: vec![],
    };
    
    let main_block = DocumentBlock {
        block_type: BlockType::BulletList,
        text: Some("项目组成部分：".to_string()),
        children: vec![sub_block1, sub_block2],
    };
    
    let request = CreateDocumentBlockRequest::builder()
        .document_id(document_id)
        .index(0) // 添加到开头
        .children(vec![main_block])
        .build();

    let result = docx_service.create_document_block(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.block_id, "doxcnComplexBlock001");
    assert_eq!(data.children.len(), 2);
    assert_eq!(data.children[0].block_id, "doxcnSubBlock001");
    assert_eq!(data.children[1].block_id, "doxcnSubBlock002");
}

// ===================== 请求构建器测试 =====================

#[test]
fn test_create_docx_request_builder() {
    let request = CreateDocxRequest::builder()
        .title("测试文档")
        .folder_token("folder123")
        .build();

    assert_eq!(request.title, "测试文档");
    assert_eq!(request.folder_token, Some("folder123".to_string()));
}

#[test]
fn test_get_docx_request_builder() {
    let request = GetDocxRequest::new("doc123");
    assert_eq!(request.document_id, "doc123");
}

#[test]
fn test_create_document_block_request_builder() {
    let block = DocumentBlock {
        block_type: BlockType::Text,
        text: Some("测试内容".to_string()),
        children: vec![],
    };
    
    let request = CreateDocumentBlockRequest::builder()
        .document_id("doc123")
        .index(0)
        .children(vec![block])
        .build();

    assert_eq!(request.document_id, "doc123");
    assert_eq!(request.index, 0);
    assert_eq!(request.children.len(), 1);
    assert_eq!(request.children[0].text, Some("测试内容".to_string()));
}

#[test]
fn test_document_block_types() {
    // 测试不同类型的文档块
    let text_block = DocumentBlock {
        block_type: BlockType::Text,
        text: Some("普通文本".to_string()),
        children: vec![],
    };
    assert_eq!(text_block.block_type, BlockType::Text);

    let heading_block = DocumentBlock {
        block_type: BlockType::Heading1,
        text: Some("一级标题".to_string()),
        children: vec![],
    };
    assert_eq!(heading_block.block_type, BlockType::Heading1);

    let list_block = DocumentBlock {
        block_type: BlockType::BulletList,
        text: Some("列表项".to_string()),
        children: vec![],
    };
    assert_eq!(list_block.block_type, BlockType::BulletList);
}

// ===================== 属性测试 =====================

proptest! {
    #[test]
    fn test_document_id_invariants(
        doc_id in prop::string::string_regex("doccn[a-zA-Z0-9]{16,32}").unwrap()
    ) {
        let request = GetDocxRequest::new(&doc_id);
        assert_eq!(request.document_id, doc_id);
        assert!(doc_id.starts_with("doccn"));
    }
}

proptest! {
    #[test]
    fn test_document_title_invariants(
        title in "[\\u4e00-\\u9fa5a-zA-Z0-9_\\s]{1,100}"
    ) {
        let request = CreateDocxRequest::builder()
            .title(&title)
            .build();
        
        assert_eq!(request.title, title);
        assert!(title.len() >= 1 && title.len() <= 100);
    }
}

proptest! {
    #[test]
    fn test_block_content_invariants(
        content in "[\\s\\S]{1,10000}"
    ) {
        let block = DocumentBlock {
            block_type: BlockType::Text,
            text: Some(content.clone()),
            children: vec![],
        };
        
        assert_eq!(block.text, Some(content.clone()));
        assert!(content.len() >= 1 && content.len() <= 10000);
    }
}

// ===================== 错误处理测试 =====================

#[rstest]
#[tokio::test]
async fn test_document_not_found_error(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let invalid_document_id = "doccnInvalidDocument";
    let mock_response = json!({
        "code": 1248010,
        "msg": "文档不存在"
    });

    Mock::given(method("GET"))
        .and(path(format!("/open-apis/docx/v1/documents/{}", invalid_document_id)))
        .respond_with(ResponseTemplate::new(404).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let docx_service = DocxService::new(config);
    let request = GetDocxRequest::new(invalid_document_id);

    let result = docx_service.get(request, None).await;
    
    if result.is_ok() {
        let response = result.unwrap();
        assert_ne!(response.code, 0);
        assert!(response.msg.contains("文档") || response.msg.contains("不存在"));
    }
}

#[rstest]
#[tokio::test]
async fn test_permission_denied_error(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let document_id = "doccnNoPermission";
    let mock_response = json!({
        "code": 1248006,
        "msg": "无权限访问该文档"
    });

    Mock::given(method("GET"))
        .and(path(format!("/open-apis/docx/v1/documents/{}", document_id)))
        .respond_with(ResponseTemplate::new(403).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let docx_service = DocxService::new(config);
    let request = GetDocxRequest::new(document_id);

    let result = docx_service.get(request, None).await;
    
    if result.is_ok() {
        let response = result.unwrap();
        assert_eq!(response.code, 1248006);
        assert!(response.msg.contains("权限"));
    }
}

#[rstest]
#[tokio::test]
async fn test_invalid_block_position_error(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let document_id = "doccnValidDocument";
    let mock_response = json!({
        "code": 1248025,
        "msg": "块位置无效"
    });

    Mock::given(method("POST"))
        .and(path(format!("/open-apis/docx/v1/documents/{}/blocks", document_id)))
        .respond_with(ResponseTemplate::new(400).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let docx_service = DocxService::new(config);
    let block = DocumentBlock {
        block_type: BlockType::Text,
        text: Some("测试内容".to_string()),
        children: vec![],
    };
    
    let request = CreateDocumentBlockRequest::builder()
        .document_id(document_id)
        .index(999999) // 无效的索引位置
        .children(vec![block])
        .build();

    let result = docx_service.create_document_block(request, None).await;
    
    if result.is_ok() {
        let response = result.unwrap();
        assert_ne!(response.code, 0);
        assert!(response.msg.contains("位置") || response.msg.contains("无效"));
    }
}

// ===================== 集成场景测试 =====================

#[rstest]
#[tokio::test]
async fn test_complete_document_workflow(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    // 1. 创建文档
    let create_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "document": {
                "document_id": "doccnWorkflowTest",
                "revision_id": 1,
                "title": "集成测试文档"
            }
        }
    });

    Mock::given(method("POST"))
        .and(path("/open-apis/docx/v1/documents"))
        .respond_with(ResponseTemplate::new(200).set_body_json(create_response))
        .mount(&mock_server)
        .await;

    // 2. 添加文档块
    let add_block_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "block_id": "doxcnWorkflowBlock001",
            "children": []
        }
    });

    Mock::given(method("POST"))
        .and(path("/open-apis/docx/v1/documents/doccnWorkflowTest/blocks"))
        .respond_with(ResponseTemplate::new(200).set_body_json(add_block_response))
        .mount(&mock_server)
        .await;

    // 3. 获取文档内容
    let get_blocks_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "blocks": [
                {
                    "block_id": "doxcnWorkflowBlock001",
                    "block_type": "text",
                    "text": {
                        "elements": [
                            {
                                "type": "text_run",
                                "text_run": {
                                    "content": "这是集成测试添加的内容",
                                    "text_element_style": {}
                                }
                            }
                        ]
                    },
                    "children": []
                }
            ],
            "has_more": false,
            "page_token": ""
        }
    });

    Mock::given(method("GET"))
        .and(path("/open-apis/docx/v1/documents/doccnWorkflowTest/blocks"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_blocks_response))
        .mount(&mock_server)
        .await;

    // 4. 更新文档块
    let update_block_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "block_id": "doxcnWorkflowBlock001",
            "children": []
        }
    });

    Mock::given(method("PATCH"))
        .and(path("/open-apis/docx/v1/documents/doccnWorkflowTest/blocks/doxcnWorkflowBlock001"))
        .respond_with(ResponseTemplate::new(200).set_body_json(update_block_response))
        .mount(&mock_server)
        .await;

    // 执行完整的文档操作流程
    let docx_service = DocxService::new(config);

    // 1. 创建文档
    let create_req = CreateDocxRequest::builder()
        .title("集成测试文档")
        .folder_token("test_folder_token")
        .build();

    let create_result = docx_service.create(create_req, None).await;
    assert!(create_result.is_ok());
    let create_data = create_result.unwrap().data.unwrap();
    let document_id = create_data.document.document_id;
    assert_eq!(document_id, "doccnWorkflowTest");

    // 2. 添加文档块
    let block = DocumentBlock {
        block_type: BlockType::Text,
        text: Some("这是集成测试添加的内容".to_string()),
        children: vec![],
    };
    
    let add_block_req = CreateDocumentBlockRequest::builder()
        .document_id(&document_id)
        .index(-1)
        .children(vec![block])
        .build();

    let add_result = docx_service.create_document_block(add_block_req, None).await;
    assert!(add_result.is_ok());
    let add_data = add_result.unwrap().data.unwrap();
    let block_id = add_data.block_id;

    // 3. 获取文档内容验证
    let get_blocks_req = GetDocumentBlocksRequest::builder()
        .document_id(&document_id)
        .build();

    let get_result = docx_service.get_document_blocks(get_blocks_req, None).await;
    assert!(get_result.is_ok());
    let get_data = get_result.unwrap().data.unwrap();
    assert_eq!(get_data.blocks.len(), 1);
    assert_eq!(get_data.blocks[0].block_id, block_id);

    // 4. 更新文档块
    let updated_block = DocumentBlock {
        block_type: BlockType::Text,
        text: Some("这是更新后的集成测试内容".to_string()),
        children: vec![],
    };
    
    let update_req = UpdateDocumentBlockRequest::builder()
        .document_id(&document_id)
        .block_id(&block_id)
        .children(vec![updated_block])
        .build();

    let update_result = docx_service.update_document_block(update_req, None).await;
    assert!(update_result.is_ok());
}

#[cfg(test)]
mod test_utils {
    use super::*;

    /// 创建测试用的文档块
    pub fn create_test_block(block_type: BlockType, content: &str) -> DocumentBlock {
        DocumentBlock {
            block_type,
            text: Some(content.to_string()),
            children: vec![],
        }
    }

    /// 创建测试用的嵌套文档块
    pub fn create_nested_block(
        parent_type: BlockType,
        parent_content: &str,
        children: Vec<DocumentBlock>,
    ) -> DocumentBlock {
        DocumentBlock {
            block_type: parent_type,
            text: Some(parent_content.to_string()),
            children,
        }
    }

    /// 验证文档结构的一致性
    pub fn validate_document_structure(blocks: &[DocumentBlock], expected_depth: usize) {
        for block in blocks {
            if expected_depth > 0 {
                assert!(block.children.len() <= 10, "文档嵌套层级过深");
                if !block.children.is_empty() {
                    validate_document_structure(&block.children, expected_depth - 1);
                }
            }
        }
    }

    /// 计算文档内容长度
    pub fn calculate_content_length(blocks: &[DocumentBlock]) -> usize {
        let mut total_length = 0;
        for block in blocks {
            if let Some(ref text) = block.text {
                total_length += text.len();
            }
            total_length += calculate_content_length(&block.children);
        }
        total_length
    }
}