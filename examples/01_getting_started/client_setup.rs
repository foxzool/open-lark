//! 客户端初始化示例
//!
//! 演示如何简单配置和初始化Open-Lark SDK客户端。
//!
//! 学习目标：
//! - 掌握基础客户端创建
//! - 理解环境变量配置
//! - 了解不同客户端创建方式
//!
//! 运行方式：
//! ```bash
//! cargo run --example client_setup
//! ```

// 引入依赖
use openlark_client::{Client, LarkClient};

// 加载 .env 文件（如果存在）
fn load_env_file() {
    match dotenvy::dotenv() {
        Ok(_) => {
            println!("📁 已加载环境文件: .env");
        }
        Err(_) => {
            // .env 文件不存在，继续使用系统环境变量
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 客户端初始化示例");
    println!("═════════════════════════════════════════════════\n");

    // 加载 .env 文件
    load_env_file();

    // 步骤1: 环境变量检查
    println!("📍 步骤1: 检查环境变量");
    let app_id = std::env::var("OPENLARK_APP_ID").unwrap_or_default();
    let app_secret = std::env::var("OPENLARK_APP_SECRET").unwrap_or_default();

    if app_id.is_empty() || app_secret.is_empty() {
        println!("❌ 缺少必需的环境变量");
        println!("💡 请设置以下环境变量:");
        println!("   export OPENLARK_APP_ID=\"your_app_id\"");
        println!("   export OPENLARK_APP_SECRET=\"your_app_secret\"");
        println!("🔧 或创建 .env 文件:");
        println!("   OPENLARK_APP_ID=your_app_id");
        println!("   OPENLARK_APP_SECRET=your_app_secret");
        return Err("环境变量配置错误".into());
    }
    println!("✅ 环境变量检查通过\n");

    // 步骤2: 创建客户端（从环境变量）
    println!("📍 步骤2: 从环境变量创建客户端");
    match Client::from_env() {
        Ok(client) => {
            println!("✅ 客户端创建成功");
            println!(
                "📱 应用ID: {}...",
                &client.app_id().chars().take(8).collect::<String>()
            );
        }
        Err(e) => {
            println!("❌ 客户端创建失败: {}", e);
            return Err(e.into());
        }
    }
    println!();

    // 步骤3: 创建客户端（构建器模式）
    println!("📍 步骤3: 使用构建器模式创建客户端");
    match Client::builder()
        .app_id(&app_id)
        .app_secret(&app_secret)
        .base_url("https://open.feishu.cn")
        .build()
    {
        Ok(client) => {
            println!("✅ 构建器客户端创建成功");
            println!("🔗 基础URL: {}", client.config().base_url);
        }
        Err(e) => {
            println!("❌ 构建器客户端创建失败: {}", e);
            return Err(e.into());
        }
    }
    println!();

    // 步骤4: 创建测试客户端
    println!("📍 步骤4: 创建测试客户端（使用默认值）");
    match Client::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build()
    {
        Ok(_) => {
            println!("✅ 测试客户端创建成功");
        }
        Err(e) => {
            println!("❌ 测试客户端创建失败: {}", e);
        }
    }
    println!();

    println!("🎉 客户端初始化示例完成！");
    println!("💡 下一步: 运行认证示例 -> cargo run --example authentication");
    println!("═════════════════════════════════════════════════\n");

    Ok(())
}
