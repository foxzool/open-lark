use dotenvy::dotenv;
use open_lark::{prelude::*, service::drive::v1::folder::GetFolderMetaRequest};
use std::env;

/// 获取文件夹元数据示例
///
/// 该接口用于根据文件夹的token获取文件夹的详细元数据信息。
///
/// <https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/get-folder-meta>
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量获取配置
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID 必须设置");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET 必须设置");
    let user_access_token = env::var("USER_ACCESS_TOKEN").expect("USER_ACCESS_TOKEN 必须设置");

    // 创建客户端
    let client = LarkClient::builder(app_id, app_secret)
        .with_enable_token_cache(true)
        .build();
    
    let option = RequestOption::builder()
        .user_access_token(user_access_token)
        .build();

    println!("开始获取文件夹元数据...");

    // 首先获取根目录的token，然后获取根目录的详细信息
    match client.drive.v1.folder.get_root_folder_meta(None).await {
        Ok(root_response) => {
            if let Some(root_data) = root_response.data {
                let folder_token = root_data.token;
                info!("获取到根目录token: {}", folder_token);

                // 构建获取文件夹元数据的请求
                let request = GetFolderMetaRequest::new(folder_token);

                // 调用API获取文件夹元数据
                match client.drive.v1.folder.get_folder_meta(request, Some(option)).await {
                    Ok(response) => {
                        println!("API调用成功");
                        println!("响应状态码: {}", response.code);
                        println!("响应消息: {}", response.msg);

                        if let Some(data) = response.data {
                            println!("文件夹元数据信息:");
                            println!("  - Token: {}", data.token);
                            println!("  - ID: {}", data.id);
                            println!("  - 名称: {}", data.name);
                            println!("  - 拥有者ID: {}", data.owner_id);
                            println!("  - 创建时间: {}", data.create_time);
                            println!("  - 修改时间: {}", data.edit_time);
                            println!("  - URL: {}", data.url);

                            if let Some(parent_token) = &data.parent_token {
                                println!("  - 父文件夹Token: {}", parent_token);
                            } else {
                                println!("  - 父文件夹Token: 无（根目录）");
                            }

                            if let Some(creator_id) = &data.creator_id {
                                println!("  - 创建者ID: {}", creator_id);
                            }

                            if let Some(description) = &data.description {
                                println!("  - 描述: {}", description);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("获取文件夹元数据失败: {}", e);
                    }
                }
            } else {
                eprintln!("获取根目录信息失败：没有返回数据");
            }
        }
        Err(e) => {
            eprintln!("获取根目录信息失败: {}", e);
        }
    }

    Ok(())
}
