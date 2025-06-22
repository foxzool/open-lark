use open_lark::{
    prelude::*,
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

    // 1. 列出现有的自定义角色
    println!("1. 列出现有的自定义角色...");

    let list_request = ListAppRoleRequest::builder()
        .app_token(app_token)
        .page_size(20)
        .build();

    let list_response = client.bitable.v1.app_role.list(list_request, None).await?;

    println!("现有自定义角色列表:");
    for role in &list_response.data.items {
        println!("  - {}: {} ", role.role_name, role.role_id);
        if let Some(table_roles) = &role.table_roles {
            for table_role in table_roles {
                println!("    表权限: {} -> {}", table_role.table_id, table_role.role);
            }
        }
    }
    println!("总计 {} 个自定义角色", list_response.data.total);

    // 2. 新增自定义角色
    println!("\n2. 新增自定义角色...");

    // 定义数据表权限
    let table_roles = vec![
        TableRole {
            table_id: "tblxxxxxx".to_string(),
            role: "editor".to_string(),
            rec_rule: Some("all".to_string()),
        },
        TableRole {
            table_id: "tblyyyyyy".to_string(),
            role: "reader".to_string(),
            rec_rule: Some("all".to_string()),
        },
    ];

    // 定义数据表默认权限
    let block_roles = vec![BlockRole {
        block_id: "blkxxxxxx".to_string(),
        role: "reader".to_string(),
    }];

    let create_request = CreateAppRoleRequest::builder()
        .app_token(app_token)
        .role_name("测试自定义角色")
        .table_roles(table_roles)
        .block_roles(block_roles)
        .build();

    match client
        .bitable
        .v1
        .app_role
        .create(create_request, None)
        .await
    {
        Ok(create_response) => {
            let role = &create_response.data.role;
            println!("自定义角色创建成功:");
            println!("  - 角色名称: {}", role.role_name);
            println!("  - 角色ID: {}", role.role_id);

            if let Some(table_roles) = &role.table_roles {
                println!("  - 数据表权限:");
                for table_role in table_roles {
                    println!(
                        "    {} -> {} (记录规则: {:?})",
                        table_role.table_id, table_role.role, table_role.rec_rule
                    );
                }
            }

            if let Some(block_roles) = &role.block_roles {
                println!("  - 数据表默认权限:");
                for block_role in block_roles {
                    println!("    {} -> {}", block_role.block_id, block_role.role);
                }
            }

            let role_id = role.role_id.clone();

            // 3. 更新自定义角色
            println!("\n3. 更新自定义角色...");

            let new_table_roles = vec![TableRole {
                table_id: "tblxxxxxx".to_string(),
                role: "owner".to_string(), // 升级权限
                rec_rule: Some("all".to_string()),
            }];

            let update_request = UpdateAppRoleRequest::builder()
                .app_token(app_token)
                .role_id(&role_id)
                .role_name("更新后的自定义角色")
                .table_roles(new_table_roles)
                .build();

            match client
                .bitable
                .v1
                .app_role
                .update(update_request, None)
                .await
            {
                Ok(update_response) => {
                    let updated_role = &update_response.data.role;
                    println!("角色更新成功:");
                    println!("  - 新角色名称: {}", updated_role.role_name);
                    println!("  - 角色ID: {}", updated_role.role_id);

                    if let Some(table_roles) = &updated_role.table_roles {
                        println!("  - 更新后的数据表权限:");
                        for table_role in table_roles {
                            println!(
                                "    {} -> {} (记录规则: {:?})",
                                table_role.table_id, table_role.role, table_role.rec_rule
                            );
                        }
                    }
                }
                Err(e) => {
                    println!("更新自定义角色失败: {:?}", e);
                }
            }

            // 4. 再次列出自定义角色，查看变化
            println!("\n4. 再次列出自定义角色（查看变化）...");

            let list_request_after = ListAppRoleRequest::builder()
                .app_token(app_token)
                .page_size(20)
                .build();

            let list_response_after = client
                .bitable
                .v1
                .app_role
                .list(list_request_after, None)
                .await?;

            println!("更新后的自定义角色列表:");
            for role in &list_response_after.data.items {
                println!("  - {}: {}", role.role_name, role.role_id);
            }
            println!(
                "总计 {} 个自定义角色（增加了 {} 个）",
                list_response_after.data.total,
                list_response_after.data.total - list_response.data.total
            );

            // 5. 删除创建的测试角色（清理资源）
            println!("\n5. 删除创建的测试角色（清理资源）...");

            let delete_request = DeleteAppRoleRequest::builder()
                .app_token(app_token)
                .role_id(&role_id)
                .build();

            match client
                .bitable
                .v1
                .app_role
                .delete(delete_request, None)
                .await
            {
                Ok(delete_response) => {
                    println!("自定义角色删除成功:");
                    println!("  - 删除的角色ID: {}", delete_response.data.role_id);
                    println!("  - 删除状态: {}", delete_response.data.deleted);
                }
                Err(e) => {
                    println!("删除自定义角色失败: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("创建自定义角色失败: {:?}", e);
            println!("请确保提供正确的数据表ID和权限设置");
        }
    }

    println!("\n自定义角色操作示例完成！");
    println!("注意：此示例需要有效的数据表ID和权限配置，请根据实际情况调整参数");
    Ok(())
}
