/// 多维表格记录查询示例
///
/// 这个示例演示如何使用飞书SDK查询多维表格中的记录。
///
/// 使用方法：
/// cargo run --example query_records
///
/// 环境变量：
/// APP_ID=your_app_id
/// APP_SECRET=your_app_secret
/// APP_TOKEN=your_bitable_app_token
/// TABLE_ID=your_table_id
use open_lark::prelude::*;
use open_lark::{
    core::trait_system::ExecutableBuilder,
    service::cloud_docs::bitable::v1::app_table_record::search::*,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");
    let app_token =
        std::env::var("APP_TOKEN").unwrap_or_else(|_| "bascnCMII2ORuEjIDXvVecCKNEc".to_string()); // 示例token
    let table_id = std::env::var("TABLE_ID").unwrap_or_else(|_| "tblsRc9GRRXKqhvW".to_string()); // 示例table_id

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    println!("🗃️ 飞书多维表格记录查询示例");
    println!("应用Token: {app_token}");
    println!("表格ID: {table_id}");
    println!("{}", "=".repeat(50));

    // 基础查询
    query_all_records(&client, &app_token, &table_id).await?;

    // 带条件查询
    query_with_filter(&client, &app_token, &table_id).await?;

    // 带排序和分页查询
    query_with_sort_and_pagination(&client, &app_token, &table_id).await?;

    Ok(())
}

/// 查询所有记录
async fn query_all_records(
    client: &LarkClient,
    app_token: &str,
    table_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📋 查询所有记录...");

    // 使用增强Builder模式查询记录
    match SearchRecordRequest::builder()
        .app_token(app_token)
        .table_id(table_id)
        .page_size(20)
        .automatic(true) // 返回自动计算字段
        .execute(&client.bitable.v1.app_table_record)
        .await
    {
        Ok(response) => {
            if let Some(data) = &response.data {
                println!("✅ 记录查询成功!");
                println!("   总记录数: {}", data.items.len());
                println!("   是否有更多: {}", data.has_more);

                if !data.items.is_empty() {
                    println!("\n📄 记录列表:");
                    for (index, record) in data.items.iter().enumerate() {
                        println!(
                            "   {}. 记录ID: {}",
                            index + 1,
                            record.record_id.as_ref().unwrap_or(&"N/A".to_string())
                        );
                        if let Some(created_time) = &record.created_time {
                            println!("      创建时间: {created_time}");
                        }
                        if let Some(modified_time) = &record.last_modified_time {
                            println!("      修改时间: {modified_time}");
                        }

                        // 显示字段数据
                        if !record.fields.is_empty() {
                            println!("      字段数据:");
                            for (field_name, value) in &record.fields {
                                // 简化值显示（实际使用中可以根据字段类型格式化）
                                let display_value = format_field_value(value);
                                println!("        {field_name}: {display_value}");
                            }
                        }
                        println!(); // 空行分隔
                    }
                } else {
                    println!("📭 表格为空，没有记录");
                }

                if data.has_more {
                    println!("💡 提示: 还有更多记录可以通过分页获取");
                    if let Some(page_token) = &data.page_token {
                        println!("   下一页Token: {page_token}");
                    }
                }
            } else {
                println!("⚠️ 请求成功，但未返回数据");
            }
        }
        Err(e) => {
            println!("❌ 查询记录失败: {e:?}");
            println!("\n💡 常见错误解决方案:");
            println!("   1. 检查APP_ID和APP_SECRET是否正确");
            println!("   2. 确认APP_TOKEN是否为有效的多维表格应用token");
            println!("   3. 验证TABLE_ID是否正确");
            println!("   4. 确保应用有多维表格的读取权限");
            return Err(e.into());
        }
    }

    Ok(())
}

/// 带筛选条件查询
async fn query_with_filter(
    client: &LarkClient,
    app_token: &str,
    table_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 带筛选条件查询...");

    // 构建筛选条件：查找特定字段值
    let filter = FilterInfo {
        conjunction: "and".to_string(),
        conditions: vec![FilterCondition {
            field_name: "名称".to_string(), // 假设有"名称"字段
            operator: "contains".to_string(),
            value: Some(vec!["测试".to_string()]),
        }],
    };

    match SearchRecordRequest::builder()
        .app_token(app_token)
        .table_id(table_id)
        .filter(filter)
        .page_size(10)
        .execute(&client.bitable.v1.app_table_record)
        .await
    {
        Ok(response) => {
            if let Some(data) = &response.data {
                println!("✅ 筛选查询成功!");
                println!("   筛选后记录数: {}", data.items.len());

                if !data.items.is_empty() {
                    println!("\n📋 筛选结果:");
                    for (index, record) in data.items.iter().enumerate() {
                        println!(
                            "   {}. 记录ID: {}",
                            index + 1,
                            record.record_id.as_ref().unwrap_or(&"N/A".to_string())
                        );

                        // 显示"名称"字段（如果存在）
                        if let Some(name_value) = record.fields.get("名称") {
                            println!("      名称: {}", format_field_value(name_value));
                        }
                    }
                } else {
                    println!("📭 没有匹配筛选条件的记录");
                }
            }
        }
        Err(e) => {
            println!("❌ 筛选查询失败: {e:?}");
            println!("   注意: 筛选字段名需要与实际表格字段匹配");
        }
    }

    Ok(())
}

/// 带排序和分页查询
async fn query_with_sort_and_pagination(
    client: &LarkClient,
    app_token: &str,
    table_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📊 带排序的分页查询...");

    // 构建排序条件
    let sort_conditions = vec![SortCondition {
        field_name: "创建时间".to_string(), // 假设有"创建时间"字段
        desc: Some(true),                   // 降序排序
    }];

    let mut page_count = 0;
    let mut page_token: Option<String> = None;

    loop {
        page_count += 1;
        println!("\n📄 获取第 {page_count} 页...");

        let mut request_builder = SearchRecordRequest::builder()
            .app_token(app_token)
            .table_id(table_id)
            .sort(sort_conditions.clone())
            .page_size(5) // 小页面用于演示
            .automatic(false); // 不返回自动计算字段，提高性能

        // 添加分页token
        if let Some(token) = &page_token {
            request_builder = request_builder.page_token(token);
        }

        match request_builder
            .execute(&client.bitable.v1.app_table_record)
            .await
        {
            Ok(response) => {
                if let Some(data) = &response.data {
                    println!("   本页记录数: {}", data.items.len());

                    // 显示记录摘要
                    for record in &data.items {
                        println!(
                            "     - 记录ID: {}",
                            record.record_id.as_ref().unwrap_or(&"N/A".to_string())
                        );
                        if let Some(time_value) = record.fields.get("创建时间") {
                            println!("       创建时间: {}", format_field_value(time_value));
                        }
                    }

                    // 检查是否还有更多
                    if data.has_more {
                        page_token = data.page_token.clone();
                        println!("   → 还有更多页面...");

                        // 演示限制：最多3页
                        if page_count >= 3 {
                            println!("   ⏹️ 演示限制：最多显示3页");
                            break;
                        }
                    } else {
                        println!("   ✅ 已获取所有记录");
                        break;
                    }
                } else {
                    println!("   ⚠️ 本页无数据");
                    break;
                }
            }
            Err(e) => {
                println!("   ❌ 第{page_count}页获取失败: {e:?}");
                break;
            }
        }
    }

    println!("\n📈 分页查询总结:");
    println!("   总页数: {page_count}");

    Ok(())
}

/// 格式化字段值显示
fn format_field_value(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Array(arr) => {
            format!("[{}]", arr.len())
        }
        serde_json::Value::Object(_) => "[对象]".to_string(),
        serde_json::Value::Null => "[空]".to_string(),
    }
}

/// 构建复杂筛选条件示例（供参考）
#[allow(dead_code)]
fn build_complex_filter() -> FilterInfo {
    FilterInfo {
        conjunction: "and".to_string(),
        conditions: vec![
            FilterCondition {
                field_name: "状态".to_string(),
                operator: "is".to_string(),
                value: Some(vec!["进行中".to_string()]),
            },
            FilterCondition {
                field_name: "优先级".to_string(),
                operator: "isGreater".to_string(),
                value: Some(vec!["2".to_string()]),
            },
            FilterCondition {
                field_name: "创建时间".to_string(),
                operator: "isAfter".to_string(),
                value: Some(vec!["2024-01-01".to_string()]),
            },
        ],
    }
}
