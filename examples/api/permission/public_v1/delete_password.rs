use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").unwrap();
    let app_secret = std::env::var("APP_SECRET").unwrap();

    let client = LarkClient::builder(app_id, app_secret)
        .with_enable_token_cache(true)
        .build();

    // 关闭文档密码保护
    let request = DeletePasswordRequest::builder()
        .token("doccnxxxxxx")
        .as_doc()
        .build();

    match client.permission.delete_password(&request, None).await {
        Ok(response) => {
            println!("密码保护关闭操作完成");

            let deletion_info = response.data.deletion_info();
            println!("删除状态: {}", deletion_info.deletion_status());
            println!("删除摘要: {}", response.data.deletion_summary());

            if let Some(delete_time) = deletion_info.delete_time_formatted() {
                println!("删除信息: {}", delete_time);
            }

            if let Some(prev_hint) = &deletion_info.previous_password_hint {
                println!("原密码提示: {}", prev_hint);
            }

            if let Some(op_id) = &deletion_info.operation_id {
                println!("操作ID: {}", op_id);
            }

            println!("安全评估: {}", if let Some(data) = response.data { data.security_assessment() } else { "N/A".to_string() });

            // 安全警告
            let warnings = response.data.security_warnings();
            if !warnings.is_empty() {
                println!("\n安全警告:");
                for warning in warnings {
                    println!("{}", warning);
                }
            }

            // 后续操作建议
            let recommendations = response.data.follow_up_recommendations();
            if !recommendations.is_empty() {
                println!("\n后续建议:");
                for (i, rec) in recommendations.iter().enumerate() {
                    println!("{}. {}", i + 1, rec);
                }
            }

            // 操作记录
            println!("\n操作记录:");
            println!("{}", response.data.operation_log());

            // 根据删除结果给出不同的提示
            if response.data.is_deleted() {
                println!("\n✓ 密码保护已成功关闭");
                println!("  文档现在可以通过链接直接访问（无需密码）");
            } else {
                println!("\n✗ 密码保护关闭失败");
                println!("  密码保护仍然有效");
            }
        }
        Err(e) => {
            eprintln!("关闭密码保护失败: {:?}", e);
        }
    }

    // 关闭电子表格密码保护
    println!("\n--- 关闭电子表格密码保护 ---");
    let sheet_request = DeletePasswordRequest::for_sheet("shtcnxxxxxx");

    match client
        .permission
        .delete_password(&sheet_request, None)
        .await
    {
        Ok(response) => {
            println!("电子表格密码保护关闭操作完成");

            let deletion_info = response.data.deletion_info();
            println!(
                "操作结果: {}",
                if deletion_info.is_successfully_removed() {
                    "成功"
                } else {
                    "失败"
                }
            );

            if deletion_info.is_successfully_removed() {
                println!("安全影响: {}", deletion_info.security_impact());

                // 显示删除原因
                let reasons = deletion_info.deletion_reasons();
                if !reasons.is_empty() {
                    println!("可能的删除原因:");
                    for (i, reason) in reasons.iter().enumerate() {
                        println!("{}. {}", i + 1, reason);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("关闭电子表格密码保护失败: {:?}", e);
        }
    }

    // 关闭多维表格密码保护
    println!("\n--- 关闭多维表格密码保护 ---");
    let bitable_request = DeletePasswordRequest::for_bitable("bblcnxxxxxx");

    match client
        .permission
        .delete_password(&bitable_request, None)
        .await
    {
        Ok(response) => {
            println!("多维表格密码保护关闭操作完成");

            let deletion_info = response.data.deletion_info();

            // 详细状态检查
            println!("详细状态:");
            println!("- 密码已删除: {}", deletion_info.password_removed);
            println!("- 有删除时间: {}", deletion_info.has_delete_time());
            println!("- 有操作ID: {}", deletion_info.has_operation_id());
            println!("- 有密码提示: {}", deletion_info.has_password_hint());

            // 获取操作摘要
            println!("操作摘要: {}", deletion_info.deletion_summary());

            if deletion_info.is_successfully_removed() {
                println!("\n🔓 密码保护已关闭");
                println!("文档安全级别已降低，请考虑其他安全措施");
            }
        }
        Err(e) => {
            eprintln!("关闭多维表格密码保护失败: {:?}", e);
        }
    }

    // 关闭知识库密码保护
    println!("\n--- 关闭知识库密码保护 ---");
    let wiki_request = DeletePasswordRequest::for_wiki("wikicnxxxxxx");

    match client.permission.delete_password(&wiki_request, None).await {
        Ok(response) => {
            println!("知识库密码保护关闭操作完成");

            // 完整的状态报告
            println!("\n状态报告:");
            println!("{}", response.data.operation_log());

            if response.data.is_deleted() {
                let warnings = response.data.security_warnings();
                println!("\n重要提醒:");
                for warning in warnings {
                    println!("{}", warning);
                }

                let recommendations = if let Some(data) = &response.data {
                    data.follow_up_recommendations()
                } else {
                    vec![]
                };
                println!("\n推荐操作:");
                for (i, rec) in recommendations.iter().enumerate() {
                    println!("{}. {}", i + 1, rec);
                }
            } else {
                println!("\n密码保护关闭失败，可能的原因:");
                let reasons = if let Some(data) = &response.data {
                    data.deletion_info().deletion_reasons()
                } else {
                    vec![]
                };
                for reason in reasons {
                    if reason.contains("失败") || reason.contains("错误") || reason.contains("权限")
                    {
                        println!("- {}", reason);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("关闭知识库密码保护失败: {:?}", e);
        }
    }

    Ok(())
}
