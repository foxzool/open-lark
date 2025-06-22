use dotenvy::dotenv;
use log::error;
use open_lark::{prelude::*, service::bitable::v1::app_table_record::UpdateRecordRequest};
use serde_json::json;
use std::env;

/// 更新记录
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClient::builder(&app_id, &app_secret).build();
    // 构建请求体
    let req = UpdateRecordRequest::builder()
        .app_token("UHNYbRv2BaxTllsMLt8cykiSnsb")
        .table_id("tbli6aSCRbJKDFIe")
        .record_id("recqavj45m")
        .fields(json!({"文本": "更新文本"}))
        .build();
    // 发起请求
    let resp = client
        .bitable
        .v1
        .app_table_record
        .update(req, None)
        .await
        .unwrap();

    match resp.data {
        None => {
            error!("{:?}", resp)
        }
        Some(data) => {
            println!("create  bitable app record response: {:#?}", data);
        }
    }
}
