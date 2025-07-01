use dotenvy::dotenv;
use open_lark::{core::constants::AppType, prelude::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET must be set");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .build();

    // 获取认证信息
    match client.verification.v1.get(None).await {
        Ok(response) => {
            if response.success() {
                if let Some(data) = response.data {
                    println!("✅ 认证信息获取成功:");
                    let verification = &data.verification;
                    println!("应用ID: {:?}", verification.app_id);
                    println!("应用名称: {:?}", verification.app_name);
                    println!("应用状态: {:?}", verification.app_status);
                    println!("认证状态: {:?}", verification.verification_status);
                    println!("认证类型: {:?}", verification.verification_type);
                    println!("认证时间: {:?}", verification.verification_time);
                    println!("过期时间: {:?}", verification.expire_time);

                    if let Some(scopes) = &verification.scopes {
                        println!("权限范围:");
                        for scope in scopes {
                            println!("  - {scope}");
                        }
                    }

                    if let Some(tenant_info) = &verification.tenant_info {
                        println!("租户信息:");
                        println!("  租户key: {:?}", tenant_info.tenant_key);
                        println!("  租户名称: {:?}", tenant_info.tenant_name);
                    }
                } else {
                    println!("⚠️ 响应成功但数据为空");
                }
            } else {
                println!("❌ 获取认证信息失败:");
                println!("错误码: {}", response.code());
                println!("错误信息: {}", response.msg());
                if let Some(err) = response.err() {
                    println!("详细错误: {err:?}");
                }

                // 展示详细错误处理信息
                response.print_error_details();
            }
        }
        Err(e) => {
            println!("❌ 请求失败: {e:?}");
        }
    }

    Ok(())
}
