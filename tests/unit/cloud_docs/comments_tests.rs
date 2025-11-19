//! Comments 服务单元测试
//!
//! 本模块测试飞书云文档评论系统 API 的各种功能，包括：
//! - 评论的 CRUD 操作
//! - 回复管理
//! - 批量查询
//! - 评论状态管理
//! - 评论权限控制

use wiremock::{
    matchers::{method, path, path_regex, query_param, body_json},
    Mock, MockServer, ResponseTemplate,
};
use serde_json::json;
use rstest::{fixture, rstest};
use proptest::prelude::*;

use open_lark::{
    core::{config::Config, constants::AccessTokenType, req_option::RequestOption},
    service::cloud_docs::comments::{
        CommentsService,
        create::{CreateCommentRequest, CreateCommentResponseData, Comment},
        get::{GetCommentRequest, GetCommentResponseData},
        list::{ListCommentsRequest, ListCommentsResponseData},
        update::{UpdateCommentRequest, UpdateCommentResponseData},
        patch::{PatchCommentRequest, PatchCommentResponseData},
        delete::{DeleteCommentRequest, DeleteCommentResponseData},
        list_replies::{ListCommentRepliesRequest, ListCommentRepliesResponseData, Reply},
        create_reply::{CreateCommentReplyRequest, CreateCommentReplyResponseData},
        update_reply::{UpdateCommentReplyRequest, UpdateCommentReplyResponseData},
        delete_reply::{DeleteCommentReplyRequest, DeleteCommentReplyResponseData},
        batch_query::{BatchQueryCommentsRequest, BatchQueryCommentsResponseData},
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

// ===================== 评论 CRUD 测试 =====================

#[rstest]
#[tokio::test]
async fn test_create_comment_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let file_token = "doccnBKgoMyY5OMbUG6FzxAnnNf";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "comment_id": "7101214603932246020",
            "user_id": "ou_123456789abcdef",
            "create_time": 1674552412,
            "update_time": 1674552412,
            "is_solved": false,
            "solved_time": 0,
            "solver_user_id": "",
            "has_more": false,
            "page_token": "",
            "is_whole": false,
            "quote": "这是被引用的原始内容",
            "reply_list": {
                "replies": [],
                "has_more": false,
                "page_token": ""
            }
        }
    });

    Mock::given(method("POST"))
        .and(path("/open-apis/drive/v1/comments"))
        .and(query_param("file_token", file_token))
        .and(query_param("file_type", "docx"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let comments_service = CommentsService::new(config);
    let request = CreateCommentRequest::builder()
        .file_token(file_token)
        .file_type("docx")
        .obj_type("doc")
        .content("这是一个新的评论，对文档内容进行了详细的反馈。")
        .quote("这是被引用的原始内容")
        .build();

    let result = comments_service.create(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.code, 0);
    assert_eq!(response.msg, "success");
    
    let data = response.data.unwrap();
    assert_eq!(data.comment_id, "7101214603932246020");
    assert_eq!(data.user_id, "ou_123456789abcdef");
    assert_eq!(data.is_solved, false);
    assert_eq!(data.quote, "这是被引用的原始内容");
    assert_eq!(data.reply_list.replies.len(), 0);
}

#[rstest]
#[tokio::test]
async fn test_get_comment_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let file_token = "doccnBKgoMyY5OMbUG6FzxAnnNf";
    let comment_id = "7101214603932246020";
    
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "comment_id": comment_id,
            "user_id": "ou_123456789abcdef",
            "create_time": 1674552412,
            "update_time": 1674562412,
            "is_solved": true,
            "solved_time": 1674562412,
            "solver_user_id": "ou_987654321fedcba",
            "content": "这是评论的详细内容，包含了用户的反馈信息。",
            "extra": {
                "image_list": ["https://example.com/image1.png"]
            },
            "quote": "被引用的原始内容片段",
            "reply_list": {
                "replies": [
                    {
                        "reply_id": "7101214603932246021",
                        "user_id": "ou_987654321fedcba",
                        "create_time": 1674555012,
                        "update_time": 1674555012,
                        "content": "感谢您的反馈，我们会认真考虑这个建议。",
                        "extra": {}
                    }
                ],
                "has_more": false,
                "page_token": ""
            }
        }
    });

    Mock::given(method("GET"))
        .and(path("/open-apis/drive/v1/comments"))
        .and(query_param("file_token", file_token))
        .and(query_param("file_type", "docx"))
        .and(query_param("comment_id", comment_id))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let comments_service = CommentsService::new(config);
    let request = GetCommentRequest::builder()
        .file_token(file_token)
        .file_type("docx")
        .comment_id(comment_id)
        .build();

    let result = comments_service.get(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.comment_id, comment_id);
    assert_eq!(data.user_id, "ou_123456789abcdef");
    assert_eq!(data.is_solved, true);
    assert_eq!(data.solver_user_id, "ou_987654321fedcba");
    assert!(data.content.contains("评论的详细内容"));
    assert_eq!(data.reply_list.replies.len(), 1);
    
    let reply = &data.reply_list.replies[0];
    assert_eq!(reply.reply_id, "7101214603932246021");
    assert_eq!(reply.user_id, "ou_987654321fedcba");
    assert!(reply.content.contains("感谢您的反馈"));
}

#[rstest]
#[tokio::test]
async fn test_list_comments_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let file_token = "doccnBKgoMyY5OMbUG6FzxAnnNf";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "items": [
                {
                    "comment_id": "7101214603932246020",
                    "user_id": "ou_123456789abcdef",
                    "create_time": 1674552412,
                    "update_time": 1674552412,
                    "is_solved": false,
                    "solved_time": 0,
                    "solver_user_id": "",
                    "content": "第一条评论：关于产品功能的建议",
                    "quote": "产品功能相关内容",
                    "reply_list": {
                        "replies": [],
                        "has_more": false,
                        "page_token": ""
                    }
                },
                {
                    "comment_id": "7101214603932246021",
                    "user_id": "ou_987654321fedcba", 
                    "create_time": 1674555012,
                    "update_time": 1674555012,
                    "is_solved": true,
                    "solved_time": 1674560012,
                    "solver_user_id": "ou_111222333444555",
                    "content": "第二条评论：关于技术实现的疑问",
                    "quote": "技术实现相关内容",
                    "reply_list": {
                        "replies": [
                            {
                                "reply_id": "7101214603932246022",
                                "user_id": "ou_111222333444555",
                                "create_time": 1674558012,
                                "content": "技术问题已解决"
                            }
                        ],
                        "has_more": false,
                        "page_token": ""
                    }
                }
            ],
            "has_more": false,
            "page_token": ""
        }
    });

    Mock::given(method("GET"))
        .and(path("/open-apis/drive/v1/comments/batch_query"))
        .and(query_param("file_token", file_token))
        .and(query_param("file_type", "docx"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let comments_service = CommentsService::new(config);
    let request = ListCommentsRequest::builder()
        .file_token(file_token)
        .file_type("docx")
        .page_size(10)
        .build();

    let result = comments_service.list(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.items.len(), 2);
    assert_eq!(data.has_more, false);
    
    // 验证第一条评论
    let first_comment = &data.items[0];
    assert_eq!(first_comment.comment_id, "7101214603932246020");
    assert_eq!(first_comment.is_solved, false);
    assert!(first_comment.content.contains("产品功能的建议"));
    assert_eq!(first_comment.reply_list.replies.len(), 0);
    
    // 验证第二条评论
    let second_comment = &data.items[1];
    assert_eq!(second_comment.comment_id, "7101214603932246021");
    assert_eq!(second_comment.is_solved, true);
    assert!(second_comment.content.contains("技术实现的疑问"));
    assert_eq!(second_comment.reply_list.replies.len(), 1);
}

#[rstest]
#[tokio::test]
async fn test_update_comment_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let file_token = "doccnBKgoMyY5OMbUG6FzxAnnNf";
    let comment_id = "7101214603932246020";
    
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "comment_id": comment_id,
            "user_id": "ou_123456789abcdef",
            "create_time": 1674552412,
            "update_time": 1674565012,
            "is_solved": false,
            "content": "这是更新后的评论内容，增加了更多的细节和建议。",
            "extra": {
                "image_list": ["https://example.com/updated_image.png"]
            }
        }
    });

    Mock::given(method("PUT"))
        .and(path("/open-apis/drive/v1/comments"))
        .and(query_param("file_token", file_token))
        .and(query_param("file_type", "docx"))
        .and(query_param("comment_id", comment_id))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let comments_service = CommentsService::new(config);
    let request = UpdateCommentRequest::builder()
        .file_token(file_token)
        .file_type("docx")
        .comment_id(comment_id)
        .content("这是更新后的评论内容，增加了更多的细节和建议。")
        .build();

    let result = comments_service.update(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.comment_id, comment_id);
    assert_eq!(data.update_time, 1674565012);
    assert!(data.content.contains("更新后的评论内容"));
    assert!(data.content.contains("更多的细节"));
}

#[rstest]
#[tokio::test]
async fn test_patch_comment_status_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let file_token = "doccnBKgoMyY5OMbUG6FzxAnnNf";
    let comment_id = "7101214603932246020";
    
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "comment_id": comment_id,
            "user_id": "ou_123456789abcdef",
            "create_time": 1674552412,
            "update_time": 1674565012,
            "is_solved": true,
            "solved_time": 1674565012,
            "solver_user_id": "ou_987654321fedcba"
        }
    });

    Mock::given(method("PATCH"))
        .and(path("/open-apis/drive/v1/comments"))
        .and(query_param("file_token", file_token))
        .and(query_param("file_type", "docx"))
        .and(query_param("comment_id", comment_id))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let comments_service = CommentsService::new(config);
    let request = PatchCommentRequest::builder()
        .file_token(file_token)
        .file_type("docx")
        .comment_id(comment_id)
        .is_solved(true)
        .build();

    let result = comments_service.patch(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.comment_id, comment_id);
    assert_eq!(data.is_solved, true);
    assert_eq!(data.solved_time, 1674565012);
    assert_eq!(data.solver_user_id, "ou_987654321fedcba");
}

#[rstest]
#[tokio::test]
async fn test_delete_comment_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let file_token = "doccnBKgoMyY5OMbUG6FzxAnnNf";
    let comment_id = "7101214603932246020";
    
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {}
    });

    Mock::given(method("DELETE"))
        .and(path("/open-apis/drive/v1/comments"))
        .and(query_param("file_token", file_token))
        .and(query_param("file_type", "docx"))
        .and(query_param("comment_id", comment_id))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let comments_service = CommentsService::new(config);
    let request = DeleteCommentRequest::builder()
        .file_token(file_token)
        .file_type("docx")
        .comment_id(comment_id)
        .build();

    let result = comments_service.delete(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.code, 0);
}

// ===================== 回复管理测试 =====================

#[rstest]
#[tokio::test]
async fn test_create_comment_reply_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let file_token = "doccnBKgoMyY5OMbUG6FzxAnnNf";
    let comment_id = "7101214603932246020";
    
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "reply_id": "7101214603932246025",
            "user_id": "ou_987654321fedcba",
            "create_time": 1674558012,
            "update_time": 1674558012,
            "content": "谢谢您的建议，我们会仔细评估这个功能的可行性。",
            "extra": {}
        }
    });

    Mock::given(method("POST"))
        .and(path("/open-apis/drive/v1/comments/reply"))
        .and(query_param("file_token", file_token))
        .and(query_param("file_type", "docx"))
        .and(query_param("comment_id", comment_id))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let comments_service = CommentsService::new(config);
    let request = CreateCommentReplyRequest::builder()
        .file_token(file_token)
        .file_type("docx")
        .comment_id(comment_id)
        .content("谢谢您的建议，我们会仔细评估这个功能的可行性。")
        .build();

    let result = comments_service.create_reply(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.reply_id, "7101214603932246025");
    assert_eq!(data.user_id, "ou_987654321fedcba");
    assert!(data.content.contains("谢谢您的建议"));
    assert!(data.content.contains("评估这个功能"));
}

#[rstest]
#[tokio::test]
async fn test_list_comment_replies_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let file_token = "doccnBKgoMyY5OMbUG6FzxAnnNf";
    let comment_id = "7101214603932246020";
    
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "replies": [
                {
                    "reply_id": "7101214603932246025",
                    "user_id": "ou_987654321fedcba",
                    "create_time": 1674558012,
                    "update_time": 1674558012,
                    "content": "第一条回复：感谢反馈",
                    "extra": {}
                },
                {
                    "reply_id": "7101214603932246026",
                    "user_id": "ou_111222333444555",
                    "create_time": 1674560012,
                    "update_time": 1674560012,
                    "content": "第二条回复：问题已记录",
                    "extra": {
                        "image_list": ["https://example.com/reply_image.png"]
                    }
                }
            ],
            "has_more": false,
            "page_token": ""
        }
    });

    Mock::given(method("GET"))
        .and(path("/open-apis/drive/v1/comments/reply"))
        .and(query_param("file_token", file_token))
        .and(query_param("file_type", "docx"))
        .and(query_param("comment_id", comment_id))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let comments_service = CommentsService::new(config);
    let request = ListCommentRepliesRequest::builder()
        .file_token(file_token)
        .file_type("docx")
        .comment_id(comment_id)
        .page_size(20)
        .build();

    let result = comments_service.list_replies(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.replies.len(), 2);
    assert_eq!(data.has_more, false);
    
    let first_reply = &data.replies[0];
    assert_eq!(first_reply.reply_id, "7101214603932246025");
    assert!(first_reply.content.contains("感谢反馈"));
    
    let second_reply = &data.replies[1];
    assert_eq!(second_reply.reply_id, "7101214603932246026");
    assert!(second_reply.content.contains("问题已记录"));
}

#[rstest]
#[tokio::test]
async fn test_update_comment_reply_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let file_token = "doccnBKgoMyY5OMbUG6FzxAnnNf";
    let comment_id = "7101214603932246020";
    let reply_id = "7101214603932246025";
    
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "reply_id": reply_id,
            "user_id": "ou_987654321fedcba",
            "create_time": 1674558012,
            "update_time": 1674568012,
            "content": "更新后的回复：我们已经开始着手这个功能的开发工作。",
            "extra": {}
        }
    });

    Mock::given(method("PUT"))
        .and(path("/open-apis/drive/v1/comments/reply"))
        .and(query_param("file_token", file_token))
        .and(query_param("file_type", "docx"))
        .and(query_param("comment_id", comment_id))
        .and(query_param("reply_id", reply_id))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let comments_service = CommentsService::new(config);
    let request = UpdateCommentReplyRequest::builder()
        .file_token(file_token)
        .file_type("docx")
        .comment_id(comment_id)
        .reply_id(reply_id)
        .content("更新后的回复：我们已经开始着手这个功能的开发工作。")
        .build();

    let result = comments_service.update_reply(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.reply_id, reply_id);
    assert_eq!(data.update_time, 1674568012);
    assert!(data.content.contains("更新后的回复"));
    assert!(data.content.contains("开始着手"));
}

#[rstest]
#[tokio::test]
async fn test_delete_comment_reply_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let file_token = "doccnBKgoMyY5OMbUG6FzxAnnNf";
    let comment_id = "7101214603932246020";
    let reply_id = "7101214603932246025";
    
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {}
    });

    Mock::given(method("DELETE"))
        .and(path("/open-apis/drive/v1/comments/reply"))
        .and(query_param("file_token", file_token))
        .and(query_param("file_type", "docx"))
        .and(query_param("comment_id", comment_id))
        .and(query_param("reply_id", reply_id))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let comments_service = CommentsService::new(config);
    let request = DeleteCommentReplyRequest::builder()
        .file_token(file_token)
        .file_type("docx")
        .comment_id(comment_id)
        .reply_id(reply_id)
        .build();

    let result = comments_service.delete_reply(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.code, 0);
}

// ===================== 批量操作测试 =====================

#[rstest]
#[tokio::test]
async fn test_batch_query_comments_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let file_token = "doccnBKgoMyY5OMbUG6FzxAnnNf";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "items": [
                {
                    "comment_id": "7101214603932246020",
                    "user_id": "ou_123456789abcdef",
                    "create_time": 1674552412,
                    "is_solved": false,
                    "content": "批量查询：第一条评论",
                    "reply_list": {
                        "replies": [],
                        "has_more": false
                    }
                },
                {
                    "comment_id": "7101214603932246021",
                    "user_id": "ou_987654321fedcba",
                    "create_time": 1674555012,
                    "is_solved": true,
                    "content": "批量查询：第二条评论",
                    "reply_list": {
                        "replies": [
                            {
                                "reply_id": "7101214603932246030",
                                "user_id": "ou_111222333444555",
                                "content": "批量查询的回复"
                            }
                        ],
                        "has_more": false
                    }
                }
            ],
            "has_more": true,
            "page_token": "next_page_token_12345"
        }
    });

    Mock::given(method("GET"))
        .and(path("/open-apis/drive/v1/comments/batch_query"))
        .and(query_param("file_token", file_token))
        .and(query_param("file_type", "docx"))
        .and(query_param("is_solved", "false"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let comments_service = CommentsService::new(config);
    let request = BatchQueryCommentsRequest::builder()
        .file_token(file_token)
        .file_type("docx")
        .is_solved(false) // 只查询未解决的评论
        .page_size(50)
        .build();

    let result = comments_service.batch_query(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.items.len(), 2);
    assert_eq!(data.has_more, true);
    assert_eq!(data.page_token, Some("next_page_token_12345".to_string()));
    
    // 验证第一条评论
    let first_comment = &data.items[0];
    assert_eq!(first_comment.is_solved, false);
    assert!(first_comment.content.contains("第一条评论"));
    
    // 验证第二条评论（虽然查询条件是未解决，但API可能返回已解决的）
    let second_comment = &data.items[1];
    assert!(second_comment.content.contains("第二条评论"));
    assert_eq!(second_comment.reply_list.replies.len(), 1);
}

// ===================== 请求构建器测试 =====================

#[test]
fn test_create_comment_request_builder() {
    let request = CreateCommentRequest::builder()
        .file_token("doc123")
        .file_type("docx")
        .obj_type("doc")
        .content("测试评论内容")
        .quote("引用内容")
        .build();

    assert_eq!(request.file_token, "doc123");
    assert_eq!(request.file_type, "docx");
    assert_eq!(request.obj_type, "doc");
    assert_eq!(request.content, "测试评论内容");
    assert_eq!(request.quote, Some("引用内容".to_string()));
}

#[test]
fn test_update_comment_request_builder() {
    let request = UpdateCommentRequest::builder()
        .file_token("doc123")
        .file_type("docx")
        .comment_id("comment456")
        .content("更新后的评论内容")
        .build();

    assert_eq!(request.file_token, "doc123");
    assert_eq!(request.file_type, "docx");
    assert_eq!(request.comment_id, "comment456");
    assert_eq!(request.content, "更新后的评论内容");
}

#[test]
fn test_patch_comment_request_builder() {
    let request = PatchCommentRequest::builder()
        .file_token("doc123")
        .file_type("docx")
        .comment_id("comment456")
        .is_solved(true)
        .build();

    assert_eq!(request.file_token, "doc123");
    assert_eq!(request.comment_id, "comment456");
    assert_eq!(request.is_solved, true);
}

#[test]
fn test_create_comment_reply_request_builder() {
    let request = CreateCommentReplyRequest::builder()
        .file_token("doc123")
        .file_type("docx")
        .comment_id("comment456")
        .content("回复内容")
        .build();

    assert_eq!(request.file_token, "doc123");
    assert_eq!(request.file_type, "docx");
    assert_eq!(request.comment_id, "comment456");
    assert_eq!(request.content, "回复内容");
}

#[test]
fn test_batch_query_comments_request_builder() {
    let request = BatchQueryCommentsRequest::builder()
        .file_token("doc123")
        .file_type("docx")
        .is_solved(false)
        .is_whole(true)
        .page_size(30)
        .page_token("token123")
        .build();

    assert_eq!(request.file_token, "doc123");
    assert_eq!(request.file_type, "docx");
    assert_eq!(request.is_solved, Some(false));
    assert_eq!(request.is_whole, Some(true));
    assert_eq!(request.page_size, Some(30));
    assert_eq!(request.page_token, Some("token123".to_string()));
}

// ===================== 属性测试 =====================

proptest! {
    #[test]
    fn test_comment_content_invariants(
        content in "[\\s\\S]{1,5000}"
    ) {
        let request = CreateCommentRequest::builder()
            .file_token("doc123")
            .file_type("docx")
            .obj_type("doc")
            .content(&content)
            .build();

        assert_eq!(request.content, content);
        assert!(content.len() >= 1 && content.len() <= 5000);
    }
}

proptest! {
    #[test]
    fn test_file_token_invariants(
        file_token in prop::string::string_regex("doccn[a-zA-Z0-9]{16,32}").unwrap()
    ) {
        let request = GetCommentRequest::builder()
            .file_token(&file_token)
            .file_type("docx")
            .comment_id("comment123")
            .build();

        assert_eq!(request.file_token, file_token);
        assert!(file_token.starts_with("doccn"));
    }
}

proptest! {
    #[test]
    fn test_comment_status_invariants(
        is_solved in prop::bool::ANY
    ) {
        let request = PatchCommentRequest::builder()
            .file_token("doc123")
            .file_type("docx")
            .comment_id("comment456")
            .is_solved(is_solved)
            .build();

        assert_eq!(request.is_solved, is_solved);
    }
}

// ===================== 错误处理测试 =====================

#[rstest]
#[tokio::test]
async fn test_comment_not_found_error(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let file_token = "doccnBKgoMyY5OMbUG6FzxAnnNf";
    let invalid_comment_id = "invalid_comment_id";
    
    let mock_response = json!({
        "code": 1248030,
        "msg": "评论不存在"
    });

    Mock::given(method("GET"))
        .and(path("/open-apis/drive/v1/comments"))
        .and(query_param("file_token", file_token))
        .and(query_param("comment_id", invalid_comment_id))
        .respond_with(ResponseTemplate::new(404).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let comments_service = CommentsService::new(config);
    let request = GetCommentRequest::builder()
        .file_token(file_token)
        .file_type("docx")
        .comment_id(invalid_comment_id)
        .build();

    let result = comments_service.get(request, None).await;
    
    if result.is_ok() {
        let response = result.unwrap();
        assert_ne!(response.code, 0);
        assert!(response.msg.contains("评论") || response.msg.contains("不存在"));
    }
}

#[rstest]
#[tokio::test]
async fn test_permission_denied_error(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let file_token = "doccnNoPermission";
    let mock_response = json!({
        "code": 1248006,
        "msg": "无权限访问该文档的评论"
    });

    Mock::given(method("GET"))
        .and(path("/open-apis/drive/v1/comments/batch_query"))
        .and(query_param("file_token", file_token))
        .respond_with(ResponseTemplate::new(403).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let comments_service = CommentsService::new(config);
    let request = BatchQueryCommentsRequest::builder()
        .file_token(file_token)
        .file_type("docx")
        .build();

    let result = comments_service.batch_query(request, None).await;
    
    if result.is_ok() {
        let response = result.unwrap();
        assert_eq!(response.code, 1248006);
        assert!(response.msg.contains("权限"));
    }
}

#[rstest]
#[tokio::test]
async fn test_comment_content_too_long_error(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let file_token = "doccnBKgoMyY5OMbUG6FzxAnnNf";
    let mock_response = json!({
        "code": 1248035,
        "msg": "评论内容过长"
    });

    Mock::given(method("POST"))
        .and(path("/open-apis/drive/v1/comments"))
        .and(query_param("file_token", file_token))
        .respond_with(ResponseTemplate::new(400).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let comments_service = CommentsService::new(config);
    let long_content = "x".repeat(10000); // 假设超出长度限制
    
    let request = CreateCommentRequest::builder()
        .file_token(file_token)
        .file_type("docx")
        .obj_type("doc")
        .content(&long_content)
        .build();

    let result = comments_service.create(request, None).await;
    
    if result.is_ok() {
        let response = result.unwrap();
        assert_ne!(response.code, 0);
        assert!(response.msg.contains("内容") || response.msg.contains("过长"));
    }
}

// ===================== 集成场景测试 =====================

#[rstest]
#[tokio::test]
async fn test_comment_workflow_integration(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let file_token = "doccnWorkflowTest";

    // 1. 创建评论
    let create_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "comment_id": "comment_workflow_test",
            "user_id": "ou_creator",
            "create_time": 1674552412,
            "is_solved": false,
            "content": "工作流测试评论"
        }
    });

    Mock::given(method("POST"))
        .and(path("/open-apis/drive/v1/comments"))
        .and(query_param("file_token", file_token))
        .respond_with(ResponseTemplate::new(200).set_body_json(create_response))
        .mount(&mock_server)
        .await;

    // 2. 添加回复
    let reply_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "reply_id": "reply_workflow_test",
            "user_id": "ou_responder",
            "create_time": 1674555012,
            "content": "工作流测试回复"
        }
    });

    Mock::given(method("POST"))
        .and(path("/open-apis/drive/v1/comments/reply"))
        .and(query_param("file_token", file_token))
        .and(query_param("comment_id", "comment_workflow_test"))
        .respond_with(ResponseTemplate::new(200).set_body_json(reply_response))
        .mount(&mock_server)
        .await;

    // 3. 获取评论详情（包含回复）
    let get_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "comment_id": "comment_workflow_test",
            "user_id": "ou_creator",
            "create_time": 1674552412,
            "is_solved": false,
            "content": "工作流测试评论",
            "reply_list": {
                "replies": [
                    {
                        "reply_id": "reply_workflow_test",
                        "user_id": "ou_responder",
                        "create_time": 1674555012,
                        "content": "工作流测试回复"
                    }
                ],
                "has_more": false
            }
        }
    });

    Mock::given(method("GET"))
        .and(path("/open-apis/drive/v1/comments"))
        .and(query_param("file_token", file_token))
        .and(query_param("comment_id", "comment_workflow_test"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_response))
        .mount(&mock_server)
        .await;

    // 4. 标记为已解决
    let patch_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "comment_id": "comment_workflow_test",
            "is_solved": true,
            "solved_time": 1674560012,
            "solver_user_id": "ou_resolver"
        }
    });

    Mock::given(method("PATCH"))
        .and(path("/open-apis/drive/v1/comments"))
        .and(query_param("file_token", file_token))
        .and(query_param("comment_id", "comment_workflow_test"))
        .respond_with(ResponseTemplate::new(200).set_body_json(patch_response))
        .mount(&mock_server)
        .await;

    // 执行完整的评论工作流程
    let comments_service = CommentsService::new(config);

    // 1. 创建评论
    let create_req = CreateCommentRequest::builder()
        .file_token(file_token)
        .file_type("docx")
        .obj_type("doc")
        .content("工作流测试评论")
        .build();

    let create_result = comments_service.create(create_req, None).await;
    assert!(create_result.is_ok());
    let create_data = create_result.unwrap().data.unwrap();
    let comment_id = create_data.comment_id;
    assert_eq!(comment_id, "comment_workflow_test");
    assert_eq!(create_data.is_solved, false);

    // 2. 添加回复
    let reply_req = CreateCommentReplyRequest::builder()
        .file_token(file_token)
        .file_type("docx")
        .comment_id(&comment_id)
        .content("工作流测试回复")
        .build();

    let reply_result = comments_service.create_reply(reply_req, None).await;
    assert!(reply_result.is_ok());
    let reply_data = reply_result.unwrap().data.unwrap();
    assert_eq!(reply_data.reply_id, "reply_workflow_test");
    assert!(reply_data.content.contains("工作流测试回复"));

    // 3. 获取评论详情验证
    let get_req = GetCommentRequest::builder()
        .file_token(file_token)
        .file_type("docx")
        .comment_id(&comment_id)
        .build();

    let get_result = comments_service.get(get_req, None).await;
    assert!(get_result.is_ok());
    let get_data = get_result.unwrap().data.unwrap();
    assert_eq!(get_data.reply_list.replies.len(), 1);
    assert_eq!(get_data.reply_list.replies[0].reply_id, "reply_workflow_test");

    // 4. 标记为已解决
    let patch_req = PatchCommentRequest::builder()
        .file_token(file_token)
        .file_type("docx")
        .comment_id(&comment_id)
        .is_solved(true)
        .build();

    let patch_result = comments_service.patch(patch_req, None).await;
    assert!(patch_result.is_ok());
    let patch_data = patch_result.unwrap().data.unwrap();
    assert_eq!(patch_data.is_solved, true);
    assert_eq!(patch_data.solver_user_id, "ou_resolver");
}

#[cfg(test)]
mod test_utils {
    use super::*;

    /// 创建测试用的评论
    pub fn create_test_comment(
        comment_id: &str,
        user_id: &str,
        content: &str,
        is_solved: bool,
    ) -> Comment {
        Comment {
            comment_id: comment_id.to_string(),
            user_id: user_id.to_string(),
            create_time: 1674552412,
            update_time: 1674552412,
            is_solved,
            solved_time: if is_solved { 1674560012 } else { 0 },
            solver_user_id: if is_solved {
                "ou_solver".to_string()
            } else {
                "".to_string()
            },
            content: content.to_string(),
            extra: None,
            quote: "".to_string(),
            reply_list: ReplyList {
                replies: vec![],
                has_more: false,
                page_token: "".to_string(),
            },
        }
    }

    /// 创建测试用的回复
    pub fn create_test_reply(reply_id: &str, user_id: &str, content: &str) -> Reply {
        Reply {
            reply_id: reply_id.to_string(),
            user_id: user_id.to_string(),
            create_time: 1674555012,
            update_time: 1674555012,
            content: content.to_string(),
            extra: None,
        }
    }

    /// 验证评论状态
    pub fn assert_comment_status(comment: &Comment, expected_solved: bool) {
        assert_eq!(comment.is_solved, expected_solved);
        if expected_solved {
            assert!(comment.solved_time > 0);
            assert!(!comment.solver_user_id.is_empty());
        } else {
            assert_eq!(comment.solved_time, 0);
            assert!(comment.solver_user_id.is_empty());
        }
    }

    /// 统计评论和回复总数
    pub fn count_total_comments_and_replies(comments: &[Comment]) -> (usize, usize) {
        let comment_count = comments.len();
        let reply_count: usize = comments
            .iter()
            .map(|c| c.reply_list.replies.len())
            .sum();
        (comment_count, reply_count)
    }
}