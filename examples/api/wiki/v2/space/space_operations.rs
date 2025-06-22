use open_lark::{
    prelude::*,
    service::wiki::v2::space::{
        create::CreateSpaceRequest,
        get::GetSpaceRequest,
        list::ListSpaceRequest,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    // 1. 获取知识空间列表
    println!("1. 获取知识空间列表...");

    let list_request = ListSpaceRequest::builder().page_size(20).build();

    match client.wiki.v2.space.list(list_request, None).await {
        Ok(list_response) => {
            println!("知识空间列表:");
            if list_response.data.items.is_empty() {
                println!("  暂无知识空间");
            } else {
                for space in &list_response.data.items {
                    println!("  空间信息:");
                    println!("    - ID: {}", space.space_id);
                    println!("    - 名称: {}", space.name);
                    if let Some(desc) = &space.description {
                        println!("    - 描述: {}", desc);
                    }
                    if let Some(space_type) = &space.space_type {
                        println!("    - 类型: {}", space_type);
                    }
                    if let Some(visibility) = &space.visibility {
                        println!("    - 可见性: {}", visibility);
                    }
                    println!();
                }
            }
            println!("是否有更多: {}", list_response.data.has_more);

            // 如果有知识空间，获取第一个空间的详细信息
            if let Some(first_space) = list_response.data.items.first() {
                println!("\n2. 获取知识空间详细信息...");
                let space_id = &first_space.space_id;

                let get_request = GetSpaceRequest::builder().space_id(space_id).build();

                match client.wiki.v2.space.get(get_request, None).await {
                    Ok(get_response) => {
                        let space_info = &get_response.data.space;
                        println!("知识空间详细信息:");
                        println!("  - ID: {}", space_info.space_id);
                        println!("  - 名称: {}", space_info.name);
                        if let Some(desc) = &space_info.description {
                            println!("  - 描述: {}", desc);
                        }
                        if let Some(space_type) = &space_info.space_type {
                            println!("  - 类型: {}", space_type);
                        }
                        if let Some(visibility) = &space_info.visibility {
                            println!("  - 可见性: {}", visibility);
                        }
                        if let Some(create_time) = space_info.create_time {
                            println!("  - 创建时间: {}", create_time);
                        }
                        if let Some(update_time) = space_info.update_time {
                            println!("  - 更新时间: {}", update_time);
                        }
                    }
                    Err(e) => {
                        println!("获取知识空间详细信息失败: {:?}", e);
                    }
                }
            } else {
                println!("\n没有找到知识空间，跳过详细信息获取");
            }
        }
        Err(e) => {
            println!("获取知识空间列表失败: {:?}", e);
        }
    }

    // 3. 创建新的知识空间（仅作为演示，实际使用时请谨慎）
    println!("\n3. 创建知识空间演示...");

    let create_request = CreateSpaceRequest::builder()
        .name("API测试知识空间")
        .description("这是通过API创建的测试知识空间")
        .build();

    match client.wiki.v2.space.create(create_request, None).await {
        Ok(create_response) => {
            let created_space = &create_response.data.space;
            println!("知识空间创建成功:");
            println!("  - ID: {}", created_space.space_id);
            println!("  - 名称: {}", created_space.name);
            if let Some(desc) = &created_space.description {
                println!("  - 描述: {}", desc);
            }
            if let Some(space_type) = &created_space.space_type {
                println!("  - 类型: {}", space_type);
            }
            if let Some(visibility) = &created_space.visibility {
                println!("  - 可见性: {}", visibility);
            }

            // 验证创建结果 - 再次获取空间信息
            println!("\n4. 验证创建结果...");

            let verify_request = GetSpaceRequest::builder()
                .space_id(&created_space.space_id)
                .build();

            match client.wiki.v2.space.get(verify_request, None).await {
                Ok(verify_response) => {
                    println!("验证成功，空间已正确创建:");
                    println!("  - 名称: {}", verify_response.data.space.name);
                    println!("  - 状态: 正常");
                }
                Err(e) => {
                    println!("验证失败: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("创建知识空间失败: {:?}", e);
            println!("注意：可能需要相应的权限才能创建知识空间");
        }
    }

    println!("\n知识空间操作示例完成！");
    println!("提示：");
    println!("- 创建知识空间需要相应的权限");
    println!("- 知识空间的类型和可见性由系统根据用户权限自动设置");
    println!("- 可以通过知识空间ID进行后续的节点和内容管理");

    Ok(())
}
