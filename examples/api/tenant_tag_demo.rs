use dotenvy::dotenv;
use open_lark::{
    prelude::*,
    service::tenant_tag::{
        models::{TagStatus, TagType, UserIdType},
        tag::{CreateTagRequest, PatchTagRequest},
        tag_binding::{CreateTagBindingRequest, GetTagBindingRequest, UpdateTagBindingRequest},
    },
};

/// 企业自定义群标签模块功能演示
///
/// 展示标签创建、修改、查询、绑定和解绑等完整生命周期管理
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    println!("🚀 开始演示企业自定义群标签功能...\\n");

    // 演示标签管理功能
    demo_tag_management(&client).await?;

    // 演示标签绑定功能
    demo_tag_binding(&client).await?;

    println!("✅ 企业自定义群标签功能演示完成！");
    Ok(())
}

/// 演示标签管理功能
async fn demo_tag_management(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("🏷️ 演示标签管理功能...");

    // 创建标签
    let create_request = CreateTagRequest {
        name: "重要群组".to_string(),
        description: Some("用于标记重要的群组".to_string()),
        tag_type: TagType::Chat,
    };

    match client
        .tenant_tag
        .tag
        .create(create_request, Some(UserIdType::OpenId), None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!(
                    "  ✅ 标签创建成功: {} (ID: {})",
                    data.tag.name, data.tag.tag_id
                );
                let tag_id = data.tag.tag_id;

                // 修改标签
                let patch_request = PatchTagRequest {
                    name: Some("超级重要群组".to_string()),
                    description: Some("更新后的描述：用于标记超级重要的群组".to_string()),
                    status: Some(TagStatus::Active),
                };

                match client
                    .tenant_tag
                    .tag
                    .patch(&tag_id, patch_request, Some(UserIdType::OpenId), None)
                    .await
                {
                    Ok(patch_response) => {
                        if let Some(patch_data) = patch_response.data {
                            println!("  ✅ 标签修改成功: {}", patch_data.tag.name);
                        }
                    }
                    Err(e) => {
                        println!("  ❌ 标签修改失败: {e:?}");
                    }
                }

                // 查询标签列表
                match client
                    .tenant_tag
                    .tag
                    .list(
                        Some(TagType::Chat),
                        Some(20),
                        None,
                        Some(UserIdType::OpenId),
                        None,
                    )
                    .await
                {
                    Ok(list_response) => {
                        if let Some(list_data) = list_response.data {
                            println!("  📋 查询到 {} 个标签", list_data.tags.len());
                            for tag in &list_data.tags {
                                println!("    - {}: {} ({:?})", tag.tag_id, tag.name, tag.status);
                            }
                        }
                    }
                    Err(e) => {
                        println!("  ❌ 查询标签列表失败: {e:?}");
                    }
                }
            }
        }
        Err(e) => {
            println!("  ❌ 标签创建失败: {e:?}");
        }
    }

    println!();
    Ok(())
}

/// 演示标签绑定功能
async fn demo_tag_binding(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔗 演示标签绑定功能...");

    let chat_id = "oc_example_chat_id";

    // 查询现有绑定关系
    let get_request = GetTagBindingRequest {
        entity_id: chat_id.to_string(),
        entity_type: "chat".to_string(),
        tag_id: None,
        page_size: Some(10),
        page_token: None,
        user_id_type: Some(UserIdType::OpenId),
    };

    match client.tenant_tag.tag_binding.get(get_request, None).await {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("  📋 当前群组绑定了 {} 个标签", data.bindings.len());
                for binding in &data.bindings {
                    println!(
                        "    - 标签ID: {}, 实体ID: {}",
                        binding.tag_id, binding.entity_id
                    );
                }
            }
        }
        Err(e) => {
            println!("  ❌ 查询绑定关系失败: {e:?}");
        }
    }

    // 绑定标签到群
    let bind_request = CreateTagBindingRequest {
        tag_ids: vec!["tag_001".to_string(), "tag_002".to_string()],
        chat_id: chat_id.to_string(),
    };

    match client
        .tenant_tag
        .tag_binding
        .create(bind_request, Some(UserIdType::OpenId), None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!(
                    "  ✅ 标签绑定成功: 成功 {} 个，失败 {} 个",
                    data.success_tag_ids.len(),
                    data.failed_tag_ids.len()
                );

                if !data.success_tag_ids.is_empty() {
                    println!("    成功绑定的标签: {:?}", data.success_tag_ids);
                }
                if !data.failed_tag_ids.is_empty() {
                    println!("    绑定失败的标签: {:?}", data.failed_tag_ids);
                }
            }
        }
        Err(e) => {
            println!("  ❌ 标签绑定失败: {e:?}");
        }
    }

    // 解绑标签与群
    let unbind_request = UpdateTagBindingRequest {
        tag_ids: vec!["tag_001".to_string()],
        chat_id: chat_id.to_string(),
    };

    match client
        .tenant_tag
        .tag_binding
        .update(unbind_request, Some(UserIdType::OpenId), None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!(
                    "  ✅ 标签解绑成功: 成功 {} 个，失败 {} 个",
                    data.success_tag_ids.len(),
                    data.failed_tag_ids.len()
                );

                if !data.success_tag_ids.is_empty() {
                    println!("    成功解绑的标签: {:?}", data.success_tag_ids);
                }
                if !data.failed_tag_ids.is_empty() {
                    println!("    解绑失败的标签: {:?}", data.failed_tag_ids);
                }
            }
        }
        Err(e) => {
            println!("  ❌ 标签解绑失败: {e:?}");
        }
    }

    println!();
    Ok(())
}
