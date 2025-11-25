//! 认证流程示例
//!
//! 演示如何使用Open-Lark SDK进行真实的飞书应用认证。
//! 此示例将调用实际的飞书开放平台API获取访问令牌。
//!
//! 学习目标：
//! - 理解真实的应用访问令牌（app_access_token）获取流程
//! - 了解令牌验证和状态检查的实际操作
//! - 掌握真实的令牌刷新机制和最佳实践
//! - 体验完整的飞书认证服务集成
//!
//! 环境要求：
//! - 需要真实的飞书应用凭据（APP_ID 和 APP_SECRET）
//! - 确保网络连接正常，能够访问飞书API服务器
//!
//! 运行方式：
//! ```bash
//! export OPENLARK_APP_ID="your_real_app_id"
//! export OPENLARK_APP_SECRET="your_real_app_secret"
//! cargo run --example authentication  # 认证服务现在是默认功能，无需特性标志
//! ```

// 引入依赖
// 导入共通工具函数
#[path = "../common/utils.rs"]
mod utils;

use utils::{print_error, print_example_footer, print_example_header, print_step, print_success};

// 检查网络连接
async fn check_network_connectivity() -> Result<(), Box<dyn std::error::Error>> {
    print_step(1, "检查网络连接状态");

    println!("🌐 正在检查飞书服务器连接...");

    let urls_to_check = [
        "https://open.feishu.cn",
        "https://open.feishu.cn/open-apis/auth/v3/app_access_token/internal",
    ];

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    for url in &urls_to_check {
        match client.head(*url).send().await {
            Ok(response) => {
                let status = response.status();
                if status.is_success() || status.as_u16() == 404 {
                    println!("✅ 网络连接正常: {} (状态: {})", url, status);
                    return Ok(());
                } else {
                    println!("⚠️  服务器响应异常: {} (状态: {})", url, status);
                }
            }
            Err(e) => {
                println!("❌ 网络连接失败: {} - {}", url, e);

                // 提供网络问题的详细诊断
                print_error("💡 网络问题诊断:");
                print_error("   1. 检查网络连接是否正常");
                print_error("   2. 确认防火墙未阻止 HTTPS 连接");
                print_error("   3. 检查代理服务器设置");
                print_error("   4. 验证 DNS 解析是否正常");
                print_error("   5. 尝试访问其他网站确认网络状态");

                return Err(format!("无法连接到飞书服务器: {}", e).into());
            }
        }
    }

    Ok(())
}

// 加载 .env 文件（如果存在）
fn load_env_file() {
    // 尝试从多个位置加载 .env 文件
    let env_paths = [
        ".env",                    // 当前目录
        "examples/.env",           // examples 目录
        "01_getting_started/.env", // 当前示例目录
    ];

    for path in &env_paths {
        if std::path::Path::new(path).exists() {
            match dotenvy::from_filename(path) {
                Ok(_) => {
                    println!("📁 已加载环境文件: {}", path);
                    return;
                }
                Err(e) => {
                    println!("⚠️  警告: 无法加载环境文件 {}: {}", path, e);
                }
            }
        }
    }

    println!("ℹ️  未找到 .env 文件，将使用系统环境变量");
}

// 使用 openlark-client 的认证服务（现在默认可用）
use openlark_client::services::AuthService;
use openlark_client::{Client, LarkClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    print_example_header(
        "认证流程",
        "演示Open-Lark SDK的应用认证和令牌管理 (支持 .env 文件)",
    );

    // 加载 .env 文件
    load_env_file();

    // 步骤1: 检查网络连接状态
    check_network_connectivity().await?;

    // 步骤2: 检查配置环境
    print_step(2, "检查认证配置环境");

    // 严格验证环境变量
    let app_id = std::env::var("OPENLARK_APP_ID").unwrap_or_default();
    let app_secret = std::env::var("OPENLARK_APP_SECRET").unwrap_or_default();

    if app_id.is_empty() || app_secret.is_empty() {
        print_error("❌ 缺少必需的环境变量配置");
        print_error("💡 请设置以下环境变量:");
        print_error("   export OPENLARK_APP_ID=\"your_real_app_id\"");
        print_error("   export OPENLARK_APP_SECRET=\"your_real_app_secret\"");
        print_error("📝 获取方式:");
        print_error("   1. 访问飞书开放平台: https://open.feishu.cn/app");
        print_error("   2. 创建或选择您的应用");
        print_error("   3. 在应用详情页面的「凭证与基础信息」中获取");
        print_error("🔧 也支持在当前目录创建 .env 文件:");
        print_error("   OPENLARK_APP_ID=your_real_app_id");
        print_error("   OPENLARK_APP_SECRET=your_real_app_secret");
        return Err("环境变量配置错误".into());
    }

    print_success("✅ 环境变量验证通过");
    println!(
        "📱 应用ID: {}...",
        &app_id.chars().take(8).collect::<String>()
    );
    println!(
        "🔑 应用密钥: {}...",
        &app_secret.chars().take(8).collect::<String>()
    );

    // 步骤3: 创建真实客户端和认证服务
    print_step(3, "创建真实客户端和认证服务");

    // 创建客户端配置
    let client = match Client::from_env() {
        Ok(client) => {
            print_success("✅ 飞书客户端创建成功");
            client
        }
        Err(e) => {
            print_error(&format!("❌ 飞书客户端创建失败: {}", e));
            print_error("💡 请确保设置了正确的环境变量:");
            print_error("   export OPENLARK_APP_ID=\"your_app_id\"");
            print_error("   export OPENLARK_APP_SECRET=\"your_app_secret\"");
            return Err(e.into());
        }
    };

    // 创建认证服务
    let auth_service = AuthService::new(client.config());
    print_success("✅ 认证服务创建成功");

    // 显示应用信息（脱敏）
    println!(
        "📱 应用ID: {}...",
        &client.app_id().chars().take(8).collect::<String>()
    );
    println!(
        "🔑 应用密钥: {}...",
        &client.app_secret().chars().take(8).collect::<String>()
    );

    // 步骤4: 获取真实应用访问令牌
    print_step(4, "获取真实应用访问令牌");

    let token_info = match auth_service.get_internal_app_access_token().await {
        Ok(token) => {
            print_success("🎉 真实令牌获取成功！");
            token
        }
        Err(e) => {
            print_error(&format!("❌ 令牌获取失败: {}", e));
            print_error("💡 故障排查指南:");
            print_error("   1. 🔧 应用凭证检查:");
            print_error("      - 确认 APP_ID 和 APP_SECRET 完全正确");
            print_error("      - 检查应用是否已发布并激活");
            print_error("   2. 🌐 网络连接检查:");
            print_error("      - 确保网络连接正常");
            print_error("      - 检查防火墙设置");
            print_error("      - 验证能访问 https://open.feishu.cn");
            print_error("   3. 🏢 应用权限检查:");
            print_error("      - 确认应用已获得必要权限");
            print_error("      - 检查应用状态是否为「已上线」");
            print_error("   4. 🔄 服务器状态:");
            print_error("      - 访问飞书开放平台状态页面");
            print_error("      - 稍后重试，可能是临时服务问题");
            return Err(e.into());
        }
    };

    // 步骤5: 显示令牌信息并验证
    print_step(5, "显示令牌信息并验证");

    println!("📋 令牌详细信息:");
    println!(
        "  🔑 访问令牌: {}...",
        token_info.access_token.chars().take(16).collect::<String>()
    );
    println!("  🏷️  令牌类型: {}", token_info.token_type);
    println!(
        "  ⏰ 有效期限: {} 秒 ({} 小时)",
        token_info.expires_in,
        token_info.expires_in / 3600
    );
    println!(
        "  🎯 过期时间: {}",
        token_info.expires_at.format("%Y-%m-%d %H:%M:%S UTC")
    );
    println!("  📜 权限范围: {:?}", token_info.scope);
    println!("  ⏳ 剩余时间: {} 秒", token_info.remaining_seconds());

    // 验证令牌
    print_step(6, "验证令牌有效性");

    match auth_service.verify_token(&token_info.access_token).await {
        Ok(verification_result) => {
            if verification_result.valid {
                print_success("✅ 令牌验证通过，令牌有效");
                if let Some(user_id) = verification_result.user_id {
                    println!("👤 关联用户ID: {}", user_id);
                }
                if let Some(tenant_key) = verification_result.tenant_key {
                    println!("🏢 租户标识: {}", tenant_key);
                }
                if !verification_result.scope.is_empty() {
                    println!("📜 权限范围: {}", verification_result.scope.join(", "));
                }
            } else {
                print_error("❌ 令牌验证失败，令牌无效或已过期");
            }
        }
        Err(e) => {
            print_error(&format!("❌ 令牌验证过程出错: {}", e));
            print_error("💡 令牌验证失败的可能原因:");
            print_error("   1. 🚫 令牌无效: 访问令牌可能已过期或被撤销");
            print_error("   2. 🌐 网络问题: 无法连接到飞书验证服务器");
            print_error("   3. 🔑 权限不足: 应用没有验证用户信息的权限");
            print_error("   4. ⏰ 令牌过期: app_access_token 有效期通常为2小时");
            print_error("🔧 解决方案:");
            print_error("   - 重新获取访问令牌");
            print_error("   - 检查应用权限配置");
            print_error("   - 验证网络连接状态");
        }
    }

    // 步骤7: 令牌状态检查
    print_step(7, "令牌状态检查");

    println!("🔍 令牌状态检查:");
    println!(
        "  ✅ 是否过期: {}",
        if token_info.is_expired() {
            "是"
        } else {
            "否"
        }
    );
    println!(
        "  ⚠️  需要刷新(提前30分钟): {}",
        if token_info.needs_refresh(30) {
            "是"
        } else {
            "否"
        }
    );

    if token_info.is_expired() {
        print_error("⚠️  令牌已过期，需要重新获取");
    } else if token_info.needs_refresh(30) {
        print_error("⚠️  令牌即将过期，建议刷新");
    } else {
        print_success("✅ 令牌状态良好，可以正常使用");
    }

    // 步骤8: 认证服务高级用法
    print_step(8, "认证服务高级用法");

    println!("🚀 认证服务高级用法:");
    println!("  1. 自动令牌管理 - SDK自动处理令牌刷新");
    println!("  2. 令牌缓存机制 - 提高性能，减少网络请求");
    println!("  3. 并发安全支持 - 多线程环境下的令牌管理");

    // 演示令牌刷新
    print_step(9, "令牌刷新机制演示");

    println!("🔄 令牌刷新机制:");
    println!("  - 当 app_access_token 即将过期时，可以使用 refresh_token 获取新令牌");
    println!("  - 新令牌的有效期通常为 2 小时");
    println!("  - 建议在令牌过期前 30 分钟进行刷新");

    // 模拟即将过期的令牌
    let mut near_expiry_token = token_info.clone();
    near_expiry_token.expires_at = chrono::Utc::now() + chrono::Duration::minutes(15);

    println!("🔍 模拟即将过期的令牌检查:");
    println!("  - 剩余时间: {} 秒", near_expiry_token.remaining_seconds());
    println!(
        "  - 需要刷新: {}",
        if near_expiry_token.needs_refresh(30) {
            "是"
        } else {
            "否"
        }
    );

    // 步骤9: 最佳实践
    print_step(10, "认证最佳实践");
    let best_practices = vec![
        "🔐 妥善保管应用密钥，不要在代码中硬编码",
        "🔄 定期刷新令牌，避免服务中断",
        "📊 监控令牌使用情况和有效期",
        "🚫 不要在前端代码中存储敏感的认证信息",
        "🔍 使用HTTPS协议进行所有认证请求",
        "⚡ 实施令牌缓存策略，提高应用性能",
        "🛡️ 设置适当的错误处理和重试机制",
        "📝 记录认证操作日志，便于问题排查",
    ];

    println!("认证最佳实践:");
    for (i, practice) in best_practices.iter().enumerate() {
        println!("  {}. {}", i + 1, practice);
    }

    print_example_footer(Some(
        "下一步：学习第一个API调用 -> cargo run --example first_api_call --features communication",
    ));

    Ok(())
}
