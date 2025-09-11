/// 用户搜索示例
///
/// 这个示例演示如何使用飞书SDK搜索企业内的用户信息。
///
/// 使用方法：
/// cargo run --example core_search_user
///
/// 环境变量：
/// APP_ID=your_app_id
/// APP_SECRET=your_app_secret
/// USER_ACCESS_TOKEN=your_user_access_token (必需，用户搜索需要用户令牌)
use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");
    let user_access_token =
        std::env::var("USER_ACCESS_TOKEN").expect("USER_ACCESS_TOKEN environment variable not set");

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    println!("🔍 飞书用户搜索示例");
    println!("{}", "=".repeat(50));

    // 演示基础搜索功能
    basic_user_search(&client, &user_access_token).await?;

    // 演示分页搜索功能
    paginated_search(&client, &user_access_token).await?;

    // 演示搜索过滤功能
    advanced_search_demo(&client, &user_access_token).await?;

    Ok(())
}

/// 基础用户搜索
async fn basic_user_search(
    client: &LarkClient,
    user_access_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n👤 基础用户搜索...");

    let search_query = "张"; // 搜索姓张的用户
    println!("   搜索关键词: \"{search_query}\"");

    let request_option = open_lark::core::req_option::RequestOption::builder()
        .user_access_token(user_access_token)
        .build();

    let request = open_lark::service::search::v1::user::SearchUserRequest::builder()
        .query(search_query)
        .page_size(10)
        .build();

    match client
        .search
        .v1
        .user
        .search_user(request, Some(request_option))
        .await
    {
        Ok(response) => {
            println!("✅ 用户搜索成功!");
            println!("   找到用户数: {}", response.users.len());
            println!("   是否有更多: {}", response.has_more);

            if !response.users.is_empty() {
                println!("\n📋 搜索结果:");
                for (index, user) in response.users.iter().enumerate() {
                    println!("   {}. {}", index + 1, user.name);
                    println!("      用户ID: {}", user.open_id);

                    if let Some(user_id) = &user.user_id {
                        println!("      员工ID: {user_id}");
                    }

                    if !user.department_ids.is_empty() {
                        println!("      部门数量: {}", user.department_ids.len());
                    }

                    println!("      头像URL: {}", user.avatar.avatar_72);
                    println!(); // 空行分隔
                }
            } else {
                println!("📭 没有找到匹配的用户");
                println!("💡 建议:");
                println!("   1. 尝试使用更短的关键词");
                println!("   2. 检查关键词拼写");
                println!("   3. 确认用户在当前企业内");
            }

            if response.has_more {
                if let Some(page_token) = &response.page_token {
                    println!("💡 提示: 还有更多用户可以通过分页获取");
                    println!("   下一页Token: {page_token}");
                }
            }
        }
        Err(e) => {
            println!("❌ 用户搜索失败: {e:?}");
            println!("\n💡 常见错误解决方案:");
            println!("   1. 检查USER_ACCESS_TOKEN是否有效");
            println!("   2. 确认应用有用户搜索权限");
            println!("   3. 验证用户在当前企业内");
            println!("   4. 检查搜索关键词格式");
            return Err(e.into());
        }
    }

    Ok(())
}

/// 分页搜索演示
async fn paginated_search(
    client: &LarkClient,
    user_access_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📄 分页搜索演示...");

    let search_query = "李"; // 搜索姓李的用户
    println!("   搜索关键词: \"{search_query}\"");
    println!("   每页大小: 3");

    let request_option = open_lark::core::req_option::RequestOption::builder()
        .user_access_token(user_access_token)
        .build();

    let mut page_token: Option<String> = None;
    let mut page_number = 1;
    let mut total_users = 0;

    loop {
        let mut request_builder =
            open_lark::service::search::v1::user::SearchUserRequest::builder()
                .query(search_query)
                .page_size(3);

        if let Some(token) = &page_token {
            request_builder = request_builder.page_token(token);
        }

        let request = request_builder.build();
        match client
            .search
            .v1
            .user
            .search_user(request, Some(request_option.clone()))
            .await
        {
            Ok(response) => {
                println!("\n📋 第{page_number}页结果:");
                println!("   本页用户数: {}", response.users.len());

                for (index, user) in response.users.iter().enumerate() {
                    println!("   {}. {} ({})", index + 1, user.name, user.open_id);
                }

                total_users += response.users.len();

                if response.has_more {
                    page_token = response.page_token.clone();
                    page_number += 1;

                    if page_number > 3 {
                        // 限制演示页数
                        println!("\n💡 演示限制: 只显示前3页结果");
                        println!("   实际还有更多页面可以获取");
                        break;
                    }
                } else {
                    println!("\n✅ 搜索完成，共{page_number}页，{total_users}个用户");
                    break;
                }
            }
            Err(e) => {
                println!("❌ 第{page_number}页搜索失败: {e:?}");
                break;
            }
        }
    }

    Ok(())
}

/// 高级搜索功能演示
async fn advanced_search_demo(
    client: &LarkClient,
    user_access_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔧 高级搜索功能演示...");

    let search_queries = vec![
        ("王", "搜索姓王的用户"),
        ("admin", "搜索包含admin的用户"),
        ("test", "搜索测试用户"),
    ];

    let request_option = open_lark::core::req_option::RequestOption::builder()
        .user_access_token(user_access_token)
        .build();

    for (query, description) in search_queries {
        println!("\n🔍 {description}: \"{query}\"");

        let request = open_lark::service::search::v1::user::SearchUserRequest::builder()
            .query(query)
            .page_size(5)
            .build();

        match client
            .search
            .v1
            .user
            .search_user(request, Some(request_option.clone()))
            .await
        {
            Ok(response) => {
                println!("   结果: 找到{}个用户", response.users.len());

                if !response.users.is_empty() {
                    // 显示用户的详细信息
                    for user in &response.users {
                        println!(
                            "     - {} ({}) 部门数:{}",
                            user.name,
                            user.open_id,
                            user.department_ids.len()
                        );
                    }

                    if response.has_more {
                        println!("     + 还有更多结果...");
                    }
                }
            }
            Err(e) => {
                println!("   错误: {e:?}");
            }
        }
    }

    println!("\n💡 搜索功能说明:");
    println!("   1. 支持按姓名、用户名等关键词搜索");
    println!("   2. 支持分页查询，避免一次返回过多数据");
    println!("   3. 返回用户基本信息、头像、部门信息等");
    println!("   4. 需要用户访问令牌(USER_ACCESS_TOKEN)");
    println!("   5. 搜索结果受当前用户权限限制");

    Ok(())
}

/// 展示用户详细信息
#[allow(dead_code)]
async fn display_user_details(
    client: &LarkClient,
    user_access_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📊 用户详细信息展示...");

    let request_option = open_lark::core::req_option::RequestOption::builder()
        .user_access_token(user_access_token)
        .build();

    let request = open_lark::service::search::v1::user::SearchUserRequest::builder()
        .query("张")
        .page_size(1)
        .build();

    match client
        .search
        .v1
        .user
        .search_user(request, Some(request_option))
        .await
    {
        Ok(response) => {
            if let Some(user) = response.users.first() {
                println!("📋 用户详细信息:");
                println!("   姓名: {}", user.name);
                println!("   OpenID: {}", user.open_id);

                if let Some(user_id) = &user.user_id {
                    println!("   用户ID: {user_id}");
                }

                println!("   部门数量: {}", user.department_ids.len());
                if !user.department_ids.is_empty() {
                    println!("   部门ID列表:");
                    for (index, dept_id) in user.department_ids.iter().enumerate() {
                        println!("     {}. {}", index + 1, dept_id);
                    }
                }

                println!("   头像信息:");
                println!("     72x72: {}", user.avatar.avatar_72);
                println!("     240x240: {}", user.avatar.avatar_240);
                println!("     640x640: {}", user.avatar.avatar_640);
                println!("     原始尺寸: {}", user.avatar.avatar_origin);
            }
        }
        Err(e) => {
            println!("❌ 获取用户详细信息失败: {e:?}");
        }
    }

    Ok(())
}
