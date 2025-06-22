use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").unwrap();
    let app_secret = std::env::var("APP_SECRET").unwrap();

    let client = LarkClient::builder(app_id, app_secret)
        .with_enable_token_cache(true)
        .build();

    // 获取文档权限设置 (v2)
    let request = GetPermissionPublicV2Request::builder()
        .token("doccnxxxxxx")
        .as_doc()
        .build();

    match client
        .permission
        .get_permission_public_v2(&request, None)
        .await
    {
        Ok(response) => {
            println!("获取权限设置成功 (v2)");

            let settings = response.data.public_settings();
            println!("分享设置: {}", settings.share_level_description());
            println!("权限摘要: {}", settings.permissions_summary());
            println!("安全级别: {}", settings.security_level());

            // 详细权限信息
            println!("\n详细权限:");
            println!("- 链接分享: {}", settings.link_share_setting);
            println!(
                "- 密码保护: {}",
                if settings.has_password_protection() {
                    "开启"
                } else {
                    "关闭"
                }
            );
            println!(
                "- 允许复制: {}",
                if settings.allow_copy { "是" } else { "否" }
            );
            println!(
                "- 允许评论: {}",
                if settings.allow_comment { "是" } else { "否" }
            );
            println!(
                "- 保存副本: {}",
                if settings.allow_save_copy {
                    "是"
                } else {
                    "否"
                }
            );
            println!(
                "- 组织外分享: {}",
                if settings.allows_external_share() {
                    "允许"
                } else {
                    "不允许"
                }
            );

            // v2 新增功能
            println!("\nv2 高级功能:");
            if let Some(scope) = settings.share_scope_description() {
                println!("- 分享范围: {}", scope);
            }

            if let Some(access) = &settings.access_setting {
                println!("- 访问设置: {}", access);
            }

            // 过期时间检查
            println!("\n过期状态:");
            println!("{}", response.data.expiration_status());

            if settings.is_expired() {
                println!("⚠️ 文档分享已过期");
            } else if let Some(remaining) = settings.remaining_valid_time() {
                if remaining < 86400 {
                    println!("⚠️ 文档分享即将在24小时内过期");
                }
            }

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

            // 高级功能报告
            println!("\n高级功能:");
            println!("{}", response.data.advanced_features_report());

            let advanced_features = settings.advanced_features_status();
            if !advanced_features.is_empty() {
                println!("功能详情:");
                for feature in advanced_features {
                    println!("- {}", feature);
                }
            }

            // 访问权限检查
            println!("\n访问权限:");
            println!(
                "- 链接分享: {}",
                if settings.is_link_share_enabled() {
                    "已开启"
                } else {
                    "已关闭"
                }
            );
            println!(
                "- 组织内访问: {}",
                if settings.is_tenant_accessible() {
                    "允许"
                } else {
                    "不允许"
                }
            );
            println!(
                "- 任何人访问: {}",
                if settings.is_anyone_accessible() {
                    "允许"
                } else {
                    "不允许"
                }
            );
            println!(
                "- 编辑权限: {}",
                if settings.is_editable() {
                    "允许"
                } else {
                    "仅可读"
                }
            );
            println!(
                "- 高级配置: {}",
                if response.data.has_advanced_config() {
                    "已启用"
                } else {
                    "未启用"
                }
            );
        }
        Err(e) => {
            eprintln!("获取权限设置失败: {:?}", e);
        }
    }

    // 获取电子表格权限设置示例
    println!("\n--- 获取电子表格权限设置 (v2) ---");
    let sheet_request = GetPermissionPublicV2Request::for_sheet("shtcnxxxxxx");

    match client
        .permission
        .get_permission_public_v2(&sheet_request, None)
        .await
    {
        Ok(response) => {
            println!("电子表格权限设置:");
            let settings = response.data.public_settings();
            println!("- 分享级别: {}", settings.share_level_description());
            println!("- 安全级别: {}", settings.security_level());
            println!(
                "- 组织外分享: {}",
                if settings.allows_external_share() {
                    "允许"
                } else {
                    "不允许"
                }
            );

            if response.data.has_advanced_config() {
                println!("- 存在高级配置");
                println!("  {}", response.data.advanced_features_report());
            }

            // 过期时间检查
            if settings.has_expire_time() {
                println!("- {}", response.data.expiration_status());
            } else {
                println!("- 永久有效");
            }
        }
        Err(e) => {
            eprintln!("获取电子表格权限设置失败: {:?}", e);
        }
    }

    // 获取多维表格权限设置示例
    println!("\n--- 获取多维表格权限设置 (v2) ---");
    let bitable_request = GetPermissionPublicV2Request::for_bitable("bblcnxxxxxx");

    match client
        .permission
        .get_permission_public_v2(&bitable_request, None)
        .await
    {
        Ok(response) => {
            println!("多维表格权限设置:");
            let settings = response.data.public_settings();

            // 综合安全性评估
            println!("- 综合评估: {}", response.data.settings_summary());

            // 检查安全风险
            let recommendations = response.data.security_recommendations();
            let risk_count = recommendations
                .iter()
                .filter(|r| r.contains("建议") || r.contains("注意") || r.contains("风险"))
                .count();

            if risk_count > 0 {
                println!("- 安全风险: 发现{}个潜在问题", risk_count);
            } else {
                println!("- 安全风险: 配置合理");
            }

            // 功能特性统计
            let features = settings.permissions_summary();
            println!("- 功能特性: {}", features);
        }
        Err(e) => {
            eprintln!("获取多维表格权限设置失败: {:?}", e);
        }
    }

    Ok(())
}
