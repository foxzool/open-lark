use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let app_id = std::env::var("APP_ID").unwrap();
    let app_secret = std::env::var("APP_SECRET").unwrap();

    let client = LarkClient::builder(app_id, app_secret)
        .with_enable_token_cache(true)
        .build();

    // 获取文档权限设置
    let request = GetPermissionPublicRequest::builder()
        .token("doccnxxxxxx")
        .as_doc()
        .build();

    match client.permission.get_permission_public(&request, None).await {
        Ok(response) => {
            println!("获取权限设置成功");
            
            let settings = response.data.public_settings();
            println!("分享设置: {}", settings.share_level_description());
            println!("权限摘要: {}", settings.permissions_summary());
            println!("安全级别: {}", settings.security_level());
            
            // 详细权限信息
            println!("\n详细权限:");
            println!("- 链接分享: {}", settings.link_share_setting);
            println!("- 密码保护: {}", if settings.has_password_protection() { "开启" } else { "关闭" });
            println!("- 允许复制: {}", if settings.allow_copy { "是" } else { "否" });
            println!("- 允许评论: {}", if settings.allow_comment { "是" } else { "否" });
            println!("- 保存副本: {}", if settings.allow_save_copy { "是" } else { "否" });
            
            // 安全性分析
            println!("\n安全性分析:");
            println!("设置摘要: {}", response.data.settings_summary());
            
            let recommendations = response.data.security_recommendations();
            if !recommendations.is_empty() {
                println!("\n安全建议:");
                for (i, rec) in recommendations.iter().enumerate() {
                    println!("{}. {}", i + 1, rec);
                }
            }
            
            // 访问权限检查
            println!("\n访问权限:");
            println!("- 链接分享: {}", if settings.is_link_share_enabled() { "已开启" } else { "已关闭" });
            println!("- 组织内访问: {}", if settings.is_tenant_accessible() { "允许" } else { "不允许" });
            println!("- 任何人访问: {}", if settings.is_anyone_accessible() { "允许" } else { "不允许" });
            println!("- 编辑权限: {}", if settings.is_editable() { "允许" } else { "仅可读" });
        }
        Err(e) => {
            eprintln!("获取权限设置失败: {:?}", e);
        }
    }

    // 获取电子表格权限设置示例
    println!("\n--- 获取电子表格权限设置 ---");
    let sheet_request = GetPermissionPublicRequest::for_sheet("shtcnxxxxxx");

    match client.permission.get_permission_public(&sheet_request, None).await {
        Ok(response) => {
            println!("电子表格权限设置:");
            let settings = response.data.public_settings();
            println!("- 分享级别: {}", settings.share_level_description());
            println!("- 安全级别: {}", settings.security_level());
            
            if response.data.has_external_config() {
                println!("- 存在外部访问配置");
            }
        }
        Err(e) => {
            eprintln!("获取电子表格权限设置失败: {:?}", e);
        }
    }

    Ok(())
}