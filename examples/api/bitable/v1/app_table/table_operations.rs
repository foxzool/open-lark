use dotenvy::dotenv;
use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID not found");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET not found");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    // 替换为实际的多维表格 app_token
    let app_token = "bascnmBA*****yGehy8"; // 请替换为实际的 app_token

    // 创建单个数据表示例
    println!("--- 1. 创建单个数据表 ---");

    use open_lark::service::bitable::v1::app_table::{TableData, TableField};

    // 创建包含多种字段类型的数据表
    let table = TableData::new("项目管理表")
        .with_default_view_name("主视图")
        .with_fields(vec![
            TableField::text("项目名称"),
            TableField::text("项目描述"),
            TableField::single_select(
                "优先级",
                vec!["高".to_string(), "中".to_string(), "低".to_string()],
            ),
            TableField::multi_select(
                "标签",
                vec![
                    "前端".to_string(),
                    "后端".to_string(),
                    "设计".to_string(),
                    "测试".to_string(),
                ],
            ),
            TableField::date("截止日期"),
            TableField::number("预计工时"),
        ]);

    let create_req =
        open_lark::service::bitable::v1::app_table::CreateTableRequest::new(app_token, table);

    let table_id = match client.bitable.v1.app_table.create(create_req, None).await {
        Ok(resp) => {
            if let Some(data) = resp.data {
                println!("✅ 创建数据表成功!");
                println!("🆔 Table ID: {}", data.table_id);
                println!("👁️  Default View ID: {}", data.default_view_id);
                println!("📋 Field ID 列表: {:?}", data.field_id_list);
                println!("🔢 字段数量: {}", data.field_id_list.len());
                Some(data.table_id)
            } else {
                eprintln!("❌ 响应数据为空");
                None
            }
        }
        Err(e) => {
            eprintln!("❌ 创建数据表失败: {:?}", e);
            None
        }
    };

    // 批量创建数据表示例
    println!("\n--- 2. 批量创建数据表 ---");

    let table1 = TableData::new("用户信息表").with_fields(vec![
        TableField::text("用户名"),
        TableField::text("邮箱"),
        TableField::single_select("状态", vec!["激活".to_string(), "禁用".to_string()]),
    ]);

    let table2 = TableData::new("订单记录表").with_fields(vec![
        TableField::text("订单号"),
        TableField::number("订单金额"),
        TableField::single_select(
            "订单状态",
            vec![
                "待支付".to_string(),
                "已支付".to_string(),
                "已发货".to_string(),
                "已完成".to_string(),
            ],
        ),
        TableField::date("创建时间"),
    ]);

    let batch_create_req =
        open_lark::service::bitable::v1::app_table::BatchCreateTablesRequest::builder()
            .app_token(app_token)
            .add_table(table1)
            .add_table(table2)
            .build();

    let batch_table_ids = match client
        .bitable
        .v1
        .app_table
        .batch_create(batch_create_req, None)
        .await
    {
        Ok(resp) => {
            if let Some(data) = resp.data {
                println!("✅ 批量创建数据表成功!");
                println!("📊 创建表格数量: {}", data.table_ids.len());
                for (i, table_id) in data.table_ids.iter().enumerate() {
                    println!("  {}. Table ID: {}", i + 1, table_id);
                }
                data.table_ids
            } else {
                eprintln!("❌ 响应数据为空");
                Vec::new()
            }
        }
        Err(e) => {
            eprintln!("❌ 批量创建数据表失败: {:?}", e);
            Vec::new()
        }
    };

    // 列出所有数据表
    println!("\n--- 3. 列出所有数据表 ---");
    let list_req = open_lark::service::bitable::v1::app_table::ListTablesRequest::builder()
        .app_token(app_token)
        .page_size(50)
        .build();

    match client.bitable.v1.app_table.list(list_req, None).await {
        Ok(resp) => {
            if let Some(data) = resp.data {
                println!("✅ 获取数据表列表成功!");
                println!("📊 总数量: {}", data.total);
                println!("🔄 是否还有更多: {}", data.has_more);
                println!("📋 数据表列表:");

                for (i, table) in data.items.iter().enumerate() {
                    println!(
                        "  {}. {} (ID: {}, 版本: {})",
                        i + 1,
                        table.name,
                        table.table_id,
                        table.revision
                    );
                }
            } else {
                println!("❌ 响应数据为空");
            }
        }
        Err(e) => {
            eprintln!("❌ 获取数据表列表失败: {:?}", e);
        }
    }

    // 更新数据表名称
    if let Some(table_id) = &table_id {
        println!("\n--- 4. 更新数据表名称 ---");
        let patch_req = open_lark::service::bitable::v1::app_table::PatchTableRequest::builder()
            .app_token(app_token)
            .table_id(table_id)
            .name("更新后的项目管理表")
            .build();

        match client.bitable.v1.app_table.patch(patch_req, None).await {
            Ok(resp) => {
                if let Some(data) = resp.data {
                    println!("✅ 更新数据表名称成功!");
                    println!("📋 新名称: {}", data.name);
                } else {
                    println!("❌ 响应数据为空");
                }
            }
            Err(e) => {
                eprintln!("❌ 更新数据表名称失败: {:?}", e);
            }
        }
    }

    // 删除单个数据表 (演示用)
    if let Some(table_id) = &table_id {
        println!("\n--- 5. 删除单个数据表 (演示用) ---");
        println!("⚠️  注意：这将永久删除数据表和其中的所有数据!");

        // 取消注释以下代码来执行删除操作
        // let delete_req = open_lark::service::bitable::v1::app_table::DeleteTableRequest::new(app_token, table_id);
        //
        // match client.bitable.v1.app_table.delete(delete_req, None).await {
        // Ok(resp) => {
        // if let Some(data) = resp.data {
        // println!("✅ 删除数据表成功: {}", data.deleted);
        // } else {
        // println!("❌ 响应数据为空");
        // }
        // }
        // Err(e) => {
        // eprintln!("❌ 删除数据表失败: {:?}", e);
        // }
        // }
    }

    // 批量删除数据表 (演示用)
    if !batch_table_ids.is_empty() {
        println!("\n--- 6. 批量删除数据表 (演示用) ---");
        println!("⚠️  注意：这将永久删除多个数据表和其中的所有数据!");

        // 取消注释以下代码来执行批量删除操作
        // let batch_delete_req = open_lark::service::bitable::v1::app_table::BatchDeleteTablesRequest::new(
        // app_token,
        // batch_table_ids.clone()
        // );
        //
        // match client.bitable.v1.app_table.batch_delete(batch_delete_req, None).await {
        // Ok(resp) => {
        // if let Some(data) = resp.data {
        // println!("✅ 批量删除数据表成功: {}", data.deleted);
        // println!("🗑️  删除的表格数量: {}", batch_table_ids.len());
        // } else {
        // println!("❌ 响应数据为空");
        // }
        // }
        // Err(e) => {
        // eprintln!("❌ 批量删除数据表失败: {:?}", e);
        // }
        // }
    }

    println!("\n💡 数据表功能说明:");
    println!("- 数据表是多维表格中存储数据的基本单位");
    println!("- 每个数据表包含多个字段（列）和记录（行）");
    println!("- 支持多种字段类型：文本、数字、单选、多选、日期等");
    println!("- 可以创建多个视图来展示不同的数据视角");

    println!("\n📊 字段类型说明:");
    println!("- 文本字段 (type=1): 存储文本信息");
    println!("- 数字字段 (type=2): 存储数值信息");
    println!("- 单选字段 (type=3): 从预设选项中选择一个");
    println!("- 多选字段 (type=4): 从预设选项中选择多个");
    println!("- 日期字段 (type=5): 存储日期时间信息");

    println!("\n🎯 接下来可以进行的操作:");
    println!("- 使用字段 API 管理数据表的字段结构");
    println!("- 使用记录 API 向数据表中添加和查询数据");
    println!("- 使用视图 API 创建不同的数据展示方式");

    Ok(())
}
