/**
 * OpenLark SDK 基础入门完整教程
 *
 * 本教程整合了OpenLark SDK的核心概念和基础操作，包含6章完整内容：
 *
 * 第1章：SDK简介与环境配置
 * 第2章：客户端建立与配置
 * 第3章：认证机制详解
 * 第4章：第一个API调用
 * 第5章：服务模块与功能标志
 * 第6章：错误处理与最佳实践
 *
 * 运行方法：
 * # 基础运行（仅包含核心功能）
 * cargo run --example basic_introduction
 *
 * # 完整功能运行（推荐）
 * cargo run --example basic_introduction --features "client,auth,communication,docs,ai,hr"
 *
 * 环境配置：
 * 1. 复制 .env-example 到 .env
 * 2. 在 .env 中配置真实的 APP_ID 和 APP_SECRET
 * 3. 可选配置 USER_ACCESS_TOKEN 用于用户级API
 */

use openlark_client::{Client, prelude::*};
use openlark_client::Result;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    println!("🚀 OpenLark SDK 基础入门完整教程");
    println!("================================");
    println!("📚 教程包含：SDK简介、客户端建立、认证机制、API调用、服务模块、错误处理");
    println!();

    // 第1章：SDK简介与环境配置
    chapter_1_introduction().await;
    println!();

    // 第2章：客户端建立与配置
    chapter_2_client_setup().await?;
    println!();

    // 第3章：认证机制详解
    chapter_3_authentication().await?;
    println!();

    // 第4章：第一个API调用
    chapter_4_first_api_call().await?;
    println!();

    // 第5章：服务模块与功能标志
    chapter_5_services_modules().await?;
    println!();

    // 第6章：错误处理与最佳实践
    chapter_6_error_handling().await?;

    println!("🎉 教程完成！");
    println!("💡 接下来可以：");
    println!("   • 查看 services/ 目录中的服务特定示例");
    println!("   • 阅读 patterns/ 目录中的最佳实践");
    println!("   • 探索 enterprise/ 目录中的企业级场景");

    Ok(())
}

/**
 * 第1章：SDK简介与环境配置
 */
async fn chapter_1_introduction() {
    println!("📖 第1章：SDK简介与环境配置");
    println!("===========================");

    println!("🔍 OpenLark SDK 简介:");
    println!("   • 高覆盖率：51个服务模块，1,134+个API");
    println!("   • 企业级：高级错误处理、重试机制、监控支持");
    println!("   • 类型安全：零警告编译、完整测试覆盖");
    println!("   • 中文文档：100%中文文档，专为中国开发者优化");
    println!("   • 现代架构：模块化crates、构建器模式、异步支持");
    println!();

    println!("🏗️ 架构特点:");
    println!("   • 模块化设计：按需启用服务模块");
    println!("   • 功能标志：减少二进制文件大小");
    println!("   • 统一客户端：LarkClient 提供一致接口");
    println!("   • 自动令牌管理：SDK自动处理令牌获取和刷新");
    println!();

    println!("🔧 环境配置检查:");

    // 检查必需的环境变量
    let app_id = env::var("OPENLARK_APP_ID");
    let app_secret = env::var("OPENLARK_APP_SECRET");
    let user_token = env::var("OPENLARK_USER_ACCESS_TOKEN");

    match (&app_id, &app_secret) {
        (Ok(id), Ok(secret)) => {
            println!("   ✅ 基础配置已设置:");
            println!("     • APP_ID: {}...", &id[..id.len().min(8)]);
            println!("     • APP_SECRET: {}...", &secret[..secret.len().min(8)]);
        }
        _ => {
            println!("   ⚠️  基础配置缺失:");
            println!("     • 请在 .env 文件中配置 OPENLARK_APP_ID 和 OPENLARK_APP_SECRET");
            println!("     • 可以从 .env-example 复制模板");
        }
    }

    match user_token {
        Ok(token) => {
            println!("   ✅ 用户令牌已设置: {}...", &token[..token.len().min(8)]);
        }
        Err(_) => {
            println!("   ℹ️  用户令牌未设置（可选）");
        }
    }

    println!();
    println!("📋 支持的服务模块:");
    println!("   • communication - 即时消息、联系人管理");
    println!("   • docs - 云文档、表格、知识库");
    println!("   • hr - 人力资源、考勤管理");
    println!("   • ai - AI智能服务");
    println!("   • auth - 认证和权限管理");
    println!();
}

/**
 * 第2章：客户端建立与配置
 */
async fn chapter_2_client_setup() -> openlark_client::Result<()> {
    println!("📖 第2章：客户端建立与配置");
    println!("=========================");

    println!("🔧 客户端创建方式:");
    println!();

    // 方式1：构建器模式（推荐）
    println!("📋 方式1：构建器模式（推荐）");
    println!("----------------------------");

    match Client::builder()
        .app_id("demo_app_id")
        .app_secret("demo_app_secret")
        .base_url("https://open.feishu.cn")
        .enable_log(true)
        .build()
    {
        Ok(client) => {
            println!("✅ 构建器模式创建成功");
            println!("   • App ID: {}", client.config().app_id);
            println!("   • Base URL: {}", client.config().base_url);
        }
        Err(e) => {
            println!("❌ 构建器模式创建失败: {}", e);
        }
    }
    println!();

    // 方式2：从环境变量创建
    println!("📋 方式2：从环境变量创建");
    println!("------------------------");

    match Client::from_env() {
        Ok(client) => {
            println!("✅ 从环境变量创建成功");
            println!("   • App ID: {}...",
                     &client.config().app_id[..client.config().app_id.len().min(8)]);
            println!("   • Base URL: {}", client.config().base_url);
            println!("   • 令牌缓存: 启用 (SDK自动管理)");

            // 演示客户端配置
            demo_client_features(&client).await?;
        }
        Err(e) => {
            println!("⚠️  从环境变量创建失败: {}", e);
            println!("💡 请在 .env 文件中配置:");
            println!("   OPENLARK_APP_ID=your_app_id");
            println!("   OPENLARK_APP_SECRET=your_app_secret");
            println!("   OPENLARK_BASE_URL=https://open.feishu.cn (可选)");
        }
    }

    println!();
    println!("💡 客户端配置最佳实践:");
    println!("   • 使用环境变量存储敏感信息");
    println!("   • 启用令牌缓存提高性能");
    println!("   • 根据环境设置不同的base_url");
    println!("   • 在生产环境中启用日志记录");

    Ok(())
}

/**
 * 演示客户端功能特性
 */
async fn demo_client_features(client: &Client) -> openlark_client::Result<()> {
    println!();
    println!("🔧 客户端功能特性:");

    // 服务注册信息
    let registry = client.registry();
    let services = registry.list_services();

    println!("   • 已注册服务: {} 个", services.len());
    for service in services.iter().take(5) { // 最多显示5个
        println!("     - {}", service.name);
    }

    Ok(())
}

/**
 * 第3章：认证机制详解
 */
async fn chapter_3_authentication() -> openlark_client::Result<()> {
    println!("📖 第3章：认证机制详解");
    println!("=====================");

    println!("🔐 认证类型:");
    println!("   • 应用级认证：tenant_access_token（应用权限）");
    println!("   • 用户级认证：user_access_token（用户权限）");
    println!();

    // 尝试创建客户端进行认证演示
    let client = match Client::from_env() {
        Ok(client) => client,
        Err(_) => {
            println!("⚠️  跳过实际认证演示（需要有效的环境配置）");
            println!("💡 以下为认证代码示例:");
            show_authentication_examples();
            return Ok(());
        }
    };

    #[cfg(feature = "auth")]
    {
        println!("🔑 实际认证演示:");

        match client.auth().get_app_access_token().await {
            Ok(token_info) => {
                println!("✅ 应用级认证成功");
                println!("   • 访问令牌: {}...",
                         &token_info.access_token[..token_info.access_token.len().min(20)]);
                println!("   • 令牌类型: {}", token_info.token_type);
                println!("   • 过期时间: {} 秒", token_info.expires_in);

                // 验证令牌
                match client.auth().verify_token(&token_info.access_token).await {
                    Ok(verification) => {
                        if verification.valid {
                            println!("   ✅ 令牌验证成功");
                        } else {
                            println!("   ❌ 令牌验证失败");
                        }
                    }
                    Err(e) => {
                        println!("   ⚠️  令牌验证失败: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("❌ 应用级认证失败: {}", e);
                println!("💡 可能的原因:");
                println!("   • App ID 或 App Secret 错误");
                println!("   • 网络连接问题");
                println!("   • 应用权限配置错误");
            }
        }
    }

    #[cfg(not(feature = "auth"))]
    {
        println!("ℹ️  认证功能未启用");
        println!("💡 请启用 auth 功能标志: --features auth");
        show_authentication_examples();
    }

    println!();
    println!("🛡️ 认证最佳实践:");
    println!("   • SDK自动处理令牌获取和刷新");
    println!("   • 令牌缓存减少重复认证请求");
    println!("   • 定期检查应用权限设置");
    println!("   • 使用最小权限原则配置应用");

    Ok(())
}

/**
 * 显示认证代码示例
 */
fn show_authentication_examples() {
    println!();
    println!("📝 认证代码示例:");
    println!("```rust");
    println!("use openlark_client::prelude::*;");
    println!();
    println!("// 创建客户端");
    println!("let client = Client::from_env()?;");
    println!();
    println!("// 应用级认证");
    println!("let token_info = client.auth().get_app_access_token().await?;");
    println!("println!(\"令牌: {{}}\", token_info.access_token);");
    println!();
    println!("// 用户级认证（需要授权码）");
    println!("let user_token = client.auth()");
    println!("    .get_user_access_token(\"auth_code\").await?;");
    println!();
    println!("// 令牌验证");
    println!("let verification = client.auth()");
    println!("    .verify_token(&token_info.access_token).await?;");
    println!("```");
}

/**
 * 第4章：第一个API调用
 */
async fn chapter_4_first_api_call() -> openlark_client::Result<()> {
    println!("📖 第4章：第一个API调用");
    println!("=====================");

    println!("📋 API调用流程:");
    println!("   1. 🔐 客户端认证 - SDK自动处理");
    println!("   2. 📝 调用API方法 - 使用服务接口");
    println!("   3. 📊 处理响应 - 解析返回数据");
    println!("   4. ⚠️  错误处理 - 捕获和处理异常");
    println!();

    let client = match Client::from_env() {
        Ok(client) => client,
        Err(_) => {
            println!("⚠️  跳过实际API调用演示（需要有效的环境配置）");
            show_api_call_examples();
            return Ok(());
        }
    };

    #[cfg(feature = "auth")]
    {
        println!("📞 实际API调用演示:");

        match client.auth().get_app_access_token().await {
            Ok(token_info) => {
                println!("✅ API调用成功");
                println!("📱 响应数据结构:");
                println!("   • access_token: String (访问令牌)");
                println!("   • token_type: String (令牌类型)");
                println!("   • expires_in: u32 (过期时间，秒)");
                println!("   • refresh_token: Option<String> (刷新令牌)");
                println!("   • scope: Option<String> (权限范围)");
                println!();
                println!("💡 数据访问示例:");
                println!("   • 访问令牌: {}...",
                         &token_info.access_token[..token_info.access_token.len().min(20)]);
                println!("   • 令牌类型: {}", token_info.token_type);
                println!("   • 过期时间: {} 秒", token_info.expires_in);
            }
            Err(e) => {
                println!("❌ API调用失败: {}", e);
                analyze_api_error(&e);
            }
        }
    }

    #[cfg(not(feature = "auth"))]
    {
        println!("ℹ️  认证功能未启用，无法进行实际API调用");
        show_api_call_examples();
    }

    println!();
    println!("💡 API调用最佳实践:");
    println!("   • 使用 ? 操作符进行错误传播");
    println!("   • 检查响应数据的完整性");
    println!("   • 合理设置超时时间");
    println!("   • 实施适当的重试机制");

    Ok(())
}

/**
 * 显示API调用示例
 */
fn show_api_call_examples() {
    println!();
    println!("📝 API调用代码示例:");
    println!("```rust");
    println!("use openlark_client::prelude::*;");
    println!();
    println!("// 认证API调用");
    println!("let token_info = client.auth()");
    println!("    .get_app_access_token().await?;");
    println!();
    println!("// 通讯API调用（需要 communication 功能）");
    println!("#[cfg(feature = \"communication\")]");
    println!("let response = client.communication()");
    println!("    .send_text(\"user_id\", \"Hello World!\").await?;");
    println!();
    println!("// 文档API调用（需要 docs 功能）");
    println!("#[cfg(feature = \"docs\")]");
    println!("let spreadsheet = client.docs()");
    println!("    .create_spreadsheet(\"My Sheet\", None).await?;");
    println!();
    println!("// AI API调用（需要 ai 功能）");
    println!("#[cfg(feature = \"ai\")]");
    println!("let ai_response = client.ai()");
    println!("    .chat_completion(messages, None, None, None).await?;");
    println!("```");
}

/**
 * 分析API错误
 */
fn analyze_api_error(error: &openlark_client::Error) {
    println!("🔍 错误分析:");
    match error {
        openlark_client::Error::NetworkError(e) => {
            println!("   • 类型: 网络错误");
            println!("   • 原因: {}", e);
            println!("   • 建议: 检查网络连接和防火墙设置");
        }
        openlark_client::Error::APIError { code, message } => {
            println!("   • 类型: API错误");
            println!("   • 代码: {}", code);
            println!("   • 消息: {}", message);
            println!("   • 建议: 检查应用权限和参数设置");
        }
        _ => {
            println!("   • 类型: 其他错误");
            println!("   • 错误: {}", error);
            println!("   • 建议: 检查配置和网络连接");
        }
    }
}

/**
 * 第5章：服务模块与功能标志
 */
async fn chapter_5_services_modules() -> openlark_client::Result<()> {
    println!("📖 第5章：服务模块与功能标志");
    println!("=============================");

    println!("🔧 功能标志系统:");
    println!("   • 按需编译：仅包含需要的服务模块");
    println!("   • 减少体积：显著减小最终二进制文件大小");
    println!("   • 灵活配置：支持多种功能组合");
    println!();

    let client = Client::builder()
        .app_id("demo_app_id")
        .app_secret("demo_app_secret")
        .build()?;

    println!("📋 当前启用的服务模块:");
    let registry = client.registry();

    // 检查各个服务模块的启用状态
    check_service_status(registry, "auth", "认证服务", "应用认证、令牌管理");
    check_service_status(registry, "communication", "通讯服务", "IM消息、联系人管理");
    check_service_status(registry, "docs", "文档服务", "云文档、表格、知识库");
    check_service_status(registry, "hr", "人力资源服务", "考勤管理、CoreHR");
    check_service_status(registry, "ai", "AI智能服务", "AI对话、智能分析");

    println!();
    println!("📝 功能标志使用示例:");
    println!("```toml");
    println!("# Cargo.toml");
    println!("[dependencies]");
    println!("open-lark = {{ version = \"0.13.2\", features = [");
    println!("    \"client\",        # 统一客户端");
    println!("    \"auth\",          # 认证服务");
    println!("    \"communication\", # 通讯服务");
    println!("    \"docs\",          # 文档服务");
    println!("] }}");
    println!("```");
    println!();
    println!("💡 运行时指定功能:");
    println!("```bash");
    println!("cargo run --example basic_introduction --features \"client,auth,communication\"");
    println!("```");

    println!();
    println!("🚀 服务模块演示:");

    // 根据启用的功能显示相应的API示例
    #[cfg(feature = "auth")]
    {
        println!("   ✅ 认证服务API示例:");
        println!("       client.auth().get_app_access_token().await");
    }

    #[cfg(feature = "communication")]
    {
        println!("   ✅ 通讯服务API示例:");
        println!("       client.communication().send_text(user_id, message).await");
    }

    #[cfg(feature = "docs")]
    {
        println!("   ✅ 文档服务API示例:");
        println!("       client.docs().create_spreadsheet(title, None).await");
    }

    #[cfg(feature = "ai")]
    {
        println!("   ✅ AI服务API示例:");
        println!("       client.ai().chat_completion(messages, None, None, None).await");
    }

    #[cfg(feature = "hr")]
    {
        println!("   ✅ 人力资源服务API示例:");
        println!("       client.hr().get_attendance_records(user_id, start_date, end_date).await");
    }

    println!();
    println!("💡 功能标志最佳实践:");
    println!("   • 仅启用需要的功能模块");
    println!("   • 在开发和生产环境使用一致的功能配置");
    println!("   • 定期审查未使用的功能标志");
    println!("   • 使用条件编译处理可选功能");

    Ok(())
}

/**
 * 检查服务状态
 */
fn check_service_status(registry: &openlark_client::ServiceRegistry, service_name: &str, display_name: &str, description: &str) {
    println!("   📋 {} ({})", display_name, service_name);

    #[cfg(feature = "auth")]
    if service_name == "auth" {
        if registry.has_service(service_name) {
            println!("       ✅ 已启用 - {}", description);
        } else {
            println!("       ❌ 未注册 - {}", description);
        }
        return;
    }

    #[cfg(feature = "communication")]
    if service_name == "communication" {
        if registry.has_service(service_name) {
            println!("       ✅ 已启用 - {}", description);
        } else {
            println!("       ❌ 未注册 - {}", description);
        }
        return;
    }

    #[cfg(feature = "docs")]
    if service_name == "docs" {
        if registry.has_service(service_name) {
            println!("       ✅ 已启用 - {}", description);
        } else {
            println!("       ❌ 未注册 - {}", description);
        }
        return;
    }

    #[cfg(feature = "hr")]
    if service_name == "hr" {
        if registry.has_service(service_name) {
            println!("       ✅ 已启用 - {}", description);
        } else {
            println!("       ❌ 未注册 - {}", description);
        }
        return;
    }

    #[cfg(feature = "ai")]
    if service_name == "ai" {
        if registry.has_service(service_name) {
            println!("       ✅ 已启用 - {}", description);
        } else {
            println!("       ❌ 未注册 - {}", description);
        }
        return;
    }

    // 如果对应功能未启用
    println!("       ⚪️  功能未启用 - {} (使用 --features {} 启用)", description, service_name);
}

/**
 * 第6章：错误处理与最佳实践
 */
async fn chapter_6_error_handling() -> openlark_client::Result<()> {
    println!("📖 第6章：错误处理与最佳实践");
    println!("=============================");

    println!("🔍 常见错误类型:");
    println!("   1️⃣ 配置错误: App ID/Secret无效、格式错误");
    println!("   2️⃣ 网络错误: 连接超时、DNS解析失败、网络中断");
    println!("   3️⃣ API错误: 权限不足、资源不存在、参数错误");
    println!("   4️⃣ 业务错误: 令牌过期、调用频率限制");
    println!();

    println!("🔧 错误处理策略:");

    // 策略1: 立即失败 - 配置错误
    println!("   1️⃣ 立即失败策略（配置错误）:");
    match Client::builder()
        .app_id("")
        .app_secret("")
        .build()
    {
        Ok(_) => println!("       ⚠️  意外成功"),
        Err(e) => {
            println!("       ✅ 正确识别配置错误");
            handle_configuration_error(&e);
        }
    }

    // 策略2: 环境变量检查
    println!("   2️⃣ 环境变量策略:");
    match Client::from_env() {
        Ok(client) => {
            println!("       ✅ 环境变量配置正确");

            #[cfg(feature = "auth")]
            {
                // 尝试实际API调用
                match client.auth().get_app_access_token().await {
                    Ok(_) => println!("       ✅ API调用成功"),
                    Err(e) => {
                        println!("       ⚠️  API调用失败");
                        handle_api_error(&e);
                    }
                }
            }
        }
        Err(e) => {
            println!("       ⚠️  环境变量配置错误");
            handle_configuration_error(&e);
        }
    }

    println!();
    println!("💡 错误处理最佳实践:");
    println!("   • 使用 ? 操作符进行错误传播");
    println!("   • 提供用户友好的错误消息");
    println!("   • 记录详细的错误日志用于调试");
    println!("   • 实施适当的重试机制");
    println!("   • 区分可恢复和不可恢复错误");
    println!();

    println!("🛡️ 生产环境建议:");
    println!("   • 启用结构化日志记录");
    println!("   • 实施监控和告警机制");
    println!("   • 配置合理的超时和重试参数");
    println!("   • 定期检查和更新应用权限");
    println!("   • 实施优雅降级策略");

    Ok(())
}

/**
 * 处理配置错误
 */
fn handle_configuration_error(error: &openlark_client::Error) {
    match error {
        openlark_client::Error::InvalidConfig(msg) => {
            println!("       💡 配置错误: {}", msg);
            println!("       📝 解决方案:");
            println!("         1. 检查 .env 文件是否存在");
            println!("         2. 验证 OPENLARK_APP_ID 是否正确");
            println!("         3. 验证 OPENLARK_APP_SECRET 是否正确");
        }
        openlark_client::Error::NetworkError(e) => {
            println!("       💡 网络错误: {}", e);
            println!("       📝 解决方案:");
            println!("         1. 检查网络连接");
            println!("         2. 验证防火墙设置");
            println!("         3. 检查代理配置");
        }
        _ => {
            println!("       💡 其他错误: {}", error);
            println!("       📝 请检查配置和网络连接");
        }
    }
}

/**
 * 处理API错误
 */
#[cfg(feature = "auth")]
fn handle_api_error(error: &openlark_client::Error) {
    match error {
        openlark_client::Error::APIError { code, message } => {
            println!("       💡 API错误 - 代码: {}, 消息: {}", code, message);

            match code.as_str() {
                "99991663" => {
                    println!("       📝 解决方案: 检查App ID和App Secret是否正确");
                }
                "99991664" => {
                    println!("       📝 解决方案: 重新配置应用密钥");
                }
                "99991400" => {
                    println!("       📝 解决方案: 重新获取访问令牌");
                }
                "99991403" => {
                    println!("       📝 解决方案: 检查应用权限设置");
                }
                "429" => {
                    println!("       📝 解决方案: 降低请求频率，实施限流");
                }
                _ => {
                    println!("       📝 解决方案: 检查请求参数和应用权限");
                }
            }
        }
        _ => {
            println!("       💡 其他API错误: {}", error);
            println!("       📝 请检查网络连接和配置");
        }
    }
}

/**
 * 用户友好的错误消息
 */
fn user_friendly_error_message(error: &openlark_client::Error) -> String {
    match error {
        openlark_client::Error::InvalidConfig(_) => {
            "应用配置有误，请检查App ID和App Secret是否正确".to_string()
        }
        openlark_client::Error::NetworkError(_) => {
            "网络连接失败，请检查网络连接后重试".to_string()
        }
        openlark_client::Error::APIError { code, .. } => {
            match code.as_str() {
                "99991663" => "应用ID或密钥无效，请联系管理员".to_string(),
                "99991403" => "权限不足，请联系管理员配置应用权限".to_string(),
                "429" => "请求过于频繁，请稍后重试".to_string(),
                _ => "系统繁忙，请稍后重试".to_string(),
            }
        }
        openlark_client::Error::InvalidParameter(msg) => {
            format!("参数错误: {}", msg)
        }
        _ => {
            "系统错误，请稍后重试".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        // 设置测试环境变量
        env::set_var("OPENLARK_APP_ID", "test_app_id");
        env::set_var("OPENLARK_APP_SECRET", "test_app_secret");

        let result = Client::from_env();
        assert!(result.is_ok(), "从环境变量创建客户端应该成功");

        // 清理环境变量
        env::remove_var("OPENLARK_APP_ID");
        env::remove_var("OPENLARK_APP_SECRET");
    }

    #[test]
    fn test_builder_pattern() {
        let result = Client::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        assert!(result.is_ok(), "构建器模式创建客户端应该成功");

        if let Ok(client) = result {
            assert_eq!(client.config().app_id, "test_app_id");
            assert_eq!(client.config().app_secret, "test_app_secret");
        }
    }

    #[test]
    fn test_error_handling() {
        // 测试配置错误处理
        let result = Client::builder()
            .app_id("")
            .app_secret("")
            .build();

        assert!(result.is_err(), "空配置应该返回错误");

        if let Err(e) = result {
            let friendly_msg = user_friendly_error_message(&e);
            assert!(!friendly_msg.is_empty(), "用户友好消息不应为空");
        }
    }

    #[test]
    fn test_service_registry() {
        let client = Client::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .expect("客户端创建应该成功");

        let registry = client.registry();
        let services = registry.list_services();

        // 至少应该有基础服务
        assert!(!services.is_empty(), "应该至少有一个服务注册");
    }
}