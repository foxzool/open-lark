use dotenvy::dotenv;
use open_lark::{prelude::*, service::workplace::workplace_access_data::*};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID 环境变量未设置");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET 环境变量未设置");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .build();

    // 获取工作台访问数据
    println!("=== 获取工作台访问数据 ===");
    let search_request = AccessDataSearchRequest {
        page_size: Some(10),
        start_time: Some(1640995200), // 2022-01-01 00:00:00 UTC
        end_time: Some(1672531199),   // 2022-12-31 23:59:59 UTC
        ..Default::default()
    };

    match client
        .workplace
        .workplace_access_data
        .search(search_request, None)
        .await
    {
        Ok(response) => {
            let search_data = &response.access_data;
            println!("查询工作台访问数据成功：");
            for access_data in &search_data.items {
                if let Some(user_id) = &access_data.user_id {
                    println!("  - 用户ID: {user_id}");
                }
                if let Some(access_time) = access_data.access_time {
                    println!("    访问时间: {access_time}");
                }
                if let Some(access_type) = &access_data.access_type {
                    println!("    访问类型: {access_type}");
                }
                if let Some(access_count) = access_data.access_count {
                    println!("    访问次数: {access_count}");
                }
                if let Some(duration) = access_data.duration {
                    println!("    访问时长: {duration}秒");
                }
                if let Some(platform) = &access_data.platform {
                    println!("    平台: {platform}");
                }
                println!();
            }

            if let Some(has_more) = search_data.has_more {
                if has_more {
                    println!("还有更多数据，可使用 page_token 继续查询");
                }
            }
        }
        Err(e) => {
            eprintln!("查询工作台访问数据失败: {e:?}");
        }
    }

    // 获取定制工作台访问数据
    println!("=== 获取定制工作台访问数据 ===");
    let custom_search_request = CustomAccessDataSearchRequest {
        page_size: Some(10),
        start_time: Some(1640995200),
        end_time: Some(1672531199),
        ..Default::default()
    };

    match client
        .workplace
        .workplace_access_data
        .search_custom(custom_search_request, None)
        .await
    {
        Ok(response) => {
            let custom_data = &response.access_data;
            println!("查询定制工作台访问数据成功：");
            for access_data in &custom_data.items {
                if let Some(user_id) = &access_data.user_id {
                    println!("  - 用户ID: {user_id}");
                }
                if let Some(custom_workplace_id) = &access_data.custom_workplace_id {
                    println!("    定制工作台ID: {custom_workplace_id}");
                }
                if let Some(access_time) = access_data.access_time {
                    println!("    访问时间: {access_time}");
                }
                if let Some(access_count) = access_data.access_count {
                    println!("    访问次数: {access_count}");
                }
                if let Some(duration) = access_data.duration {
                    println!("    访问时长: {duration}秒");
                }
                println!();
            }
        }
        Err(e) => {
            eprintln!("查询定制工作台访问数据失败: {e:?}");
        }
    }

    // 获取定制工作台小组件访问数据
    println!("=== 获取定制工作台小组件访问数据 ===");
    let widget_search_request = CustomWidgetAccessDataSearchRequest {
        page_size: Some(10),
        start_time: Some(1640995200),
        end_time: Some(1672531199),
        ..Default::default()
    };

    match client
        .workplace
        .workplace_access_data
        .search_custom_widget(widget_search_request, None)
        .await
    {
        Ok(response) => {
            let widget_data = &response.access_data;
            println!("查询定制工作台小组件访问数据成功：");
            for widget_access in &widget_data.items {
                if let Some(user_id) = &widget_access.user_id {
                    println!("  - 用户ID: {user_id}");
                }
                if let Some(custom_workplace_id) = &widget_access.custom_workplace_id {
                    println!("    定制工作台ID: {custom_workplace_id}");
                }
                if let Some(widget_id) = &widget_access.widget_id {
                    println!("    小组件ID: {widget_id}");
                }
                if let Some(widget_name) = &widget_access.widget_name {
                    println!("    小组件名称: {widget_name}");
                }
                if let Some(access_time) = widget_access.access_time {
                    println!("    访问时间: {access_time}");
                }
                if let Some(access_count) = widget_access.access_count {
                    println!("    访问次数: {access_count}");
                }
                if let Some(click_count) = widget_access.click_count {
                    println!("    点击次数: {click_count}");
                }
                println!();
            }
        }
        Err(e) => {
            eprintln!("查询定制工作台小组件访问数据失败: {e:?}");
        }
    }

    // 按用户筛选查询
    println!("=== 按用户筛选查询工作台访问数据 ===");
    let user_filtered_request = AccessDataSearchRequest {
        page_size: Some(5),
        user_id: Some("user_001".to_string()),
        access_type: Some("login".to_string()),
        ..Default::default()
    };

    match client
        .workplace
        .workplace_access_data
        .search(user_filtered_request, None)
        .await
    {
        Ok(response) => {
            let filtered_data = &response.access_data;
            println!("按用户筛选查询工作台访问数据成功：");
            for access_data in &filtered_data.items {
                if let Some(user_id) = &access_data.user_id {
                    println!("  - 用户ID: {user_id}");
                }
                if let Some(access_type) = &access_data.access_type {
                    println!("    访问类型: {access_type}");
                }
                if let Some(platform) = &access_data.platform {
                    println!("    平台: {platform}");
                }
                if let Some(device_type) = &access_data.device_type {
                    println!("    设备类型: {device_type}");
                }
                println!();
            }
        }
        Err(e) => {
            eprintln!("按用户筛选查询工作台访问数据失败: {e:?}");
        }
    }

    Ok(())
}
