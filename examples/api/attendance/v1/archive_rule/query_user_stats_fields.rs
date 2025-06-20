use dotenvy::dotenv;
use open_lark::{
    prelude::LarkClient,
    service::attendance::v1::models::QueryArchiveStatsFieldsRequest,
};

/// 查询归档报表表头示例
///
/// 该接口用于查询指定归档规则的报表字段定义信息。
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // 从环境变量获取应用配置
    let app_id = std::env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET must be set");

    // 创建 LarkClient
    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 构建查询归档报表表头请求
    let req = QueryArchiveStatsFieldsRequest {
        archive_rule_id: "archive_rule_123".to_string(),
        employee_type: "employee_id".to_string(),
        ..Default::default()
    };

    // 调用 API
    match client
        .attendance
        .v1
        .archive_rule
        .query_user_stats_fields(req, None)
        .await
    {
        Ok(resp) => {
            println!("查询归档报表表头成功!");
            if let Some(data) = resp.data {
                println!("报表字段数量: {}", data.fields.len());
                for field in data.fields {
                    println!("字段: {} - {}", field.field_name, field.field_type);
                }
            }
        }
        Err(e) => {
            eprintln!("查询归档报表表头失败: {:?}", e);
        }
    }

    Ok(())
}
