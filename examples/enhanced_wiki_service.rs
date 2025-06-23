use dotenv::dotenv;
use open_lark::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    // 创建客户端
    let client = LarkClient::builder(app_id, app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    println!("{}", "=".repeat(60));
    println!("🔍 增强Builder模式 - Wiki服务演示");
    println!("{}", "=".repeat(60));

    // 演示场景：知识空间管理和搜索
    demo_wiki_operations(&client).await?;

    Ok(())
}

async fn demo_wiki_operations(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📚 Wiki服务 - 增强Builder模式演示");
    println!("{}", "-".repeat(50));

    // 场景1：搜索Wiki内容
    println!("\n🔍 1. 搜索Wiki内容");
    println!("传统方式:");
    println!("  let req = SearchWikiRequest::builder().query(\"项目文档\").build();");
    println!("  let result = client.wiki.v2.search_wiki(req, None).await?;");

    println!("\n增强方式:");
    println!("  let result = SearchWikiRequest::builder()");
    println!("      .query(\"项目文档\")");
    println!("      .page_size(20)");
    println!("      .search_all_spaces()");
    println!("      .execute(&client.wiki.v2).await?;");

    // 注意：实际执行时需要有效的token和权限
    println!("\n✅ 搜索操作准备就绪");

    // 场景2：创建知识空间节点
    println!("\n📝 2. 创建知识空间节点");
    println!("传统方式:");
    println!("  let req = CreateSpaceNodeRequest::builder()");
    println!("      .space_id(\"spcxxxxxx\")");
    println!("      .as_doc()");
    println!("      .title(\"新项目文档\")");
    println!("      .build();");
    println!("  let result = client.wiki.v2.space_node.create(req, None).await?;");

    println!("\n增强方式:");
    println!("  let result = CreateSpaceNodeRequest::builder()");
    println!("      .space_id(\"spcxxxxxx\")");
    println!("      .as_doc()");
    println!("      .title(\"新项目文档\")");
    println!("      .as_origin_node()");
    println!("      .execute(&client.wiki.v2.space_node).await?;");

    println!("\n✅ 节点创建操作准备就绪");

    // 场景3：获取空间节点列表
    println!("\n📋 3. 获取空间节点列表");
    println!("传统方式:");
    println!("  let req = ListSpaceNodeRequest::builder()");
    println!("      .space_id(\"spcxxxxxx\")");
    println!("      .page_size(50)");
    println!("      .root_nodes()");
    println!("      .build();");
    println!("  let result = client.wiki.v2.space_node.list(req, None).await?;");

    println!("\n增强方式:");
    println!("  let result = ListSpaceNodeRequest::builder()");
    println!("      .space_id(\"spcxxxxxx\")");
    println!("      .page_size(50)");
    println!("      .root_nodes()");
    println!("      .execute(&client.wiki.v2.space_node).await?;");

    println!("\n✅ 节点列表操作准备就绪");

    // 场景4：添加空间成员
    println!("\n👥 4. 添加空间成员");
    println!("传统方式:");
    println!("  let req = CreateSpaceMemberRequest::builder()");
    println!("      .space_id(\"spcxxxxxx\")");
    println!("      .member_type(\"user\")");
    println!("      .member_id(\"ou_xxxxxx\")");
    println!("      .as_editor()");
    println!("      .build();");
    println!("  let result = client.wiki.v2.space_member.create(req, None).await?;");

    println!("\n增强方式:");
    println!("  let result = CreateSpaceMemberRequest::builder()");
    println!("      .space_id(\"spcxxxxxx\")");
    println!("      .member_type(\"user\")");
    println!("      .member_id(\"ou_xxxxxx\")");
    println!("      .as_editor()");
    println!("      .execute(&client.wiki.v2.space_member).await?;");

    println!("\n✅ 成员添加操作准备就绪");

    // 场景5：获取任务状态
    println!("\n⏱️ 5. 获取任务状态");
    println!("传统方式:");
    println!("  let req = GetTaskRequest::builder()");
    println!("      .task_id(\"taskxxxxxx\")");
    println!("      .build();");
    println!("  let result = client.wiki.v2.task.get(req, None).await?;");

    println!("\n增强方式:");
    println!("  let result = GetTaskRequest::builder()");
    println!("      .task_id(\"taskxxxxxx\")");
    println!("      .execute(&client.wiki.v2.task).await?;");

    println!("\n✅ 任务状态查询操作准备就绪");

    // 总结增强效果
    println!("\n{}", "=".repeat(60));
    println!("📊 增强效果总结:");
    println!("{}", "=".repeat(60));
    println!("✨ 减少了代码步骤：从 3-4 步减少到 2 步");
    println!("🔗 更流畅的API：直接链式调用到执行");
    println!("🛡️ 保持兼容性：传统方式依然可用");
    println!("🎯 类型安全：编译时确保正确的service类型");
    println!("🚀 更好的开发体验：接近现代SDK的使用模式");

    println!("\n📈 当前增强进度：");
    println!("• ✅ Drive服务 (3个builders)");
    println!("• ✅ IM服务 (3个builders)");
    println!("• ✅ Bitable服务 (2个builders)");
    println!("• ✅ Search服务 (1个builder)");
    println!("• ✅ Sheets服务 (2个builders)");
    println!("• ✅ Wiki服务 (5个builders)");
    println!("📊 总计：16个builders已增强");

    Ok(())
}
