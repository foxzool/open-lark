//! 表格信息获取示例
//!
//! 本示例演示如何使用 open-lark SDK 获取电子表格的基本信息，
//! 包括表格标题、工作表列表、权限信息等。

use open_lark::prelude::*;
use open_lark::service::sheets::v2::metainfo::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化客户端（需要配置环境变量中的 APP_ID 和 APP_SECRET）
    let config = Config::from_env()?;
    let client = LarkClient::new(config);

    // 示例电子表格 token
    let spreadsheet_token = "your_spreadsheet_token_here";

    // 获取表格基本信息
    println!("📊 获取表格基本信息...");
    match client
        .sheets
        .v2
        .metainfo
        .get_basic_meta(spreadsheet_token)
        .await
    {
        Ok(response) => {
            let meta = response.data?;
            println!("✅ 表格标题: {}", meta.title);
            println!("📁 工作表数量: {}", meta.sheet_count);
            println!("👤 所有者: {} ({})", meta.owner.name, meta.owner.user_id);

            // 显示工作表列表
            println!("\n📋 工作表列表:");
            for sheet in &meta.sheets {
                println!(
                    "  - {} (ID: {}, 类型: {}, 行数: {}, 列数: {})",
                    sheet.title,
                    sheet.sheet_id,
                    sheet.sheet_type,
                    sheet.row_count,
                    sheet.column_count
                );
            }
        }
        Err(e) => {
            eprintln!("❌ 获取表格信息失败: {}", e);
            return Err(e.into());
        }
    }

    // 获取完整信息（包括权限和自定义属性）
    println!("\n🔐 获取表格完整信息（包括权限）...");
    match client
        .sheets
        .v2
        .metainfo
        .get_full_meta(spreadsheet_token)
        .await
    {
        Ok(response) => {
            let meta = response.data?;

            // 显示权限信息
            if let Some(permissions) = &meta.permissions {
                println!("\n🔒 权限信息:");
                println!("  - 可编辑: {}", permissions.editable);
                println!("  - 可评论: {}", permissions.commentable);
                println!("  - 可分享: {}", permissions.shareable);
                println!("  - 权限类型: {}", permissions.permission_type);
            }

            // 显示自定义属性
            if let Some(custom_props) = &meta.custom_properties {
                println!("\n🏷️  自定义属性:");
                for (key, value) in custom_props {
                    println!("  - {}: {}", key, value);
                }
            }

            println!("\n✅ 完整信息获取成功!");
        }
        Err(e) => {
            eprintln!("❌ 获取完整信息失败: {}", e);
        }
    }

    // 使用构建器模式获取信息
    println!("\n🔧 使用构建器模式获取信息...");
    let builder_request = client
        .sheets
        .v2
        .metainfo
        .get_spreadsheet_meta_builder(spreadsheet_token)
        .include_permissions(true)
        .include_custom_properties(false)
        .language("zh_CN");

    match builder_request.execute().await {
        Ok(response) => {
            let meta = response.data?;
            println!("✅ 构建器模式获取成功: {}", meta.title);
            println!("🕒 更新时间: {}", meta.update_time);
        }
        Err(e) => {
            eprintln!("❌ 构建器模式获取失败: {}", e);
        }
    }

    println!("\n🎉 表格信息示例执行完成!");
    Ok(())
}
