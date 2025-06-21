use open_lark::{
    core::config::{AppType, Config},
    service::bitable::v1::app_dashboard::{CopyDashboardRequest, ListDashboardRequest},
    LarkClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    // 创建客户端
    let client = LarkClient::builder(app_id, app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    // 配置必要参数
    let app_token = "bascnmBA*****yGehy8";

    // 1. 列出仪表盘
    println!("1. 列出仪表盘...");

    let list_request = ListDashboardRequest::builder()
        .app_token(app_token)
        .page_size(10)
        .build();

    let list_response = client
        .bitable
        .v1
        .app_dashboard
        .list(list_request, None)
        .await?;

    println!("仪表盘列表:");
    for dashboard in &list_response.data.items {
        println!("  - {}: {}", dashboard.name, dashboard.block_id);
    }
    println!("总计 {} 个仪表盘", list_response.data.total);

    // 如果有仪表盘，复制第一个
    if !list_response.data.items.is_empty() {
        let first_dashboard = &list_response.data.items[0];
        println!("\n2. 复制仪表盘: {}", first_dashboard.name);

        let copy_request = CopyDashboardRequest::builder()
            .app_token(app_token)
            .block_id(&first_dashboard.block_id)
            .name(format!("{} - 副本", first_dashboard.name))
            .build();

        match client
            .bitable
            .v1
            .app_dashboard
            .copy(copy_request, None)
            .await
        {
            Ok(copy_response) => {
                println!("仪表盘复制成功:");
                println!("  原仪表盘: {}", first_dashboard.name);
                println!("  复制后的仪表盘: {}", copy_response.data.dashboard.name);
                println!("  新仪表盘ID: {}", copy_response.data.dashboard.block_id);
            }
            Err(e) => {
                println!("仪表盘复制失败: {:?}", e);
            }
        }

        // 3. 重新列出仪表盘，查看复制结果
        println!("\n3. 重新列出仪表盘（查看复制结果）...");

        let list_request_after = ListDashboardRequest::builder()
            .app_token(app_token)
            .page_size(20)
            .build();

        let list_response_after = client
            .bitable
            .v1
            .app_dashboard
            .list(list_request_after, None)
            .await?;

        println!("更新后的仪表盘列表:");
        for dashboard in &list_response_after.data.items {
            println!("  - {}: {}", dashboard.name, dashboard.block_id);
        }
        println!(
            "总计 {} 个仪表盘（增加了 {} 个）",
            list_response_after.data.total,
            list_response_after.data.total - list_response.data.total
        );
    } else {
        println!("暂无仪表盘可复制，跳过复制操作");
    }

    println!("\n仪表盘操作示例完成！");
    Ok(())
}
