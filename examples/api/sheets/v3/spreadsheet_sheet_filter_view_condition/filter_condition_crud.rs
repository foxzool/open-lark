use dotenvy::dotenv;
use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let app_id = std::env::var("APP_ID").expect("APP_ID not found");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET not found");

    let client = LarkClient::builder(app_id, app_secret)
        .with_enable_token_cache(true)
        .build();

    // 先创建一个筛选视图用于演示
    println!("--- 1. 创建筛选视图 ---");
    let filter_view_req = open_lark::service::sheets::v3::spreadsheet_sheet_filter_view::CreateFilterViewRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8") // 替换为实际的表格 token
        .sheet_id("Sheet1") // 替换为实际的工作表 ID
        .filter_view_name("销售数据筛选条件测试")
        .range("A1:E100")
        .build();

    let filter_view_id = match client.sheets.v3.spreadsheet_sheet_filter_view.create(filter_view_req, None).await {
        Ok(resp) => {
            println!("✅ 创建筛选视图成功: {}", resp.data.filter_view_id);
            resp.data.filter_view_id
        }
        Err(e) => {
            eprintln!("❌ 创建筛选视图失败: {:?}", e);
            return Ok(());
        }
    };

    // 创建筛选条件示例
    println!("\n--- 2. 创建筛选条件 ---");
    
    use open_lark::service::sheets::v3::spreadsheet_sheet_filter_view_condition::FilterCondition;
    
    // 创建一个销售额大于5000的筛选条件
    let condition = FilterCondition::greater_than("销售额", "5000");
    
    let create_req = open_lark::service::sheets::v3::spreadsheet_sheet_filter_view_condition::CreateFilterViewConditionRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8")
        .sheet_id("Sheet1")
        .filter_view_id(&filter_view_id)
        .condition(condition)
        .build();

    let condition_id = match client.sheets.v3.spreadsheet_sheet_filter_view.create_condition(create_req, None).await {
        Ok(resp) => {
            println!("✅ 创建筛选条件成功!");
            println!("🆔 条件 ID: {}", resp.data.condition_id);
            println!("🔍 条件类型: {}", resp.data.condition.filter_type);
            println!("📊 筛选列: {}", resp.data.condition.col_name);
            println!("💰 筛选值: {:?}", resp.data.condition.compare_values);
            resp.data.condition_id
        }
        Err(e) => {
            eprintln!("❌ 创建筛选条件失败: {:?}", e);
            return Ok(());
        }
    };

    // 查询所有筛选条件
    println!("\n--- 3. 查询所有筛选条件 ---");
    let query_req = open_lark::service::sheets::v3::spreadsheet_sheet_filter_view_condition::QueryFilterViewConditionsRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8")
        .sheet_id("Sheet1")
        .filter_view_id(&filter_view_id)
        .build();

    match client.sheets.v3.spreadsheet_sheet_filter_view.query_conditions(query_req, None).await {
        Ok(resp) => {
            println!("✅ 查询筛选条件成功!");
            println!("📊 共找到 {} 个筛选条件:", resp.data.items.len());
            for (i, item) in resp.data.items.iter().enumerate() {
                println!("  {}. ID: {}, 列: {}, 类型: {}, 值: {:?}", 
                    i + 1, 
                    item.condition_id, 
                    item.condition.col_name,
                    item.condition.filter_type,
                    item.condition.compare_values
                );
            }
        }
        Err(e) => {
            eprintln!("❌ 查询筛选条件失败: {:?}", e);
        }
    }

    // 获取特定筛选条件
    println!("\n--- 4. 获取特定筛选条件 ---");
    let get_req = open_lark::service::sheets::v3::spreadsheet_sheet_filter_view_condition::GetFilterViewConditionRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8")
        .sheet_id("Sheet1")
        .filter_view_id(&filter_view_id)
        .condition_id(&condition_id)
        .build();

    match client.sheets.v3.spreadsheet_sheet_filter_view.get_condition(get_req, None).await {
        Ok(resp) => {
            println!("✅ 获取筛选条件详情成功!");
            println!("📊 列名: {}", resp.data.condition_info.condition.col_name);
            println!("🔍 筛选类型: {}", resp.data.condition_info.condition.filter_type);
            println!("💰 筛选值: {:?}", resp.data.condition_info.condition.compare_values);
        }
        Err(e) => {
            eprintln!("❌ 获取筛选条件详情失败: {:?}", e);
        }
    }

    // 更新筛选条件
    println!("\n--- 5. 更新筛选条件 ---");
    let new_condition = FilterCondition::contains("产品名称", "手机");
    
    let update_req = open_lark::service::sheets::v3::spreadsheet_sheet_filter_view_condition::UpdateFilterViewConditionRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8")
        .sheet_id("Sheet1")
        .filter_view_id(&filter_view_id)
        .condition_id(&condition_id)
        .condition(new_condition)
        .build();

    match client.sheets.v3.spreadsheet_sheet_filter_view.update_condition(update_req, None).await {
        Ok(resp) => {
            println!("✅ 更新筛选条件成功!");
            println!("📊 新的列名: {}", resp.data.condition.col_name);
            println!("🔍 新的筛选类型: {}", resp.data.condition.filter_type);
            println!("💰 新的筛选值: {:?}", resp.data.condition.compare_values);
        }
        Err(e) => {
            eprintln!("❌ 更新筛选条件失败: {:?}", e);
        }
    }

    // 删除筛选条件 (演示用)
    println!("\n--- 6. 删除筛选条件 (演示用) ---");
    println!("⚠️  注意：这将永久删除筛选条件!");
    
    // 取消注释以下代码来执行删除操作
    /*
    let delete_req = open_lark::service::sheets::v3::spreadsheet_sheet_filter_view_condition::DeleteFilterViewConditionRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8")
        .sheet_id("Sheet1")
        .filter_view_id(&filter_view_id)
        .condition_id(&condition_id)
        .build();

    match client.sheets.v3.spreadsheet_sheet_filter_view.delete_condition(delete_req, None).await {
        Ok(resp) => {
            println!("✅ 删除筛选条件成功: {}", resp.data.success);
        }
        Err(e) => {
            eprintln!("❌ 删除筛选条件失败: {:?}", e);
        }
    }
    */

    println!("\n💡 筛选条件功能说明:");
    println!("- 筛选条件属于筛选视图的子功能，用于定义具体的筛选规则");
    println!("- 支持多种筛选类型：等于、不等于、包含、大于、小于等");
    println!("- 可以为一个筛选视图创建多个筛选条件");
    println!("- 筛选条件支持完整的CRUD操作");
    
    println!("\n🔍 支持的筛选类型:");
    println!("- equal: 等于");
    println!("- notEqual: 不等于");
    println!("- contains: 包含");
    println!("- greaterThan: 大于");
    println!("- lessThan: 小于");

    Ok(())
}