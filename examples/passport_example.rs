//! Passport服务示例
//!
//! 演示如何使用passport服务的退出登录功能

use open_lark::core::config::{Config, ConfigBuilder};
use open_lark::prelude::*;
use open_lark::service::passport::models::LogoutRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    env_logger::init();

    // 创建客户端配置
    let config = Config::builder()
        .app_id("your_app_id")
        .app_secret("your_app_secret")
        .build();

    // 创建客户端（需要passport功能标志）
    #[cfg(feature = "passport")]
    {
        let client = LarkClient::new(config);

        // 示例1: 使用传统方法调用logout API
        let logout_request = LogoutRequest {
            user_ids: vec!["test_user_123".to_string()],
            user_id_type: "open_id".to_string(),
        };

        match client.passport.v1.sessions.logout(&logout_request).await {
            Ok(response) => {
                println!("退出登录成功!");
                println!("成功退出用户数: {}", response.success_count);
                println!("失败用户数: {}", response.failed_count);
            }
            Err(e) => {
                eprintln!("退出登录失败: {}", e);
            }
        }

        // 示例2: 使用构建器模式调用logout API
        match client
            .passport
            .v1
            .sessions
            .logout_builder()
            .user_id("test_user_456")
            .user_id_type("user_id")
            .execute()
            .await
        {
            Ok(response) => {
                println!("构建器模式退出登录成功!");
                println!("成功退出用户数: {}", response.success_count);
            }
            Err(e) => {
                eprintln!("构建器模式退出登录失败: {}", e);
            }
        }
    }

    #[cfg(not(feature = "passport"))]
    {
        println!(
            "请启用passport功能标志: cargo run --example passport_example --features passport"
        );
    }

    Ok(())
}
