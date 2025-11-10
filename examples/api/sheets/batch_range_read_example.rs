//! 多个范围读取 API 使用示例
//!
//! 演示如何使用 BatchRangeReadService 进行批量数据读取操作

use open_lark::prelude::*;
use open_lark::service::sheets::v2::{BatchRangeReadService, BatchRangeReadRequest};

#[tokio::main]
async fn main() -> SDKResult<()> {
    // 创建配置
    let config = Config::builder()
        .app_id("your_app_id")
        .app_secret("your_app_secret")
        .build();

    // 创建服务实例
    let service = BatchRangeReadService::new(config);

    println!("🚀 开始演示多个范围读取功能...\n");

    // 示例1：使用传统方法创建请求
    println!("📖 示例1：使用传统方法批量读取范围");
    let request = BatchRangeReadRequest::new(
        "your_spreadsheet_token",
        vec![
            "Sheet1!A1:C10",      // 读取A1到C10的数据
            "Sheet1!E1:F20",      // 读取E1到F20的数据
            "Sheet2!A1:D15",      // 读取Sheet2的A1到D15数据
        ]
    )
    .value_render_option("FormattedValue")
    .date_time_render_option("FormattedString")
    .user_id_type("open_id");

    match service.read_ranges(request).await {
        Ok(response) => {
            println!("✅ 批量范围读取成功！");
            println!("📊 读取到 {} 个范围的数据", response.data.value_ranges.len());
            println!("📈 表格版本号: {}", response.data.revision);
            println!("📋 是否完全读取: {}", response.data.read_all);

            // 遍历并显示每个范围的数据
            for (index, value_range) in response.data.value_ranges.iter().enumerate() {
                println!("\n--- 范围 {} ---", index + 1);
                println!("📍 范围标识: {}", value_range.range);
                println!("📐 主要维度: {}", value_range.major_dimension);
                println!("🔢 版本号: {}", value_range.revision);
                println!("📄 数据内容: {}", serde_json::to_string_pretty(&value_range.values).unwrap_or_default());
            }
        }
        Err(error) => {
            println!("❌ 批量范围读取失败: {}", error);
        }
    }

    println!("\n" + "=".repeat(50).as_str() + "\n");

    // 示例2：使用构建器模式
    println!("🔧 示例2：使用构建器模式进行批量读取");
    let builder_result = service
        .read_ranges_builder("your_spreadsheet_token")
        .add_range("Sheet1!A1:B5")
        .add_range("Sheet1!D1:E10")
        .add_range("Summary!A1:Z100")
        .value_render_option("UnformattedValue")
        .date_time_render_option("FormattedString")
        .execute()
        .await;

    match builder_result {
        Ok(response) => {
            println!("✅ 构建器模式批量读取成功！");
            println!("📊 读取到 {} 个范围的数据", response.data.value_ranges.len());

            // 显示范围统计信息
            let mut total_cells = 0;
            for value_range in &response.data.value_ranges {
                if let Some(array) = value_range.values.as_array() {
                    let cells_in_range: usize = array.iter()
                        .map(|row| row.as_array().map_or(0, |r| r.len()))
                        .sum();
                    total_cells += cells_in_range;
                    println!("📈 范围 {} 包含 {} 个单元格", value_range.range, cells_in_range);
                }
            }
            println!("📊 总计读取 {} 个单元格", total_cells);
        }
        Err(error) => {
            println!("❌ 构建器模式批量读取失败: {}", error);
        }
    }

    println!("\n" + "=".repeat(50).as_str() + "\n");

    // 示例3：使用便捷方法
    println!("⚡ 示例3：使用便捷方法快速读取");
    match service.read_ranges_simple(
        "your_spreadsheet_token",
        vec!["Sheet1!A1:C3", "Sheet1!E1:G3"]
    ).await {
        Ok(response) => {
            println!("✅ 便捷方法批量读取成功！");
            println!("📊 读取到 {} 个范围的数据", response.data.value_ranges.len());
        }
        Err(error) => {
            println!("❌ 便捷方法批量读取失败: {}", error);
        }
    }

    println!("\n🎉 多个范围读取演示完成！");

    Ok(())
}