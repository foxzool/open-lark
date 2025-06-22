use dotenvy::dotenv;
use open_lark::prelude::*;
use std::env;

/// 获取我的空间（root folder）元数据示例
///
/// 该接口用于根据用户的访问凭证获取用户的根目录信息，包括根目录的token等。
///
/// <https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/get-root-folder-meta>
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量获取配置
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID 必须设置");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET 必须设置");
    let user_access_token = env::var("USER_ACCESS_TOKEN").expect("USER_ACCESS_TOKEN 必须设置");

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();
    
    let option = RequestOption::builder()
        .user_access_token(user_access_token)
        .build();

    println!("开始获取用户根目录元数据...");

    // 调用API获取根目录元数据
    match client.drive.v1.folder.get_root_folder_meta(Some(option)).await {
        Ok(response) => {
            println!("API调用成功");
            println!("响应状态码: {}", response.code());
            println!("响应消息: {}", response.msg());

            if let Some(data) = response.data {
                println!("根目录信息:");
                println!("  - 根目录 Token: {}", data.token);
                println!("  - 用户 ID: {}", data.user_id);
            }
        }
        Err(e) => {
            eprintln!("API调用失败: {}", e);
        }
    }

    Ok(())
}
