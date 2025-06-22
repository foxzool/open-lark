use dotenvy::dotenv;
use open_lark::prelude::*;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let app_id = std::env::var("APP_ID").expect("APP_ID not found");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET not found");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    // 向多个范围写入数据示例
    
    // 准备第一个范围的数据 - 员工信息表头
    let header_data = vec![
        vec![json!("姓名"), json!("年龄"), json!("部门"), json!("职位")],
    ];

    // 准备第二个范围的数据 - 员工信息
    let employee_data = vec![
        vec![json!("张三"), json!(25), json!("研发部"), json!("工程师")],
        vec![json!("李四"), json!(28), json!("产品部"), json!("产品经理")],
        vec![json!("王五"), json!(30), json!("设计部"), json!("设计师")],
        vec![json!("赵六"), json!(27), json!("测试部"), json!("测试工程师")],
    ];

    // 准备第三个范围的数据 - 统计信息
    let summary_data = vec![
        vec![json!("统计信息"), json!("")],
        vec![json!("总人数"), json!(4)],
        vec![json!("平均年龄"), json!(27.5)],
    ];

    let req = open_lark::service::sheets::v3::data_operation::WriteDataToMultipleRangesRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8") // 替换为实际的表格 token
        .add_range_data("Sheet1!A1:D1", header_data) // 写入表头
        .add_range_data("Sheet1!A2:D5", employee_data) // 写入员工数据
        .add_range_data("Sheet1!F1:G3", summary_data) // 写入统计信息
        .build();

    match client.sheets.v3.data_operation.write_data_to_multiple_ranges(req, None).await {
        Ok(resp) => {
            println!("✅ 向多个范围写入数据成功!");
            println!("📊 表格 Token: {}", resp.data.spreadsheet_token);
            println!("🔄 版本号: {}", resp.data.revision);
            println!("📊 总计更新:");
            println!("  - 总行数: {}", resp.data.total_updated_rows);
            println!("  - 总列数: {}", resp.data.total_updated_columns);
            println!("  - 总单元格数: {}", resp.data.total_updated_cells);
            
            println!("\n📝 各范围详情:");
            for (i, response) in resp.data.responses.iter().enumerate() {
                println!("  范围 {}: {}", i + 1, response.updated_range);
                println!("    - 行数: {}, 列数: {}, 单元格数: {}", 
                    response.updated_rows, 
                    response.updated_columns, 
                    response.updated_cells
                );
            }
        }
        Err(e) => {
            eprintln!("❌ 向多个范围写入数据失败: {:?}", e);
        }
    }

    // 使用另一种方式构建请求 - 先准备所有范围数据
    println!("\n--- 使用预构建范围数据的方式 ---");
    
    use open_lark::service::sheets::v3::data_operation::MultiRangeValueData;

    let range_data_list = vec![
        MultiRangeValueData::new(
            "Sheet2!A1:C1",
            vec![vec![json!("项目"), json!("进度"), json!("负责人")]],
        ),
        MultiRangeValueData::new(
            "Sheet2!A2:C4",
            vec![
                vec![json!("项目A"), json!("80%"), json!("张三")],
                vec![json!("项目B"), json!("60%"), json!("李四")],
                vec![json!("项目C"), json!("90%"), json!("王五")],
            ],
        ),
        MultiRangeValueData::new(
            "Sheet2!E1:F2",
            vec![
                vec![json!("完成状态"), json!("数量")],
                vec![json!("进行中"), json!(3)],
            ],
        ),
    ];

    let req2 = open_lark::service::sheets::v3::data_operation::WriteDataToMultipleRangesRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8") // 替换为实际的表格 token
        .value_ranges(range_data_list)
        .build();

    match client.sheets.v3.data_operation.write_data_to_multiple_ranges(req2, None).await {
        Ok(resp) => {
            println!("✅ 第二次批量写入成功!");
            println!("📊 共更新了 {} 个单元格", resp.data.total_updated_cells);
            println!("📍 涉及 {} 个范围", resp.data.responses.len());
        }
        Err(e) => {
            eprintln!("❌ 第二次批量写入失败: {:?}", e);
        }
    }

    println!("\n💡 使用建议:");
    println!("- 适合一次性更新表格中多个不连续的区域");
    println!("- 比多次单独调用API更高效");
    println!("- 支持不同范围使用不同的数据结构");
    println!("- 返回每个范围的详细更新信息");

    Ok(())
}