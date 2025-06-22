use dotenvy::dotenv;
use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID not found");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET not found");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    // 创建多维表格示例
    println!("--- 1. 创建多维表格 ---");

    let create_req = open_lark::service::bitable::v1::app::CreateAppRequest::builder()
        .name("API测试多维表格")
        .time_zone("Asia/Shanghai")
        .build();

    let app_token = match client.bitable.v1.app.create(create_req, None).await {
        Ok(resp) => {
            if let Some(data) = resp.data {
                println!("✅ 创建多维表格成功!");
                println!("🆔 App Token: {}", data.app.app_token);
                println!("📋 名称: {}", data.app.name);
                println!("🔢 版本号: {}", data.app.revision);
                println!("🔗 链接: {}", data.app.url);
                data.app.app_token
            } else {
                eprintln!("❌ 响应数据为空");
                return Ok(());
            }
        }
        Err(e) => {
            eprintln!("❌ 创建多维表格失败: {:?}", e);
            return Ok(());
        }
    };

    // 获取多维表格元数据
    println!("\n--- 2. 获取多维表格元数据 ---");
    let get_req = open_lark::service::bitable::v1::app::GetAppRequest::new(&app_token);

    match client.bitable.v1.app.get(get_req, None).await {
        Ok(resp) => {
            if let Some(data) = resp.data {
                println!("✅ 获取多维表格元数据成功!");
                println!("🆔 App Token: {}", data.app.app_token);
                println!("📋 名称: {}", data.app.name);
                println!("🔢 版本号: {}", data.app.revision);
                println!(
                    "🔐 高级权限: {}",
                    if data.app.is_advanced {
                        "已开启"
                    } else {
                        "未开启"
                    }
                );
                println!("🌏 时区: {}", data.app.time_zone);
            } else {
                println!("❌ 响应数据为空");
            }
        }
        Err(e) => {
            eprintln!("❌ 获取多维表格元数据失败: {:?}", e);
        }
    }

    // 更新多维表格元数据
    println!("\n--- 3. 更新多维表格元数据 ---");
    let update_req = open_lark::service::bitable::v1::app::UpdateAppRequest::builder()
        .app_token(&app_token)
        .name("更新后的API测试多维表格")
        .is_advanced(false)
        .build();

    match client.bitable.v1.app.update(update_req, None).await {
        Ok(resp) => {
            if let Some(data) = resp.data {
                println!("✅ 更新多维表格元数据成功!");
                println!("🆔 App Token: {}", data.app.app_token);
                println!("📋 新名称: {}", data.app.name);
                println!("🔢 新版本号: {}", data.app.revision);
                println!(
                    "🔐 高级权限: {}",
                    if data.app.is_advanced {
                        "已开启"
                    } else {
                        "未开启"
                    }
                );
            } else {
                println!("❌ 响应数据为空");
            }
        }
        Err(e) => {
            eprintln!("❌ 更新多维表格元数据失败: {:?}", e);
        }
    }

    // 复制多维表格
    println!("\n--- 4. 复制多维表格 ---");
    let copy_req = open_lark::service::bitable::v1::app::CopyAppRequest::builder()
        .app_token(&app_token)
        .name("复制的多维表格")
        .time_zone("Asia/Shanghai")
        .build();

    match client.bitable.v1.app.copy(copy_req, None).await {
        Ok(resp) => {
            if let Some(data) = resp.data {
                println!("✅ 复制多维表格成功!");
                println!("🆔 新 App Token: {}", data.app.app_token);
                println!("📋 新名称: {}", data.app.name);
                println!("🔢 版本号: {}", data.app.revision);
                println!("🔗 新链接: {}", data.app.url);

                println!("\n💡 复制说明:");
                println!("- 原表格 Token: {}", app_token);
                println!("- 复制表格 Token: {}", data.app.app_token);
            } else {
                println!("❌ 响应数据为空");
            }
        }
        Err(e) => {
            eprintln!("❌ 复制多维表格失败: {:?}", e);
        }
    }

    println!("\n💡 多维表格功能说明:");
    println!("- 多维表格是类似 Excel 的在线表格工具");
    println!("- 支持多种数据类型：文本、数字、日期、单选、多选等");
    println!("- 可以创建多个数据表，每个表有独立的字段和记录");
    println!("- 支持视图功能，可以对数据进行筛选、排序、分组");
    println!("- 高级权限功能可以精细控制用户访问权限");

    println!("\n🎯 接下来可以进行的操作:");
    println!("- 使用数据表 API 创建和管理表格");
    println!("- 使用字段 API 定义数据结构");
    println!("- 使用记录 API 添加和查询数据");
    println!("- 使用视图 API 创建不同的数据视图");

    Ok(())
}
