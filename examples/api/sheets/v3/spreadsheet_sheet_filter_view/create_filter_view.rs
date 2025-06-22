use dotenvy::dotenv;
use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let app_id = std::env::var("APP_ID").expect("APP_ID not found");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET not found");

    let client = LarkClient::builder(app_id, app_secret)
        .with_app_type(AppType::SelfBuilt)
        .with_enable_token_cache(true)
        .build();

    // 创建筛选视图示例
    let req = open_lark::service::sheets::v3::spreadsheet_sheet_filter_view::CreateFilterViewRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8") // 替换为实际的表格 token
        .sheet_id("Sheet1") // 替换为实际的工作表 ID
        .filter_view_name("销售数据筛选") // 筛选视图名称
        .range("A1:E100") // 筛选范围
        .build();

    match client.sheets.v3.spreadsheet_sheet_filter_view.create(req, None).await {
        Ok(resp) => {
            println!("✅ 创建筛选视图成功!");
            println!("🆔 筛选视图 ID: {}", resp.data.filter_view_id);
            println!("📝 筛选视图名称: {}", resp.data.filter_view_name);
            println!("📍 筛选范围: {}", resp.data.range);
            
            // 保存筛选视图ID，后续操作需要使用
            let filter_view_id = resp.data.filter_view_id.clone();
            
            // 查询所有筛选视图
            println!("\n--- 查询所有筛选视图 ---");
            let query_req = open_lark::service::sheets::v3::spreadsheet_sheet_filter_view::QueryFilterViewRequest::builder()
                .spreadsheet_token("shtcnmBA*****yGehy8")
                .sheet_id("Sheet1")
                .build();
            
            match client.sheets.v3.spreadsheet_sheet_filter_view.query(query_req, None).await {
                Ok(query_resp) => {
                    println!("✅ 查询筛选视图成功!");
                    println!("📊 共找到 {} 个筛选视图:", query_resp.data.items.len());
                    for (i, item) in query_resp.data.items.iter().enumerate() {
                        println!("  {}. {} (ID: {}, 范围: {})", 
                            i + 1, item.filter_view_name, item.filter_view_id, item.range);
                    }
                }
                Err(e) => {
                    eprintln!("❌ 查询筛选视图失败: {:?}", e);
                }
            }
            
            // 获取特定筛选视图
            println!("\n--- 获取特定筛选视图详情 ---");
            let get_req = open_lark::service::sheets::v3::spreadsheet_sheet_filter_view::GetFilterViewRequest::builder()
                .spreadsheet_token("shtcnmBA*****yGehy8")
                .sheet_id("Sheet1")
                .filter_view_id(&filter_view_id)
                .build();
            
            match client.sheets.v3.spreadsheet_sheet_filter_view.get(get_req, None).await {
                Ok(get_resp) => {
                    println!("✅ 获取筛选视图详情成功!");
                    println!("📝 名称: {}", get_resp.data.filter_view.filter_view_name);
                    println!("📍 范围: {}", get_resp.data.filter_view.range);
                }
                Err(e) => {
                    eprintln!("❌ 获取筛选视图详情失败: {:?}", e);
                }
            }
            
            // 更新筛选视图
            println!("\n--- 更新筛选视图 ---");
            let patch_req = open_lark::service::sheets::v3::spreadsheet_sheet_filter_view::PatchFilterViewRequest::builder()
                .spreadsheet_token("shtcnmBA*****yGehy8")
                .sheet_id("Sheet1")
                .filter_view_id(&filter_view_id)
                .filter_view_name("更新后的销售数据筛选") // 新的名称
                .range("A1:F150") // 新的范围
                .build();
            
            match client.sheets.v3.spreadsheet_sheet_filter_view.patch(patch_req, None).await {
                Ok(patch_resp) => {
                    println!("✅ 更新筛选视图成功!");
                    println!("📝 新名称: {}", patch_resp.data.filter_view_name);
                    println!("📍 新范围: {}", patch_resp.data.range);
                }
                Err(e) => {
                    eprintln!("❌ 更新筛选视图失败: {:?}", e);
                }
            }
            
            // 删除筛选视图 (注意：这会永久删除筛选视图)
            println!("\n--- 删除筛选视图 (演示用) ---");
            println!("⚠️  注意：这将永久删除筛选视图，请谨慎操作!");
            
            // 取消注释以下代码来执行删除操作
            /*
            let delete_req = open_lark::service::sheets::v3::spreadsheet_sheet_filter_view::DeleteFilterViewRequest::builder()
                .spreadsheet_token("shtcnmBA*****yGehy8")
                .sheet_id("Sheet1")
                .filter_view_id(&filter_view_id)
                .build();
            
            match client.sheets.v3.spreadsheet_sheet_filter_view.delete(delete_req, None).await {
                Ok(delete_resp) => {
                    println!("✅ 删除筛选视图成功: {}", delete_resp.data.success);
                }
                Err(e) => {
                    eprintln!("❌ 删除筛选视图失败: {:?}", e);
                }
            }
            */
            
        }
        Err(e) => {
            eprintln!("❌ 创建筛选视图失败: {:?}", e);
        }
    }

    println!("\n💡 筛选视图功能说明:");
    println!("- 筛选视图是一种保存的筛选器，可以快速切换不同的数据视图");
    println!("- 支持创建、查询、获取、更新和删除操作");
    println!("- 每个筛选视图有唯一的ID和名称");
    println!("- 可以设置不同的筛选范围来查看特定区域的数据");

    Ok(())
}