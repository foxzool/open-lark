//! Wiki 服务单元测试
//!
//! 本模块测试飞书知识库 Wiki API 的各种功能，包括：
//! - 知识空间管理
//! - 节点创建和组织
//! - 内容编辑和版本管理
//! - 权限和成员管理
//! - 空间设置配置

use wiremock::{
    matchers::{method, path, path_regex, query_param},
    Mock, MockServer, ResponseTemplate,
};
use serde_json::json;
use rstest::{fixture, rstest};
use proptest::prelude::*;

use open_lark::{
    core::{config::Config, constants::AccessTokenType, req_option::RequestOption},
    service::cloud_docs::wiki::{
        v2::{
            space::{
                SpaceService,
                create::{CreateSpaceRequest, CreateSpaceResponseData, Space},
                get::{GetSpaceRequest, GetSpaceResponseData},
                list::{ListSpaceRequest, ListSpaceResponseData},
            },
            space_node::{
                SpaceNodeService,
                create::{CreateSpaceNodeRequest, CreateSpaceNodeResponseData, SpaceNode},
                get::{GetSpaceNodeRequest, GetSpaceNodeResponseData},
                list::{ListSpaceNodeRequest, ListSpaceNodeResponseData},
                update_title::{UpdateSpaceNodeTitleRequest, UpdateSpaceNodeTitleResponseData},
            },
            space_member::{
                SpaceMemberService,
                create::{CreateSpaceMemberRequest, CreateSpaceMemberResponseData},
                list::{ListSpaceMemberRequest, ListSpaceMemberResponseData},
                delete::{DeleteSpaceMemberRequest, DeleteSpaceMemberResponseData},
            },
            space_setting::{
                SpaceSettingService,
                update::{UpdateSpaceSettingRequest, UpdateSpaceSettingResponseData},
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

// ===================== 知识空间管理测试 =====================

#[rstest]
#[tokio::test]
async fn test_create_space_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "space": {
                "space_id": "7101214603932246020",
                "name": "产品设计知识库",
                "description": "包含所有产品设计相关文档和规范",
                "space_type": "team",
                "visibility": "public"
            }
        }
    });

    Mock::given(method("POST"))
        .and(path("/open-apis/wiki/v2/spaces"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let space_service = SpaceService::new(config);
    let request = CreateSpaceRequest::builder()
        .name("产品设计知识库")
        .description("包含所有产品设计相关文档和规范")
        .space_type("team")
        .visibility("public")
        .build();

    let result = space_service.create(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.code, 0);
    assert_eq!(response.msg, "success");
    
    let data = response.data.unwrap();
    assert_eq!(data.space.space_id, "7101214603932246020");
    assert_eq!(data.space.name, "产品设计知识库");
    assert_eq!(data.space.description, Some("包含所有产品设计相关文档和规范".to_string()));
    assert_eq!(data.space.space_type, "team");
    assert_eq!(data.space.visibility, "public");
}

#[rstest]
#[tokio::test]
async fn test_get_space_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let space_id = "7101214603932246020";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "space": {
                "space_id": space_id,
                "name": "现有知识库",
                "description": "现有的知识库空间",
                "space_type": "personal",
                "visibility": "private",
                "owner_id": "ou_123456789"
            }
        }
    });

    Mock::given(method("GET"))
        .and(path(format!("/open-apis/wiki/v2/spaces/{}", space_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let space_service = SpaceService::new(config);
    let request = GetSpaceRequest::new(space_id);

    let result = space_service.get(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    assert_eq!(data.space.space_id, space_id);
    assert_eq!(data.space.name, "现有知识库");
    assert_eq!(data.space.space_type, "personal");
    assert_eq!(data.space.visibility, "private");
}

#[rstest]
#[tokio::test]
async fn test_list_spaces_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "items": [
                {
                    "space_id": "7101214603932246020",
                    "name": "技术文档",
                    "description": "团队技术文档库",
                    "space_type": "team",
                    "visibility": "public"
                },
                {
                    "space_id": "7101214603932246021",
                    "name": "个人笔记",
                    "description": "我的个人知识整理",
                    "space_type": "personal", 
                    "visibility": "private"
                }
            ],
            "has_more": false,
            "page_token": null
        }
    });

    Mock::given(method("GET"))
        .and(path("/open-apis/wiki/v2/spaces"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let space_service = SpaceService::new(config);
    let request = ListSpaceRequest::builder()
        .page_size(10)
        .build();

    let result = space_service.list(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.items.len(), 2);
    assert_eq!(data.has_more, false);
    
    let first_space = &data.items[0];
    assert_eq!(first_space.space_id, "7101214603932246020");
    assert_eq!(first_space.name, "技术文档");
    assert_eq!(first_space.space_type, "team");
    
    let second_space = &data.items[1];
    assert_eq!(second_space.space_id, "7101214603932246021");
    assert_eq!(second_space.name, "个人笔记");
    assert_eq!(second_space.space_type, "personal");
}

// ===================== 节点管理测试 =====================

#[rstest]
#[tokio::test]
async fn test_create_space_node_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let space_id = "7101214603932246020";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "node": {
                "space_id": space_id,
                "node_token": "wikcn123456789abcdef",
                "obj_token": "doccn123456789abcdef",
                "obj_type": "doc",
                "parent_node_token": "",
                "node_type": "origin",
                "origin_node_token": "",
                "origin_space_id": "",
                "has_child": false,
                "title": "API设计规范",
                "obj_create_time": "1674552412000",
                "obj_edit_time": "1674552412000",
                "node_create_time": "1674552412000",
                "creator": "ou_123456789",
                "owner": "ou_123456789"
            }
        }
    });

    Mock::given(method("POST"))
        .and(path(format!("/open-apis/wiki/v2/spaces/{}/nodes", space_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let node_service = SpaceNodeService::new(config);
    let request = CreateSpaceNodeRequest::builder()
        .space_id(space_id)
        .obj_type("doc")
        .title("API设计规范")
        .build();

    let result = node_service.create(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.node.space_id, space_id);
    assert_eq!(data.node.node_token, "wikcn123456789abcdef");
    assert_eq!(data.node.obj_type, "doc");
    assert_eq!(data.node.title, "API设计规范");
    assert_eq!(data.node.has_child, false);
    assert_eq!(data.node.node_type, "origin");
}

#[rstest]
#[tokio::test]
async fn test_list_space_nodes_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let space_id = "7101214603932246020";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "items": [
                {
                    "space_id": space_id,
                    "node_token": "wikcn123456789abcdef",
                    "obj_token": "doccn123456789abcdef",
                    "obj_type": "doc",
                    "parent_node_token": "",
                    "node_type": "origin",
                    "has_child": true,
                    "title": "开发文档目录",
                    "obj_create_time": "1674552412000",
                    "creator": "ou_123456789"
                },
                {
                    "space_id": space_id,
                    "node_token": "wikcn987654321fedcba",
                    "obj_token": "doccn987654321fedcba", 
                    "obj_type": "docx",
                    "parent_node_token": "wikcn123456789abcdef",
                    "node_type": "origin",
                    "has_child": false,
                    "title": "接口文档v1.0",
                    "obj_create_time": "1674552500000",
                    "creator": "ou_987654321"
                }
            ],
            "has_more": false,
            "page_token": null
        }
    });

    Mock::given(method("GET"))
        .and(path(format!("/open-apis/wiki/v2/spaces/{}/nodes", space_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let node_service = SpaceNodeService::new(config);
    let request = ListSpaceNodeRequest::builder()
        .space_id(space_id)
        .page_size(50)
        .build();

    let result = node_service.list(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.items.len(), 2);
    assert_eq!(data.has_more, false);
    
    let first_node = &data.items[0];
    assert_eq!(first_node.node_token, "wikcn123456789abcdef");
    assert_eq!(first_node.obj_type, "doc");
    assert_eq!(first_node.title, "开发文档目录");
    assert_eq!(first_node.has_child, true);
    assert_eq!(first_node.parent_node_token, "");
    
    let second_node = &data.items[1];
    assert_eq!(second_node.node_token, "wikcn987654321fedcba");
    assert_eq!(second_node.obj_type, "docx");
    assert_eq!(second_node.title, "接口文档v1.0");
    assert_eq!(second_node.has_child, false);
    assert_eq!(second_node.parent_node_token, "wikcn123456789abcdef");
}

#[rstest]
#[tokio::test]
async fn test_update_space_node_title_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let space_id = "7101214603932246020";
    let node_token = "wikcn123456789abcdef";
    
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "node": {
                "space_id": space_id,
                "node_token": node_token,
                "title": "更新后的标题"
            }
        }
    });

    Mock::given(method("PUT"))
        .and(path(format!("/open-apis/wiki/v2/spaces/{}/nodes/{}/update_title", space_id, node_token)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let node_service = SpaceNodeService::new(config);
    let request = UpdateSpaceNodeTitleRequest::builder()
        .space_id(space_id)
        .node_token(node_token)
        .title("更新后的标题")
        .build();

    let result = node_service.update_title(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.node.space_id, space_id);
    assert_eq!(data.node.node_token, node_token);
    assert_eq!(data.node.title, "更新后的标题");
}

// ===================== 成员管理测试 =====================

#[rstest]
#[tokio::test]
async fn test_create_space_member_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let space_id = "7101214603932246020";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "member": {
                "member_type": "user",
                "member_id": "ou_123456789abcdef",
                "role": "admin",
                "type": "user"
            }
        }
    });

    Mock::given(method("POST"))
        .and(path(format!("/open-apis/wiki/v2/spaces/{}/members", space_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let member_service = SpaceMemberService::new(config);
    let request = CreateSpaceMemberRequest::builder()
        .space_id(space_id)
        .member_type("user")
        .member_id("ou_123456789abcdef")
        .role("admin")
        .build();

    let result = member_service.create(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.member.member_type, "user");
    assert_eq!(data.member.member_id, "ou_123456789abcdef");
    assert_eq!(data.member.role, "admin");
}

#[rstest]
#[tokio::test]
async fn test_list_space_members_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let space_id = "7101214603932246020";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "items": [
                {
                    "member_type": "user",
                    "member_id": "ou_123456789abcdef",
                    "role": "admin",
                    "type": "user"
                },
                {
                    "member_type": "user",
                    "member_id": "ou_987654321fedcba",
                    "role": "editor",
                    "type": "user"
                },
                {
                    "member_type": "user",
                    "member_id": "ou_111222333444555",
                    "role": "reader",
                    "type": "user"
                }
            ],
            "has_more": false,
            "page_token": null
        }
    });

    Mock::given(method("GET"))
        .and(path(format!("/open-apis/wiki/v2/spaces/{}/members", space_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let member_service = SpaceMemberService::new(config);
    let request = ListSpaceMemberRequest::builder()
        .space_id(space_id)
        .page_size(20)
        .build();

    let result = member_service.list(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.items.len(), 3);
    assert_eq!(data.has_more, false);
    
    // 验证管理员成员
    let admin_member = &data.items[0];
    assert_eq!(admin_member.member_id, "ou_123456789abcdef");
    assert_eq!(admin_member.role, "admin");
    
    // 验证编辑者成员
    let editor_member = &data.items[1];
    assert_eq!(editor_member.member_id, "ou_987654321fedcba");
    assert_eq!(editor_member.role, "editor");
    
    // 验证只读成员
    let reader_member = &data.items[2];
    assert_eq!(reader_member.member_id, "ou_111222333444555");
    assert_eq!(reader_member.role, "reader");
}

#[rstest]
#[tokio::test]
async fn test_delete_space_member_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let space_id = "7101214603932246020";
    let member_id = "ou_987654321fedcba";
    
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "member": {
                "member_type": "user",
                "member_id": member_id,
                "role": "editor",
                "type": "user"
            }
        }
    });

    Mock::given(method("DELETE"))
        .and(path(format!("/open-apis/wiki/v2/spaces/{}/members/{}", space_id, member_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let member_service = SpaceMemberService::new(config);
    let request = DeleteSpaceMemberRequest::builder()
        .space_id(space_id)
        .member_id(member_id)
        .member_type("user")
        .build();

    let result = member_service.delete(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.member.member_id, member_id);
    assert_eq!(data.member.member_type, "user");
    assert_eq!(data.member.role, "editor");
}

// ===================== 空间设置测试 =====================

#[rstest]
#[tokio::test]
async fn test_update_space_setting_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let space_id = "7101214603932246020";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "setting": {
                "create_setting": "admin_and_member",
                "security_setting": "anyone_editable",
                "comment_setting": "anyone_can_see_and_comment"
            }
        }
    });

    Mock::given(method("PUT"))
        .and(path(format!("/open-apis/wiki/v2/spaces/{}/setting", space_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let setting_service = SpaceSettingService::new(config);
    let request = UpdateSpaceSettingRequest::builder()
        .space_id(space_id)
        .create_setting("admin_and_member")
        .security_setting("anyone_editable")
        .comment_setting("anyone_can_see_and_comment")
        .build();

    let result = setting_service.update(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.setting.create_setting, "admin_and_member");
    assert_eq!(data.setting.security_setting, "anyone_editable");
    assert_eq!(data.setting.comment_setting, "anyone_can_see_and_comment");
}

// ===================== 请求构建器测试 =====================

#[test]
fn test_create_space_request_builder() {
    let request = CreateSpaceRequest::builder()
        .name("测试知识库")
        .description("用于单元测试的知识库")
        .space_type("team")
        .visibility("public")
        .build();

    assert_eq!(request.name, "测试知识库");
    assert_eq!(request.description, Some("用于单元测试的知识库".to_string()));
    assert_eq!(request.space_type, Some("team".to_string()));
    assert_eq!(request.visibility, Some("public".to_string()));
}

#[test]
fn test_create_space_node_request_builder() {
    let request = CreateSpaceNodeRequest::builder()
        .space_id("space123")
        .obj_type("doc")
        .title("新建文档")
        .parent_node_token("parent_node")
        .build();

    assert_eq!(request.space_id, "space123");
    assert_eq!(request.obj_type, "doc");
    assert_eq!(request.title, "新建文档");
    assert_eq!(request.parent_node_token, Some("parent_node".to_string()));
}

#[test]
fn test_create_space_member_request_builder() {
    let request = CreateSpaceMemberRequest::builder()
        .space_id("space123")
        .member_type("user")
        .member_id("ou_user123")
        .role("editor")
        .build();

    assert_eq!(request.space_id, "space123");
    assert_eq!(request.member_type, "user");
    assert_eq!(request.member_id, "ou_user123");
    assert_eq!(request.role, "editor");
}

// ===================== 属性测试 =====================

proptest! {
    #[test]
    fn test_space_id_invariants(
        space_id in prop::string::string_regex("[0-9]{15,20}").unwrap()
    ) {
        let request = GetSpaceRequest::new(&space_id);
        assert_eq!(request.space_id, space_id);
    }
}

proptest! {
    #[test]
    fn test_space_node_invariants(
        space_id in prop::string::string_regex("[0-9]{15,20}").unwrap(),
        node_token in prop::string::string_regex("wikcn[a-zA-Z0-9]{16,32}").unwrap(),
        title in "[\\u4e00-\\u9fa5a-zA-Z0-9_\\s]{1,100}"
    ) {
        let request = UpdateSpaceNodeTitleRequest::builder()
            .space_id(&space_id)
            .node_token(&node_token)
            .title(&title)
            .build();

        assert_eq!(request.space_id, space_id);
        assert_eq!(request.node_token, node_token);
        assert_eq!(request.title, title);
    }
}

proptest! {
    #[test]
    fn test_member_role_invariants(
        role in prop::sample::select(vec!["admin", "editor", "reader"])
    ) {
        let request = CreateSpaceMemberRequest::builder()
            .space_id("space123")
            .member_type("user")
            .member_id("ou_member123")
            .role(&role)
            .build();

        assert_eq!(request.role, role);
        assert!(matches!(role.as_str(), "admin" | "editor" | "reader"));
    }
}

// ===================== 错误处理测试 =====================

#[rstest]
#[tokio::test]
async fn test_space_not_found_error(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let invalid_space_id = "999999999999999999";
    let mock_response = json!({
        "code": 1248010,
        "msg": "知识空间不存在"
    });

    Mock::given(method("GET"))
        .and(path(format!("/open-apis/wiki/v2/spaces/{}", invalid_space_id)))
        .respond_with(ResponseTemplate::new(404).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let space_service = SpaceService::new(config);
    let request = GetSpaceRequest::new(invalid_space_id);

    let result = space_service.get(request, None).await;
    
    if result.is_ok() {
        let response = result.unwrap();
        assert_ne!(response.code, 0);
        assert!(response.msg.contains("知识空间") || response.msg.contains("不存在"));
    }
}

#[rstest]
#[tokio::test]
async fn test_permission_denied_error(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let space_id = "7101214603932246020";
    let mock_response = json!({
        "code": 1248006,
        "msg": "无权限访问该知识空间"
    });

    Mock::given(method("GET"))
        .and(path(format!("/open-apis/wiki/v2/spaces/{}", space_id)))
        .respond_with(ResponseTemplate::new(403).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let space_service = SpaceService::new(config);
    let request = GetSpaceRequest::new(space_id);

    let result = space_service.get(request, None).await;
    
    if result.is_ok() {
        let response = result.unwrap();
        assert_eq!(response.code, 1248006);
        assert!(response.msg.contains("权限"));
    }
}

// ===================== 集成场景测试 =====================

#[rstest]
#[tokio::test]
async fn test_wiki_workflow_integration(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    // 1. 创建知识空间
    let create_space_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "space": {
                "space_id": "wiki_integration_test",
                "name": "集成测试知识库",
                "description": "用于测试完整流程",
                "space_type": "team",
                "visibility": "public"
            }
        }
    });

    Mock::given(method("POST"))
        .and(path("/open-apis/wiki/v2/spaces"))
        .respond_with(ResponseTemplate::new(200).set_body_json(create_space_response))
        .mount(&mock_server)
        .await;

    // 2. 创建根节点
    let create_node_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "node": {
                "space_id": "wiki_integration_test",
                "node_token": "wikcn_root_test",
                "obj_token": "doccn_root_test",
                "obj_type": "doc",
                "parent_node_token": "",
                "node_type": "origin",
                "has_child": false,
                "title": "项目文档根目录",
                "creator": "ou_test_creator"
            }
        }
    });

    Mock::given(method("POST"))
        .and(path("/open-apis/wiki/v2/spaces/wiki_integration_test/nodes"))
        .respond_with(ResponseTemplate::new(200).set_body_json(create_node_response))
        .mount(&mock_server)
        .await;

    // 3. 添加成员
    let create_member_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "member": {
                "member_type": "user",
                "member_id": "ou_test_member",
                "role": "editor",
                "type": "user"
            }
        }
    });

    Mock::given(method("POST"))
        .and(path("/open-apis/wiki/v2/spaces/wiki_integration_test/members"))
        .respond_with(ResponseTemplate::new(200).set_body_json(create_member_response))
        .mount(&mock_server)
        .await;

    // 4. 更新空间设置
    let update_setting_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "setting": {
                "create_setting": "admin_and_member",
                "security_setting": "anyone_editable",
                "comment_setting": "anyone_can_see_and_comment"
            }
        }
    });

    Mock::given(method("PUT"))
        .and(path("/open-apis/wiki/v2/spaces/wiki_integration_test/setting"))
        .respond_with(ResponseTemplate::new(200).set_body_json(update_setting_response))
        .mount(&mock_server)
        .await;

    // 执行集成测试流程
    let space_service = SpaceService::new(config.clone());
    let node_service = SpaceNodeService::new(config.clone());
    let member_service = SpaceMemberService::new(config.clone());
    let setting_service = SpaceSettingService::new(config);

    // 1. 创建知识空间
    let create_space_req = CreateSpaceRequest::builder()
        .name("集成测试知识库")
        .description("用于测试完整流程")
        .space_type("team")
        .visibility("public")
        .build();

    let space_result = space_service.create(create_space_req, None).await;
    assert!(space_result.is_ok());
    let space_data = space_result.unwrap().data.unwrap();
    let space_id = space_data.space.space_id;
    assert_eq!(space_id, "wiki_integration_test");

    // 2. 创建根节点
    let create_node_req = CreateSpaceNodeRequest::builder()
        .space_id(&space_id)
        .obj_type("doc")
        .title("项目文档根目录")
        .build();

    let node_result = node_service.create(create_node_req, None).await;
    assert!(node_result.is_ok());
    let node_data = node_result.unwrap().data.unwrap();
    assert_eq!(node_data.node.title, "项目文档根目录");
    assert_eq!(node_data.node.obj_type, "doc");

    // 3. 添加成员
    let create_member_req = CreateSpaceMemberRequest::builder()
        .space_id(&space_id)
        .member_type("user")
        .member_id("ou_test_member")
        .role("editor")
        .build();

    let member_result = member_service.create(create_member_req, None).await;
    assert!(member_result.is_ok());
    let member_data = member_result.unwrap().data.unwrap();
    assert_eq!(member_data.member.role, "editor");

    // 4. 更新空间设置
    let update_setting_req = UpdateSpaceSettingRequest::builder()
        .space_id(&space_id)
        .create_setting("admin_and_member")
        .security_setting("anyone_editable")
        .comment_setting("anyone_can_see_and_comment")
        .build();

    let setting_result = setting_service.update(update_setting_req, None).await;
    assert!(setting_result.is_ok());
    let setting_data = setting_result.unwrap().data.unwrap();
    assert_eq!(setting_data.setting.create_setting, "admin_and_member");
}

#[cfg(test)]
mod test_utils {
    use super::*;

    /// 创建测试用的空间
    pub fn create_test_space(name: &str, space_type: &str, visibility: &str) -> Space {
        Space {
            space_id: format!("test_space_{}", rand::random::<u32>()),
            name: name.to_string(),
            description: Some(format!("测试空间: {}", name)),
            space_type: space_type.to_string(),
            visibility: visibility.to_string(),
            owner_id: Some("ou_test_owner".to_string()),
        }
    }

    /// 创建测试用的节点
    pub fn create_test_space_node(
        space_id: &str,
        title: &str,
        obj_type: &str,
        parent_token: Option<&str>,
    ) -> SpaceNode {
        SpaceNode {
            space_id: space_id.to_string(),
            node_token: format!("wikcn_test_{}", rand::random::<u32>()),
            obj_token: format!("doccn_test_{}", rand::random::<u32>()),
            obj_type: obj_type.to_string(),
            parent_node_token: parent_token.unwrap_or("").to_string(),
            node_type: "origin".to_string(),
            origin_node_token: "".to_string(),
            origin_space_id: "".to_string(),
            has_child: false,
            title: title.to_string(),
            obj_create_time: "1674552412000".to_string(),
            obj_edit_time: "1674552412000".to_string(),
            node_create_time: "1674552412000".to_string(),
            creator: "ou_test_creator".to_string(),
            owner: "ou_test_owner".to_string(),
        }
    }

    /// 验证空间权限设置
    pub fn assert_space_permissions(
        create_setting: &str,
        security_setting: &str,
        comment_setting: &str,
    ) {
        assert!(matches!(
            create_setting,
            "anyone" | "admin_and_member" | "only_admin"
        ));
        assert!(matches!(
            security_setting,
            "anyone_editable" | "only_full_access" | "only_comment"
        ));
        assert!(matches!(
            comment_setting,
            "anyone_can_see_and_comment" | "only_full_access" | "no_comment"
        ));
    }
}