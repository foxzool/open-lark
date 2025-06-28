use open_lark::{prelude::*, service::contact::v3::user::ListUsersRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET must be set");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    // 构建获取用户列表请求
    let req = ListUsersRequest {
        page_size: Some(10),
        page_token: None,
        user_id_type: Some("user_id".to_string()),
        department_id_type: Some("department_id".to_string()),
    };

    match client.contact.v3.user.list(&req).await {
        Ok(resp) => {
            println!("获取用户列表成功:");
            println!("用户数量: {}", resp.items.len());
            for user in resp.items {
                if let Some(name) = &user.name {
                    println!("- 用户姓名: {}", name);
                }
                if let Some(email) = &user.email {
                    println!("  邮箱: {}", email);
                }
                if let Some(user_id) = &user.user_id {
                    println!("  用户ID: {}", user_id);
                }
                println!("---");
            }

            if let Some(has_more) = resp.has_more {
                println!("是否还有更多数据: {}", has_more);
            }
            if let Some(page_token) = &resp.page_token {
                println!("下一页标记: {}", page_token);
            }
        }
        Err(e) => eprintln!("获取用户列表失败: {:?}", e),
    }

    Ok(())
}
