use dotenv::dotenv;
use open_lark::prelude::*;
use open_lark::service::drive::v1::folder::CreateFolderRequest;
use std::env;
use tracing::info;

/// 新建文件夹示例
///
/// 该接口用于根据父文件夹的token在其中创建一个新的空文件夹。
///
/// <https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/create_folder>
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量获取配置
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app_id = env::var("APP_ID").expect("APP_ID 必须设置");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET 必须设置");
    let user_access_token = env::var("USER_ACCESS_TOKEN").expect("USER_ACCESS_TOKEN 必须设置");

    // 创建客户端，使用用户访问凭证
    let client = LarkClient::builder(app_id, app_secret)
        .with_user_access_token(user_access_token)
        .build();

    info!("开始创建新文件夹...");

    // 首先获取根目录的token，作为父文件夹
    match client.drive.v1.folder.get_root_folder_meta(None).await {
        Ok(root_response) => {
            if let Some(root_data) = root_response.data {
                let parent_token = root_data.token;
                info!("获取到根目录token作为父文件夹: {}", parent_token);

                // 构建新建文件夹的请求
                let folder_name = format!("测试文件夹_{}", chrono::Utc::now().timestamp());
                let request = CreateFolderRequest::new(folder_name.clone(), parent_token);

                // 调用API创建文件夹
                match client.drive.v1.folder.create_folder(request, None).await {
                    Ok(response) => {
                        info!("API调用成功");
                        println!("响应状态码: {}", response.code);
                        println!("响应消息: {}", response.msg);
                        
                        if let Some(data) = response.data {
                            println!("新建文件夹成功:");
                            println!("  - 文件夹名称: {}", folder_name);
                            println!("  - 文件夹Token: {}", data.token);
                            println!("  - 文件夹URL: {}", data.url);
                            
                            // 验证创建成功 - 尝试获取刚创建的文件夹元数据
                            info!("验证文件夹创建成功，获取文件夹元数据...");
                            let get_meta_request = open_lark::service::drive::v1::folder::GetFolderMetaRequest::new(data.token.clone());
                            
                            match client.drive.v1.folder.get_folder_meta(get_meta_request, None).await {
                                Ok(meta_response) => {
                                    if let Some(meta_data) = meta_response.data {
                                        println!("  - 验证成功，文件夹详细信息:");
                                        println!("    - ID: {}", meta_data.id);
                                        println!("    - 创建时间: {}", meta_data.create_time);
                                        println!("    - 拥有者ID: {}", meta_data.owner_id);
                                    }
                                }
                                Err(e) => {
                                    eprintln!("验证文件夹创建失败: {}", e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("创建文件夹失败: {}", e);
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