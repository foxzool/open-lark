use std::env;

use dotenvy::dotenv;
use log::error;
use open_lark::{prelude::*, service::bitable::v1::app_table_record::search::SearchRecordRequest};

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
    let req = SearchRecordRequest::builder()
        .app_token("UHNYbRv2BaxTllsMLt8cykiSnsb")
        .table_id("tbli6aSCRbJKDFIe")
        .build();
    // 发起请求
    let resp = client
        .bitable
        .v1
        .app_table_record
        .search(req, None)
        .await
        .unwrap();
    // println!("{:?}", resp);
    match resp.data {
        None => {
            error!("{:?}", resp)
        }
        Some(data) => {
            println!("search  bitable app record response: {:#?}", data);
        }
    }
}
