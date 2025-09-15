use open_lark::core::trait_system::ExecutableBuilder;
/// 创建电子表格示例
///
/// 这个示例演示如何使用飞书SDK创建新的电子表格并进行基本数据操作。
///
/// 使用方法：
/// cargo run --example core_create_sheet
///
/// 环境变量：
/// APP_ID=your_app_id
/// APP_SECRET=your_app_secret
/// USER_ACCESS_TOKEN=your_user_access_token
/// FOLDER_TOKEN=target_folder_token (可选，默认创建在根目录)
use open_lark::prelude::*;
use serde_json::json;

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

    println!("📊 飞书电子表格创建示例");
    println!("{}", "=".repeat(50));

    // 创建新表格
    let spreadsheet_token = create_new_spreadsheet(&client).await?;

    // 获取表格信息
    get_spreadsheet_info(&client, &spreadsheet_token).await?;

    // 初始化表格数据
    initialize_spreadsheet_data(&client, &spreadsheet_token).await?;

    // 添加更多数据
    add_sample_data(&client, &spreadsheet_token).await?;

    // 展示最终结果
    display_final_result(&client, &spreadsheet_token).await?;

    Ok(())
}

/// 创建新的电子表格
async fn create_new_spreadsheet(client: &LarkClient) -> Result<String, Box<dyn std::error::Error>> {
    println!("\n📝 创建新电子表格...");

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    let title = format!("SDK创建的表格_{timestamp}");

    println!("   表格标题: {title}");

    // 获取文件夹token（如果指定）
    let folder_token = std::env::var("FOLDER_TOKEN").ok();
    if let Some(ref folder) = folder_token {
        println!("   目标文件夹: {folder}");
    } else {
        println!("   目标文件夹: 根目录");
    }

    let mut builder =
        open_lark::service::sheets::v3::spreadsheet::CreateSpreedSheetRequest::builder()
            .title(&title);

    if let Some(folder) = folder_token {
        builder = builder.folder_token(folder);
    }

    match builder.execute(&client.sheets.v3.spreadsheet).await {
        Ok(response) => {
            if let Some(data) = &response.data {
                let spreadsheet = &data.spreadsheet;
                println!("✅ 电子表格创建成功!");
                println!("   表格Token: {}", spreadsheet.spreadsheet_token);
                println!("   表格标题: {}", spreadsheet.title);
                println!("   文件夹Token: {}", spreadsheet.folder_token);
                println!("   访问链接: {}", spreadsheet.url);

                Ok(spreadsheet.spreadsheet_token.clone())
            } else {
                Err("创建表格响应数据为空".into())
            }
        }
        Err(e) => {
            println!("❌ 创建电子表格失败: {e:?}");
            println!("\n💡 常见错误解决方案:");
            println!("   1. 检查用户访问令牌权限");
            println!("   2. 确认是否有创建文档的权限");
            println!("   3. 验证文件夹Token是否正确（如果指定）");
            println!("   4. 检查表格标题是否符合要求");
            Err(e.into())
        }
    }
}

/// 获取表格详细信息
async fn get_spreadsheet_info(
    _client: &LarkClient,
    _spreadsheet_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📋 获取表格详细信息...");

    match open_lark::service::sheets::v3::spreadsheet::GetSpreadsheetRequest::builder()
        .spreadsheet_token(_spreadsheet_token)
        .execute(&_client.sheets.v3.spreadsheet)
        .await
    {
        Ok(response) => {
            if let Some(data) = &response.data {
                let spreadsheet = &data.spreadsheet;
                println!("✅ 表格信息获取成功!");
                println!("   表格标题: {}", spreadsheet.title);
                println!("   所有者ID: {}", spreadsheet.owner_id);
                println!("   表格Token: {}", spreadsheet.token);
                println!("   表格URL: {}", spreadsheet.url);

                // 注意：基础的获取表格信息API不返回工作表列表
                // 如需获取工作表信息，需要使用其他专门的API
            } else {
                println!("⚠️ 获取表格信息成功，但未返回数据");
            }
        }
        Err(e) => {
            println!("❌ 获取表格信息失败: {e:?}");
            return Err(e.into());
        }
    }

    Ok(())
}

/// 初始化表格数据
async fn initialize_spreadsheet_data(
    client: &LarkClient,
    spreadsheet_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📊 初始化表格数据...");

    // 创建表头
    let header_data = vec![vec![
        json!("员工ID"),
        json!("姓名"),
        json!("部门"),
        json!("职位"),
        json!("入职日期"),
        json!("薪资"),
    ]];

    let range = "Sheet1!A1:F1";
    println!("   写入表头范围: {range}");

    match open_lark::service::sheets::v3::data_operation::AppendDataRequest::builder()
        .spreadsheet_token(spreadsheet_token)
        .range(range)
        .insert_data_option("OVERWRITE")
        .values(header_data)
        .execute(&client.sheets.v3.data_operation)
        .await
    {
        Ok(response) => {
            println!("✅ 表头创建成功!");
            println!("   更新范围: {}", response.updates.updated_range);
            println!("   更新单元格数: {}", response.updates.updated_cells);
        }
        Err(e) => {
            println!("❌ 初始化表格数据失败: {e:?}");
            return Err(e.into());
        }
    }

    Ok(())
}

/// 添加示例数据
async fn add_sample_data(
    client: &LarkClient,
    spreadsheet_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n➕ 添加员工数据...");

    // 准备员工数据
    let employee_data = vec![
        vec![
            json!("EMP001"),
            json!("张三"),
            json!("技术部"),
            json!("高级工程师"),
            json!("2022-01-15"),
            json!(15000),
        ],
        vec![
            json!("EMP002"),
            json!("李四"),
            json!("产品部"),
            json!("产品经理"),
            json!("2021-08-20"),
            json!(18000),
        ],
        vec![
            json!("EMP003"),
            json!("王五"),
            json!("设计部"),
            json!("UI设计师"),
            json!("2023-03-10"),
            json!(12000),
        ],
        vec![
            json!("EMP004"),
            json!("赵六"),
            json!("市场部"),
            json!("市场专员"),
            json!("2022-11-05"),
            json!(10000),
        ],
        vec![
            json!("EMP005"),
            json!("钱七"),
            json!("技术部"),
            json!("前端工程师"),
            json!("2023-06-01"),
            json!(13000),
        ],
    ];

    let range = "Sheet1!A2:F6";
    println!("   写入数据范围: {range}");
    println!("   员工记录数: {}", employee_data.len());

    match open_lark::service::sheets::v3::data_operation::AppendDataRequest::builder()
        .spreadsheet_token(spreadsheet_token)
        .range(range)
        .insert_data_option("OVERWRITE")
        .values(employee_data)
        .execute(&client.sheets.v3.data_operation)
        .await
    {
        Ok(response) => {
            println!("✅ 员工数据添加成功!");
            println!("   更新行数: {}", response.updates.updated_rows);
            println!("   更新列数: {}", response.updates.updated_columns);
            println!("   总更新单元格数: {}", response.updates.updated_cells);

            // 添加统计信息
            add_statistics_data(client, spreadsheet_token).await?;
        }
        Err(e) => {
            println!("❌ 添加员工数据失败: {e:?}");
            return Err(e.into());
        }
    }

    Ok(())
}

/// 添加统计信息
async fn add_statistics_data(
    client: &LarkClient,
    spreadsheet_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📈 添加统计信息...");

    // 创建统计数据
    let stats_data = vec![
        vec![
            json!(""),
            json!(""),
            json!(""),
            json!(""),
            json!(""),
            json!(""),
        ], // 空行分隔
        vec![
            json!("统计信息"),
            json!(""),
            json!(""),
            json!(""),
            json!(""),
            json!(""),
        ],
        vec![
            json!("总员工数"),
            json!("5"),
            json!(""),
            json!(""),
            json!(""),
            json!(""),
        ],
        vec![
            json!("平均薪资"),
            json!("13600"),
            json!(""),
            json!(""),
            json!(""),
            json!(""),
        ],
        vec![
            json!("部门分布"),
            json!("技术部: 2人"),
            json!("其他: 3人"),
            json!(""),
            json!(""),
            json!(""),
        ],
    ];

    let range = "Sheet1!A8:F12";

    match open_lark::service::sheets::v3::data_operation::AppendDataRequest::builder()
        .spreadsheet_token(spreadsheet_token)
        .range(range)
        .insert_data_option("OVERWRITE")
        .values(stats_data)
        .execute(&client.sheets.v3.data_operation)
        .await
    {
        Ok(_) => {
            println!("✅ 统计信息添加成功!");
        }
        Err(e) => {
            println!("⚠️ 添加统计信息失败: {e:?}");
        }
    }

    Ok(())
}

/// 展示最终结果
async fn display_final_result(
    client: &LarkClient,
    spreadsheet_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎯 展示最终表格内容...");

    let range = "Sheet1!A1:F12";

    match open_lark::service::sheets::v3::data_operation::ReadingSingleRangeRequest::builder()
        .spreadsheet_token(spreadsheet_token)
        .range(range)
        .value_render_option("FormattedValue")
        .execute(&client.sheets.v3.data_operation)
        .await
    {
        Ok(response) => {
            let value_range = &response.value_range;
            println!("✅ 表格创建和数据填充完成!");
            println!("   表格版本: {}", value_range.revision);
            println!("   数据行数: {}", value_range.values.len());

            println!("\n📊 最终表格内容预览:");

            for (row_index, row) in value_range.values.iter().enumerate() {
                if row_index == 0 {
                    // 表头行
                    print!("   表头: ");
                } else if row_index <= 6 {
                    // 数据行
                    print!("   数据{row_index}: ");
                } else {
                    // 统计行
                    print!("   统计: ");
                }

                let row_text: Vec<String> = row.iter().map(format_cell_value).collect();
                println!("{}", row_text.join(" | "));
            }

            println!("\n🎉 表格创建演示完成!");
            println!("💡 您可以在飞书应用中查看和编辑这个表格");
        }
        Err(e) => {
            println!("❌ 读取最终结果失败: {e:?}");
        }
    }

    Ok(())
}

/// 格式化单元格值
fn format_cell_value(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Array(_) => "[数组]".to_string(),
        serde_json::Value::Object(_) => "[对象]".to_string(),
        serde_json::Value::Null => "".to_string(),
    }
}

/// 演示更复杂的表格操作（供参考）
#[allow(dead_code)]
async fn advanced_spreadsheet_operations(
    _client: &LarkClient,
    _spreadsheet_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔧 高级表格操作演示...");

    println!("   💡 可实现的高级功能:");
    println!("     - 创建多个工作表");
    println!("     - 设置单元格格式和样式");
    println!("     - 插入图表和图片");
    println!("     - 设置数据验证规则");
    println!("     - 创建筛选和排序");
    println!("     - 保护特定范围");
    println!("     - 合并和拆分单元格");

    // 这里可以根据需要添加具体的高级操作实现

    Ok(())
}
