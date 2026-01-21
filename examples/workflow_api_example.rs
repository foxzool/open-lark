/// 示例：使用工作流 API
///
/// 此示例展示如何使用 openlark-workflow 模块创建和查询任务
use openlark_workflow::prelude::*;
use openlark_workflow::WorkflowService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder()
        .app_id("your_app_id")
        .app_secret("your_app_secret")
        .base_url("https://open.feishu.cn")
        .build();

    let workflow_service = WorkflowService::new(config);

    #[cfg(feature = "workflow")]
    {
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
    }

    Ok(())
}
