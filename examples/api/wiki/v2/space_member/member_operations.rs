use open_lark::{
    service::wiki::v2::space_member::{
        CreateSpaceMemberRequest, DeleteSpaceMemberRequest, ListSpaceMemberRequest,
    },
    prelude::LarkClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    // 配置必要参数
    let space_id = "spcxxxxxx"; // 需要一个真实的知识空间ID

    // 1. 获取知识空间成员列表
    println!("1. 获取知识空间成员列表...");

    let list_request = ListSpaceMemberRequest::builder()
        .space_id(space_id)
        .page_size(20)
        .build();

    match client.wiki.v2.space_member.list(list_request, None).await {
        Ok(list_response) => {
            println!("知识空间成员列表:");
            if let Some(data) = &list_response.data {
                if data.items.is_empty() {
                    println!("  暂无成员");
                } else {
                    for member in &data.items {
                        println!("  成员信息:");
                        println!("    - 成员ID: {}", member.member_id);
                        println!("    - 成员类型: {}", member.member_type);
                        println!("    - 权限角色: {}", member.role);
                        if let Some(member_type) = &member.r#type {
                            println!("    - 类型: {}", member_type);
                        }
                        println!();
                    }
                }
                println!("是否有更多: {}", data.has_more);
            }
        }
        Err(e) => {
            println!("获取成员列表失败: {:?}", e);
            println!("请确保知识空间ID正确，或者有权限访问该空间");
        }
    }

    // 2. 添加新成员
    println!("\n2. 添加知识空间成员...");

    let create_request = CreateSpaceMemberRequest::builder()
        .space_id(space_id)
        .member_type("user")
        .member_id("ou_test_user_1") // 需要一个真实的用户ID
        .as_editor() // 设置为协作者
        .build();

    match client
        .wiki
        .v2
        .space_member
        .create(create_request, None)
        .await
    {
        Ok(create_response) => {
            if let Some(data) = &create_response.data {
                let member = &data.member;
                println!("成员添加成功:");
                println!("  - 成员ID: {}", member.member_id);
                println!("  - 成员类型: {}", member.member_type);
                println!("  - 权限角色: {}", member.role);
            }
        }
        Err(e) => {
            println!("添加成员失败: {:?}", e);
            println!("可能的原因：");
            println!("- 用户ID不存在或无效");
            println!("- 没有管理权限");
            println!("- 用户已经是空间成员");
        }
    }

    // 3. 再次列出成员，查看变化
    println!("\n3. 再次列出成员（查看变化）...");

    let list_request_after = ListSpaceMemberRequest::builder()
        .space_id(space_id)
        .page_size(20)
        .build();

    match client
        .wiki
        .v2
        .space_member
        .list(list_request_after, None)
        .await
    {
        Ok(list_response_after) => {
            println!("更新后的成员列表:");
            for member in &list_response_after.data.as_ref().unwrap().items {
                println!(
                    "  - {} ({}): {}",
                    member.member_id, member.member_type, member.role
                );
            }
            println!("总计 {} 个成员", list_response_after.data.as_ref().unwrap().items.len());
        }
        Err(e) => {
            println!("获取更新后成员列表失败: {:?}", e);
        }
    }

    // 4. 删除成员（清理测试数据）
    println!("\n4. 删除知识空间成员（清理测试数据）...");

    let delete_request = DeleteSpaceMemberRequest::builder()
        .space_id(space_id)
        .member_id("ou_test_user_1")
        .member_type("user")
        .build();

    match client
        .wiki
        .v2
        .space_member
        .delete(delete_request, None)
        .await
    {
        Ok(delete_response) => {
            println!("删除成员成功:");
            println!("  - 删除的成员ID: {}", delete_response.data.as_ref().unwrap().member_id);
            println!("  - 删除状态: {}", delete_response.data.as_ref().unwrap().deleted);
        }
        Err(e) => {
            println!("删除成员失败: {:?}", e);
            println!("可能的原因：");
            println!("- 成员不存在");
            println!("- 没有管理权限");
            println!("- 不能删除空间创建者");
        }
    }

    // 5. 最终列出成员，确认删除结果
    println!("\n5. 最终列出成员（确认删除结果）...");

    let final_list_request = ListSpaceMemberRequest::builder()
        .space_id(space_id)
        .page_size(20)
        .build();

    match client
        .wiki
        .v2
        .space_member
        .list(final_list_request, None)
        .await
    {
        Ok(final_list_response) => {
            println!("最终成员列表:");
            for member in &final_list_response.data.as_ref().unwrap().items {
                println!(
                    "  - {} ({}): {}",
                    member.member_id, member.member_type, member.role
                );
            }
            println!("总计 {} 个成员", final_list_response.data.as_ref().unwrap().items.len());
        }
        Err(e) => {
            println!("获取最终成员列表失败: {:?}", e);
        }
    }

    println!("\n知识空间成员操作示例完成！");
    println!("提示：");
    println!("- 需要知识空间的管理权限才能添加/删除成员");
    println!("- 权限角色包括：admin(管理员)、edit_member(协作者)、view_member(阅读者)");
    println!("- 空间创建者不能被删除");
    println!("- 用户必须在企业内才能被添加为成员");

    Ok(())
}
