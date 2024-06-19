use std::env;

use dotenvy::dotenv;

use open_lark::{client::LarkClientBuilder, service::bitable::v1::GetAppRequest};

/// 获取多维表格元数据
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClientBuilder::new(&app_id, &app_secret).build();
    // 构建请求体
    let req = GetAppRequest::builder()
        .app_token("Iht6bSSnvaL4mxsGPMPcInvun8b")
        .build();
    // 发起请求
    let resp = client.bitable.v1.app.get(req, None).await.unwrap();
    if let Some(data) = resp.data {
        println!("patch bitable response: {:#?}", data);
    }
}
