use dotenvy::dotenv;
use open_lark::{prelude::*, service::drive::v1::folder::ListFilesRequest};
use std::env;

/// 获取文件夹中的文件清单示例
///
/// 该接口用于根据文件夹的token获取文件夹中的文件清单。
///
/// <https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/list>
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

    println!("开始获取文件夹中的文件清单...");

    // 首先获取根目录的token，用于后续查询
    match client.drive.v1.folder.get_root_folder_meta(Some(option.clone())).await {
        Ok(root_response) => {
            if let Some(root_data) = root_response.data {
                let folder_token = root_data.token;
                println!("获取到根目录token: {}", folder_token);

                // 构建获取文件清单的请求
                let request = ListFilesRequest::builder(folder_token)
                    .page_size(10) // 限制返回10个文件
                    .order_by("created_time".to_string()) // 按创建时间排序
                    .direction("DESC".to_string()) // 降序排列
                    .build();

                // 调用API获取文件清单
                match client.drive.v1.folder.list_files(request, Some(option)).await {
                    Ok(response) => {
                        println!("API调用成功");
                        println!("响应状态码: {}", response.code());
                        println!("响应消息: {}", response.msg());

                        if let Some(data) = response.data {
                            println!("文件清单信息:");
                            println!("  - 是否还有更多文件: {}", data.has_more);
                            if let Some(page_token) = &data.page_token {
                                println!("  - 下一页标记: {}", page_token);
                            }
                            println!("  - 文件数量: {}", data.files.len());

                            for (i, file) in data.files.iter().enumerate() {
                                println!("  文件 {}:", i + 1);
                                println!("    - Token: {}", file.token);
                                println!("    - 名称: {}", file.name);
                                println!("    - 类型: {}", file.file_type);

                                if let Some(size) = file.size {
                                    println!("    - 大小: {} 字节", size);
                                }
                                if let Some(mime_type) = &file.mime_type {
                                    println!("    - MIME类型: {}", mime_type);
                                }
                                if let Some(created_time) = &file.created_time {
                                    println!("    - 创建时间: {}", created_time);
                                }
                                if let Some(url) = &file.url {
                                    println!("    - 链接: {}", url);
                                }
                                println!();
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("获取文件清单失败: {}", e);
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
