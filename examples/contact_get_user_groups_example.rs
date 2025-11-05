//! Contact服务示例 - 查询用户所属用户组
//!
//! 演示如何使用contact服务的查询用户所属用户组功能

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
        let _client = LarkClient::new(config);

        println!("Contact服务 - 查询用户所属用户组 API已修复完成!");
        println!();
        println!("API信息:");
        println!("- 接口: GET /open-apis/contact/v3/groups/member_belong");
        println!("- 功能: 查询指定用户所属的所有用户组");
        println!("- 支持参数:");
        println!("  * member_id: 成员ID");
        println!("  * member_id_type: 成员ID类型 (open_id/user_id/union_id)");
        println!("  * page_size: 分页大小");
        println!("  * page_token: 分页标记");
        println!();
        println!("使用方式:");
        println!("1. 传统方法: group_service.get_user_groups(&request)");
        println!("2. 构建器模式: group_service.get_user_groups_builder()...");
        println!();
        println!("注意: 实际API调用需要有效的app_id和app_secret认证信息");
        println!("      以及相应的用户权限才能访问用户组数据");
    }

    #[cfg(not(feature = "contact"))]
    {
        println!("请启用contact功能标志: cargo run --example contact_get_user_groups_example --features contact");
    }

    Ok(())
}