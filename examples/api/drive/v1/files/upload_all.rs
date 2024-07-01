use std::{env, io::Read};

use dotenvy::dotenv;
use simd_adler32::Adler32;

use open_lark::{client::LarkClient, service::drive::v1::files::UploadAllRequest};

/// 向云空间指定目录下上传一个小文件
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClient::builder(&app_id, &app_secret).build();

    let mut file = std::fs::File::open("tmp/1.txt").unwrap();
    let file_size = file.metadata().unwrap().len() as usize;
    let file_name = "1.txt";
    let mut buffer = vec![];
    file.read_to_end(&mut buffer).unwrap();
    let mut alder = Adler32::new();
    alder.write(&buffer);
    let checksum = alder.finish();

    let req = UploadAllRequest::builder()
        .file_name(file_name)
        .parent_type("explorer")
        .parent_node("nodcnBh4MAgg2GpI5IkRVZuw3Jd")
        .size(file_size as i32)
        .checksum(checksum.to_string())
        .file(buffer)
        .build();

    // 发起请求
    let resp = client.drive.v1.files.upload_all(req, None).await.unwrap();
    if let Some(data) = resp.data {
        println!("上传成功响应: {:#?}", data);
    }
}
