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

    // 增加保护范围示例
    println!("--- 1. 增加保护范围 ---");

    use open_lark::service::sheets::v3::protect_range::ProtectRangeData;

    // 创建行保护范围（保护第1-10行）
    let protect_range = ProtectRangeData::row_range("Sheet1", 1, 10);

    let add_req = open_lark::service::sheets::v3::protect_range::AddProtectRangeRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8") // 替换为实际的表格 token
        .protect_range(protect_range)
        .build();

    let protect_id = match client
        .sheets
        .v3
        .spreadsheet
        .add_protect_range(add_req, None)
        .await
    {
        Ok(resp) => {
            if let Some(data) = resp.data {
                println!("✅ 增加保护范围成功!");
                println!("🆔 保护范围 ID: {}", data.protect_id);
                println!("📏 保护维度: {}", data.protect_range.dimension);
                println!("📋 工作表 ID: {}", data.protect_range.sheet_id);
                println!("🔢 起始索引: {}", data.protect_range.start_index);
                println!("🔢 结束索引: {}", data.protect_range.end_index);
                data.protect_id
            } else {
                eprintln!("❌ 响应数据为空");
                return Ok(());
            }
        }
        Err(e) => {
            eprintln!("❌ 增加保护范围失败: {:?}", e);
            return Ok(());
        }
    };

    // 再添加一个列保护范围
    println!("\n--- 2. 添加列保护范围 ---");
    let column_protect = ProtectRangeData::column_range("Sheet1", 1, 3);

    let add_column_req =
        open_lark::service::sheets::v3::protect_range::AddProtectRangeRequest::builder()
            .spreadsheet_token("shtcnmBA*****yGehy8")
            .protect_range(column_protect)
            .build();

    let column_protect_id = match client
        .sheets
        .v3
        .spreadsheet
        .add_protect_range(add_column_req, None)
        .await
    {
        Ok(resp) => {
            if let Some(data) = resp.data {
                println!("✅ 添加列保护范围成功!");
                println!("🆔 保护范围 ID: {}", data.protect_id);
                println!("📏 保护维度: {}", data.protect_range.dimension);
                data.protect_id
            } else {
                eprintln!("❌ 响应数据为空");
                return Ok(());
            }
        }
        Err(e) => {
            eprintln!("❌ 添加列保护范围失败: {:?}", e);
            return Ok(());
        }
    };

    // 获取所有保护范围
    println!("\n--- 3. 获取所有保护范围 ---");
    let get_req = open_lark::service::sheets::v3::protect_range::GetProtectRangesRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8")
        .sheet_id("Sheet1")
        .build();

    match client
        .sheets
        .v3
        .spreadsheet
        .get_protect_ranges(get_req, None)
        .await
    {
        Ok(resp) => {
            if let Some(data) = resp.data {
                println!("✅ 获取保护范围成功!");
                println!("📊 共找到 {} 个保护范围:", data.items.len());
                for (i, item) in data.items.iter().enumerate() {
                    println!(
                        "  {}. ID: {}, 维度: {}, 工作表: {}, 范围: {}-{}",
                        i + 1,
                        item.protect_id,
                        item.protect_range.dimension,
                        item.protect_range.sheet_id,
                        item.protect_range.start_index,
                        item.protect_range.end_index
                    );
                }
            } else {
                println!("❌ 响应数据为空");
            }
        }
        Err(e) => {
            eprintln!("❌ 获取保护范围失败: {:?}", e);
        }
    }

    // 修改保护范围
    println!("\n--- 4. 修改保护范围 ---");
    let new_protect_range = ProtectRangeData::row_range("Sheet1", 5, 15); // 修改为第5-15行

    let update_req =
        open_lark::service::sheets::v3::protect_range::UpdateProtectRangeRequest::builder()
            .spreadsheet_token("shtcnmBA*****yGehy8")
            .protect_id(&protect_id)
            .protect_range(new_protect_range)
            .build();

    match client
        .sheets
        .v3
        .spreadsheet
        .update_protect_range(update_req, None)
        .await
    {
        Ok(resp) => {
            if let Some(data) = resp.data {
                println!("✅ 修改保护范围成功!");
                println!("🆔 保护范围 ID: {}", data.protect_id);
                println!("📏 新的保护维度: {}", data.protect_range.dimension);
                println!("🔢 新的起始索引: {}", data.protect_range.start_index);
                println!("🔢 新的结束索引: {}", data.protect_range.end_index);
            } else {
                println!("❌ 响应数据为空");
            }
        }
        Err(e) => {
            eprintln!("❌ 修改保护范围失败: {:?}", e);
        }
    }

    // 删除保护范围 (演示用)
    println!("\n--- 5. 删除保护范围 (演示用) ---");
    println!("⚠️  注意：这将永久删除保护范围!");

    // 取消注释以下代码来执行删除操作
    // let delete_req = open_lark::service::sheets::v3::protect_range::DeleteProtectRangeRequest::builder()
    // .spreadsheet_token("shtcnmBA*****yGehy8")
    // .protect_id(&column_protect_id)
    // .build();
    //
    // match client.sheets.v3.spreadsheet.delete_protect_range(delete_req, None).await {
    // Ok(resp) => {
    // println!("✅ 删除保护范围成功: {}", resp.data.success);
    // if let Some(deleted_id) = resp.data.protect_id {
    // println!("🗑️  已删除保护范围 ID: {}", deleted_id);
    // }
    // }
    // Err(e) => {
    // eprintln!("❌ 删除保护范围失败: {:?}", e);
    // }
    // }

    println!("\n💡 保护范围功能说明:");
    println!("- 保护范围用于防止指定行或列被意外修改");
    println!("- 支持行保护和列保护两种维度");
    println!("- 可以通过起始和结束索引精确控制保护范围");
    println!("- 支持完整的CRUD操作：增加、修改、获取、删除");

    println!("\n🔐 保护维度类型:");
    println!("- ROWS: 行保护，保护指定的行范围");
    println!("- COLUMNS: 列保护，保护指定的列范围");

    println!("\n📏 索引说明:");
    println!("- start_index: 保护范围的起始位置（从1开始）");
    println!("- end_index: 保护范围的结束位置（包含）");
    println!("- 例如：start_index=1, end_index=10 表示保护第1到第10行/列");

    Ok(())
}
