use open_lark::{prelude::*, service::permission::public_v1::CreatePasswordRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").unwrap();
    let app_secret = std::env::var("APP_SECRET").unwrap();

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    // 为文档设置简单密码
    let request = CreatePasswordRequest::builder()
        .token("doccnxxxxxx")
        .as_doc()
        .password("123456")
        .build();

    match client.permission.create_password(&request, None).await {
        Ok(response) => {
            println!("密码保护设置成功");

            if let Some(data) = response.data {
                let password_info = data.password_info();
                println!("密码: {}", password_info.password);
                println!("密码信息: {}", password_info.password_summary());

                if let Some(create_time) = password_info.create_time_formatted() {
                    println!("创建信息: {}", create_time);
                }

                if let Some(expire_time) = password_info.expire_time_formatted() {
                    println!("过期信息: {}", expire_time);
                }

                println!("创建摘要: {}", data.creation_summary());

                // 安全提醒
                let tips = data.security_tips();
                if !tips.is_empty() {
                    println!("\n安全提醒:");
                    for (i, tip) in tips.iter().enumerate() {
                        println!("{}. {}", i + 1, tip);
                    }
                }

                // 操作建议
                let recommendations = data.operation_recommendations();
                if !recommendations.is_empty() {
                    println!("\n操作建议:");
                    for (i, rec) in recommendations.iter().enumerate() {
                        println!("{}. {}", i + 1, rec);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("设置密码保护失败: {:?}", e);
        }
    }

    // 为电子表格设置随机密码
    println!("\n--- 设置随机密码 ---");
    let random_request = CreatePasswordRequest::builder()
        .token("shtcnxxxxxx")
        .as_sheet()
        .random_password(8)
        .build();

    match client.permission.create_password(&random_request, None).await {
        Ok(response) => {
            println!("电子表格随机密码设置成功");

            if let Some(data) = response.data {
                let password_info = data.password_info();
                println!("随机密码: {}", password_info.password);
                println!("密码强度: {}", password_info.password_strength());
                println!(
                    "密码类型: {}",
                    if password_info.is_numeric_password() {
                        "纯数字"
                    } else {
                        "混合字符"
                    }
                );
                println!("密码长度: {}", password_info.password_length());

                println!(
                    "密码到期时间: {}",
                    password_info
                        .expire_time_formatted()
                        .unwrap_or("无过期时间".to_string())
                );
            }
        }
        Err(e) => {
            eprintln!("设置随机密码失败: {:?}", e);
        }
    }

    // 为多维表格设置数字密码
    println!("\n--- 设置数字密码 ---");
    let numeric_request = CreatePasswordRequest::builder()
        .token("bascnxxxxxx")
        .as_bitable()
        .simple_password(6)
        .build();

    match client.permission.create_password(&numeric_request, None).await {
        Ok(response) => {
            println!("多维表格数字密码设置成功");

            if let Some(data) = response.data {
                let password_info = data.password_info();
                println!("数字密码: {}", password_info.password);
                println!("是否纯数字: {}", password_info.is_numeric_password());
                println!("密码强度: {}", password_info.password_strength());

                if password_info.is_numeric_password() {
                    println!("⚠️ 密码强度较弱，建议使用更复杂的密码");
                }
            }
        }
        Err(e) => {
            eprintln!("设置数字密码失败: {:?}", e);
        }
    }

    // 为知识库设置复杂密码
    println!("\n--- 设置复杂密码 ---");
    let wiki_request = CreatePasswordRequest::for_wiki("wikicnxxxxxx", "Complex@Password123");

    match client.permission.create_password(&wiki_request, None).await {
        Ok(response) => {
            println!("知识库密码设置成功");

            if let Some(data) = response.data {
                let password_info = data.password_info();
                println!("复杂密码强度: {}", password_info.password_strength());
                println!(
                    "密码复杂度评级: {}",
                    match password_info.password_strength() {
                        "strong" => "高",
                        "medium" => "中",
                        _ => "低"
                    }
                );

                let security_tips = data.security_tips();
                println!("安全提醒数量: {}", security_tips.len());

                if password_info.password_strength() == "strong" {
                    println!("✓ 密码强度良好");
                }
            }
        }
        Err(e) => {
            eprintln!("设置复杂密码失败: {:?}", e);
        }
    }

    println!("\n⚠️ 密码保护重要提醒:");
    println!("   1. 定期更换密码以确保文档安全");
    println!("   2. 避免使用简单或常见的密码");
    println!("   3. 妥善保管密码，避免泄露给无关人员");
    println!("   4. 考虑设置密码过期时间");
    println!("   5. 对重要文档使用复杂密码");

    Ok(())
}