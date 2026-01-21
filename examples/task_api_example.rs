/// 示例：使用任务 API
///
/// 此示例展示如何使用 openlark-task 模块创建和查询任务
use openlark_task::prelude::*;
use openlark_task::TaskService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder()
        .app_id("your_app_id")
        .app_secret("your_app_secret")
        .base_url("https://open.feishu.cn")
        .build();

    let task_service = TaskService::new(config);

    #[cfg(feature = "task")]
    {
        let response = task_service
            .v2()
            .task()
            .create()
            .summary("完成项目文档")
            .description("编写并完成 OpenLark SDK 的任务模块文档")
            .priority(3)
            .execute()
            .await?;

        println!("任务创建成功: {}", response.task_guid);
    }

    Ok(())
}
