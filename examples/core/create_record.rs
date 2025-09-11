/// 多维表格创建记录示例
///
/// 这个示例演示如何使用飞书SDK在多维表格中创建新记录。
///
/// 使用方法：
/// cargo run --example create_record
///
/// 环境变量：
/// APP_ID=your_app_id
/// APP_SECRET=your_app_secret
/// APP_TOKEN=your_bitable_app_token
/// TABLE_ID=your_table_id
use open_lark::prelude::*;
use open_lark::{
    core::trait_system::ExecutableBuilder,
    service::cloud_docs::bitable::v1::app_table_record::create::*,
};
use serde_json::{json, Value};
use std::collections::HashMap;

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

    println!("📝 飞书多维表格创建记录示例");
    println!("应用Token: {app_token}");
    println!("表格ID: {table_id}");
    println!("{}", "=".repeat(50));

    // 创建简单记录
    create_simple_record(&client, &app_token, &table_id).await?;

    // 创建复杂记录
    create_complex_record(&client, &app_token, &table_id).await?;

    // 批量创建记录
    create_multiple_records(&client, &app_token, &table_id).await?;

    Ok(())
}

/// 创建简单记录
async fn create_simple_record(
    client: &LarkClient,
    app_token: &str,
    table_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n➕ 创建简单记录...");

    // 生成唯一标识符用于幂等操作
    let client_token = uuid::Uuid::new_v4().to_string();

    // 使用增强Builder模式创建记录
    match CreateRecordRequest::builder()
        .app_token(app_token)
        .table_id(table_id)
        .client_token(&client_token) // 幂等标识
        .add_field("名称", json!("SDK测试记录"))
        .add_field("描述", json!("这是通过飞书Rust SDK创建的测试记录"))
        .add_field("状态", json!("待处理"))
        .add_field("创建时间", json!(chrono::Utc::now().timestamp_millis()))
        .execute(&client.bitable.v1.app_table_record)
        .await
    {
        Ok(response) => {
            println!("✅ 简单记录创建成功!");
            if let Some(record_id) = &response.record.record_id {
                println!("   记录ID: {record_id}");
            }
            if let Some(created_time) = &response.record.created_time {
                println!("   创建时间: {created_time}");
            }
            println!("   客户端Token: {client_token}");

            // 显示创建的字段
            println!("   创建的字段:");
            for (field_name, value) in &response.record.fields {
                println!("     {}: {}", field_name, format_field_value(value));
            }
        }
        Err(e) => {
            println!("❌ 创建记录失败: {e:?}");
            println!("\n💡 常见错误解决方案:");
            println!("   1. 检查APP_ID和APP_SECRET是否正确");
            println!("   2. 确认APP_TOKEN是否为有效的多维表格应用token");
            println!("   3. 验证TABLE_ID是否正确");
            println!("   4. 确保应用有多维表格的写入权限");
            println!("   5. 检查字段名是否与表格中的字段匹配");
            return Err(e.into());
        }
    }

    Ok(())
}

/// 创建复杂记录（包含多种字段类型）
async fn create_complex_record(
    client: &LarkClient,
    app_token: &str,
    table_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔧 创建复杂记录（多种字段类型）...");

    let client_token = uuid::Uuid::new_v4().to_string();

    // 构建复杂的字段数据
    let mut fields = HashMap::new();

    // 文本字段
    fields.insert("标题".to_string(), json!("复杂记录示例"));

    // 数字字段
    fields.insert("数量".to_string(), json!(100));
    fields.insert("价格".to_string(), json!(99.99));

    // 布尔字段
    fields.insert("是否完成".to_string(), json!(false));

    // 日期字段（毫秒时间戳）
    fields.insert(
        "截止日期".to_string(),
        json!(chrono::Utc::now().timestamp_millis()),
    );

    // 单选字段
    fields.insert("优先级".to_string(), json!("高"));

    // 多选字段
    fields.insert("标签".to_string(), json!(["SDK", "测试", "自动化"]));

    // 链接字段
    fields.insert(
        "相关链接".to_string(),
        json!({
            "text": "飞书开放平台",
            "link": "https://open.feishu.cn"
        }),
    );

    // JSON对象字段
    fields.insert(
        "元数据".to_string(),
        json!({
            "source": "rust_sdk",
            "version": "1.0.0",
            "timestamp": chrono::Utc::now().to_rfc3339()
        }),
    );

    // 创建Record结构
    let record = open_lark::service::bitable::v1::Record {
        record_id: None,
        fields,
        created_by: None,
        created_time: None,
        last_modified_by: None,
        last_modified_time: None,
    };

    match CreateRecordRequest::builder()
        .app_token(app_token)
        .table_id(table_id)
        .client_token(&client_token)
        .fields(record)
        .execute(&client.bitable.v1.app_table_record)
        .await
    {
        Ok(response) => {
            println!("✅ 复杂记录创建成功!");
            if let Some(record_id) = &response.record.record_id {
                println!("   记录ID: {record_id}");
            }
            println!("   字段数量: {}", response.record.fields.len());

            // 分类显示字段
            println!("   创建的字段详情:");
            for (field_name, value) in &response.record.fields {
                let field_type = identify_field_type(value);
                println!(
                    "     {} ({}): {}",
                    field_name,
                    field_type,
                    format_field_value(value)
                );
            }
        }
        Err(e) => {
            println!("❌ 创建复杂记录失败: {e:?}");
            return Err(e.into());
        }
    }

    Ok(())
}

/// 演示批量创建记录的概念（实际使用batch_create API）
async fn create_multiple_records(
    client: &LarkClient,
    app_token: &str,
    table_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📦 演示创建多个记录...");

    let records_data = [
        ("任务1", "完成功能开发", "进行中"),
        ("任务2", "编写测试用例", "待开始"),
        ("任务3", "代码审查", "已完成"),
    ];

    for (index, (title, description, status)) in records_data.iter().enumerate() {
        println!("   创建记录 {}/{}...", index + 1, records_data.len());

        let client_token = uuid::Uuid::new_v4().to_string();

        match CreateRecordRequest::builder()
            .app_token(app_token)
            .table_id(table_id)
            .client_token(&client_token)
            .add_field("任务名称", json!(title))
            .add_field("任务描述", json!(description))
            .add_field("状态", json!(status))
            .add_field("序号", json!(index + 1))
            .execute(&client.bitable.v1.app_table_record)
            .await
        {
            Ok(response) => {
                println!(
                    "   ✅ 记录 {} 创建成功 (ID: {})",
                    title,
                    response
                        .record
                        .record_id
                        .as_ref()
                        .unwrap_or(&"N/A".to_string())
                );
            }
            Err(e) => {
                println!("   ❌ 记录 {title} 创建失败: {e:?}");
            }
        }

        // 添加小延迟避免频率限制
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    println!("\n💡 提示: 实际生产环境建议使用 batch_create API 进行批量操作，效率更高");

    Ok(())
}

/// 格式化字段值显示
fn format_field_value(value: &Value) -> String {
    match value {
        Value::String(s) => {
            if s.len() > 50 {
                format!("{}...", &s[..50])
            } else {
                s.clone()
            }
        }
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Array(arr) => {
            format!("[{}个元素]", arr.len())
        }
        Value::Object(_) => "[对象]".to_string(),
        Value::Null => "[空]".to_string(),
    }
}

/// 识别字段类型
fn identify_field_type(value: &Value) -> &'static str {
    match value {
        Value::String(_) => "文本",
        Value::Number(n) => {
            if n.is_i64() {
                "整数"
            } else {
                "小数"
            }
        }
        Value::Bool(_) => "布尔",
        Value::Array(_) => "数组",
        Value::Object(_) => "对象",
        Value::Null => "空值",
    }
}
