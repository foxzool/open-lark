use std::env;

use dotenvy::dotenv;

use open_lark::{prelude::*, service::bitable::v1::AppTableFieldService};

/// 查询记录
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClient::builder(&app_id, &app_secret).build();
    // 构建请求体 - 直接使用服务方法避免访问私有模块
    let app_token = "Iht6bSSnvaL4mxsGPMPcInvun8b";
    let table_id = "tblxIYN0WCaZNC0Y";
    
    // 发起请求
    let resp = client
        .bitable
        .v1
        .app_table_field
        .list_field(app_token, table_id, None)
        .await
        .unwrap();
    if let Some(data) = resp.data {
        println!("list  bitable app table field response: {:#?}", data);
    }
}
