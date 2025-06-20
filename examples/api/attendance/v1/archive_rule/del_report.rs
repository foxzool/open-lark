use dotenvy::dotenv;
use open_lark::{
    prelude::LarkClient,
    service::attendance::v1::models::DelArchiveReportRequest,
};

/// 删除归档报表行数据示例
///
/// 该接口用于删除指定的归档报表数据记录。
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // 从环境变量获取应用配置
    let app_id = std::env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET must be set");

    // 创建 LarkClient
    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 构建删除归档报表行数据请求
    let req = DelArchiveReportRequest {
        archive_rule_id: "archive_rule_123".to_string(),
        employee_type: "employee_id".to_string(),
        record_ids: vec![
            "record_123".to_string(),
            "record_456".to_string(),
            "record_789".to_string(),
        ],
        ..Default::default()
    };

    // 调用 API
    match client
        .attendance
        .v1
        .archive_rule
        .del_report(req, None)
        .await
    {
        Ok(resp) => {
            println!("删除归档报表行数据成功!");
            if let Some(data) = resp.data {
                println!("成功删除记录数: {}", data.success_count);
                println!("失败删除记录数: {}", data.failed_count);
                if let Some(failed_records) = data.failed_records {
                    if !failed_records.is_empty() {
                        println!("删除失败详情:");
                        for failed in failed_records {
                            println!("删除失败的记录ID: {} - 错误: {}", failed.record_id, failed.reason);
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("删除归档报表行数据失败: {:?}", e);
        }
    }

    Ok(())
}
