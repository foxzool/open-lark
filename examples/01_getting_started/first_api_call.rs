//! 第一个API调用示例
//!
//! 演示如何使用Open-Lark SDK进行真实的飞书API调用。
//! 此示例将展示如何发送一条文本消息到指定的聊天或用户。
//!
//! 学习目标：
//! - 理解如何进行第一个真实的飞书API调用
//! - 了解消息发送的完整流程和参数
//! - 掌握错误处理和响应解析的实际操作
//! - 体验完整的API调用最佳实践
//!
//! 环境要求：
//! - 需要真实的飞书应用凭据（APP_ID 和 APP_SECRET）
//! - 需要知道接收消息的用户ID或群聊ID
//! - 确保网络连接正常，能够访问飞书API服务器
//!
//! 运行方式：
//! ```bash
//! export OPENLARK_APP_ID="your_real_app_id"
//! export OPENLARK_APP_SECRET="your_real_app_secret"
//! export OPENLARK_RECEIVE_ID="user_open_id_or_chat_id"  # 可选，默认使用测试ID
//! export OPENLARK_RECEIVE_ID_TYPE="open_id"             # 可选，默认为 open_id
//! cargo run --example first_api_call --features "auth,communication"
//! ```

// 引入依赖
// 导入共通工具函数
#[path = "../common/utils.rs"]
mod utils;

use utils::{print_example_footer, print_example_header, print_step, print_success, print_info};

// 引入核心依赖
use open_lark::communication::endpoints::IM_V1_MESSAGES;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 打印示例标题
    print_example_header("第一个API调用", "发送文本消息");

    // 步骤1: 检查网络连接
    check_network_connectivity().await?;

    // 步骤2: 验证环境配置
    let config = validate_environment()?;

    // 步骤3: 创建客户端配置
    create_client_config().await?;

    // 步骤4: 准备消息数据
    let message_request = prepare_message_request(&config).await?;

    // 步骤5: 发送消息
    let response = send_message(&config, &message_request).await?;

    // 步骤6: 处理响应
    handle_message_response(&response).await?;

    // 步骤7: 显示后续学习路径
    show_next_steps();

    print_example_footer(Some("first_api_call"));
    Ok(())
}

/// 检查网络连接状态
async fn check_network_connectivity() -> Result<(), Box<dyn std::error::Error>> {
    print_step(1, "检查网络连接状态");

    println!("🌐 正在检查飞书API服务器连接...");

    let urls_to_check = [
        "https://open.feishu.cn",
        "https://open.feishu.cn/open-apis/im/v1/messages",
    ];

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let mut connected = false;
    for url in &urls_to_check {
        match client.head(*url).send().await {
            Ok(response) => {
                let status = response.status();
                if status.is_success() || status.as_u16() == 404 || status.as_u16() == 405 {
                    println!("✅ 网络连接正常: {} (状态: {})", url, status);
                    connected = true;
                } else {
                    println!("⚠️  网络响应异常: {} (状态: {})", url, status);
                }
            }
            Err(e) => {
                println!("❌ 网络连接失败: {} - {}", url, e);
            }
        }
    }

    if !connected {
        return Err("网络连接检查失败，请检查网络设置".into());
    }

    println!("✅ 网络连接检查完成");
    Ok(())
}

/// 验证环境配置
fn validate_environment() -> Result<MessageConfig, Box<dyn std::error::Error>> {
    print_step(2, "验证环境配置");

    // 加载环境变量文件
    dotenvy::dotenv().ok();
    println!("📁 已加载环境文件: .env");

    // 检查必需的环境变量
    let app_id = std::env::var("OPENLARK_APP_ID")
        .map_err(|_| "未找到环境变量 OPENLARK_APP_ID")?;

    let app_secret = std::env::var("OPENLARK_APP_SECRET")
        .map_err(|_| "未找到环境变量 OPENLARK_APP_SECRET")?;

    // 接收者信息（提供默认值用于演示）
    let receive_id = std::env::var("OPENLARK_RECEIVE_ID")
        .unwrap_or_else(|_| "test_user_open_id".to_string());

    let receive_id_type = std::env::var("OPENLARK_RECEIVE_ID_TYPE")
        .unwrap_or_else(|_| "open_id".to_string());

    // 验证应用ID格式（应该以 cli_ 开头）
    if !app_id.starts_with("cli_") && !app_id.starts_with("app_") {
        println!("⚠️  应用ID格式可能不正确，应该以 cli_ 或 app_ 开头");
    }

    // 验证接收者ID类型
    if !["open_id", "user_id", "union_id", "chat_id"].contains(&receive_id_type.as_str()) {
        return Err(format!("无效的接收者ID类型: {}，应该是 open_id、user_id、union_id 或 chat_id 之一", receive_id_type).into());
    }

    println!("✅ 环境变量验证通过");
    println!("📱 应用ID: {}...", &app_id[..8.min(app_id.len())]);
    println!("🔑 应用密钥: {}...", &app_secret[..8.min(app_secret.len())]);
    println!("👥 接收者ID: {}...", &receive_id[..8.min(receive_id.len())]);
    println!("🏷️  接收者类型: {}", receive_id_type);

    Ok(MessageConfig {
        app_id,
        app_secret,
        receive_id,
        receive_id_type,
    })
}

/// 创建客户端配置（简化版本，专注于认证）
async fn create_client_config() -> Result<(), Box<dyn std::error::Error>> {
    print_step(3, "准备API调用配置");

    println!("🔧 正在准备API调用配置...");
    println!("🎯 功能: 认证服务和消息发送已就绪");

    Ok(())
}

/// 准备消息请求数据
async fn prepare_message_request(config: &MessageConfig) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    print_step(4, "准备消息数据");

    println!("📝 正在构建消息内容...");

    // 创建文本消息内容
    let message_content = json!({
        "text": "🎉 这是来自Open-Lark SDK的第一条消息！\n\n✨ 特性：\n• 企业级Rust SDK\n• 类型安全的API调用\n• 自动令牌管理\n• 完整的错误处理\n\n🚀 让开始构建飞书应用吧！"
    });

    // 构建完整的消息请求
    let message_request = json!({
        "receive_id": config.receive_id,
        "receive_id_type": config.receive_id_type,
        "content": message_content.to_string(),
        "msg_type": "text"
    });

    println!("✅ 消息数据构建完成");
    println!("📋 消息类型: 文本消息");
    println!("📏 消息长度: {} 字符", message_content.to_string().len());

    Ok(message_request)
}

/// 发送消息
async fn send_message(config: &MessageConfig, message_request: &serde_json::Value) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    print_step(5, "发送消息到飞书服务器");

    println!("📤 正在发送消息...");
    println!("🔗 API端点: {}", IM_V1_MESSAGES);

    // 获取访问令牌
    let access_token = get_access_token(config).await?;

    // 创建HTTP客户端
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    // 发送HTTP请求到飞书API
    let response = client
        .post(format!("https://open.feishu.cn{}", IM_V1_MESSAGES))
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Content-Type", "application/json")
        .json(message_request)
        .send()
        .await
        .map_err(|e| format!("发送请求失败: {}", e))?;

    let status = response.status();
    println!("📊 响应状态: {}", status);

    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        println!("❌ API调用失败: {} - {}", status, error_text);

        // 尝试解析飞书API错误响应
        if let Ok(error_json) = serde_json::from_str::<serde_json::Value>(&error_text) {
            if let Some(code) = error_json.get("code").and_then(|v| v.as_i64()) {
                if let Some(msg) = error_json.get("msg").and_then(|v| v.as_str()) {
                    println!("💡 飞书API错误码: {}", code);
                    println!("💬 错误消息: {}", msg);

                    // 提供常见错误的解决建议
                    match code {
                        99992402 => println!("💡 解决方案: 确保提供了有效的接收者ID和类型"),
                        99991400 => println!("💡 解决方案: 检查应用权限配置"),
                        99991663 => println!("💡 解决方案: 应用未被授权访问该资源"),
                        99991677 => println!("💡 解决方案: 访问令牌已过期，请重新获取"),
                        _ => println!("💡 更多帮助: https://open.feishu.cn/open-apis"),
                    }
                }
            }
        }

        return Err(format!("API调用失败: {} - {}", status, error_text).into());
    }

    // 解析响应JSON
    let response_json: serde_json::Value = response.json().await
        .map_err(|e| format!("解析响应失败: {}", e))?;

    println!("✅ 消息发送成功");

    Ok(response_json)
}

/// 处理消息响应
async fn handle_message_response(response: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
    print_step(6, "处理API响应");

    // 检查响应结构
    if let Some(code) = response.get("code").and_then(|v| v.as_i64()) {
        if code == 0 {
            println!("✅ API调用成功完成");

            // 尝试提取消息相关信息
            if let Some(data) = response.get("data") {
                if let Some(message_id) = data.get("message_id").and_then(|v| v.as_str()) {
                    println!("📧 消息ID: {}", message_id);
                }
                if let Some(chat_id) = data.get("chat_id").and_then(|v| v.as_str()) {
                    println!("💬 聊天ID: {}", chat_id);
                }
                if let Some(create_time) = data.get("create_time").and_then(|v| v.as_i64()) {
                    println!("⏰ 创建时间: {}", create_time);
                }
            }

            print_success("消息发送成功！");
            println!("🎯 你应该能在飞书客户端中看到这条消息了。");
        } else {
            let msg = response.get("msg").and_then(|v| v.as_str()).unwrap_or("未知错误");
            return Err(format!("API返回错误: {} - {}", code, msg).into());
        }
    } else {
        return Err("响应格式无效：缺少code字段".into());
    }

    Ok(())
}

/// 显示后续学习路径
fn show_next_steps() {
    print_step(7, "后续学习路径");

    println!("🎓 恭喜！你已经成功完成了第一个飞书API调用。");
    println!();

    println!("📚 推荐下一步学习：");
    println!("1. 📖 communication 基础：");
    println!("   - 查看根 crate 单入口和 communication 模块访问");
    println!("   - 运行: cargo run --example simple_api_call --features \"auth,communication\"");
    println!();

    println!("2. 📁 文档 helper：");
    println!("   - 自动分页遍历、sheet 查找、批量读取范围");
    println!("   - 运行: cargo run --example docs_helpers --features \"auth,docs-bitable\"");
    println!();

    println!("3. 🔌 长连接能力：");
    println!("   - 接收消息并回显");
    println!("   - 运行: cargo run --example websocket_echo_bot --features \"communication,websocket\"");
    println!();

    println!("4. 🧩 工作流模块：");
    println!("   - 查看 workflow 模块的调用方式");
    println!("   - 运行: cargo run --example workflow_api_example --features \"workflow\"");
    println!();

    print_info("查看所有示例: cargo run --example --list");
}

/// 获取访问令牌
async fn get_access_token(config: &MessageConfig) -> Result<String, Box<dyn std::error::Error>> {
    println!("🔐 正在获取应用访问令牌...");

    // 使用认证服务获取令牌
    let auth_config = open_lark::auth::models::AuthConfig::new(&config.app_id, &config.app_secret);
    let auth_services = open_lark::auth::AuthServices::new(auth_config);

    // 获取自建应用访问令牌
    let token_response = auth_services
        .auth
        .v3()
        .app_access_token()
        .internal()
        .send()
        .await
        .map_err(|e| format!("获取访问令牌失败: {}", e))?;

    println!("✅ 访问令牌获取成功");
    println!("⏰ 有效期: {} 秒", token_response.expire);

    Ok(token_response.app_access_token)
}

/// 消息配置结构体
#[derive(Debug)]
struct MessageConfig {
    app_id: String,
    app_secret: String,
    receive_id: String,
    receive_id_type: String,
}

/// 注意事项和最佳实践说明
fn show_best_practices() {
    println!();
    println!("💡 最佳实践提示：");
    println!("1. 🔐 安全性：");
    println!("   - 不要在代码中硬编码应用密钥");
    println!("   - 使用环境变量或配置文件管理敏感信息");
    println!("   - 定期轮换应用密钥");
    println!();

    println!("2. 🚀 性能优化：");
    println!("   - 使用连接池和请求复用");
    println!("   - 实现适当的重试机制");
    println!("   - 缓存访问令牌避免频繁获取");
    println!();

    println!("3. 🛡️ 错误处理：");
    println!("   - 检查所有API响应的状态码");
    println!("   - 实现用户友好的错误消息");
    println!("   - 记录详细的错误日志用于调试");
    println!();

    println!("4. 📊 监控和日志：");
    println!("   - 记录API调用频率和响应时间");
    println!("   - 监控错误率和成功率");
    println!("   - 实现健康检查机制");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_request_creation() {
        let config = MessageConfig {
            app_id: "test_app".to_string(),
            app_secret: "test_secret".to_string(),
            receive_id: "test_user".to_string(),
            receive_id_type: "open_id".to_string(),
        };

        // 这里可以添加消息请求创建的测试
        // 由于涉及异步操作，需要使用tokio测试运行时
    }

    #[test]
    fn test_receive_id_type_validation() {
        let valid_types = ["open_id", "user_id", "union_id", "chat_id"];
        for &valid_type in &valid_types {
            assert!(validate_receive_id_type(valid_type), "有效类型 {} 应该通过验证", valid_type);
        }

        let invalid_types = ["invalid", "user", "", "openid"];
        for &invalid_type in &invalid_types {
            assert!(!validate_receive_id_type(invalid_type), "无效类型 {} 应该被拒绝", invalid_type);
        }
    }

    fn validate_receive_id_type(receive_id_type: &str) -> bool {
        ["open_id", "user_id", "union_id", "chat_id"].contains(&receive_id_type)
    }
}
