//! Contact服务示例 - 获取企业自定义用户字段
//!
//! 演示如何使用contact服务的获取企业自定义用户字段功能

use open_lark::prelude::*;
use open_lark::core::config::{Config, ConfigBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    env_logger::init();

    // 创建客户端配置
    let config = Config::builder()
        .app_id("your_app_id")
        .app_secret("your_app_secret")
        .build();

    // 创建客户端（需要contact功能标志）
    #[cfg(feature = "contact")]
    {
        let client = LarkClient::new(config);

        println!("Contact服务 - 获取企业自定义用户字段 API已修复完成!");
        println!();
        println!("API信息:");
        println!("- 接口: GET /open-apis/contact/v3/custom_attrs");
        println!("- 功能: 查询企业内所有自定义用户字段的列表");
        println!("- 支持参数:");
        println!("  * page_size: 分页大小");
        println!("  * page_token: 分页标记");
        println!();
        println!("使用方式:");
        println!("1. 传统方法: custom_attr_service.list(&request)");
        println!("2. 构建器模式: custom_attr_service.list_custom_attrs_builder()...");
        println!();
        println!("示例代码:");
        println!();
        println!("// 创建查询请求");
        println!("let request = ListCustomAttrsRequest {{");
        println!("    page_size: Some(50),");
        println!("    page_token: None,");
        println!("}};");
        println!();
        println!("// 执行查询");
        println!("let response = client.contact.v3.custom_attr.list(&request).await?;");
        println!("println!(\"自定义字段数量: {{}}\", response.items.len());");
        println!();
        println!("// 使用构建器模式");
        println!("let response = client");
        println!("    .contact");
        println!("    .v3");
        println!("    .custom_attr");
        println!("    .list_custom_attrs_builder()");
        println!("    .page_size(20)");
        println!("    .execute(&client.contact.v3.custom_attr)");
        println!("    .await?;");
        println!();
        println!("返回数据示例:");
        println!("- items: 自定义字段列表");
        println!("  - key: 字段标识");
        println!("  - name: 字段名称");
        println!("  - description: 字段描述");
        println!("  - type: 字段类型");
        println!("  - required: 是否必填");
        println!("  - options: 选项列表（枚举类型）");
        println!("- has_more: 是否还有更多数据");
        println!("- page_token: 下一页标记");
        println!();
        println!("注意: 实际API调用需要有效的app_id和app_secret认证信息");
        println!("      以及相应的用户权限才能访问自定义字段数据");
    }

    #[cfg(not(feature = "contact"))]
    {
        println!("请启用contact功能标志: cargo run --example contact_custom_attrs_example --features contact");
    }

    Ok(())
}