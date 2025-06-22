use open_lark::{
    prelude::*,
    service::wiki::v2::space_node::{
        copy::CopySpaceNodeRequest,
        create::CreateSpaceNodeRequest,
        get::GetSpaceNodeRequest,
        list::ListSpaceNodeRequest,
        r#move::MoveSpaceNodeRequest,
        update_title::UpdateSpaceNodeTitleRequest,
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

    // 配置必要参数
    let space_id = "spcxxxxxx"; // 需要一个真实的知识空间ID

    // 1. 创建文档节点
    println!("1. 创建文档节点...");

    let create_request = CreateSpaceNodeRequest::builder()
        .space_id(space_id)
        .as_doc()
        .title("测试文档")
        .as_origin_node()
        .build();

    let mut node_token = String::new();

    match client.wiki.v2.space_node.create(create_request, None).await {
        Ok(create_response) => {
            let node = &create_response.node;
            node_token = node.node_token.clone();
            println!("文档节点创建成功:");
            println!("  - 节点Token: {}", node.node_token);
            println!("  - 文档类型: {}", node.obj_type);
            if let Some(title) = &node.title {
                println!("  - 文档标题: {}", title);
            }
            if let Some(obj_token) = &node.obj_token {
                println!("  - 文档Token: {}", obj_token);
            }
        }
        Err(e) => {
            println!("创建文档节点失败: {:?}", e);
            println!("可能的原因：");
            println!("- 空间ID不存在或无效");
            println!("- 没有创建权限");
            println!("- 标题重复或包含特殊字符");
            return Ok(());
        }
    }

    // 2. 获取节点信息
    println!("\n2. 获取节点信息...");

    let get_request = GetSpaceNodeRequest::builder()
        .space_id(space_id)
        .node_token(&node_token)
        .build();

    match client.wiki.v2.space_node.get(get_request, None).await {
        Ok(get_response) => {
            let node = &get_response.node;
            println!("节点信息获取成功:");
            println!("  - 节点Token: {}", node.node_token);
            println!("  - 文档类型: {}", node.obj_type);
            if let Some(title) = &node.title {
                println!("  - 文档标题: {}", title);
            }
            if let Some(create_time) = &node.node_create_time {
                println!("  - 创建时间: {}", create_time);
            }
            if let Some(has_child) = node.has_child {
                println!("  - 是否有子节点: {}", has_child);
            }
        }
        Err(e) => {
            println!("获取节点信息失败: {:?}", e);
        }
    }

    // 3. 获取根节点列表
    println!("\n3. 获取根节点列表...");

    let list_request = ListSpaceNodeRequest::builder()
        .space_id(space_id)
        .root_nodes()
        .page_size(20)
        .build();

    match client.wiki.v2.space_node.list(list_request, None).await {
        Ok(list_response) => {
            println!("根节点列表获取成功:");
            println!("  - 节点数量: {}", list_response.items.len());
            println!("  - 是否有更多: {}", list_response.has_more);

            for (index, item) in list_response.items.iter().enumerate() {
                println!("  节点 {}:", index + 1);
                println!("    - Token: {}", item.node_token);
                println!("    - 类型: {}", item.obj_type);
                if let Some(title) = &item.title {
                    println!("    - 标题: {}", title);
                }
                if let Some(has_child) = item.has_child {
                    println!("    - 有子节点: {}", has_child);
                }
            }
        }
        Err(e) => {
            println!("获取根节点列表失败: {:?}", e);
        }
    }

    // 4. 更新节点标题
    println!("\n4. 更新节点标题...");

    let update_title_request = UpdateSpaceNodeTitleRequest::builder()
        .space_id(space_id)
        .node_token(&node_token)
        .title("更新后的文档标题")
        .build();

    match client
        .wiki
        .v2
        .space_node
        .update_title(update_title_request, None)
        .await
    {
        Ok(update_response) => {
            let node = &update_response.node;
            println!("节点标题更新成功:");
            println!("  - 节点Token: {}", node.node_token);
            if let Some(title) = &node.title {
                println!("  - 新标题: {}", title);
            }
        }
        Err(e) => {
            println!("更新节点标题失败: {:?}", e);
        }
    }

    // 5. 复制节点
    println!("\n5. 复制节点...");

    let copy_request = CopySpaceNodeRequest::builder()
        .space_id(space_id)
        .node_token(&node_token)
        .to_root()
        .title("复制的文档")
        .build();

    let mut copied_node_token = String::new();

    match client.wiki.v2.space_node.copy(copy_request, None).await {
        Ok(copy_response) => {
            let node = &copy_response.node;
            copied_node_token = node.node_token.clone();
            println!("节点复制成功:");
            println!("  - 原节点Token: {}", node_token);
            println!("  - 新节点Token: {}", node.node_token);
            if let Some(title) = &node.title {
                println!("  - 新标题: {}", title);
            }
        }
        Err(e) => {
            println!("复制节点失败: {:?}", e);
        }
    }

    // 6. 移动节点（如果有父节点的话）
    println!("\n6. 移动节点到根目录...");

    let move_request = MoveSpaceNodeRequest::builder()
        .space_id(space_id)
        .node_token(&copied_node_token)
        .to_root()
        .append_to_end()
        .build();

    match client.wiki.v2.space_node.r#move(move_request, None).await {
        Ok(move_response) => {
            let node = &move_response.node;
            println!("节点移动成功:");
            println!("  - 节点Token: {}", node.node_token);
            if let Some(parent_token) = &node.parent_node_token {
                println!("  - 父节点Token: {}", parent_token);
            } else {
                println!("  - 已移动到根目录");
            }
        }
        Err(e) => {
            println!("移动节点失败: {:?}", e);
        }
    }

    // 7. 创建不同类型的节点示例
    println!("\n7. 创建不同类型的节点...");

    // 创建电子表格节点
    let sheet_request = CreateSpaceNodeRequest::builder()
        .space_id(space_id)
        .as_sheet()
        .title("测试电子表格")
        .build();

    match client.wiki.v2.space_node.create(sheet_request, None).await {
        Ok(response) => {
            println!("电子表格节点创建成功: {}", response.node.node_token);
        }
        Err(e) => {
            println!("创建电子表格节点失败: {:?}", e);
        }
    }

    // 创建思维笔记节点
    let mindnote_request = CreateSpaceNodeRequest::builder()
        .space_id(space_id)
        .as_mindnote()
        .title("测试思维笔记")
        .build();

    match client
        .wiki
        .v2
        .space_node
        .create(mindnote_request, None)
        .await
    {
        Ok(response) => {
            println!("思维笔记节点创建成功: {}", response.node.node_token);
        }
        Err(e) => {
            println!("创建思维笔记节点失败: {:?}", e);
        }
    }

    // 创建多维表格节点
    let bitable_request = CreateSpaceNodeRequest::builder()
        .space_id(space_id)
        .as_bitable()
        .title("测试多维表格")
        .build();

    match client
        .wiki
        .v2
        .space_node
        .create(bitable_request, None)
        .await
    {
        Ok(response) => {
            println!("多维表格节点创建成功: {}", response.node.node_token);
        }
        Err(e) => {
            println!("创建多维表格节点失败: {:?}", e);
        }
    }

    println!("\n知识空间节点操作示例完成！");
    println!("提示：");
    println!("- 需要知识空间的编辑权限才能创建和操作节点");
    println!("- 支持文档(doc)、电子表格(sheet)、思维笔记(mindnote)、多维表格(bitable)四种类型");
    println!("- 节点可以在同一空间内移动，也可以复制到其他空间");
    println!("- 快捷方式节点指向原始文档，便于组织管理");
    println!("- 所有操作都支持异步执行，建议添加适当的错误处理");

    Ok(())
}
