use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").unwrap();
    let app_secret = std::env::var("APP_SECRET").unwrap();

    let client = LarkClient::builder(app_id, app_secret)
        .with_enable_token_cache(true)
        .build();

    // 更新文档密码
    let request = UpdatePasswordRequest::builder()
        .token("doccnxxxxxx")
        .as_doc()
        .password("NewPassword@2023")
        .build();

    match client.permission.update_password(&request, None).await {
        Ok(response) => {
            println!("密码更新成功");

            let password_info = response.data.password_info();
            println!("新密码: {}", password_info.password);
            println!("密码变更摘要: {}", password_info.change_summary());

            if let Some(update_time) = password_info.update_time_formatted() {
                println!("更新信息: {}", update_time);
            }

            if let Some(expire_time) = password_info.expire_time_formatted() {
                println!("过期信息: {}", expire_time);
            }

            if let Some(prev_hint) = &password_info.previous_password_hint {
                println!("原密码提示: {}", prev_hint);
            }

            println!("更新摘要: {}", response.data.update_summary());
            println!("安全评估: {}", response.data.security_assessment());
            println!("安全改进: {}", password_info.security_improvement());

            // 安全建议
            let recommendations = response.data.security_recommendations();
            if !recommendations.is_empty() {
                println!("\n安全建议:");
                for (i, rec) in recommendations.iter().enumerate() {
                    println!("{}. {}", i + 1, rec);
                }
            }

            // 操作提示
            let tips = response.data.operation_tips();
            if !tips.is_empty() {
                println!("\n操作提示:");
                for (i, tip) in tips.iter().enumerate() {
                    println!("{}. {}", i + 1, tip);
                }
            }
        }
        Err(e) => {
            eprintln!("更新密码失败: {:?}", e);
        }
    }

    // 生成随机密码更新
    println!("\n--- 生成随机密码更新 ---");
    let random_request = UpdatePasswordRequest::builder()
        .token("shtcnxxxxxx")
        .as_sheet()
        .random_password(12)
        .build();

    match client
        .permission
        .update_password(&random_request, None)
        .await
    {
        Ok(response) => {
            println!("电子表格随机密码更新成功");

            let password_info = response.data.password_info();
            println!("新随机密码: {}", password_info.password);
            println!("密码长度: {}", password_info.password_length());
            println!("密码强度: {}", password_info.password_strength());
            println!("密码类型: {}", password_info.password_type());

            println!(
                "是否更新成功: {}",
                if response.data.is_updated() {
                    "是"
                } else {
                    "否"
                }
            );
        }
        Err(e) => {
            eprintln!("生成随机密码更新失败: {:?}", e);
        }
    }

    // 增强密码更新
    println!("\n--- 增强密码更新 ---");
    let enhance_request = UpdatePasswordRequest::builder()
        .token("bblcnxxxxxx")
        .as_bitable()
        .enhance_password("SecureBase")
        .build();

    match client
        .permission
        .update_password(&enhance_request, None)
        .await
    {
        Ok(response) => {
            println!("多维表格增强密码更新成功");

            let password_info = response.data.password_info();
            println!("增强密码: {}", password_info.password);
            println!("密码强度: {}", password_info.password_strength());
            println!("安全改进: {}", password_info.security_improvement());

            // 检查密码类型
            if password_info.password_type() == "包含特殊字符" {
                println!("✓ 密码包含特殊字符，安全性更高");
            }
        }
        Err(e) => {
            eprintln!("增强密码更新失败: {:?}", e);
        }
    }

    // 使用便捷方法更新密码
    println!("\n--- 使用便捷方法更新 ---");
    let wiki_request = UpdatePasswordRequest::for_wiki("wikicnxxxxxx", "SuperSecure@Wiki456");

    match client.permission.update_password(&wiki_request, None).await {
        Ok(response) => {
            println!("知识库密码更新成功");

            let password_info = response.data.password_info();
            println!("密码强度评估: {}", password_info.password_strength());

            // 根据强度给出反馈
            match password_info.password_strength() {
                "很强" => println!("✓ 密码安全性优秀"),
                "强" => println!("✓ 密码安全性良好"),
                "中等" => println!("○ 密码安全性一般，建议增强"),
                "弱" => println!("⚠️ 密码安全性较弱，建议立即更换"),
                _ => println!("? 密码强度未知"),
            }

            // 详细分析
            println!("密码分析:");
            println!("- 长度: {} 位", password_info.password_length());
            println!("- 类型: {}", password_info.password_type());
            println!(
                "- 纯数字: {}",
                if password_info.is_numeric_password() {
                    "是"
                } else {
                    "否"
                }
            );
        }
        Err(e) => {
            eprintln!("更新知识库密码失败: {:?}", e);
        }
    }

    // 简单数字密码更新
    println!("\n--- 简单数字密码更新 ---");
    let simple_request = UpdatePasswordRequest::builder()
        .token("doccnxxxxxx")
        .as_doc()
        .simple_password(654321)
        .build();

    match client
        .permission
        .update_password(&simple_request, None)
        .await
    {
        Ok(response) => {
            println!("简单数字密码更新完成");

            let password_info = response.data.password_info();
            println!("数字密码: {}", password_info.password);

            // 如果是弱密码，给出警告
            if password_info.password_strength() == "弱" {
                println!("⚠️ 警告: 当前密码强度较弱");

                let recommendations = response.data.security_recommendations();
                if !recommendations.is_empty() {
                    println!("强烈建议:");
                    for rec in recommendations {
                        if rec.contains("立即") {
                            println!("- {}", rec);
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("简单数字密码更新失败: {:?}", e);
        }
    }

    Ok(())
}
