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

    // 获取企业信息
    match client.tenant.v2.tenant.query(None).await {
        Ok(response) => {
            if response.success() {
                if let Some(data) = response.data {
                    println!("✅ 企业信息获取成功:");
                    println!("企业名称: {:?}", data.tenant.name);
                    println!("显示名称: {:?}", data.tenant.display_name);
                    println!("企业ID: {:?}", data.tenant.tenant_key);
                    if let Some(avatar) = &data.tenant.avatar {
                        println!("头像信息:");
                        println!("  72x72: {:?}", avatar.avatar_72);
                        println!("  240x240: {:?}", avatar.avatar_240);
                        println!("  640x640: {:?}", avatar.avatar_640);
                        println!("  原始尺寸: {:?}", avatar.avatar_origin);
                    }
                } else {
                    println!("⚠️ 响应成功但数据为空");
                }
            } else {
                println!("❌ 获取企业信息失败:");
                println!("错误码: {}", response.code());
                println!("错误信息: {}", response.msg());
                if let Some(err) = response.err() {
                    println!("详细错误: {err:?}");
                }
            }
        }
        Err(e) => {
            println!("❌ 请求失败: {e:?}");
        }
    }

    println!("\n{}\n", "=".repeat(50));

    // 获取企业席位信息
    match client
        .tenant
        .v2
        .tenant_product_assign_info
        .query(None)
        .await
    {
        Ok(response) => {
            if response.success() {
                if let Some(data) = response.data {
                    println!("✅ 企业席位信息获取成功:");
                    let info = &data.tenant_product_assign_info;
                    println!("总席位数: {:?}", info.total_seat_count);
                    println!("已分配席位数: {:?}", info.assigned_seat_count);
                    println!("历史最大分配席位数: {:?}", info.max_assigned_seat_count);
                    println!("购买时间: {:?}", info.purchase_time);
                    println!("到期时间: {:?}", info.expire_time);
                    println!("产品名称: {:?}", info.product_name);
                    println!("服务状态: {:?}", info.service_status);
                } else {
                    println!("⚠️ 响应成功但数据为空");
                }
            } else {
                println!("❌ 获取企业席位信息失败:");
                println!("错误码: {}", response.code());
                println!("错误信息: {}", response.msg());
                if let Some(err) = response.err() {
                    println!("详细错误: {err:?}");
                }
            }
        }
        Err(e) => {
            println!("❌ 请求失败: {e:?}");
        }
    }

    Ok(())
}
