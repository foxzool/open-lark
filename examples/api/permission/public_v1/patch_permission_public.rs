use open_lark::prelude::*;
use open_lark::service::permission::public_v1::PatchPermissionPublicRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").unwrap();
    let app_secret = std::env::var("APP_SECRET").unwrap();

    let client = LarkClient::builder(app_id, app_secret)
        .with_enable_token_cache(true)
        .build();

    // 更新文档权限为安全模式
    let request = PatchPermissionPublicRequest::builder()
        .token("doccnxxxxxx")
        .as_doc()
        .secure_mode()
        .build();

    match client
        .permission
        .patch_permission_public(&request, None)
        .await
    {
        Ok(response) => {
            println!("更新权限设置成功");

            let result = response.data.as_ref().unwrap().update_result();
            println!("更新摘要: {}", response.data.update_summary());
            println!("安全评估: {}", if let Some(data) = response.data { data.security_assessment() } else { "N/A".to_string() });

            // 详细变更信息
            let changes = result.changes_summary();
            if !changes.is_empty() {
                println!("\n权限变更:");
                for (i, change) in changes.iter().enumerate() {
                    println!("{}. {}", i + 1, change);
                }
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
            eprintln!("更新权限设置失败: {:?}", e);
        }
    }

    // 更新电子表格权限为开放模式
    println!("\n--- 更新电子表格权限为开放模式 ---");
    let open_request = PatchPermissionPublicRequest::builder()
        .token("shtcnxxxxxx")
        .as_sheet()
        .open_mode()
        .build();

    match client
        .permission
        .patch_permission_public(&open_request, None)
        .await
    {
        Ok(response) => {
            println!("电子表格权限更新成功");
            println!(
                "安全级别: {}",
                response.data.as_ref().unwrap().update_result().security_level()
            );
            println!("更新摘要: {}", response.data.update_summary());

            let recommendations = response.data.operation_recommendations();
            if !recommendations.is_empty() {
                println!("\n安全提醒:");
                for rec in recommendations {
                    println!("- {}", rec);
                }
            }
        }
        Err(e) => {
            eprintln!("更新电子表格权限失败: {:?}", e);
        }
    }

    // 自定义权限设置
    println!("\n--- 自定义权限设置 ---");
    let custom_request = PatchPermissionPublicRequest::builder()
        .token("doccnxxxxxx")
        .as_doc()
        .tenant_editable()
        .enable_copy()
        .disable_comment()
        .enable_watermark()
        .build();

    match client
        .permission
        .patch_permission_public(&custom_request, None)
        .await
    {
        Ok(response) => {
            println!("自定义权限设置成功");

            let result = response.data.as_ref().unwrap().update_result();
            println!(
                "分享级别: {}",
                result.share_level_description().unwrap_or("未设置")
            );
            println!("安全级别: {}", result.security_level());

            if let Some(time) = result.update_time_formatted() {
                println!("更新信息: {}", time);
            }
        }
        Err(e) => {
            eprintln!("自定义权限设置失败: {:?}", e);
        }
    }

    // 关闭分享
    println!("\n--- 关闭分享 ---");
    let close_request = PatchPermissionPublicRequest::builder()
        .token("doccnxxxxxx")
        .as_doc()
        .close_sharing()
        .build();

    match client
        .permission
        .patch_permission_public(&close_request, None)
        .await
    {
        Ok(response) => {
            println!("分享已关闭");
            println!(
                "安全级别: {}",
                response.data.as_ref().unwrap().update_result().security_level()
            );
            println!("安全评估: {}", if let Some(data) = response.data { data.security_assessment() } else { "N/A".to_string() });
        }
        Err(e) => {
            eprintln!("关闭分享失败: {:?}", e);
        }
    }

    Ok(())
}
