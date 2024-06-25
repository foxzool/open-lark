use std::env;

use dotenvy::dotenv;

use open_lark::{client::LarkClient, service::bitable::v1::SearchAppTableRecordRequest};

/// 查询记录
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClient::builder(&app_id, &app_secret).build();
    // 构建请求体
    let req = SearchAppTableRecordRequest::builder()
        .app_token("Iht6bSSnvaL4mxsGPMPcInvun8b")
        .table_id("tblxIYN0WCaZNC0Y")
        .build();
    // 发起请求
    let resp = client
        .bitable
        .v1
        .app_table_record
        .search(req, None)
        .await
        .unwrap();
    if let Some(data) = resp.data {
        println!("search  bitable app record response: {:#?}", data);
    }
}
