use open_lark::{
    core::config::{AppType, Config},
    service::bitable::v1::app_role_member::{
        BatchCreateRoleMemberRequest, BatchDeleteRoleMemberRequest, CreateRoleMemberRequest,
        DeleteRoleMemberRequest, ListRoleMemberRequest, MemberInfo,
    },
    LarkClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    // 创建客户端
    let client = LarkClient::builder(app_id, app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    // 配置必要参数
    let app_token = "bascnmBA*****yGehy8";
    let role_id = "rolxxxxxx"; // 需要一个真实的角色ID

    // 1. 列出当前协作者
    println!("1. 列出当前协作者...");

    let list_request = ListRoleMemberRequest::builder()
        .app_token(app_token)
        .role_id(role_id)
        .user_id_type("open_id")
        .page_size(20)
        .build();

    match client
        .bitable
        .v1
        .app_role_member
        .list(list_request, None)
        .await
    {
        Ok(list_response) => {
            println!("当前协作者列表:");
            for member in &list_response.data.items {
                println!(
                    "  - {} ({}): {:?}",
                    member.member_id, member.member_type, member.member_name
                );
            }
            println!("总计 {} 个协作者", list_response.data.total);
        }
        Err(e) => {
            println!("列出协作者失败: {:?}", e);
            println!("请确保角色ID正确，或者先创建一个自定义角色");
            return Ok(());
        }
    }

    // 2. 新增单个协作者
    println!("\n2. 新增单个协作者...");

    let create_request = CreateRoleMemberRequest::builder()
        .app_token(app_token)
        .role_id(role_id)
        .member_id("ou_test_user_1")
        .member_type("user")
        .user_id_type("open_id")
        .build();

    match client
        .bitable
        .v1
        .app_role_member
        .create(create_request, None)
        .await
    {
        Ok(create_response) => {
            let member = &create_response.data.member;
            println!("协作者新增成功:");
            println!("  - 成员ID: {} ({})", member.member_id, member.member_type);
            println!("  - 成员名称: {:?}", member.member_name);
        }
        Err(e) => {
            println!("新增协作者失败: {:?}", e);
        }
    }

    // 3. 批量新增协作者
    println!("\n3. 批量新增协作者...");

    let batch_members = vec![
        MemberInfo {
            member_id: "ou_test_user_2".to_string(),
            member_type: "user".to_string(),
        },
        MemberInfo {
            member_id: "ou_test_user_3".to_string(),
            member_type: "user".to_string(),
        },
        MemberInfo {
            member_id: "oc_test_chat_1".to_string(),
            member_type: "chat".to_string(),
        },
    ];

    let batch_create_request = BatchCreateRoleMemberRequest::builder()
        .app_token(app_token)
        .role_id(role_id)
        .members(batch_members)
        .user_id_type("open_id")
        .build();

    match client
        .bitable
        .v1
        .app_role_member
        .batch_create(batch_create_request, None)
        .await
    {
        Ok(batch_create_response) => {
            println!("批量新增协作者成功:");
            for member in &batch_create_response.data.members {
                println!(
                    "  - {} ({}): {:?}",
                    member.member_id, member.member_type, member.member_name
                );
            }
            println!(
                "共新增 {} 个协作者",
                batch_create_response.data.members.len()
            );
        }
        Err(e) => {
            println!("批量新增协作者失败: {:?}", e);
        }
    }

    // 4. 再次列出协作者，查看变化
    println!("\n4. 再次列出协作者（查看变化）...");

    let list_request_after = ListRoleMemberRequest::builder()
        .app_token(app_token)
        .role_id(role_id)
        .user_id_type("open_id")
        .page_size(20)
        .build();

    let list_response_after = client
        .bitable
        .v1
        .app_role_member
        .list(list_request_after, None)
        .await?;

    println!("更新后的协作者列表:");
    for member in &list_response_after.data.items {
        println!(
            "  - {} ({}): {:?}",
            member.member_id, member.member_type, member.member_name
        );
    }
    println!("总计 {} 个协作者", list_response_after.data.total);

    // 5. 删除单个协作者
    println!("\n5. 删除单个协作者...");

    let delete_request = DeleteRoleMemberRequest::builder()
        .app_token(app_token)
        .role_id(role_id)
        .member_id("ou_test_user_1")
        .user_id_type("open_id")
        .build();

    match client
        .bitable
        .v1
        .app_role_member
        .delete(delete_request, None)
        .await
    {
        Ok(delete_response) => {
            println!("删除协作者成功:");
            println!("  - 删除的成员ID: {}", delete_response.data.member_id);
            println!("  - 删除状态: {}", delete_response.data.deleted);
        }
        Err(e) => {
            println!("删除协作者失败: {:?}", e);
        }
    }

    // 6. 批量删除协作者（清理测试数据）
    println!("\n6. 批量删除协作者（清理测试数据）...");

    let batch_delete_request = BatchDeleteRoleMemberRequest::builder()
        .app_token(app_token)
        .role_id(role_id)
        .add_member_id("ou_test_user_2")
        .add_member_id("ou_test_user_3")
        .add_member_id("oc_test_chat_1")
        .user_id_type("open_id")
        .build();

    match client
        .bitable
        .v1
        .app_role_member
        .batch_delete(batch_delete_request, None)
        .await
    {
        Ok(batch_delete_response) => {
            println!("批量删除协作者结果:");
            for result in &batch_delete_response.data.results {
                println!(
                    "  - {} : {}",
                    result.member_id,
                    if result.deleted {
                        "删除成功"
                    } else {
                        "删除失败"
                    }
                );
            }
        }
        Err(e) => {
            println!("批量删除协作者失败: {:?}", e);
        }
    }

    // 7. 最终列出协作者，确认清理结果
    println!("\n7. 最终列出协作者（确认清理结果）...");

    let final_list_request = ListRoleMemberRequest::builder()
        .app_token(app_token)
        .role_id(role_id)
        .user_id_type("open_id")
        .page_size(20)
        .build();

    let final_list_response = client
        .bitable
        .v1
        .app_role_member
        .list(final_list_request, None)
        .await?;

    println!("最终协作者列表:");
    for member in &final_list_response.data.items {
        println!(
            "  - {} ({}): {:?}",
            member.member_id, member.member_type, member.member_name
        );
    }
    println!("总计 {} 个协作者", final_list_response.data.total);

    println!("\n协作者操作示例完成！");
    println!("注意：此示例需要有效的角色ID和用户ID，请根据实际情况调整参数");
    Ok(())
}
