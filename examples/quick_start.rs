//! # OpenLark 快速启动示例
//!
//! 这个示例展示了如何快速开始使用 OpenLark SDK。
//! 根据您的需求选择相应的 feature 组合。

#[cfg(feature = "communication-core")]
use open_lark::prelude::*;

#[cfg(feature = "openlark-client")]
use open_lark::LarkClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    env_logger::init();

    println!("🚀 OpenLark 快速启动示例");
    println!("==========================");

    #[cfg(feature = "communication-core")]
    {
        println!("✅ 核心通讯功能已启用");
        demonstrate_core_features().await?;
    }

    #[cfg(any(feature = "docs-suite", feature = "docs-collaboration"))]
    {
        println!("✅ 文档协作功能已启用");
        demonstrate_docs_features().await?;
    }

    #[cfg(feature = "hr-suite")]
    {
        println!("✅ 人力资源功能已启用");
        demonstrate_hr_features().await?;
    }

    #[cfg(feature = "ai-suite")]
    {
        println!("✅ AI 智能功能已启用");
        demonstrate_ai_features().await?;
    }

    #[cfg(feature = "websocket")]
    {
        println!("✅ WebSocket 功能已启用");
        println!("💡 提示：使用 WebSocket 需要额外配置");
    }

    #[cfg(feature = "otel")]
    {
        println!("✅ OpenTelemetry 可观测性已启用");
        println!("💡 提示：记得配置 OTLP 端点");
    }

    println!("\n🎉 示例运行完成！");
    println!("📚 更多功能请查看：docs/feature-guide.md");

    Ok(())
}

#[cfg(feature = "communication-core")]
async fn demonstrate_core_features() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📱 核心通讯功能演示");
    println!("-------------------");

    // 注意：这里使用模拟数据，实际使用时请配置真实的 app_id 和 app_secret
    println!("💬 发送文本消息示例：");
    println!("```rust");
    println!("let response = client");
    println!("    .communication()");
    println!("    .im()");
    println!("    .v1()");
    println!("    .message()");
    println!("    .send_text()");
    println!("    .receive_id_type(\"open_id\")");
    println!("    .receive_id(\"user_open_id\")");
    println!("    .content(\"Hello from OpenLark!\")");
    println!("    .send()");
    println!("    .await?;");
    println!("```");

    println!("\n👥 联系人查询示例：");
    println!("```rust");
    println!("let response = client");
    println!("    .communication()");
    println!("    .contact()");
    println!("    .v3()");
    println!("    .user()");
    println!("    .get()");
    println!("    .user_id(\"user_open_id\")");
    println!("    .user_id_type(\"open_id\")");
    println!("    .send()");
    println!("    .await?;");
    println!("```");

    Ok(())
}

#[cfg(any(feature = "docs-suite", feature = "docs-collaboration"))]
async fn demonstrate_docs_features() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📄 文档协作功能演示");
    println!("-------------------");

    println!("📊 表格操作示例：");
    println!("```rust");
    println!("let response = client");
    println!("    .docs()");
    println!("    .sheets()");
    println!("    .v3()");
    println!("    .range_read()");
    println!("    .range(\"A1:C10\")");
    println!("    .value_render_option(\"DisplayValue\")");
    println!("    .send()");
    println!("    .await?;");
    println!("```");

    println!("\n📝 文档上传示例：");
    println!("```rust");
    println!("let response = client");
    println!("    .docs()");
    println!("    .drive()");
    println!("    .v1()");
    println!("    .file()");
    println!("    .upload_all()");
    println!("    .parent_type(\"open_drive\")");
    println!("    .parent_id(\"folder_id\")");
    println!("    .file_path(\"./example.txt\")");
    println!("    .send()");
    println!("    .await?;");
    println!("```");

    Ok(())
}

#[cfg(feature = "hr-suite")]
async fn demonstrate_hr_features() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n👥 人力资源功能演示");
    println!("-------------------");

    println!("⏰ 考勤查询示例：");
    println!("```rust");
    println!("let response = client");
    println!("    .hr()");
    println!("    .attendance()");
    println!("    .v1()");
    println!("    .user_list()");
    println!("    .check_type(\"OnDuty\")");
    println!("    .start_time(\"2024-01-01T00:00:00\")");
    println!("    .end_time(\"2024-01-31T23:59:59\")");
    println!("    .send()");
    println!("    .await?;");
    println!("```");

    println!("\n🎯 OKR 管理示例：");
    println!("```rust");
    println!("let response = client");
    println!("    .hr()");
    println!("    .okr()");
    println!("    .v1()");
    println!("    .objective_list()");
    println!("    .user_id_type(\"open_id\")");
    println!("    .owner_id_type(\"user_id\")");
    println!("    .owner_id(\"user_open_id\")");
    println!("    .send()");
    println!("    .await?;");
    println!("```");

    Ok(())
}

#[cfg(feature = "ai-suite")]
async fn demonstrate_ai_features() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🤖 AI 智能功能演示");
    println!("-------------------");

    println!("🧠 AI 助手示例：");
    println!("```rust");
    println!("let response = client");
    println!("    .ai()");
    println!("    .aily()");
    println!("    .v1()");
    println!("    .chat()");
    println!("    .create()");
    println!("    .question(\"请帮我总结这段文本\")");
    println!("    .send()");
    println!("    .await?;");
    println!("```");

    println!("\n📋 文本分类示例：");
    println!("```rust");
    println!("let response = client");
    println!("    .ai()");
    println!("    .lingo()");
    println!("    .v1()");
    println!("    .classify()");
    println!("    .content(\"这是一段需要分类的文本\")");
    println!("    .classify_type(\"custom_model\")");
    println!("    .send()");
    println!("    .await?;");
    println!("```");

    Ok(())
}
