//! 单个范围读取 API 使用示例
//!
//! 演示如何使用 SingleRangeReadService 进行操作

use open_lark::prelude::*;
use open_lark::service::sheets::v2::SingleRangeReadService, SingleRangeReadRequest;

#[tokio::main]
async fn main() -> SDKResult<()> {
    // 创建配置
    let config = Config::builder()
        .app_id("your_app_id")
        .app_secret("your_app_secret")
        .build();

    // 创建服务实例
    let service = SingleRangeReadService::new(config);

    // 创建请求
    let request = SingleRangeReadRequest::new(
        "your_spreadsheet_token"
        // 其他参数...
    );

    // 执行API调用
    match service.read_range(request).await {
        Ok(response) => {
            println!("✅ 单个范围读取成功");
            println!("响应: {:#?}", response);
        }
        Err(error) => {
            println!("❌ 单个范围读取失败: {}", error);
        }
    }

    Ok(())
}
