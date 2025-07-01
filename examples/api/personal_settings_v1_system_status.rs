use dotenvy::dotenv;
use open_lark::{
    prelude::*,
    service::personal_settings::models::{
        BatchSystemStatusRequest, CreateSystemStatusRequest, I18nContent, ListSystemStatusRequest,
        UpdateSystemStatusRequest,
    },
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenv().ok();
    let app_id = env::var("APP_ID").expect("APP_ID not set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET not set");

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    println!("=== 个人设置 API 系统状态管理示例 ===");

    // 示例1: 创建系统状态
    println!("\n1. 创建系统状态");
    let create_req = CreateSystemStatusRequest {
        title: "工作状态".to_string(),
        i18n_title: Some(I18nContent {
            zh_cn: Some("工作状态".to_string()),
            en_us: Some("Work Status".to_string()),
            ja_jp: Some("作業状況".to_string()),
        }),
        icon_style: Some("work".to_string()),
        icon_url: Some("https://example.com/work-icon.png".to_string()),
        priority: Some(100),
    };

    match client
        .personal_settings
        .v1
        .system_status
        .create(create_req, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 创建系统状态成功");
                println!(
                    "系统状态ID: {}",
                    data.system_status.system_status_id.unwrap_or_default()
                );
                println!("标题: {}", data.system_status.title.unwrap_or_default());
                println!(
                    "优先级: {}",
                    data.system_status.priority.unwrap_or_default()
                );
            } else {
                println!("✅ 创建系统状态成功，但未返回详细信息");
            }
        }
        Err(e) => println!("❌ 创建系统状态失败: {e:?}"),
    }

    // 示例2: 获取系统状态列表
    println!("\n2. 获取系统状态列表");
    let list_req = Some(ListSystemStatusRequest {
        page: Some(1),
        page_size: Some(20),
    });

    match client
        .personal_settings
        .v1
        .system_status
        .list(list_req, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 获取系统状态列表成功");
                println!("总数量: {}", data.items.len());
                for (i, status) in data.items.iter().enumerate() {
                    println!(
                        "  {}. ID: {}, 标题: {}, 开启状态: {}",
                        i + 1,
                        status
                            .system_status_id
                            .as_ref()
                            .unwrap_or(&"无".to_string()),
                        status.title.as_ref().unwrap_or(&"无".to_string()),
                        status.is_open.unwrap_or(false)
                    );
                }
            } else {
                println!("✅ 获取系统状态列表成功，但未返回数据");
            }
        }
        Err(e) => println!("❌ 获取系统状态列表失败: {e:?}"),
    }

    // 示例3: 更新系统状态（如果有创建的状态）
    let system_status_id = "your_system_status_id"; // 实际使用时需要替换为真实的ID
    println!("\n3. 更新系统状态 (ID: {system_status_id})");
    let update_req = UpdateSystemStatusRequest {
        title: Some("更新后的工作状态".to_string()),
        i18n_title: Some(I18nContent {
            zh_cn: Some("更新后的工作状态".to_string()),
            en_us: Some("Updated Work Status".to_string()),
            ja_jp: None,
        }),
        icon_style: Some("updated-work".to_string()),
        icon_url: None,
        priority: Some(50),
    };

    match client
        .personal_settings
        .v1
        .system_status
        .patch(system_status_id, update_req, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 更新系统状态成功");
                println!(
                    "更新后标题: {}",
                    data.system_status.title.unwrap_or_default()
                );
                println!(
                    "更新后优先级: {}",
                    data.system_status.priority.unwrap_or_default()
                );
            } else {
                println!("✅ 更新系统状态成功，但未返回详细信息");
            }
        }
        Err(e) => println!("❌ 更新系统状态失败: {e:?}"),
    }

    // 示例4: 批量开启系统状态
    println!("\n4. 批量开启系统状态");
    let batch_open_req = BatchSystemStatusRequest {
        system_status_ids: vec!["status_id_1".to_string(), "status_id_2".to_string()],
    };

    match client
        .personal_settings
        .v1
        .system_status
        .batch_open(batch_open_req, None)
        .await
    {
        Ok(_) => println!("✅ 批量开启系统状态成功"),
        Err(e) => println!("❌ 批量开启系统状态失败: {e:?}"),
    }

    // 示例5: 批量关闭系统状态
    println!("\n5. 批量关闭系统状态");
    let batch_close_req = BatchSystemStatusRequest {
        system_status_ids: vec!["status_id_1".to_string(), "status_id_2".to_string()],
    };

    match client
        .personal_settings
        .v1
        .system_status
        .batch_close(batch_close_req, None)
        .await
    {
        Ok(_) => println!("✅ 批量关闭系统状态成功"),
        Err(e) => println!("❌ 批量关闭系统状态失败: {e:?}"),
    }

    // 示例6: 删除系统状态
    println!("\n6. 删除系统状态 (ID: {system_status_id})");
    match client
        .personal_settings
        .v1
        .system_status
        .delete(system_status_id, None)
        .await
    {
        Ok(_) => println!("✅ 删除系统状态成功"),
        Err(e) => println!("❌ 删除系统状态失败: {e:?}"),
    }

    println!("\n=== 个人设置系统状态管理示例完成 ===");
    Ok(())
}
