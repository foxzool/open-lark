use std::env;

use dotenvy::dotenv;

use open_lark::{
    client::LarkClientBuilder,
    core::{api_resp::ApiResponse, req_option::RequestOption},
    service::drive::v1::files::DownloadRequest,
};

/// 下载文件
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClientBuilder::new(&app_id, &app_secret).build();

    let req = DownloadRequest::builder()
        .file_token("KmNZb0WYcoVhuzxFzuScFUOUnNh")
        .build();

    // 发起请求
    let resp = client
        .drive
        .v1
        .files
        .download(
            req,
            Some(
                RequestOption::builder()
                    .add_header("Range", "bytes=0-1024")
                    .build(),
            ),
        ).await
        .unwrap();
    if let ApiResponse::Success { data, .. } = resp {
        println!("file data: {:#?}", data);
    }
}
