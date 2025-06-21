use dotenv::dotenv;
use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let app_id = std::env::var("APP_ID").expect("APP_ID not found");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET not found");

    let client = LarkClient::builder(app_id, app_secret)
        .with_app_type(AppType::SelfBuilt)
        .with_enable_token_cache(true)
        .build();

    // 创建多个样式设置
    use open_lark::service::sheets::v3::data_operation::{CellStyle, FontStyle, BorderStyle, RangeStyleData};

    // 第一个范围的样式 - 标题样式
    let title_style = CellStyle {
        font: Some(FontStyle {
            bold: Some(true),
            italic: Some(false),
            size: Some("16".to_string()),
            name: Some("Arial".to_string()),
        }),
        text_decoration: None,
        formatter: Some("text".to_string()),
        align: Some(2), // 居中对齐
        back_color: Some("#4472C4".to_string()), // 蓝色背景
        fore_color: Some("#FFFFFF".to_string()), // 白色字体
        border: Some(BorderStyle {
            border_type: Some("FULL_BORDER".to_string()),
            color: Some("#000000".to_string()),
        }),
        clean: None,
    };

    // 第二个范围的样式 - 数据样式
    let data_style = CellStyle {
        font: Some(FontStyle {
            bold: Some(false),
            italic: Some(false),
            size: Some("12".to_string()),
            name: Some("Arial".to_string()),
        }),
        text_decoration: None,
        formatter: Some("text".to_string()),
        align: Some(1), // 左对齐
        back_color: Some("#F2F2F2".to_string()), // 浅灰色背景
        fore_color: Some("#000000".to_string()), // 黑色字体
        border: Some(BorderStyle {
            border_type: Some("THIN_BORDER".to_string()),
            color: Some("#CCCCCC".to_string()),
        }),
        clean: None,
    };

    // 批量设置单元格样式示例
    let req = open_lark::service::sheets::v3::data_operation::BatchSetCellStyleRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8") // 替换为实际的表格 token
        .sheet_id("Sheet1") // 替换为实际的工作表 ID
        .add_range_style("A1:D1", title_style) // 设置标题行样式
        .add_range_style("A2:D10", data_style) // 设置数据区域样式
        .build();

    match client.sheets.v3.data_operation.batch_set_cell_style(req, None).await {
        Ok(resp) => {
            println!("✅ 批量设置单元格样式成功!");
            println!("📊 更新的单元格总数: {}", resp.data.updated_cells);
            println!("📍 更新的范围: {:?}", resp.data.updated_ranges);
            println!("✅ 成功批次数: {}", resp.data.success_count);
            
            if resp.data.failure_count > 0 {
                println!("❌ 失败批次数: {}", resp.data.failure_count);
            }
        }
        Err(e) => {
            eprintln!("❌ 批量设置单元格样式失败: {:?}", e);
        }
    }

    // 也可以用另一种方式创建请求
    println!("\n--- 使用另一种方式设置样式 ---");
    
    let highlight_style = CellStyle {
        font: Some(FontStyle {
            bold: Some(true),
            italic: Some(true),
            size: Some("14".to_string()),
            name: Some("Arial".to_string()),
        }),
        text_decoration: Some(2), // 下划线
        formatter: Some("text".to_string()),
        align: Some(2), // 居中
        back_color: Some("#FFFF00".to_string()), // 黄色背景
        fore_color: Some("#FF0000".to_string()), // 红色字体
        border: None,
        clean: None,
    };

    let range_styles = vec![
        RangeStyleData {
            range: "E1:F5".to_string(),
            style: highlight_style,
        }
    ];

    let req2 = open_lark::service::sheets::v3::data_operation::BatchSetCellStyleRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8") // 替换为实际的表格 token
        .sheet_id("Sheet1") // 替换为实际的工作表 ID
        .range_styles(range_styles)
        .build();

    match client.sheets.v3.data_operation.batch_set_cell_style(req2, None).await {
        Ok(resp) => {
            println!("✅ 第二次批量设置样式成功!");
            println!("📊 更新的单元格数: {}", resp.data.updated_cells);
        }
        Err(e) => {
            eprintln!("❌ 第二次批量设置样式失败: {:?}", e);
        }
    }

    Ok(())
}