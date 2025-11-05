//! 功能标志使用示例
//!
//! 展示不同功能标志的配置和使用方法

use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 功能标志使用示例");
    println!("演示不同功能标志的配置和用法\n");

    // 基础客户端设置
    let client = LarkClient::builder("your_app_id", "your_app_secret")
        .with_enable_token_cache(true)
        .build();

    // 演示不同功能标志的使用
    demo_auth_features(&client).await?;
    demo_im_features(&client).await?;
    demo_contact_features(&client).await?;
    demo_cloud_docs_features(&client).await?;
    demo_enterprise_features(&client).await?;
    demo_ai_features(&client).await?;

    println!("\n✅ 所有功能演示完成！");
    Ok(())
}

/// 演示认证相关功能
async fn demo_auth_features(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔐 认证功能演示");

    // 检查认证功能是否可用
    let auth_available = cfg!(any(feature = "auth", feature = "authen"));

    if auth_available {
        println!("  ✅ 认证功能已启用");

        #[cfg(any(feature = "auth", feature = "authen"))]
        {
            println!("  📄 可用的认证服务:");
            println!("    - 用户信息获取: client.auth.v1.user.get_info()");
            println!("    - 令牌管理: client.auth.v1.tenant_access_token.create()");
            println!("    - 应用权限: client.auth.v1.app_access_token.internal()");

            // 示例：获取当前用户信息（需要有效的用户ID）
            let user_id = "test_user_id"; // 替换为实际用户ID

            let request = GetUserInfoRequest::builder()
                .user_id(user_id)
                .user_id_type("open_id")
                .build();

            match client.auth.v1.user.get_info(&request).await {
                Ok(user_info) => {
                    println!("  ✅ 成功获取用户信息: {}", user_info.name);
                }
                Err(e) => {
                    println!(
                        "  ⚠️ 获取用户信息失败（可能需要有效凭证）: {}",
                        e.user_friendly_message()
                    );
                }
            }
        }
    } else {
        println!("  ❌ 认证功能未启用");
        println!("  💡 请在 Cargo.toml 中添加: features = [\"auth\"]");
    }

    println!();
    Ok(())
}

/// 演示即时消息功能
async fn demo_im_features(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("💬 即时消息功能演示");

    let im_available = cfg!(feature = "im");

    if im_available {
        println!("  ✅ 即时消息功能已启用");

        #[cfg(feature = "im")]
        {
            println!("  📄 可用的消息服务:");
            println!("    - 发送文本消息: client.im.v1.message.send()");
            println!("    - 发送富文本消息: client.im.v1.message.send()");
            println!("    - 获取消息列表: client.im.v1.message.list()");
            println!("    - 创建群聊: client.im.v1.chat.create()");

            // 示例：发送文本消息（需要有效的接收者ID）
            let receive_id = "test_user_id"; // 替换为实际用户ID

            let request = SendMessageRequest::builder()
                .receive_id_type("open_id")
                .request_body(
                    SendMessageRequestBody::builder()
                        .receive_id(receive_id)
                        .msg_type("text")
                        .content(r#"{"text":"Hello from open-lark SDK!"}"#)
                        .build(),
                )
                .build();

            println!("  📝 尝试发送测试消息...");
            match client.im.v1.message.send(&request).await {
                Ok(response) => {
                    println!("  ✅ 消息发送成功: {:?}", response.message_id);
                }
                Err(e) => {
                    println!(
                        "  ⚠️ 消息发送失败（可能需要有效接收者）: {}",
                        e.user_friendly_message()
                    );
                }
            }
        }
    } else {
        println!("  ❌ 即时消息功能未启用");
        println!("  💡 请在 Cargo.toml 中添加: features = [\"im\"]");
    }

    println!();
    Ok(())
}

/// 演示联系人管理功能
async fn demo_contact_features(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("👥 联系人管理功能演示");

    let contact_available = cfg!(feature = "contact");

    if contact_available {
        println!("  ✅ 联系人功能已启用");

        #[cfg(feature = "contact")]
        {
            println!("  📄 可用的联系人服务:");
            println!("    - 获取用户列表: client.contact.v3.user.get_list()");
            println!("    - 获取部门列表: client.contact.v3.department.get_list()");
            println!("    - 获取用户详情: client.contact.v3.user.get()");
            println!("    - 获取部门详情: client.contact.v3.department.get()");

            // 示例：获取用户列表
            let request = GetUserListRequest::builder()
                .page_size(10)
                .user_id_type("open_id")
                .build();

            println!("  📋 尝试获取用户列表...");
            match client.contact.v3.user.get_list(&request).await {
                Ok(response) => {
                    println!("  ✅ 成功获取 {} 个用户", response.data.items.len());
                    for (i, user) in response.data.items.iter().take(3).enumerate() {
                        println!("    {}. {} ({})", i + 1, user.name, user.user_id);
                    }
                }
                Err(e) => {
                    println!("  ⚠️ 获取用户列表失败: {}", e.user_friendly_message());
                }
            }
        }
    } else {
        println!("  ❌ 联系人功能未启用");
        println!("  💡 请在 Cargo.toml 中添加: features = [\"contact\"]");
    }

    println!();
    Ok(())
}

/// 演示云文档功能（包含所有别名）
async fn demo_cloud_docs_features(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("📄 云文档功能演示");

    // 检查所有可能的云文档功能标志
    let cloud_docs_available = cfg!(any(
        feature = "cloud-docs",
        feature = "docx",
        feature = "drive"
    ));

    if cloud_docs_available {
        println!("  ✅ 云文档功能已启用");

        // 显示启用的具体功能标志
        if cfg!(feature = "cloud-docs") {
            println!("    - cloud-docs 功能标志已启用");
        }
        if cfg!(feature = "docx") {
            println!("    - docx 功能标志已启用（别名）");
        }
        if cfg!(feature = "drive") {
            println!("    - drive 功能标志已启用（别名）");
        }

        #[cfg(any(feature = "cloud-docs", feature = "docx", feature = "drive"))]
        {
            println!("  📄 可用的云文档服务:");
            println!("    - 文件列表: client.cloud_docs.v1.drive.file_list()");
            println!("    - 创建文档: client.cloud_docs.v1.docx.create_document()");
            println!("    - 上传文件: client.cloud_docs.v1.drive.upload_file()");
            println!("    - 文档评论: client.cloud_docs.v1.comments.add_comment()");

            // 示例：获取文件列表（需要有效的文件夹token）
            let folder_token = "root_folder_token"; // 替换为实际文件夹token

            let request = FileListRequest::builder()
                .folder_token(folder_token)
                .page_size(10)
                .build();

            println!("  📂 尝试获取文件列表...");
            match client.cloud_docs.v1.drive.file_list(&request).await {
                Ok(response) => {
                    println!("  ✅ 成功获取 {} 个文件", response.data.items.len());
                    for (i, file) in response.data.items.iter().take(3).enumerate() {
                        println!("    {}. {} ({})", i + 1, file.name, file.type_);
                    }
                }
                Err(e) => {
                    println!("  ⚠️ 获取文件列表失败: {}", e.user_friendly_message());
                }
            }
        }
    } else {
        println!("  ❌ 云文档功能未启用");
        println!("  💡 请在 Cargo.toml 中添加: features = [\"cloud-docs\"]");
    }

    println!();
    Ok(())
}

/// 演示企业级功能
async fn demo_enterprise_features(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("🏢 企业级功能演示");

    let mut enabled_features = Vec::new();

    // 检查各种企业功能
    if cfg!(feature = "approval") {
        enabled_features.push("审批流程 (approval)");
    }
    if cfg!(feature = "attendance") {
        enabled_features.push("考勤管理 (attendance)");
    }
    if cfg!(feature = "calendar") {
        enabled_features.push("日历集成 (calendar)");
    }
    if cfg!(feature = "sheets") {
        enabled_features.push("电子表格 (sheets)");
    }
    if cfg!(feature = "bitable") {
        enabled_features.push("多维表格 (bitable)");
    }
    if cfg!(feature = "wiki") {
        enabled_features.push("知识库 (wiki)");
    }

    if enabled_features.is_empty() {
        println!("  ❌ 未启用企业级功能");
        println!("  💡 可选的企业功能:");
        println!("    - approval: 审批流程");
        println!("    - attendance: 考勤管理");
        println!("    - calendar: 日历集成");
        println!("    - sheets: 电子表格");
        println!("    - bitable: 多维表格");
        println!("    - wiki: 知识库");
    } else {
        println!("  ✅ 已启用的企业功能:");
        for feature in &enabled_features {
            println!("    - {}", feature);
        }
    }

    println!();
    Ok(())
}

/// 演示AI功能
async fn demo_ai_features(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 AI功能演示");

    let ai_available = cfg!(feature = "ai");

    if ai_available {
        println!("  ✅ AI功能已启用");

        #[cfg(feature = "ai")]
        {
            println!("  📄 可用的AI服务:");
            println!("    - 文本生成: client.ai.v1.chat.create()");
            println!("    - 图像识别: client.ai.v1.image.understand()");
            println!("    - 语音识别: client.ai.v1.speech.recognize()");

            // 示例：简单的AI聊天（需要有效配置）
            println!("  🤖 AI功能已可用，请查看具体API文档进行调用");
        }
    } else {
        println!("  ❌ AI功能未启用");
        println!("  💡 请在 Cargo.toml 中添加: features = [\"ai\"]");
    }

    println!();
    Ok(())
}

/// 功能标志检查工具
pub struct FeatureChecker;

impl FeatureChecker {
    /// 检查所有可用功能
    pub fn check_all_features() {
        println!("🔍 当前启用的功能标志:");

        let features = [
            ("auth", "认证服务"),
            ("authen", "认证服务（别名）"),
            ("im", "即时消息"),
            ("contact", "联系人管理"),
            ("cloud-docs", "云文档"),
            ("docx", "云文档（别名）"),
            ("drive", "云盘（别名）"),
            ("approval", "审批流程"),
            ("attendance", "考勤管理"),
            ("calendar", "日历集成"),
            ("ai", "AI功能"),
            ("sheets", "电子表格"),
            ("bitable", "多维表格"),
            ("wiki", "知识库"),
        ];

        let mut enabled_count = 0;
        for (feature, description) in &features {
            if cfg!(feature = "feature") {
                println!("  ✅ {} - {}", feature, description);
                enabled_count += 1;
            } else {
                println!("  ❌ {} - {}", feature, description);
            }
        }

        println!(
            "\n📊 功能统计: {}/{} 功能已启用",
            enabled_count,
            features.len()
        );
    }

    /// 检查必需功能
    pub fn check_required_features() -> bool {
        let required = ["auth", "im", "contact"];
        let missing: Vec<&str> = required
            .iter()
            .filter(|&&f| !cfg!(feature = "f"))
            .copied()
            .collect();

        if !missing.is_empty() {
            println!("⚠️ 缺少推荐功能: {:?}", missing);
            return false;
        }

        println!("✅ 所有推荐功能已启用");
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_checker() {
        FeatureChecker::check_all_features();
        FeatureChecker::check_required_features();
    }

    #[test]
    fn test_feature_availability() {
        // 测试功能标志的可用性
        assert!(cfg!(any(feature = "auth", feature = "authen")) || !cfg!(any(feature = "im")));
        // IM需要认证
    }
}
