use dotenvy::dotenv;
use log::error;
use open_lark::{
    prelude::*,
    service::bitable::v1::app_table_record::batch_create::BatchCreateRecordRequest,
    service::bitable::v1::Record,
};
use serde_json::json;
use std::env;

/// 新增多条记录
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClient::builder(&app_id, &app_secret).build();
    // 构建请求体
    let req = BatchCreateRecordRequest::builder()
        .app_token("P7t0b91y4accAks27oScppjgnuc")
        .table_id("tbl1TSGmbFDIezBZ")
        .records(vec![
            Record::from_json(json!( {
                "订单编号": "20210901001",
            })),
            Record::from_json(json!( {
                "订单编号": "20210901002",
            })),
        ])
        .build();
    // 发起请求
    let resp = client
        .bitable
        .v1
        .app_table_record
        .batch_create(req, None)
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
