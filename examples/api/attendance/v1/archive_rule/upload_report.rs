use dotenvy::dotenv;
use open_lark::{
    prelude::LarkClient,
    service::attendance::v1::models::{ArchiveReportRecord, UploadArchiveReportRequest},
};
use std::collections::HashMap;

/// 写入归档报表结果示例
///
/// 该接口用于向指定归档规则写入报表数据，支持批量上传考勤统计结果。
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // 从环境变量获取应用配置
    let app_id = std::env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET must be set");

    // 创建 LarkClient
    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 构建报表数据
    let mut field_data_1 = HashMap::new();
    field_data_1.insert("actual_work_hours".to_string(), "8.5".to_string());
    field_data_1.insert("attendance_status".to_string(), "normal".to_string());

    let mut field_data_2 = HashMap::new();
    field_data_2.insert("actual_work_hours".to_string(), "7.5".to_string());
    field_data_2.insert("attendance_status".to_string(), "late".to_string());

    let report_data = vec![
        ArchiveReportRecord {
            record_id: None,
            user_id: "user_123".to_string(),
            archive_date: "2024-06-20".to_string(),
            field_data: field_data_1,
        },
        ArchiveReportRecord {
            record_id: None,
            user_id: "user_456".to_string(),
            archive_date: "2024-06-20".to_string(),
            field_data: field_data_2,
        },
    ];

    // 构建写入归档报表结果请求
    let req = UploadArchiveReportRequest {
        archive_rule_id: "archive_rule_123".to_string(),
        employee_type: "employee_id".to_string(),
        report_data,
        ..Default::default()
    };

    // 调用 API
    match client
        .attendance
        .v1
        .archive_rule
        .upload_report(req, None)
        .await
    {
        Ok(resp) => {
            println!("写入归档报表结果成功!");
            if let Some(data) = resp.data {
                println!("成功记录数: {}", data.success_count);
                println!("失败记录数: {}", data.failed_count);
                if let Some(failed_records) = data.failed_records {
                    if !failed_records.is_empty() {
                        println!("失败记录详情:");
                        for failed in failed_records {
                            println!("失败用户: {} - 错误: {}", failed.user_id, failed.reason);
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("写入归档报表结果失败: {:?}", e);
        }
    }

    Ok(())
}
