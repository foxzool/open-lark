use open_lark::{prelude::*, service::permission::public_v1::password::delete::DeletePasswordRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").unwrap();
    let app_secret = std::env::var("APP_SECRET").unwrap();

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    // 关闭文档密码保护
    let request = DeletePasswordRequest::builder()
        .token("doccnxxxxxx")
        .as_doc()
        .build();

    match client.permission.delete_password(&request, None).await {
        Ok(response) => {
            println!("密码保护关闭操作完成");
            println!("响应状态码: {}", response.code());
            println!("响应消息: {}", response.msg());
            
            if let Some(data) = response.data {
                println!("删除操作成功: {:?}", data);
            } else {
                println!("删除操作完成，但没有返回详细数据");
            }

        }
        Err(e) => {
            eprintln!("关闭密码保护失败: {:?}", e);
        }
    }

    // 关闭电子表格密码保护
    println!("\n--- 关闭电子表格密码保护 ---");
    let sheet_request = DeletePasswordRequest::builder()
        .token("shtcnxxxxxx")
        .as_sheet()
        .build();

    match client.permission.delete_password(&sheet_request, None).await {
        Ok(response) => {
            println!("电子表格密码保护关闭操作完成");
            println!("响应状态码: {}", response.code());
            println!("响应消息: {}", response.msg());
        }
        Err(e) => {
            eprintln!("关闭电子表格密码保护失败: {:?}", e);
        }
    }

    // 关闭多维表格密码保护
    println!("\n--- 关闭多维表格密码保护 ---");
    let bitable_request = DeletePasswordRequest::builder()
        .token("bblcnxxxxxx")
        .as_bitable()
        .build();

    match client.permission.delete_password(&bitable_request, None).await
    {
        Ok(response) => {
            println!("多维表格密码保护关闭操作完成");
            println!("响应状态码: {}", response.code());
            println!("响应消息: {}", response.msg());
        }
        Err(e) => {
            eprintln!("关闭多维表格密码保护失败: {:?}", e);
        }
    }

    // 关闭知识库密码保护
    println!("\n--- 关闭知识库密码保护 ---");
    let wiki_request = DeletePasswordRequest::builder()
        .token("wikicnxxxxxx")
        .as_wiki()
        .build();

    match client.permission.delete_password(&wiki_request, None).await {
        Ok(response) => {
            println!("知识库密码保护关闭操作完成");
            println!("响应状态码: {}", response.code());
            println!("响应消息: {}", response.msg());
        }
        Err(e) => {
            eprintln!("关闭知识库密码保护失败: {:?}", e);
        }
    }

    Ok(())
}
