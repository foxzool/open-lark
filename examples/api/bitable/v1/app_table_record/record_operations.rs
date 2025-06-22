use dotenvy::dotenv;
use open_lark::{
    prelude::*,
    service::bitable::v1::app_table_record::{
        FilterCondition, FilterInfo, SortCondition, UpdateRecord,
    },
};
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID not found");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET not found");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    // 替换为实际的多维表格和数据表 token
    let app_token = "bascnmBA*****yGehy8"; // 请替换为实际的 app_token
    let table_id = "tblsRc9GRRXKqhvW"; // 请替换为实际的 table_id

    // 1. 新增单条记录
    println!("--- 1. 新增单条记录 ---");

    use open_lark::service::bitable::v1::Record;
    let new_record = Record {
        record_id: None,
        fields: HashMap::from([
            ("项目名称".to_string(), json!("开源项目管理")),
            ("负责人".to_string(), json!("张三")),
            ("状态".to_string(), json!("进行中")),
            ("进度".to_string(), json!(25)),
            ("开始日期".to_string(), json!(1704067200000i64)), // 2024-01-01
            ("是否重要".to_string(), json!(true)),
        ]),
        created_by: None,
        created_time: None,
        last_modified_by: None,
        last_modified_time: None,
    };

    let create_req =
        open_lark::service::bitable::v1::app_table_record::CreateRecordRequest::builder()
            .app_token(app_token)
            .table_id(table_id)
            .user_id_type("open_id")
            .fields(new_record)
            .build();

    let created_record_id = match client
        .bitable
        .v1
        .app_table_record
        .create(create_req, None)
        .await
    {
        Ok(resp) => {
            if let Some(data) = resp.data {
                println!("✅ 新增记录成功!");
                if let Some(record_id) = &data.record.record_id {
                    println!("🆔 Record ID: {}", record_id);
                    Some(record_id.clone())
                } else {
                    None
                }
            } else {
                eprintln!("❌ 响应数据为空");
                None
            }
        }
        Err(e) => {
            eprintln!("❌ 新增记录失败: {:?}", e);
            None
        }
    };

    // 2. 批量新增记录
    println!("\n--- 2. 批量新增记录 ---");

    let batch_records = vec![
        Record {
            record_id: None,
            fields: HashMap::from([
                ("项目名称".to_string(), json!("数据分析平台")),
                ("负责人".to_string(), json!("李四")),
                ("状态".to_string(), json!("待处理")),
                ("进度".to_string(), json!(0)),
            ]),
            created_by: None,
            created_time: None,
            last_modified_by: None,
            last_modified_time: None,
        },
        Record {
            record_id: None,
            fields: HashMap::from([
                ("项目名称".to_string(), json!("移动应用开发")),
                ("负责人".to_string(), json!("王五")),
                ("状态".to_string(), json!("已完成")),
                ("进度".to_string(), json!(100)),
            ]),
            created_by: None,
            created_time: None,
            last_modified_by: None,
            last_modified_time: None,
        },
    ];

    let batch_create_req =
        open_lark::service::bitable::v1::app_table_record::BatchCreateRecordRequest::builder()
            .app_token(app_token)
            .table_id(table_id)
            .user_id_type("open_id")
            .records(batch_records)
            .build();

    let mut batch_record_ids = Vec::new();
    match client
        .bitable
        .v1
        .app_table_record
        .batch_create(batch_create_req, None)
        .await
    {
        Ok(resp) => {
            if let Some(data) = resp.data {
                println!("✅ 批量新增记录成功!");
                println!("📊 新增数量: {}", data.records.len());
                for (i, record) in data.records.iter().enumerate() {
                    if let Some(record_id) = &record.record_id {
                        println!("  {}. Record ID: {}", i + 1, record_id);
                        batch_record_ids.push(record_id.clone());
                    }
                }
            } else {
                println!("❌ 响应数据为空");
            }
        }
        Err(e) => {
            eprintln!("❌ 批量新增记录失败: {:?}", e);
        }
    }

    // 3. 查询记录
    println!("\n--- 3. 查询记录 ---");

    // 创建筛选条件：状态为"进行中"且负责人不为空
    let filter = FilterInfo::and(vec![
        FilterCondition::equals("状态", "进行中"),
        FilterCondition::is_not_empty("负责人"),
    ]);

    // 创建排序条件：按进度降序
    let sort = vec![SortCondition {
        field_name: "进度".to_string(),
        desc: Some(true),
    }];

    let search_req =
        open_lark::service::bitable::v1::app_table_record::SearchRecordRequest::builder()
            .app_token(app_token)
            .table_id(table_id)
            .user_id_type("open_id")
            .page_size(20)
            .filter(filter)
            .sort(sort)
            .field_names(vec![
                "项目名称".to_string(),
                "负责人".to_string(),
                "状态".to_string(),
                "进度".to_string(),
            ])
            .automatic(true)
            .build();

    match client
        .bitable
        .v1
        .app_table_record
        .search(search_req, None)
        .await
    {
        Ok(resp) => {
            if let Some(data) = resp.data {
                println!("✅ 查询记录成功!");
                println!("📊 总数: {}", data.total);
                println!("🔄 是否还有更多: {}", data.has_more);
                println!("📋 记录列表:");

                for (i, record) in data.items.iter().enumerate() {
                    println!("\n  {}. Record ID: {:?}", i + 1, record.record_id);
                    for (field, value) in &record.fields {
                        println!("     {}: {}", field, value);
                    }
                }
            } else {
                println!("❌ 响应数据为空");
            }
        }
        Err(e) => {
            eprintln!("❌ 查询记录失败: {:?}", e);
        }
    }

    // 4. 更新记录
    if let Some(record_id) = &created_record_id {
        println!("\n--- 4. 更新记录 ---");

        let update_req =
            open_lark::service::bitable::v1::app_table_record::UpdateRecordRequest::builder()
                .app_token(app_token)
                .table_id(table_id)
                .record_id(record_id)
                .user_id_type("open_id")
                .fields(json!({
                    "状态": "已完成",
                    "进度": 100,
                    "完成日期": 1704153600000i64, // 2024-01-02
                }))
                .build();

        match client
            .bitable
            .v1
            .app_table_record
            .update(update_req, None)
            .await
        {
            Ok(resp) => {
                if let Some(data) = resp.data {
                    println!("✅ 更新记录成功!");
                    println!("🆔 Record ID: {:?}", data.record.record_id);
                    println!("📝 更新后的字段:");
                    for (field, value) in &data.record.fields {
                        println!("   {}: {}", field, value);
                    }
                } else {
                    println!("❌ 响应数据为空");
                }
            }
            Err(e) => {
                eprintln!("❌ 更新记录失败: {:?}", e);
            }
        }
    }

    // 5. 批量更新记录
    if !batch_record_ids.is_empty() {
        println!("\n--- 5. 批量更新记录 ---");

        let update_records = batch_record_ids
            .iter()
            .take(2)
            .enumerate()
            .map(|(i, record_id)| {
                UpdateRecord::new(
                    record_id,
                    json!({
                        "进度": 50 + i * 25,
                        "备注": format!("批量更新测试 {}", i + 1)
                    }),
                )
            })
            .collect();

        let batch_update_req =
            open_lark::service::bitable::v1::app_table_record::BatchUpdateRecordRequest::builder()
                .app_token(app_token)
                .table_id(table_id)
                .user_id_type("open_id")
                .records(update_records)
                .build();

        match client
            .bitable
            .v1
            .app_table_record
            .batch_update(batch_update_req, None)
            .await
        {
            Ok(resp) => {
                if let Some(data) = resp.data {
                    println!("✅ 批量更新记录成功!");
                    println!("📊 更新数量: {}", data.records.len());
                } else {
                    println!("❌ 响应数据为空");
                }
            }
            Err(e) => {
                eprintln!("❌ 批量更新记录失败: {:?}", e);
            }
        }
    }

    // 6. 批量获取记录
    if !batch_record_ids.is_empty() {
        println!("\n--- 6. 批量获取记录 ---");

        let batch_get_req =
            open_lark::service::bitable::v1::app_table_record::BatchGetRecordRequest::builder()
                .app_token(app_token)
                .table_id(table_id)
                .user_id_type("open_id")
                .record_ids(batch_record_ids.clone())
                .automatic(true)
                .with_shared_url(false)
                .build();

        match client
            .bitable
            .v1
            .app_table_record
            .batch_get(batch_get_req, None)
            .await
        {
            Ok(resp) => {
                if let Some(data) = resp.data {
                    println!("✅ 批量获取记录成功!");
                    println!("📊 获取数量: {}", data.records.len());
                    for (i, record) in data.records.iter().enumerate() {
                        println!("\n  {}. Record ID: {:?}", i + 1, record.record_id);
                        for (field, value) in &record.fields {
                            println!("     {}: {}", field, value);
                        }
                    }
                } else {
                    println!("❌ 响应数据为空");
                }
            }
            Err(e) => {
                eprintln!("❌ 批量获取记录失败: {:?}", e);
            }
        }
    }

    // 7. 删除记录（演示用，注释掉以避免删除数据）
    println!("\n--- 7. 删除记录 (演示用) ---");
    println!("⚠️  注意：删除操作已注释，避免删除测试数据");

    // if let Some(record_id) = &created_record_id {
    // let delete_req = open_lark::service::bitable::v1::app_table_record::DeleteRecordRequest::builder()
    // .app_token(app_token)
    // .table_id(table_id)
    // .record_id(record_id)
    // .build();
    //
    // match client.bitable.v1.app_table_record.delete(delete_req, None).await {
    // Ok(resp) => {
    // if let Some(data) = resp.data {
    // println!("✅ 删除记录成功: {}", data.deleted);
    // println!("🆔 删除的记录 ID: {}", data.record_id);
    // } else {
    // println!("❌ 响应数据为空");
    // }
    // }
    // Err(e) => {
    // eprintln!("❌ 删除记录失败: {:?}", e);
    // }
    // }
    // }

    // 8. 批量删除记录（演示用，注释掉以避免删除数据）
    println!("\n--- 8. 批量删除记录 (演示用) ---");
    println!("⚠️  注意：批量删除操作已注释，避免删除测试数据");

    // if !batch_record_ids.is_empty() {
    // let batch_delete_req = open_lark::service::bitable::v1::app_table_record::BatchDeleteRecordRequest::builder()
    // .app_token(app_token)
    // .table_id(table_id)
    // .records(batch_record_ids)
    // .build();
    //
    // match client.bitable.v1.app_table_record.batch_delete(batch_delete_req, None).await {
    // Ok(resp) => {
    // if let Some(data) = resp.data {
    // println!("✅ 批量删除记录成功!");
    // println!("📊 删除结果:");
    // for record in &data.records {
    // println!("  Record ID: {} - 删除成功: {}", record.record_id, record.deleted);
    // }
    // } else {
    // println!("❌ 响应数据为空");
    // }
    // }
    // Err(e) => {
    // eprintln!("❌ 批量删除记录失败: {:?}", e);
    // }
    // }
    // }

    println!("\n💡 记录操作说明:");
    println!("- 记录是多维表格中的数据行，每条记录包含多个字段的值");
    println!("- 支持多种字段类型：文本、数字、单选、多选、日期、复选框、人员等");
    println!("- 查询支持复杂的筛选条件和排序规则");
    println!("- 批量操作可以提高处理大量数据的效率");

    println!("\n🔍 筛选条件说明:");
    println!("- is: 等于");
    println!("- isNot: 不等于");
    println!("- contains: 包含");
    println!("- doesNotContain: 不包含");
    println!("- isEmpty: 为空");
    println!("- isNotEmpty: 不为空");
    println!("- isGreater: 大于");
    println!("- isLess: 小于");

    println!("\n📝 字段类型说明:");
    println!("- 文本: 普通文本内容");
    println!("- 数字: 数值类型");
    println!("- 单选/多选: 选项值");
    println!("- 日期: 毫秒级时间戳");
    println!("- 复选框: true/false");
    println!("- 人员: 用户 ID (open_id/union_id/user_id)");

    Ok(())
}
