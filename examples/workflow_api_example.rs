/// 示例：使用工作流 API
///
/// 此示例展示如何通过 openlark 根 crate 访问工作流模块
use open_lark::workflow::WorkflowService;
use open_lark::CoreConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CoreConfig::builder()
        .app_id("your_app_id")
        .app_secret("your_app_secret")
        .base_url("https://open.feishu.cn")
        .build();

    let workflow_service = WorkflowService::new(config);

    let response = workflow_service
        .v2()
        .task()
        .create()
        .summary("完成项目文档")
        .description("编写并完成 OpenLark SDK 的工作流模块文档")
        .priority(3)
        .execute()
        .await?;

    println!("任务创建成功: {}", response.task_guid);

    Ok(())
}
