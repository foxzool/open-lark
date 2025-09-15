use open_lark::core::trait_system::ExecutableBuilder;
/// 电子表格读取和写入示例
///
/// 这个示例演示如何使用飞书SDK读取和写入电子表格数据。
///
/// 使用方法：
/// cargo run --example core_read_write_sheet
///
/// 环境变量：
/// APP_ID=your_app_id
/// APP_SECRET=your_app_secret
/// USER_ACCESS_TOKEN=your_user_access_token
/// SPREADSHEET_TOKEN=your_spreadsheet_token (可选，如果不提供会创建新表格)
use open_lark::prelude::*;
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");
    let _user_access_token = std::env::var("USER_ACCESS_TOKEN")
        .expect("USER_ACCESS_TOKEN environment variable not set (required for sheet operations)");

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    println!("📊 飞书电子表格读写示例");
    println!("{}", "=".repeat(50));

    // 获取表格token
    let spreadsheet_token = get_spreadsheet_token(&client).await?;

    // 读取表格数据
    read_sheet_data(&client, &spreadsheet_token).await?;

    // 写入表格数据
    write_sheet_data(&client, &spreadsheet_token).await?;

    // 再次读取验证数据更新
    verify_data_update(&client, &spreadsheet_token).await?;

    Ok(())
}

/// 获取表格token
async fn get_spreadsheet_token(client: &LarkClient) -> Result<String, Box<dyn std::error::Error>> {
    // 优先使用环境变量指定的表格
    if let Ok(spreadsheet_token) = std::env::var("SPREADSHEET_TOKEN") {
        println!("📋 使用指定表格: {spreadsheet_token}");
        return Ok(spreadsheet_token);
    }

    println!("📋 未指定表格token，创建新表格进行演示...");

    // 创建新表格
    match open_lark::service::sheets::v3::spreadsheet::CreateSpreedSheetRequest::builder()
        .title("SDK读写测试表格")
        .execute(&client.sheets.v3.spreadsheet)
        .await
    {
        Ok(response) => {
            if let Some(data) = &response.data {
                println!("✅ 新表格创建成功!");
                println!("   表格Token: {}", data.spreadsheet.spreadsheet_token);
                println!("   表格标题: {}", data.spreadsheet.title);
                println!("   访问链接: {}", data.spreadsheet.url);
                Ok(data.spreadsheet.spreadsheet_token.clone())
            } else {
                Err("创建表格响应数据为空".into())
            }
        }
        Err(e) => {
            println!("❌ 创建表格失败: {e:?}");
            println!("\n💡 请通过 SPREADSHEET_TOKEN 环境变量指定一个现有表格");
            Err(e.into())
        }
    }
}

/// 读取表格数据
async fn read_sheet_data(
    client: &LarkClient,
    spreadsheet_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📖 读取表格数据...");
    println!("   表格Token: {spreadsheet_token}");

    // 读取A1:D10范围的数据
    let range = "Sheet1!A1:D10";
    println!("   读取范围: {range}");

    match open_lark::service::sheets::v3::data_operation::ReadingSingleRangeRequest::builder()
        .spreadsheet_token(spreadsheet_token)
        .range(range)
        .value_render_option("UnformattedValue") // 获取原始值
        .execute(&client.sheets.v3.data_operation)
        .await
    {
        Ok(response) => {
            let value_range = &response.value_range;
            println!("✅ 数据读取成功!");
            println!("   实际范围: {}", value_range.range);
            println!("   版本号: {}", value_range.revision);
            println!("   数据行数: {}", value_range.values.len());

            if !value_range.values.is_empty() {
                println!("\n📄 表格内容:");
                for (row_index, row) in value_range.values.iter().enumerate() {
                    print!("   行{}: ", row_index + 1);
                    for (col_index, cell) in row.iter().enumerate() {
                        if col_index > 0 {
                            print!(" | ");
                        }
                        print!("{}", format_cell_value(cell));
                    }
                    println!();
                }
            } else {
                println!("📭 表格为空，没有数据");
            }
        }
        Err(e) => {
            println!("❌ 读取表格数据失败: {e:?}");
            println!("\n💡 常见错误解决方案:");
            println!("   1. 检查用户访问令牌权限");
            println!("   2. 确认表格Token是否正确");
            println!("   3. 验证是否有表格读取权限");
            println!("   4. 检查表格范围格式是否正确");
            return Err(e.into());
        }
    }

    Ok(())
}

/// 写入表格数据
async fn write_sheet_data(
    client: &LarkClient,
    spreadsheet_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n✏️ 写入表格数据...");

    // 准备要写入的数据
    let current_time = chrono::Utc::now()
        .format("%Y-%m-%d %H:%M:%S UTC")
        .to_string();
    let data_to_append = vec![
        vec![
            json!("产品名称"),
            json!("价格"),
            json!("库存"),
            json!("更新时间"),
        ],
        vec![
            json!("iPhone 15"),
            json!(8999.0),
            json!(50),
            json!(current_time.clone()),
        ],
        vec![
            json!("MacBook Pro"),
            json!(14999.0),
            json!(25),
            json!(current_time.clone()),
        ],
        vec![
            json!("AirPods Pro"),
            json!(1899.0),
            json!(100),
            json!(current_time.clone()),
        ],
    ];

    let range = "Sheet1!A1:D4";
    println!("   写入范围: {range}");
    println!("   写入行数: {}", data_to_append.len());

    match open_lark::service::sheets::v3::data_operation::AppendDataRequest::builder()
        .spreadsheet_token(spreadsheet_token)
        .range(range)
        .insert_data_option("OVERWRITE") // 覆盖现有数据
        .values(data_to_append)
        .execute(&client.sheets.v3.data_operation)
        .await
    {
        Ok(response) => {
            let data = &response;
            println!("✅ 数据写入成功!");
            println!("   表格Token: {}", data.spreadsheet_token);
            println!("   更新范围: {}", data.table_range);
            println!("   版本号: {}", data.revision);

            let updates = &data.updates;
            println!("   更新统计:");
            println!("     更新范围: {}", updates.updated_range);
            println!("     更新行数: {}", updates.updated_rows);
            println!("     更新列数: {}", updates.updated_columns);
            println!("     更新单元格数: {}", updates.updated_cells);
        }
        Err(e) => {
            println!("❌ 写入表格数据失败: {e:?}");
            println!("\n💡 常见错误解决方案:");
            println!("   1. 检查用户访问令牌权限");
            println!("   2. 确认是否有表格编辑权限");
            println!("   3. 验证数据格式是否正确");
            println!("   4. 检查表格范围是否有效");
            return Err(e.into());
        }
    }

    Ok(())
}

/// 验证数据更新
async fn verify_data_update(
    client: &LarkClient,
    spreadsheet_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 验证数据更新...");

    // 稍等片刻确保数据已更新
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    let range = "Sheet1!A1:D4";

    match open_lark::service::sheets::v3::data_operation::ReadingSingleRangeRequest::builder()
        .spreadsheet_token(spreadsheet_token)
        .range(range)
        .value_render_option("FormattedValue") // 获取格式化值
        .execute(&client.sheets.v3.data_operation)
        .await
    {
        Ok(response) => {
            let value_range = &response.value_range;
            println!("✅ 数据验证成功!");
            println!("   当前版本: {}", value_range.revision);

            if !value_range.values.is_empty() {
                println!("\n📊 更新后的表格内容:");

                // 显示表头
                if let Some(header_row) = value_range.values.first() {
                    print!("   ");
                    for (i, cell) in header_row.iter().enumerate() {
                        if i > 0 {
                            print!(" | ");
                        }
                        print!("{:12}", format_cell_value(cell));
                    }
                    println!();
                    println!("   {}", "-".repeat(60));
                }

                // 显示数据行
                for row in value_range.values.iter().skip(1) {
                    print!("   ");
                    for (col_index, cell) in row.iter().enumerate() {
                        if col_index > 0 {
                            print!(" | ");
                        }
                        print!("{:12}", format_cell_value(cell));
                    }
                    println!();
                }

                println!("\n💡 提示: 数据已成功写入并验证完成");
            }
        }
        Err(e) => {
            println!("❌ 验证数据更新失败: {e:?}");
        }
    }

    Ok(())
}

/// 格式化单元格值显示
fn format_cell_value(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Array(_) => "[数组]".to_string(),
        Value::Object(_) => "[对象]".to_string(),
        Value::Null => "".to_string(),
    }
}

/// 演示高级表格操作（供参考）
#[allow(dead_code)]
async fn advanced_sheet_operations(
    _client: &LarkClient,
    _spreadsheet_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔧 高级表格操作演示...");

    // 读取多个范围
    let ranges = vec!["Sheet1!A1:B2", "Sheet1!D1:D5"];
    println!("   演示读取多个范围: {ranges:?}");

    // 这里可以添加批量读取、条件格式设置等高级操作
    // 具体实现取决于API的可用性

    println!("   💡 高级操作包括:");
    println!("     - 批量读取多个范围");
    println!("     - 设置单元格样式");
    println!("     - 合并单元格");
    println!("     - 插入行列");
    println!("     - 数据验证规则");

    Ok(())
}
