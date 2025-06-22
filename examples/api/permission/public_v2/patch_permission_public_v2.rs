use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let app_id = std::env::var("APP_ID").unwrap();
    let app_secret = std::env::var("APP_SECRET").unwrap();

    let client = LarkClient::builder(app_id, app_secret)
        .with_enable_token_cache(true)
        .build();

    // 设置企业级安全模式
    let request = PatchPermissionPublicV2Request::builder()
        .token("doccnxxxxxx")
        .as_doc()
        .enterprise_secure_mode()
        .expire_after_days(30)
        .build();

    match client
        .permission
        .patch_permission_public_v2(&request, None)
        .await
    {
        Ok(response) => {
            println!("设置企业级安全模式成功");

            let result = response.data.update_result();
            println!("更新摘要: {}", response.data.update_summary());
            println!("安全评估: {}", response.data.security_assessment());

            // 详细变更信息
            let changes = result.changes_summary();
            if !changes.is_empty() {
                println!("\n权限变更:");
                for (i, change) in changes.iter().enumerate() {
                    println!("{}. {}", i + 1, change);
                }
            }

            // 高级功能报告
            println!("\n高级功能:");
            println!("{}", response.data.advanced_features_report());

            // 过期状态
            if let Some(expiration) = response.data.expiration_report() {
                println!("过期设置: {}", expiration);
            }

            // 操作建议
            let recommendations = response.data.operation_recommendations();
            if !recommendations.is_empty() {
                println!("\n操作建议:");
                for (i, rec) in recommendations.iter().enumerate() {
                    println!("{}. {}", i + 1, rec);
                }
            }
        }
        Err(e) => {
            eprintln!("设置企业级安全模式失败: {:?}", e);
        }
    }

    // 设置协作模式
    println!("\n--- 设置协作模式 ---");
    let collaboration_request = PatchPermissionPublicV2Request::builder()
        .token("shtcnxxxxxx")
        .as_sheet()
        .collaboration_mode()
        .access_setting("team_collaboration")
        .share_scope("department")
        .expire_after_hours(72)
        .build();

    match client
        .permission
        .patch_permission_public_v2(&collaboration_request, None)
        .await
    {
        Ok(response) => {
            println!("电子表格协作模式设置成功");
            println!(
                "安全级别: {}",
                response.data.update_result().security_level()
            );
            println!("更新摘要: {}", response.data.update_summary());

            let advanced_changes = response.data.update_result().advanced_changes();
            if !advanced_changes.is_empty() {
                println!("\n高级功能变更:");
                for change in advanced_changes {
                    println!("- {}", change);
                }
            }

            if let Some(expiration) = response.data.expiration_report() {
                println!("过期设置: {}", expiration);
            }
        }
        Err(e) => {
            eprintln!("设置协作模式失败: {:?}", e);
        }
    }

    // 设置公开分享模式
    println!("\n--- 设置公开分享模式 ---");
    let public_request = PatchPermissionPublicV2Request::builder()
        .token("bblcnxxxxxx")
        .as_bitable()
        .public_share_mode()
        .enable_external_share()
        .share_scope("public")
        .expire_after_days(7)
        .build();

    match client
        .permission
        .patch_permission_public_v2(&public_request, None)
        .await
    {
        Ok(response) => {
            println!("多维表格公开分享模式设置成功");

            let result = response.data.update_result();
            println!("安全级别: {}", result.security_level());

            // 安全警告检查
            let recommendations = response.data.operation_recommendations();
            let warnings = recommendations
                .iter()
                .filter(|r| r.contains("建议") || r.contains("注意") || r.contains("风险"))
                .collect::<Vec<_>>();

            if !warnings.is_empty() {
                println!("\n⚠️ 安全提醒:");
                for warning in warnings {
                    println!("- {}", warning);
                }
            }

            println!("\n{}", response.data.advanced_features_report());
        }
        Err(e) => {
            eprintln!("设置公开分享模式失败: {:?}", e);
        }
    }

    // 设置开放编辑模式
    println!("\n--- 设置开放编辑模式 ---");
    let open_request = PatchPermissionPublicV2Request::builder()
        .token("wikicnxxxxxx")
        .as_wiki()
        .open_edit_mode()
        .access_setting("open_collaboration")
        .share_scope("unlimited")
        .expire_after_hours(24)
        .build();

    match client
        .permission
        .patch_permission_public_v2(&open_request, None)
        .await
    {
        Ok(response) => {
            println!("知识库开放编辑模式设置成功");

            let result = response.data.update_result();
            println!("安全级别: {}", result.security_level());
            println!("安全评估: {}", response.data.security_assessment());

            // 风险评估
            if result.security_level() == "低安全" {
                println!("\n🚨 高风险警告:");
                println!("- 当前设置允许任何人编辑，存在较高安全风险");
                println!("- 建议定期监控文档变更");
                println!("- 考虑设置密码保护");
            }

            let recommendations = response.data.operation_recommendations();
            println!("\n安全建议:");
            for rec in recommendations {
                println!("- {}", rec);
            }
        }
        Err(e) => {
            eprintln!("设置开放编辑模式失败: {:?}", e);
        }
    }

    // 自定义高级权限设置
    println!("\n--- 自定义高级权限设置 ---");
    let custom_request = PatchPermissionPublicV2Request::builder()
        .token("doccnxxxxxx")
        .as_doc()
        .tenant_editable()
        .enable_copy()
        .disable_comment()
        .enable_watermark()
        .disable_external_share()
        .access_setting("restricted")
        .share_scope("team_only")
        .expire_after_days(14)
        .build();

    match client
        .permission
        .patch_permission_public_v2(&custom_request, None)
        .await
    {
        Ok(response) => {
            println!("自定义高级权限设置成功");

            let result = response.data.update_result();
            println!(
                "分享级别: {}",
                result.share_level_description().unwrap_or("未设置")
            );
            println!("安全级别: {}", result.security_level());

            if let Some(time) = result.update_time_formatted() {
                println!("更新信息: {}", time);
            }

            if let Some(expire) = result.expire_time_formatted() {
                println!("过期信息: {}", expire);
            }

            // 显示所有变更
            let changes = result.changes_summary();
            println!("\n完整变更列表:");
            for (i, change) in changes.iter().enumerate() {
                println!("{}. {}", i + 1, change);
            }

            // 高级功能总结
            let advanced = result.advanced_changes();
            if !advanced.is_empty() {
                println!("\n高级功能变更:");
                for change in advanced {
                    println!("- {}", change);
                }
            }
        }
        Err(e) => {
            eprintln!("自定义高级权限设置失败: {:?}", e);
        }
    }

    // 设置永久有效权限
    println!("\n--- 设置永久有效权限 ---");
    let permanent_request = PatchPermissionPublicV2Request::builder()
        .token("doccnxxxxxx")
        .as_doc()
        .tenant_readable()
        .never_expire()
        .build();

    match client
        .permission
        .patch_permission_public_v2(&permanent_request, None)
        .await
    {
        Ok(response) => {
            println!("永久有效权限设置成功");

            if let Some(expiration) = response.data.expiration_report() {
                println!("过期状态: {}", expiration);
            }

            println!("安全评估: {}", response.data.security_assessment());
        }
        Err(e) => {
            eprintln!("设置永久有效权限失败: {:?}", e);
        }
    }

    Ok(())
}
