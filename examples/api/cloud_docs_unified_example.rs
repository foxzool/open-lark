//! 云文档服务统一示例
//!
//! 演示docx、drive和cloud-docs功能标志的统一使用
//! 展示所有别名都能正常工作

use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 云文档服务统一示例");
    println!("演示docx、drive和cloud-docs功能标志的统一使用\n");

    // 创建客户端，启用云文档相关功能
    let client = LarkClient::builder("your_app_id", "your_app_secret")
        .with_enable_token_cache(true)
        .build();

    println!("✅ 功能标志配置检查：");

    // 检查不同功能标志是否都可用
    #[cfg(feature = "cloud-docs")]
    {
        println!("  ✓ cloud-docs 功能已启用");

        // 使用cloud-docs功能访问文档服务
        println!("  📄 通过cloud-docs访问文档: {:?}", client.cloud_docs);
    }

    #[cfg(feature = "docx")]
    {
        println!("  ✓ docx 功能已启用 (别名映射到cloud-docs)");

        // docx功能标志现在映射到cloud-docs
        println!("  📄 通过docx别名访问文档: {:?}", client.cloud_docs);
    }

    #[cfg(feature = "drive")]
    {
        println!("  ✓ drive 功能已启用 (别名映射到cloud-docs)");

        // drive功能标志现在映射到cloud-docs
        println!("  📄 通过drive别名访问文档: {:?}", client.cloud_docs);
    }

    println!("\n🔧 API使用示例：");
    println!("所有功能标志都可以访问相同的云文档服务实例");

    // 示例：获取文档列表（所有功能标志都可以使用）
    #[cfg(any(feature = "cloud-docs", feature = "docx", feature = "drive"))]
    {
        println!("\n📋 云文档操作示例：");
        println!("1. 获取文件列表: client.cloud_docs.v1.drive.file_list(...)");
        println!("2. 创建文档: client.cloud_docs.v1.docx.create_document(...)");
        println!("3. 上传文件: client.cloud_docs.v1.drive.upload_file(...)");
        println!("4. 文档评论: client.cloud_docs.v1.comments.add_comment(...)");
    }

    println!("\n💡 功能标志别名说明：");
    println!("- cloud-docs: 主要功能标志，提供完整的云文档功能");
    println!("- docx: 文档相关API的别名，映射到cloud-docs");
    println!("- drive: 云盘相关API的别名，映射到cloud-docs");
    println!("- 所有别名功能完全兼容，提供相同的服务实例");

    println!("\n🎯 推荐使用方式：");
    println!("1. 新项目：直接使用 cloud-docs 功能标志");
    println!("2. 现有项目：可继续使用 docx/drive，完全兼容");
    println!("3. 渐进迁移：先添加 cloud-docs，再逐步替换 docx/drive");

    println!("\n✅ 示例运行完成！");

    Ok(())
}
