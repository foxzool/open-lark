/// 统一Builder模式示例
///
/// 这个示例展示了开放飞书SDK中新旧API模式的使用方法，
/// 以及从旧模式迁移到新模式的最佳实践。
use dotenvy::dotenv;
use open_lark::{
    client::LarkClient,
    core::{constants::AppType, trait_system::ExecutableBuilder},
    service::contact::{models::User, v3::user::CreateUserRequest},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    // 创建客户端
    let client = LarkClient::builder(
        &std::env::var("APP_ID").expect("APP_ID is required"),
        &std::env::var("APP_SECRET").expect("APP_SECRET is required"),
    )
    .with_app_type(AppType::SelfBuild)
    .build();

    // 示例用户数据
    let user = User {
        name: Some("测试用户".to_string()),
        en_name: Some("Test User".to_string()),
        email: Some("test@example.com".to_string()),
        mobile: Some("+86 13800138000".to_string()),
        ..Default::default()
    };

    println!("=== 开放飞书SDK Builder模式最佳实践示例 ===\n");

    // ==========================================
    // 方式一: 传统方式 (仍然支持)
    // ==========================================
    println!("📋 方式一: 传统API调用方式");
    println!("适用于: 现有代码迁移、简单快速调用\n");

    let traditional_request = CreateUserRequest {
        user: user.clone(),
        user_id_type: Some("open_id".to_string()),
        department_id_type: Some("open_department_id".to_string()),
    };

    match client.contact.v3.user.create(&traditional_request).await {
        Ok(response) => {
            println!("✅ 传统方式调用成功");
            println!("   用户ID: {:?}", response.user.user_id);
            println!("   用户名: {:?}", response.user.name);
        }
        Err(e) => {
            println!("❌ 传统方式调用失败: {e}");
            println!("   这可能是因为权限问题或测试环境限制");
        }
    }

    println!();

    // ==========================================
    // 方式二: 现代Builder模式 (推荐)
    // ==========================================
    println!("🏗️  方式二: 现代Builder模式 (推荐)");
    println!("适用于: 新代码开发、复杂参数配置、链式调用\n");

    // 使用Builder模式构建请求
    let builder_result = client
        .contact
        .v3
        .user
        .create_user_builder()
        .user(user.clone())
        .user_id_type("open_id")
        .department_id_type("open_department_id")
        .execute(&client.contact.v3.user)
        .await;

    match builder_result {
        Ok(response) => {
            println!("✅ Builder模式调用成功");
            println!("   用户ID: {:?}", response.user.user_id);
            println!("   用户名: {:?}", response.user.name);
        }
        Err(e) => {
            println!("❌ Builder模式调用失败: {e}");
            println!("   这可能是因为权限问题或测试环境限制");
        }
    }

    println!();

    // ==========================================
    // 方式三: Builder模式的高级用法
    // ==========================================
    println!("⚡ 方式三: Builder模式高级用法");
    println!("展示: 可选参数、条件构建、复用Builder\n");

    let mut advanced_builder = client
        .contact
        .v3
        .user
        .create_user_builder()
        .user(user.clone());

    // 条件性添加参数
    let use_open_id = true;
    if use_open_id {
        advanced_builder = advanced_builder.user_id_type("open_id");
    }

    // 可以选择性添加部门ID类型
    let include_department = true;
    if include_department {
        advanced_builder = advanced_builder.department_id_type("open_department_id");
    }

    // 执行请求
    match advanced_builder.execute(&client.contact.v3.user).await {
        Ok(response) => {
            println!("✅ 高级Builder调用成功");
            println!("   用户ID: {:?}", response.user.user_id);
            println!("   用户名: {:?}", response.user.name);
        }
        Err(e) => {
            println!("❌ 高级Builder调用失败: {e}");
            println!("   这可能是因为权限问题或测试环境限制");
        }
    }

    println!();

    // ==========================================
    // 错误处理最佳实践
    // ==========================================
    println!("🛡️  错误处理最佳实践");
    println!("展示: 统一错误处理、错误类型判断、重试策略\n");

    // 故意创建一个可能失败的请求（缺少必要参数）
    let incomplete_user = User {
        name: Some("不完整用户".to_string()),
        // 缺少email等必要字段
        ..Default::default()
    };

    let error_demo_result = client
        .contact
        .v3
        .user
        .create_user_builder()
        .user(incomplete_user)
        .user_id_type("open_id")
        .execute(&client.contact.v3.user)
        .await;

    match error_demo_result {
        Ok(response) => {
            println!("✅ 意外成功: {:?}", response.user.name);
        }
        Err(e) => {
            println!("❌ 预期错误示例:");
            println!("   错误信息: {e}");

            // 使用新的错误处理方法
            use open_lark::core::error::LarkAPIError;
            match &e {
                LarkAPIError::APIError { code, msg, .. } => {
                    println!("   错误码: {code}");
                    println!("   错误消息: {msg}");

                    // 根据错误码决定是否重试
                    if *code == 429 {
                        println!("   💡 建议: 请求频率过高，建议稍后重试");
                    } else if *code == 403 {
                        println!("   💡 建议: 权限不足，请检查应用权限配置");
                    }
                }
                LarkAPIError::DataError(msg) => {
                    println!("   数据错误: {msg}");
                }
                _ => {
                    println!("   其他错误类型");
                }
            }
        }
    }

    println!();

    // ==========================================
    // 最佳实践总结
    // ==========================================
    println!("📚 最佳实践总结:");
    println!("1. 🔄 新项目推荐使用Builder模式，提供更好的类型安全和可读性");
    println!("2. 🔧 现有项目可以逐步迁移，新旧模式可以并存");
    println!("3. 🛡️  统一使用新的错误处理机制，获得更详细的错误信息");
    println!("4. ⚡ Builder模式支持链式调用，代码更简洁优雅");
    println!("5. 🎯 对于简单调用，传统方式仍然有效且高效");
    println!("6. 🔍 使用.execute()方法获得统一的异步执行体验");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_lark::core::constants::AppType;

    #[test]
    fn test_builder_pattern_creation() {
        let client = LarkClient::builder("test_app_id", "test_app_secret")
            .with_app_type(AppType::SelfBuild)
            .build();

        // 测试Builder创建
        let builder = client.contact.v3.user.create_user_builder();
        let user = User {
            name: Some("测试用户".to_string()),
            ..Default::default()
        };

        let request = builder.user(user).user_id_type("open_id").build();

        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.user.name, Some("测试用户".to_string()));
    }

    #[test]
    fn test_traditional_pattern_creation() {
        let user = User {
            name: Some("传统用户".to_string()),
            email: Some("traditional@example.com".to_string()),
            ..Default::default()
        };

        let request = CreateUserRequest {
            user,
            user_id_type: Some("open_id".to_string()),
            department_id_type: None,
        };

        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.user.name, Some("传统用户".to_string()));
        assert_eq!(
            request.user.email,
            Some("traditional@example.com".to_string())
        );
    }
}
