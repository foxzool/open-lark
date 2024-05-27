use std::env;

use dotenvy::dotenv;

use open_lark::{
    client::LarkClientBuilder
    ,
    service::drive::v1::permissions::PatchPermissionRequest,
};

/// 获取云文档权限设置
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();

    // 创建 Client
    let client = LarkClientBuilder::new(&app_id, &app_secret).build();
    // 构建请求体
    let req = PatchPermissionRequest::builder()
        .token("AD0Ls3hpDh8shwtHy6OcqOetnec")
        .r#type("sheet")
        .external_access_entity("closed")
        .link_share_entity("tenant_readable")
        .build();
    // 发起请求
    let resp = client.drive.v2.permission.patch(req, None).await.unwrap();
    if let Some(data) = resp.data {
        println!("patch permission response: {:#?}", data);
    }
}
