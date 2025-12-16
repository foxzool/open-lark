/// CCM Sheet API 快速开始示例
//!
/// 最简单的使用方式，帮助用户快速上手表格API

use openlark_core::LarkClient;
use openlark_docs::ccm::ccm_sheet::old::v2::CcmSheetOldV2;
use openlark_core::config::Config;
use tokio;

use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 初始化配置
    let config = Config::builder()
        .app_id("your_app_id")              // 替换为你的应用ID
        .app_secret("your_app_secret")      // 替换为你的应用密钥
        .build()?;

    // 2. 创建客户端
    let client = LarkClient::new(config)?;
    let sheet_service = client.docs.ccm_sheet.old.v2();

    // 3. 准备表格token
    let spreadsheet_token = "your_spreadsheet_token"; // 替换为实际的表格token

    println!("🚀 CCM Sheet API 快速开始演示");

    // === 基础操作 ===

    // 读取数据
    println!("\n📖 读取表格数据");
    match read_data(&sheet_service, spreadsheet_token).await {
        Ok(_) => println!("✅ 数据读取成功"),
        Err(e) => println!("❌ 数据读取失败: {}", e),
    }

    // 写入数据
    println!("\n✏️  写入表格数据");
    match write_data(&sheet_service, spreadsheet_token).await {
        Ok(_) => println!("✅ 数据写入成功"),
        Err(e) => println!("❌ 数据写入失败: {}", e),
    }

    // 获取表格信息
    println!("\n📋 获取表格元信息");
    match get_sheet_info(&sheet_service, spreadsheet_token).await {
        Ok(_) => println!("✅ 表格信息获取成功"),
        Err(e) => println!("❌ 表格信息获取失败: {}", e),
    }

    println!("\n🎉 快速开始演示完成！");
    println!("\n📚 更多功能请参考 comprehensive_demo.rs");

    Ok(())
}

/// 读取数据示例
async fn read_data(
    sheet_service: &CcmSheetOldV2,
    spreadsheet_token: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let request = sheet_service.readsinglerange();
    let params = serde_json::json!({
        "spreadsheetToken": spreadsheet_token,
        "range": "Sheet1!A1:C10",              // 读取范围
        "valueRenderOption": "DisplayedValue" // 显示格式
    });

    let response = request.execute(serde_json::from_value(params)?).await?;

    if let Some(data) = response.data {
        println!("📊 读取结果:");
        if let Some(value_range) = data.value_range {
            if let Some(values) = value_range.values {
                for (i, row) in values.iter().enumerate() {
                    println!("   第{}行: {:?}", i + 1, row);
                }
            }
        }
    }

    Ok(())
}

/// 写入数据示例
async fn write_data(
    sheet_service: &CcmSheetOldV2,
    spreadsheet_token: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let request = sheet_service.writesinglerange();
    let params = serde_json::json!({
        "spreadsheetToken": spreadsheet_token,
        "range": "Sheet1!A1:E5",
        "values": [
            ["日期", "产品", "销量", "单价", "总收入"],
            ["2024-01-01", "iPhone", 100, 6999, 699900],
            ["2024-01-02", "iPad", 50, 3999, 199950],
            ["2024-01-03", "MacBook", 30, 12999, 389970],
            ["2024-01-04", "AirPods", 200, 1299, 259800]
        ]
    });

    let response = request.execute(serde_json::from_value(params)?).await?;

    if let Some(result) = response.data {
        println!("✅ 写入统计:");
        println!("   更新行数: {:?}", result.updated_rows);
        println!("   更新列数: {:?}", result.updated_columns);
        println!("   更新单元格: {:?}", result.updated_cells);
    }

    Ok(())
}

/// 获取表格信息示例
async fn get_sheet_info(
    sheet_service: &CcmSheetOldV2,
    spreadsheet_token: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let request = sheet_service.getspreadsheetmeta();
    let params = serde_json::json!({
        "spreadsheetToken": spreadsheet_token
    });

    let response = request.execute(serde_json::from_value(params)?).await?;

    if let Some(meta) = response.data {
        println!("📋 表格信息:");
        println!("   标题: {}", meta.title);
        println!("   Token: {}", meta.spreadsheet_token);
        println!("   创建时间: {}", meta.create_time);
        println!("   更新时间: {}", meta.update_time);
        println!("   工作表数量: {}", meta.sheets.len());

        for sheet in meta.sheets {
            println!("   工作表: {} (ID: {}, 类型: {})",
                sheet.title, sheet.sheet_id, sheet.sheet_type);
        }
    }

    Ok(())
}