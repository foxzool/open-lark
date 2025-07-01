//! 飞书汇报 v1 - 移除规则看板示例
//!
//! 该示例演示如何使用汇报 API 移除规则看板

use dotenvy::dotenv;
use open_lark::{core::req_option::RequestOption, prelude::*};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID 环境变量未设置");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET 环境变量未设置");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .build();

    println!("===== 移除规则看板 =====");

    // 要移除的看板ID（替换为实际的看板ID）
    let view_id = "view_123456";

    println!("正在移除看板: {view_id}");

    let response = client
        .report
        .rule_view
        .remove(view_id, Option::<RequestOption>::None)
        .await?;

    if response.code() == 0 {
        println!("移除成功！");

        if let Some(data) = response.data {
            if data.success {
                println!("看板移除操作已完成");

                if let Some(removed_view_id) = &data.view_id {
                    println!("被移除的看板ID: {removed_view_id}");
                }
            } else {
                println!("看板移除操作失败");

                if let Some(error_msg) = &data.error_message {
                    println!("错误信息: {error_msg}");
                }
            }
        }
    } else {
        eprintln!("移除失败: {} - {}", response.code(), response.msg());

        // 显示可能的错误原因
        match response.code() {
            1001 => eprintln!("提示: 请检查看板ID是否正确"),
            1002 => eprintln!("提示: 可能没有权限移除该看板"),
            1003 => eprintln!("提示: 看板可能已经被删除"),
            _ => eprintln!("提示: 请检查网络连接和API访问权限"),
        }
    }

    println!("\n===== 批量移除多个看板 =====");

    // 批量移除多个看板
    let view_ids = ["view_111", "view_222", "view_333"];

    for (index, view_id) in view_ids.iter().enumerate() {
        println!(
            "\n正在移除看板 {} / {}: {}",
            index + 1,
            view_ids.len(),
            view_id
        );

        match client
            .report
            .rule_view
            .remove(view_id, Option::<RequestOption>::None)
            .await
        {
            Ok(response) => {
                if response.code() == 0 {
                    if let Some(data) = response.data {
                        if data.success {
                            println!("✓ 看板 {view_id} 移除成功");
                        } else {
                            println!("✗ 看板 {} 移除失败: {:?}", view_id, data.error_message);
                        }
                    }
                } else {
                    println!("✗ 看板 {} 移除失败: {}", view_id, response.msg());
                }
            }
            Err(e) => {
                eprintln!("✗ 看板 {view_id} 移除出错: {e}");
            }
        }

        // 避免请求过于频繁，稍作延迟
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    println!("\n批量移除完成！");

    Ok(())
}
