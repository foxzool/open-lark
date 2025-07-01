use open_lark::{
    core::trait_system::ExecutableBuilder,
    prelude::*,
    service::cloud_docs::permission::member::{
        BatchCreatePermissionMemberRequest, Collaborator, Permission,
    },
};

/// 演示采用owned参数模式的权限服务API调用
///
/// 这个示例展示了从&Request模式迁移到owned Request模式后的API使用方式，
/// 消除了不必要的clone()开销，提升了性能。
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .build();

    // 创建权限请求 - 使用owned模式，无需clone
    let collaborators = vec![Collaborator {
        member_type: "user".to_string(),
        member_id: "ou_test_user_id".to_string(),
        perm: Permission::Edit,
    }];

    let request = BatchCreatePermissionMemberRequest::builder()
        .token("test_doc_token")
        .obj_type("doc")
        .members(collaborators)
        .need_notification(true)
        .build();

    // 方式1: 直接调用service方法 (owned参数，无clone开销)
    match client
        .cloud_docs
        .permission
        .batch_create_member(request, None)
        .await
    {
        Ok(response) => {
            println!("✅ 批量创建权限成功 (直接调用)");
            if let Some(data) = response.data {
                println!("创建结果: {:?}", data.members);
            }
        }
        Err(e) => {
            println!("❌ 请求失败: {e:?}");
        }
    }

    // 方式2: 使用Builder的execute方法 (owned参数，无clone开销)
    let collaborators2 = vec![Collaborator {
        member_type: "user".to_string(),
        member_id: "ou_another_user".to_string(),
        perm: Permission::View,
    }];

    let request2 = BatchCreatePermissionMemberRequest::builder()
        .token("test_doc_token")
        .obj_type("doc")
        .members(collaborators2)
        .need_notification(false)
        .execute(&client.cloud_docs.permission)
        .await;

    match request2 {
        Ok(response) => {
            println!("✅ 批量创建权限成功 (Builder execute)");
            if let Some(data) = response.data {
                println!("创建结果: {:?}", data.members);
            }
        }
        Err(e) => {
            println!("❌ 请求失败: {e:?}");
        }
    }

    println!("🚀 性能改进: 使用owned参数模式避免了不必要的clone()开销");

    Ok(())
}
