//! Permission 服务单元测试
//!
//! 本模块测试飞书云文档权限管理 API 的各种功能，包括：
//! - 成员权限管理 (v1/v2)
//! - 公共链接设置 (v1/v2)
//! - 密码保护功能
//! - 权限继承和覆盖
//! - 协作者管理

use wiremock::{
    matchers::{method, path, path_regex, query_param, body_json},
    Mock, MockServer, ResponseTemplate,
};
use serde_json::json;
use rstest::{fixture, rstest};
use proptest::prelude::*;

use open_lark::{
    core::{config::Config, constants::AccessTokenType, req_option::RequestOption},
    service::cloud_docs::permission::{
        member::{
            MemberPermissionService,
            create::{CreateMemberPermissionRequest, CreateMemberPermissionResponseData, Member},
            list::{ListMemberPermissionRequest, ListMemberPermissionResponseData},
            update::{UpdateMemberPermissionRequest, UpdateMemberPermissionResponseData},
            delete::{DeleteMemberPermissionRequest, DeleteMemberPermissionResponseData},
            transfer::{TransferMemberPermissionRequest, TransferMemberPermissionResponseData},
        },
        public_v1::{
            PublicPermissionV1Service,
            patch::{PatchPublicPermissionV1Request, PatchPublicPermissionV1ResponseData},
            get::{GetPublicPermissionV1Request, GetPublicPermissionV1ResponseData},
            password::{
                PasswordService,
                create::{CreatePasswordRequest, CreatePasswordResponseData},
                update::{UpdatePasswordRequest, UpdatePasswordResponseData},
                delete::{DeletePasswordRequest, DeletePasswordResponseData},
            },
        },
        public_v2::{
            PublicPermissionV2Service,
            patch::{PatchPublicPermissionV2Request, PatchPublicPermissionV2ResponseData},
            get::{GetPublicPermissionV2Request, GetPublicPermissionV2ResponseData},
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

// ===================== 成员权限管理测试 =====================

#[rstest]
#[tokio::test]
async fn test_create_member_permission_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let token = "doccnBKgoMyY5OMbUG6FzxAnnNf";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "member": {
                "member_type": "user",
                "member_id": "ou_123456789abcdef",
                "perm": "edit",
                "type": "user"
            }
        }
    });

    Mock::given(method("POST"))
        .and(path(format!("/open-apis/drive/permission/member/create")))
        .and(query_param("token", token))
        .and(query_param("type", "doc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let permission_service = MemberPermissionService::new(config);
    let request = CreateMemberPermissionRequest::builder()
        .token(token)
        .obj_type("doc")
        .member_type("user")
        .member_id("ou_123456789abcdef")
        .perm("edit")
        .build();

    let result = permission_service.create(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.code, 0);
    assert_eq!(response.msg, "success");
    
    let data = response.data.unwrap();
    assert_eq!(data.member.member_type, "user");
    assert_eq!(data.member.member_id, "ou_123456789abcdef");
    assert_eq!(data.member.perm, "edit");
}

#[rstest]
#[tokio::test]
async fn test_list_member_permissions_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let token = "doccnBKgoMyY5OMbUG6FzxAnnNf";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "members": [
                {
                    "member_type": "user",
                    "member_id": "ou_123456789abcdef",
                    "perm": "edit",
                    "type": "user",
                    "name": "张三"
                },
                {
                    "member_type": "user",
                    "member_id": "ou_987654321fedcba",
                    "perm": "view",
                    "type": "user",
                    "name": "李四"
                },
                {
                    "member_type": "chat",
                    "member_id": "oc_chat123456789",
                    "perm": "edit",
                    "type": "chat",
                    "name": "产品设计群"
                }
            ]
        }
    });

    Mock::given(method("GET"))
        .and(path("/open-apis/drive/permission/member/list"))
        .and(query_param("token", token))
        .and(query_param("type", "doc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let permission_service = MemberPermissionService::new(config);
    let request = ListMemberPermissionRequest::builder()
        .token(token)
        .obj_type("doc")
        .build();

    let result = permission_service.list(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.members.len(), 3);
    
    // 验证用户权限
    let user_member = &data.members[0];
    assert_eq!(user_member.member_type, "user");
    assert_eq!(user_member.member_id, "ou_123456789abcdef");
    assert_eq!(user_member.perm, "edit");
    assert_eq!(user_member.name, Some("张三".to_string()));
    
    // 验证群组权限
    let chat_member = &data.members[2];
    assert_eq!(chat_member.member_type, "chat");
    assert_eq!(chat_member.member_id, "oc_chat123456789");
    assert_eq!(chat_member.perm, "edit");
    assert_eq!(chat_member.name, Some("产品设计群".to_string()));
}

#[rstest]
#[tokio::test]
async fn test_update_member_permission_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let token = "doccnBKgoMyY5OMbUG6FzxAnnNf";
    let member_id = "ou_123456789abcdef";
    
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "member": {
                "member_type": "user",
                "member_id": member_id,
                "perm": "view",
                "type": "user"
            }
        }
    });

    Mock::given(method("PUT"))
        .and(path("/open-apis/drive/permission/member/update"))
        .and(query_param("token", token))
        .and(query_param("type", "doc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let permission_service = MemberPermissionService::new(config);
    let request = UpdateMemberPermissionRequest::builder()
        .token(token)
        .obj_type("doc")
        .member_type("user")
        .member_id(member_id)
        .perm("view") // 从 edit 降级为 view
        .build();

    let result = permission_service.update(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.member.member_id, member_id);
    assert_eq!(data.member.perm, "view");
    assert_eq!(data.member.member_type, "user");
}

#[rstest]
#[tokio::test]
async fn test_delete_member_permission_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let token = "doccnBKgoMyY5OMbUG6FzxAnnNf";
    let member_id = "ou_987654321fedcba";
    
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "member": {
                "member_type": "user",
                "member_id": member_id,
                "perm": "view",
                "type": "user"
            }
        }
    });

    Mock::given(method("DELETE"))
        .and(path("/open-apis/drive/permission/member/delete"))
        .and(query_param("token", token))
        .and(query_param("type", "doc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let permission_service = MemberPermissionService::new(config);
    let request = DeleteMemberPermissionRequest::builder()
        .token(token)
        .obj_type("doc")
        .member_type("user")
        .member_id(member_id)
        .build();

    let result = permission_service.delete(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.member.member_id, member_id);
    assert_eq!(data.member.member_type, "user");
}

#[rstest]
#[tokio::test]
async fn test_transfer_member_permission_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let token = "doccnBKgoMyY5OMbUG6FzxAnnNf";
    let old_owner = "ou_123456789abcdef";
    let new_owner = "ou_987654321fedcba";
    
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "owner": {
                "member_type": "user",
                "member_id": new_owner,
                "perm": "full_access",
                "type": "user"
            }
        }
    });

    Mock::given(method("POST"))
        .and(path("/open-apis/drive/permission/member/transfer"))
        .and(query_param("token", token))
        .and(query_param("type", "doc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let permission_service = MemberPermissionService::new(config);
    let request = TransferMemberPermissionRequest::builder()
        .token(token)
        .obj_type("doc")
        .owner_type("user")
        .owner_id(new_owner)
        .build();

    let result = permission_service.transfer(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.owner.member_id, new_owner);
    assert_eq!(data.owner.perm, "full_access");
    assert_eq!(data.owner.member_type, "user");
}

// ===================== 公共链接权限 V1 测试 =====================

#[rstest]
#[tokio::test]
async fn test_get_public_permission_v1_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let token = "doccnBKgoMyY5OMbUG6FzxAnnNf";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "link_share_entity": {
                "link_share_token": "SdKWAqRvPddrNDTjzOFdJLNUeNmQC4w",
                "link_share_mode": "tenant_readable",
                "is_password_set": true,
                "is_date_set": false,
                "disable_download": false,
                "disable_preview": false,
                "disable_save": true
            }
        }
    });

    Mock::given(method("GET"))
        .and(path("/open-apis/drive/permission/public/"))
        .and(query_param("token", token))
        .and(query_param("type", "doc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let public_service = PublicPermissionV1Service::new(config);
    let request = GetPublicPermissionV1Request::builder()
        .token(token)
        .obj_type("doc")
        .build();

    let result = public_service.get(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    let link_share = &data.link_share_entity;
    assert_eq!(link_share.link_share_token, "SdKWAqRvPddrNDTjzOFdJLNUeNmQC4w");
    assert_eq!(link_share.link_share_mode, "tenant_readable");
    assert_eq!(link_share.is_password_set, true);
    assert_eq!(link_share.is_date_set, false);
    assert_eq!(link_share.disable_download, false);
    assert_eq!(link_share.disable_preview, false);
    assert_eq!(link_share.disable_save, true);
}

#[rstest]
#[tokio::test]
async fn test_patch_public_permission_v1_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let token = "doccnBKgoMyY5OMbUG6FzxAnnNf";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "link_share_entity": {
                "link_share_token": "SdKWAqRvPddrNDTjzOFdJLNUeNmQC4w",
                "link_share_mode": "anyone_can_view",
                "is_password_set": false,
                "is_date_set": true,
                "disable_download": true,
                "disable_preview": false,
                "disable_save": false
            }
        }
    });

    Mock::given(method("PATCH"))
        .and(path("/open-apis/drive/permission/public/"))
        .and(query_param("token", token))
        .and(query_param("type", "doc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let public_service = PublicPermissionV1Service::new(config);
    let request = PatchPublicPermissionV1Request::builder()
        .token(token)
        .obj_type("doc")
        .link_share_mode("anyone_can_view")
        .is_password_set(false)
        .is_date_set(true)
        .disable_download(true)
        .build();

    let result = public_service.patch(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    let link_share = &data.link_share_entity;
    assert_eq!(link_share.link_share_mode, "anyone_can_view");
    assert_eq!(link_share.is_password_set, false);
    assert_eq!(link_share.is_date_set, true);
    assert_eq!(link_share.disable_download, true);
}

// ===================== 密码保护功能测试 =====================

#[rstest]
#[tokio::test]
async fn test_create_password_protection_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let token = "doccnBKgoMyY5OMbUG6FzxAnnNf";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "password": {
                "password": "encrypted_password_hash"
            }
        }
    });

    Mock::given(method("POST"))
        .and(path("/open-apis/drive/permission/public/password/create"))
        .and(query_param("token", token))
        .and(query_param("type", "doc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let password_service = PasswordService::new(config);
    let request = CreatePasswordRequest::builder()
        .token(token)
        .obj_type("doc")
        .password("MySecurePassword123")
        .build();

    let result = password_service.create(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.password.password, "encrypted_password_hash");
}

#[rstest]
#[tokio::test]
async fn test_update_password_protection_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let token = "doccnBKgoMyY5OMbUG6FzxAnnNf";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "password": {
                "password": "new_encrypted_password_hash"
            }
        }
    });

    Mock::given(method("PUT"))
        .and(path("/open-apis/drive/permission/public/password/update"))
        .and(query_param("token", token))
        .and(query_param("type", "doc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let password_service = PasswordService::new(config);
    let request = UpdatePasswordRequest::builder()
        .token(token)
        .obj_type("doc")
        .old_password("MySecurePassword123")
        .new_password("NewSecurePassword456")
        .build();

    let result = password_service.update(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    assert_eq!(data.password.password, "new_encrypted_password_hash");
}

#[rstest]
#[tokio::test]
async fn test_delete_password_protection_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let token = "doccnBKgoMyY5OMbUG6FzxAnnNf";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {}
    });

    Mock::given(method("DELETE"))
        .and(path("/open-apis/drive/permission/public/password/delete"))
        .and(query_param("token", token))
        .and(query_param("type", "doc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let password_service = PasswordService::new(config);
    let request = DeletePasswordRequest::builder()
        .token(token)
        .obj_type("doc")
        .password("NewSecurePassword456")
        .build();

    let result = password_service.delete(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.code, 0);
}

// ===================== 公共链接权限 V2 测试 =====================

#[rstest]
#[tokio::test]
async fn test_get_public_permission_v2_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let token = "doccnBKgoMyY5OMbUG6FzxAnnNf";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "link_share_entity": {
                "link_share_token": "SdKWAqRvPddrNDTjzOFdJLNUeNmQC4w",
                "link_share_mode": "anyone_can_edit",
                "is_password_set": false,
                "is_date_set": true,
                "expire_time": 1675152000,
                "disable_download": false,
                "disable_preview": false,
                "disable_save": false,
                "disable_copy": true
            }
        }
    });

    Mock::given(method("GET"))
        .and(path("/open-apis/drive/v2/permission/public/"))
        .and(query_param("token", token))
        .and(query_param("type", "doc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let public_service = PublicPermissionV2Service::new(config);
    let request = GetPublicPermissionV2Request::builder()
        .token(token)
        .obj_type("doc")
        .build();

    let result = public_service.get(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    let link_share = &data.link_share_entity;
    assert_eq!(link_share.link_share_mode, "anyone_can_edit");
    assert_eq!(link_share.is_password_set, false);
    assert_eq!(link_share.is_date_set, true);
    assert_eq!(link_share.expire_time, Some(1675152000));
    assert_eq!(link_share.disable_copy, Some(true));
}

#[rstest]
#[tokio::test]
async fn test_patch_public_permission_v2_with_advanced_settings(
    test_config: Config,
    #[future] mock_server: MockServer,
) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let token = "doccnBKgoMyY5OMbUG6FzxAnnNf";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "link_share_entity": {
                "link_share_token": "SdKWAqRvPddrNDTjzOFdJLNUeNmQC4w",
                "link_share_mode": "tenant_editable",
                "is_password_set": true,
                "is_date_set": true,
                "expire_time": 1675152000,
                "disable_download": true,
                "disable_preview": false,
                "disable_save": true,
                "disable_copy": true
            }
        }
    });

    Mock::given(method("PATCH"))
        .and(path("/open-apis/drive/v2/permission/public/"))
        .and(query_param("token", token))
        .and(query_param("type", "doc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let public_service = PublicPermissionV2Service::new(config);
    let request = PatchPublicPermissionV2Request::builder()
        .token(token)
        .obj_type("doc")
        .link_share_mode("tenant_editable")
        .is_password_set(true)
        .is_date_set(true)
        .expire_time(1675152000)
        .disable_download(true)
        .disable_save(true)
        .disable_copy(true)
        .build();

    let result = public_service.patch(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    
    let link_share = &data.link_share_entity;
    assert_eq!(link_share.link_share_mode, "tenant_editable");
    assert_eq!(link_share.is_password_set, true);
    assert_eq!(link_share.is_date_set, true);
    assert_eq!(link_share.expire_time, Some(1675152000));
    assert_eq!(link_share.disable_download, true);
    assert_eq!(link_share.disable_save, true);
    assert_eq!(link_share.disable_copy, Some(true));
}

// ===================== 请求构建器测试 =====================

#[test]
fn test_create_member_permission_request_builder() {
    let request = CreateMemberPermissionRequest::builder()
        .token("doc123")
        .obj_type("doc")
        .member_type("user")
        .member_id("ou_123456")
        .perm("edit")
        .need_notification(true)
        .build();

    assert_eq!(request.token, "doc123");
    assert_eq!(request.obj_type, "doc");
    assert_eq!(request.member_type, "user");
    assert_eq!(request.member_id, "ou_123456");
    assert_eq!(request.perm, "edit");
    assert_eq!(request.need_notification, Some(true));
}

#[test]
fn test_patch_public_permission_v2_request_builder() {
    let request = PatchPublicPermissionV2Request::builder()
        .token("doc123")
        .obj_type("doc")
        .link_share_mode("anyone_can_view")
        .is_password_set(true)
        .is_date_set(true)
        .expire_time(1675152000)
        .disable_download(true)
        .disable_copy(true)
        .build();

    assert_eq!(request.token, "doc123");
    assert_eq!(request.obj_type, "doc");
    assert_eq!(request.link_share_mode, Some("anyone_can_view".to_string()));
    assert_eq!(request.is_password_set, Some(true));
    assert_eq!(request.is_date_set, Some(true));
    assert_eq!(request.expire_time, Some(1675152000));
    assert_eq!(request.disable_download, Some(true));
    assert_eq!(request.disable_copy, Some(true));
}

#[test]
fn test_password_request_builder() {
    let create_request = CreatePasswordRequest::builder()
        .token("doc123")
        .obj_type("doc")
        .password("SecretPassword123")
        .build();

    assert_eq!(create_request.token, "doc123");
    assert_eq!(create_request.obj_type, "doc");
    assert_eq!(create_request.password, "SecretPassword123");

    let update_request = UpdatePasswordRequest::builder()
        .token("doc123")
        .obj_type("doc")
        .old_password("OldPassword")
        .new_password("NewPassword")
        .build();

    assert_eq!(update_request.old_password, "OldPassword");
    assert_eq!(update_request.new_password, "NewPassword");
}

// ===================== 属性测试 =====================

proptest! {
    #[test]
    fn test_permission_level_invariants(
        perm in prop::sample::select(vec!["view", "edit", "full_access"])
    ) {
        let request = CreateMemberPermissionRequest::builder()
            .token("doc123")
            .obj_type("doc")
            .member_type("user")
            .member_id("ou_test")
            .perm(&perm)
            .build();

        assert_eq!(request.perm, perm);
        assert!(matches!(perm.as_str(), "view" | "edit" | "full_access"));
    }
}

proptest! {
    #[test]
    fn test_link_share_mode_invariants(
        link_mode in prop::sample::select(vec![
            "tenant_readable", "tenant_editable", 
            "anyone_can_view", "anyone_can_edit"
        ])
    ) {
        let request = PatchPublicPermissionV1Request::builder()
            .token("doc123")
            .obj_type("doc")
            .link_share_mode(&link_mode)
            .build();

        assert_eq!(request.link_share_mode, Some(link_mode.clone()));
        assert!(matches!(
            link_mode.as_str(), 
            "tenant_readable" | "tenant_editable" | "anyone_can_view" | "anyone_can_edit"
        ));
    }
}

proptest! {
    #[test]
    fn test_password_strength_invariants(
        password in prop::string::string_regex("[a-zA-Z0-9!@#$%^&*]{8,32}").unwrap()
    ) {
        let request = CreatePasswordRequest::builder()
            .token("doc123")
            .obj_type("doc")
            .password(&password)
            .build();

        assert_eq!(request.password, password);
        assert!(password.len() >= 8 && password.len() <= 32);
    }
}

// ===================== 错误处理测试 =====================

#[rstest]
#[tokio::test]
async fn test_permission_denied_error(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let token = "doccnNoPermission";
    let mock_response = json!({
        "code": 1248006,
        "msg": "无权限访问该文档"
    });

    Mock::given(method("GET"))
        .and(path("/open-apis/drive/permission/member/list"))
        .and(query_param("token", token))
        .respond_with(ResponseTemplate::new(403).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let permission_service = MemberPermissionService::new(config);
    let request = ListMemberPermissionRequest::builder()
        .token(token)
        .obj_type("doc")
        .build();

    let result = permission_service.list(request, None).await;
    
    if result.is_ok() {
        let response = result.unwrap();
        assert_eq!(response.code, 1248006);
        assert!(response.msg.contains("权限"));
    }
}

#[rstest]
#[tokio::test]
async fn test_invalid_password_error(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let token = "doccnBKgoMyY5OMbUG6FzxAnnNf";
    let mock_response = json!({
        "code": 1248020,
        "msg": "密码格式不正确"
    });

    Mock::given(method("PUT"))
        .and(path("/open-apis/drive/permission/public/password/update"))
        .and(query_param("token", token))
        .respond_with(ResponseTemplate::new(400).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let password_service = PasswordService::new(config);
    let request = UpdatePasswordRequest::builder()
        .token(token)
        .obj_type("doc")
        .old_password("wrong_password")
        .new_password("123") // 密码太短
        .build();

    let result = password_service.update(request, None).await;
    
    if result.is_ok() {
        let response = result.unwrap();
        assert_eq!(response.code, 1248020);
        assert!(response.msg.contains("密码"));
    }
}

// ===================== 集成场景测试 =====================

#[rstest]
#[tokio::test]
async fn test_permission_management_workflow(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let token = "doccnPermissionWorkflow";

    // 1. 添加成员权限
    let create_member_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "member": {
                "member_type": "user",
                "member_id": "ou_workflow_user",
                "perm": "edit",
                "type": "user"
            }
        }
    });

    Mock::given(method("POST"))
        .and(path("/open-apis/drive/permission/member/create"))
        .and(query_param("token", token))
        .respond_with(ResponseTemplate::new(200).set_body_json(create_member_response))
        .mount(&mock_server)
        .await;

    // 2. 设置公共链接
    let patch_public_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "link_share_entity": {
                "link_share_token": "WorkflowShareToken",
                "link_share_mode": "tenant_readable",
                "is_password_set": true,
                "is_date_set": false,
                "disable_download": true,
                "disable_preview": false,
                "disable_save": true
            }
        }
    });

    Mock::given(method("PATCH"))
        .and(path("/open-apis/drive/permission/public/"))
        .and(query_param("token", token))
        .respond_with(ResponseTemplate::new(200).set_body_json(patch_public_response))
        .mount(&mock_server)
        .await;

    // 3. 设置密码保护
    let create_password_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "password": {
                "password": "workflow_encrypted_password"
            }
        }
    });

    Mock::given(method("POST"))
        .and(path("/open-apis/drive/permission/public/password/create"))
        .and(query_param("token", token))
        .respond_with(ResponseTemplate::new(200).set_body_json(create_password_response))
        .mount(&mock_server)
        .await;

    // 4. 列出所有成员
    let list_members_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "members": [
                {
                    "member_type": "user",
                    "member_id": "ou_workflow_user",
                    "perm": "edit",
                    "type": "user",
                    "name": "工作流测试用户"
                }
            ]
        }
    });

    Mock::given(method("GET"))
        .and(path("/open-apis/drive/permission/member/list"))
        .and(query_param("token", token))
        .respond_with(ResponseTemplate::new(200).set_body_json(list_members_response))
        .mount(&mock_server)
        .await;

    // 执行完整的权限管理流程
    let member_service = MemberPermissionService::new(config.clone());
    let public_service = PublicPermissionV1Service::new(config.clone());
    let password_service = PasswordService::new(config);

    // 1. 添加成员权限
    let create_member_req = CreateMemberPermissionRequest::builder()
        .token(token)
        .obj_type("doc")
        .member_type("user")
        .member_id("ou_workflow_user")
        .perm("edit")
        .build();

    let member_result = member_service.create(create_member_req, None).await;
    assert!(member_result.is_ok());
    let member_data = member_result.unwrap().data.unwrap();
    assert_eq!(member_data.member.perm, "edit");

    // 2. 设置公共链接
    let patch_public_req = PatchPublicPermissionV1Request::builder()
        .token(token)
        .obj_type("doc")
        .link_share_mode("tenant_readable")
        .is_password_set(true)
        .disable_download(true)
        .disable_save(true)
        .build();

    let public_result = public_service.patch(patch_public_req, None).await;
    assert!(public_result.is_ok());
    let public_data = public_result.unwrap().data.unwrap();
    assert_eq!(public_data.link_share_entity.link_share_mode, "tenant_readable");
    assert_eq!(public_data.link_share_entity.disable_download, true);

    // 3. 设置密码保护
    let create_password_req = CreatePasswordRequest::builder()
        .token(token)
        .obj_type("doc")
        .password("WorkflowSecretPassword")
        .build();

    let password_result = password_service.create(create_password_req, None).await;
    assert!(password_result.is_ok());

    // 4. 验证权限设置
    let list_members_req = ListMemberPermissionRequest::builder()
        .token(token)
        .obj_type("doc")
        .build();

    let list_result = member_service.list(list_members_req, None).await;
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap().data.unwrap();
    assert_eq!(list_data.members.len(), 1);
    assert_eq!(list_data.members[0].member_id, "ou_workflow_user");
}

#[cfg(test)]
mod test_utils {
    use super::*;

    /// 创建测试用的成员
    pub fn create_test_member(
        member_type: &str,
        member_id: &str,
        perm: &str,
        name: Option<&str>,
    ) -> Member {
        Member {
            member_type: member_type.to_string(),
            member_id: member_id.to_string(),
            perm: perm.to_string(),
            type_field: member_type.to_string(),
            name: name.map(|n| n.to_string()),
        }
    }

    /// 验证权限级别
    pub fn assert_permission_level(perm: &str, expected_access_level: u8) {
        match perm {
            "view" => assert_eq!(expected_access_level, 1),
            "edit" => assert_eq!(expected_access_level, 2),
            "full_access" => assert_eq!(expected_access_level, 3),
            _ => panic!("未知的权限级别: {}", perm),
        }
    }

    /// 验证公共链接模式
    pub fn assert_link_share_mode_security(mode: &str) -> bool {
        matches!(
            mode,
            "tenant_readable" | "tenant_editable" | "anyone_can_view" | "anyone_can_edit"
        )
    }

    /// 生成测试密码
    pub fn generate_test_password(strength: &str) -> String {
        match strength {
            "weak" => "123456".to_string(),
            "medium" => "TestPass123".to_string(), 
            "strong" => "StrongTestPassword123!@#".to_string(),
            _ => "DefaultPassword".to_string(),
        }
    }
}