// 多服务增强Builder模式综合演示
//
// 这个示例展示了跨多个服务的增强Builder模式用法，包括：
// - IM消息服务 (发送消息、查询历史、群组管理)
// - Drive云空间服务 (文件管理)
// - Bitable多维表格服务 (记录查询)
//
// 运行方式：
// cargo run --example multi_service_enhanced_builder
//
// 环境变量要求：
// APP_ID=your_app_id
// APP_SECRET=your_app_secret

use open_lark::prelude::*;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取环境变量
    dotenv::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("请设置 APP_ID 环境变量");
    let app_secret = std::env::var("APP_SECRET").expect("请设置 APP_SECRET 环境变量");

    // 创建Lark客户端
    let _client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    println!("🚀 多服务增强Builder模式综合演示");
    println!("{}", "=".repeat(80));

    // =============================================================================
    // 🗨️ IM消息服务演示
    // =============================================================================

    println!("\n📱 IM消息服务 - 增强Builder模式演示");
    println!("{}", "-".repeat(50));

    // 演示1: 发送消息 - CreateMessageRequestBuilder
    println!("\n📤 示例1: 发送消息");
    println!("增强Builder调用方式:");
    println!("```rust");
    println!("let body = CreateMessageRequestBody::builder()");
    println!("    .receive_id(\"ou_user_id_here\")");
    println!("    .msg_type(\"text\")");
    println!("    .content(json!({{\"text\": \"Hello from enhanced builder!\"}}).to_string())");
    println!("    .build();");
    println!();
    println!("let result = CreateMessageRequest::builder()");
    println!("    .receive_id_type(\"open_id\")");
    println!("    .request_body(body)");
    println!("    .execute(&client.im.v1.message)  // 直接执行！");
    println!("    .await?;");
    println!("```");

    // 演示2: 查询消息历史 - ListMessageRequestBuilder
    println!("\n📋 示例2: 查询消息历史");
    println!("增强Builder调用方式:");
    println!("```rust");
    println!("let messages = ListMessageRequest::builder()");
    println!("    .container_id_type(\"chat\")");
    println!("    .container_id(\"oc_chat_id_here\")");
    println!("    .start_time(1609296809)");
    println!("    .end_time(1609383209)");
    println!("    .sort_type(\"ByCreateTimeDesc\")");
    println!("    .page_size(20)");
    println!("    .execute(&client.im.v1.message)  // 直接执行！");
    println!("    .await?;");
    println!("```");

    // 演示3: 获取群组列表 - ListChatRequestBuilder
    println!("\n👥 示例3: 获取群组列表");
    println!("增强Builder调用方式:");
    println!("```rust");
    println!("let chats = ListChatRequest::builder()");
    println!("    .user_id_type(\"open_id\")");
    println!("    .sort_type(\"ByCreateTimeAsc\")");
    println!("    .page_size(50)");
    println!("    .execute(&client.im.v1.chats)  // 直接执行！");
    println!("    .await?;");
    println!("```");

    // =============================================================================
    // 💾 Drive云空间服务演示
    // =============================================================================

    println!("\n\n💾 Drive云空间服务 - 增强Builder模式演示");
    println!("{}", "-".repeat(50));

    // 演示4: 列出文件夹内容 - ListFilesRequestBuilder
    println!("\n📁 示例4: 列出文件夹内容");
    println!("增强Builder调用方式:");
    println!("```rust");
    println!("let files = ListFilesRequest::builder()");
    println!("    .folder_token(\"folder_token_here\")");
    println!("    .page_size(20)");
    println!("    .order_by(\"modified_time\")");
    println!("    .direction(\"DESC\")");
    println!("    .execute(&client.drive.v1.folder)  // 直接执行！");
    println!("    .await?;");
    println!("```");

    // 演示5: 文件上传 - UploadAllRequestBuilder
    println!("\n📤 示例5: 上传文件");
    println!("增强Builder调用方式:");
    println!("```rust");
    println!("let file_data = \"File content here\".as_bytes().to_vec();");
    println!("let result = UploadAllRequest::builder()");
    println!("    .file_name(\"enhanced_upload.txt\")");
    println!("    .parent_type(\"explorer\")");
    println!("    .parent_node(\"folder_token_here\")");
    println!("    .size(file_data.len() as i32)");
    println!("    .file(file_data)");
    println!("    .execute(&client.drive.v1.files)  // 直接执行！");
    println!("    .await?;");
    println!("```");

    // 演示6: 文件下载 - DownloadRequestBuilder
    println!("\n📥 示例6: 下载文件");
    println!("增强Builder调用方式:");
    println!("```rust");
    println!("let file_data = DownloadRequest::builder()");
    println!("    .file_token(\"file_token_here\")");
    println!("    .execute(&client.drive.v1.files)  // 直接执行！");
    println!("    .await?;");
    println!("```");

    // =============================================================================
    // 📊 Bitable多维表格服务演示
    // =============================================================================

    println!("\n\n📊 Bitable多维表格服务 - 增强Builder模式演示");
    println!("{}", "-".repeat(50));

    // 演示7: 查询记录 - SearchRecordRequestBuilder
    println!("\n🔍 示例7: 查询表格记录");
    println!("增强Builder调用方式:");
    println!("```rust");
    println!("use open_lark::service::cloud_docs::bitable::v1::app_table_record::*;");
    println!();
    println!("// 构建筛选条件");
    println!("let filter = FilterInfo::and(vec![");
    println!("    FilterCondition::equals(\"Name\", \"John\"),");
    println!("    FilterCondition::greater_than(\"Age\", \"18\"),");
    println!("]);");
    println!();
    println!("let records = SearchRecordRequest::builder()");
    println!("    .app_token(\"bascnCMII2ORuEjIDXvVecCKNEc\")");
    println!("    .table_id(\"tblsRc9GRRXKqhvW\")");
    println!("    .view_id(\"vewJHSwJHD\")");
    println!("    .filter(filter)");
    println!("    .page_size(50)");
    println!("    .execute(&client.bitable.v1.app_table_record)  // 直接执行！");
    println!("    .await?;");
    println!("```");

    // 演示8: 批量获取记录 - BatchGetRecordRequestBuilder
    println!("\n📋 示例8: 批量获取记录");
    println!("增强Builder调用方式:");
    println!("```rust");
    println!("let records = BatchGetRecordRequest::builder()");
    println!("    .app_token(\"bascnCMII2ORuEjIDXvVecCKNEc\")");
    println!("    .table_id(\"tblsRc9GRRXKqhvW\")");
    println!("    .add_record_id(\"recpCsf4QPs\")");
    println!("    .add_record_id(\"recpCsf4QXy\")");
    println!("    .add_record_id(\"recpCsf4QZz\")");
    println!("    .automatic(true)");
    println!("    .execute(&client.bitable.v1.app_table_record)  // 直接执行！");
    println!("    .await?;");
    println!("```");

    // =============================================================================
    // 📈 对比总结
    // =============================================================================

    println!("\n\n📈 增强Builder模式总结");
    println!("{}", "=".repeat(80));

    println!("\n🎯 核心改进:");
    println!("1. ✨ 代码简化: 从 5步调用 → 4步调用");
    println!("2. 🔗 方法链更流畅: .execute(&service).await 一步完成");
    println!("3. 🛡️  类型安全: 编译时保证参数正确性");
    println!("4. 🔄 向后兼容: 现有代码无需修改");
    println!("5. 💡 IDE友好: 更好的自动完成支持");

    println!("\n📊 已增强的服务统计:");
    println!("- 📱 IM服务: 3个核心Builder (消息发送、历史查询、群组列表)");
    println!("- 💾 Drive服务: 3个核心Builder (文件夹列表、上传、下载)");
    println!("- 📊 Bitable服务: 2个核心Builder (记录查询、批量获取)");
    println!("- 📈 总计: 8个增强Builder覆盖3大核心服务");

    println!("\n🔄 调用模式对比:");
    println!("传统方式:");
    println!("```rust");
    println!("let req = Request::builder().params().build();");
    println!("let result = service.method(req, None).await?;");
    println!("```");
    println!();
    println!("增强方式:");
    println!("```rust");
    println!("let result = Request::builder().params().execute(&service).await?;");
    println!("```");

    println!("\n🚀 技术特性:");
    println!("- ⚡ 零性能开销 - 纯语法糖实现");
    println!("- 🏗️ 保持架构纯粹性 - 不破坏Command Pattern");
    println!("- 🔧 易于维护 - 无需复杂代码生成");
    println!("- 📦 零破坏性 - 完全向后兼容");

    println!("\n✅ 现在您可以在所有支持的服务中使用这种更流畅的API调用方式！");

    Ok(())
}
