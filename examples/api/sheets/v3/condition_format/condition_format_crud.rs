use dotenvy::dotenv;
use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID not found");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET not found");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    // 批量创建条件格式示例
    println!("--- 1. 批量创建条件格式 ---");

    use open_lark::service::sheets::v3::condition_format::{ConditionFormatRule, FormatStyle};

    // 创建多个条件格式规则
    let red_format = FormatStyle::background_color("#FF0000")
        .with_text_color("#FFFFFF")
        .with_bold(true);

    let yellow_format = FormatStyle::background_color("#FFFF00")
        .with_text_color("#000000")
        .with_italic(true);

    let green_format = FormatStyle::background_color("#00FF00")
        .with_text_color("#000000")
        .with_underline(true);

    let condition_formats = vec![
        // 数值大于100时显示红色背景
        ConditionFormatRule::greater_than("A1:A10", 100.0, red_format),
        // 文本包含"重要"时显示黄色背景
        ConditionFormatRule::text_contains("B1:B10", "重要", yellow_format),
        // 重复值显示绿色背景
        ConditionFormatRule::duplicate_values("C1:C10", green_format),
    ];

    let create_req =
        open_lark::service::sheets::v3::condition_format::CreateConditionFormatsRequest::builder()
            .spreadsheet_token("shtcnmBA*****yGehy8") // 替换为实际的表格 token
            .sheet_id("Sheet1") // 替换为实际的工作表 ID
            .condition_formats(condition_formats)
            .build();

    let mut created_cf_ids = Vec::new();
    match client
        .sheets
        .v3
        .spreadsheet_sheet
        .create_condition_formats(create_req, None)
        .await
    {
        Ok(resp) => {
            if let Some(data) = resp.data {
                println!("✅ 批量创建条件格式成功!");
                println!("📊 创建成功数量: {}", data.created_count);
                println!("📋 条件格式详情:");
                for (i, item) in data.items.iter().enumerate() {
                    println!(
                        "  {}. ID: {}, 类型: {}, 范围: {}",
                        i + 1,
                        item.cf_id,
                        item.condition_format.condition_type,
                        item.condition_format.range
                    );
                    println!(
                        "     背景色: {:?}, 文字色: {:?}",
                        item.condition_format.format.background_color,
                        item.condition_format.format.text_color
                    );
                    created_cf_ids.push(item.cf_id.clone());
                }
            } else {
                eprintln!("❌ 响应数据为空");
                return Ok(());
            }
        }
        Err(e) => {
            eprintln!("❌ 创建条件格式失败: {:?}", e);
            return Ok(());
        }
    };

    // 获取所有条件格式
    println!("\n--- 2. 获取所有条件格式 ---");
    let get_req =
        open_lark::service::sheets::v3::condition_format::GetConditionFormatsRequest::builder()
            .spreadsheet_token("shtcnmBA*****yGehy8")
            .sheet_id("Sheet1")
            .build();

    match client
        .sheets
        .v3
        .spreadsheet_sheet
        .get_condition_formats(get_req, None)
        .await
    {
        Ok(resp) => {
            if let Some(data) = resp.data {
                println!("✅ 获取条件格式成功!");
                println!("📊 共找到 {} 个条件格式:", data.items.len());
                for (i, item) in data.items.iter().enumerate() {
                    println!(
                        "  {}. ID: {}, 类型: {}, 范围: {}",
                        i + 1,
                        item.cf_id,
                        item.condition_format.condition_type,
                        item.condition_format.range
                    );
                    if let Some(values) = &item.condition_format.condition_values {
                        println!("     条件值: {:?}", values);
                    }
                    println!(
                        "     格式: 背景色={:?}, 文字色={:?}, 加粗={:?}",
                        item.condition_format.format.background_color,
                        item.condition_format.format.text_color,
                        item.condition_format.format.bold
                    );
                }
            } else {
                println!("❌ 响应数据为空");
            }
        }
        Err(e) => {
            eprintln!("❌ 获取条件格式失败: {:?}", e);
        }
    }

    // 更新条件格式
    if !created_cf_ids.is_empty() {
        println!("\n--- 3. 更新条件格式 ---");

        let updated_format = FormatStyle::background_color("#0000FF")
            .with_text_color("#FFFFFF")
            .with_bold(false)
            .with_italic(false);

        let updated_rule = ConditionFormatRule::greater_than("A1:A15", 200.0, updated_format);

        let update_rules = vec![
            open_lark::service::sheets::v3::condition_format::UpdateConditionFormatRule::new(
                &created_cf_ids[0],
                updated_rule,
            ),
        ];

        let update_req = open_lark::service::sheets::v3::condition_format::UpdateConditionFormatsRequest::builder()
            .spreadsheet_token("shtcnmBA*****yGehy8")
            .sheet_id("Sheet1")
            .condition_formats(update_rules)
            .build();

        match client
            .sheets
            .v3
            .spreadsheet_sheet
            .update_condition_formats(update_req, None)
            .await
        {
            Ok(resp) => {
                if let Some(data) = resp.data {
                    println!("✅ 更新条件格式成功!");
                    println!("📊 更新成功数量: {}", data.updated_count);
                    for item in &data.items {
                        println!(
                            "  ID: {}, 新范围: {}, 新条件值: {:?}",
                            item.cf_id,
                            item.condition_format.range,
                            item.condition_format.condition_values
                        );
                        println!(
                            "  新格式: 背景色={:?}, 文字色={:?}",
                            item.condition_format.format.background_color,
                            item.condition_format.format.text_color
                        );
                    }
                } else {
                    println!("❌ 响应数据为空");
                }
            }
            Err(e) => {
                eprintln!("❌ 更新条件格式失败: {:?}", e);
            }
        }
    }

    // 删除条件格式 (演示用)
    println!("\n--- 4. 删除条件格式 (演示用) ---");
    println!("⚠️  注意：这将永久删除条件格式!");

    // 取消注释以下代码来执行删除操作
    // if created_cf_ids.len() > 1 {
    // let delete_ids = vec![created_cf_ids[1].clone()]; // 删除第二个条件格式
    //
    // let delete_req = open_lark::service::sheets::v3::condition_format::DeleteConditionFormatsRequest::builder()
    // .spreadsheet_token("shtcnmBA*****yGehy8")
    // .sheet_id("Sheet1")
    // .cf_ids(delete_ids)
    // .build();
    //
    // match client.sheets.v3.spreadsheet_sheet.delete_condition_formats(delete_req, None).await {
    // Ok(resp) => {
    // if let Some(data) = resp.data {
    // println!("✅ 删除条件格式结果:");
    // println!("📊 删除成功数量: {}, 失败数量: {}", data.deleted_count, data.failed_count);
    // for item in &data.items {
    // if item.success {
    // println!("  ✅ ID: {} 删除成功", item.cf_id);
    // } else {
    // println!("  ❌ ID: {} 删除失败: {:?}", item.cf_id, item.error_message);
    // }
    // }
    // } else {
    // println!("❌ 响应数据为空");
    // }
    // }
    // Err(e) => {
    // eprintln!("❌ 删除条件格式失败: {:?}", e);
    // }
    // }
    // }

    println!("\n💡 条件格式功能说明:");
    println!("- 条件格式用于根据单元格内容自动设置格式样式");
    println!("- 支持多种条件类型：数值比较、文本匹配、重复值检测等");
    println!("- 可以设置背景色、文字色、字体样式等多种格式");
    println!("- 支持批量操作，一次可以创建、更新、删除多个条件格式");

    println!("\n📋 支持的条件类型:");
    println!("- NUMBER_GREATER: 数值大于指定值");
    println!("- NUMBER_LESS: 数值小于指定值");
    println!("- NUMBER_EQ: 数值等于指定值");
    println!("- TEXT_CONTAINS: 文本包含指定内容");
    println!("- DUPLICATE: 重复值");
    println!("- BLANK: 空值");

    println!("\n🎨 支持的格式样式:");
    println!("- background_color: 背景颜色 (如 #FF0000)");
    println!("- text_color: 文字颜色 (如 #FFFFFF)");
    println!("- bold: 加粗 (true/false)");
    println!("- italic: 斜体 (true/false)");
    println!("- underline: 下划线 (true/false)");
    println!("- strikethrough: 删除线 (true/false)");

    Ok(())
}
