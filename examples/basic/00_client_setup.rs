/**
 * OpenLark SDK 客户端建立示例
 *
 * 本示例展示如何创建 OpenLark 客户端：
 * - 从凭据创建客户端
 * - 从环境变量创建客户端
 * - 基础认证功能
 *
 * 运行方法：
 * cargo run --example 00_client_setup
 *
 * 环境配置：
 * 复制 .env-example 到 .env 并配置 OPENLARK_APP_ID 和 OPENLARK_APP_SECRET
 */

use openlark_client::minimal::{AuthClient, MinimalLarkClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    println!("🚀 OpenLark SDK 客户端建立示例");
    println!("=================================");
    println!();

    // === 方式1: 从凭据创建客户端 ===
    println!("📋 方式1: 从凭据创建客户端");
    println!("-------------------------");

    let client = MinimalLarkClient::new(
        "your_app_id".to_string(),
        "your_app_secret".to_string()
    )?;

    println!("✅ 凭据方式创建客户端成功");
    println!();

    // === 方式2: 从环境变量创建客户端 ===
    println!("📋 方式2: 从环境变量创建客户端");
    println!("-----------------------------");

    match MinimalLarkClient::from_env() {
        Ok(client) => {
            println!("✅ 环境变量方式创建客户端成功");
            println!("🔧 配置信息:");
            println!("   • App ID: {}...", &client.app_id()[..client.app_id().len().min(8)]);
            println!("   • App Secret: {}...", &client.app_secret()[..client.app_secret().len().min(8)]);
        }
        Err(e) => {
            println!("⚠️  环境变量方式创建客户端失败: {}", e);
            println!("💡 请在 .env 文件中配置:");
            println!("   OPENLARK_APP_ID=your_app_id");
            println!("   OPENLARK_APP_SECRET=your_app_secret");
        }
    }
    println!();

    // === 认证功能演示 ===
    println!("📋 认证功能演示");
    println!("----------------");

    println!("🔑 获取应用访问令牌:");
    println!("```rust");
    println!("let token = client.get_app_access_token().await?;");
    println!("println!(\"令牌: {{}}\", token.access_token);");
    println!("```");
    println!();

    println!("👤 获取用户访问令牌:");
    println!("```rust");
    println!("let token = client.get_user_access_token(\"auth_code\").await?;");
    println!("```");
    println!();

    println!("🔄 刷新访问令牌:");
    println!("```rust");
    println!("let token = client.refresh_access_token(\"refresh_token\").await?;");
    println!("```");
    println!();

    // === 更多示例 ===
    println!("📚 更多示例:");
    println!("   • 01_authentication.rs - 详细认证示例");
    println!("   • 02_first_api_call.rs - 第一个API调用");
    println!("   • 03_error_handling.rs - 错误处理");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_client_creation() {
        env::set_var("OPENLARK_APP_ID", "test_app_id");
        env::set_var("OPENLARK_APP_SECRET", "test_app_secret");

        let result = MinimalLarkClient::from_env();
        assert!(result.is_ok());

        env::remove_var("OPENLARK_APP_ID");
        env::remove_var("OPENLARK_APP_SECRET");
    }

    #[test]
    fn test_from_env_without_variables() {
        env::remove_var("OPENLARK_APP_ID");
        env::remove_var("OPENLARK_APP_SECRET");

        let result = MinimalLarkClient::from_env();
        assert!(result.is_err());
    }
}