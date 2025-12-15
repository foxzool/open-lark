#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]

/// 任务管理模块
///
/// 提供异步任务的管理功能，包括任务创建、查询等。

use openlark_core::config::Config;
// 重新导出所有模块类型
pub use create::*;
pub use list::*;

// 子模块
mod create;
mod list;
// mod get;  // TODO: 实现获取任务详情

/// 任务服务
///
/// 提供异步任务的完整管理功能，包括任务创建、列表查询等。
/// 支持文档导出、导入等异步操作。
#[derive(Clone)]
pub struct TaskService {
    config: Config,
}

impl TaskService {
    /// 创建新的任务服务实例
    ///
    /// # 参数
    /// * `config` - SDK配置信息
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取任务列表
    ///
    /// 获取异步任务列表，包括任务状态、进度等信息。
    /// 支持按任务类型和状态过滤。
    ///
    /// # 参数
    /// * `request` - 获取任务列表请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回任务列表响应，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::wiki::v2::task::{TaskService, ListTasksRequest};
    ///
    /// let service = TaskService::new(config);
    /// let request = ListTasksRequest {
    ///     task_type: Some("export".to_string()),
    ///     status: Some("processing".to_string()),
    ///     page_size: Some(20),
    ///     page_token: None,
    /// };
    ///
    /// let response = service.list(request, None).await?;
    /// println!("找到{}个任务", response.data.unwrap().items.len());
    /// ```
    pub async fn list(
        &self,
        request: ListTasksRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> openlark_core::SDKResult<openlark_core::api::Response<ListTasksResponse>> {
        list_tasks(request, &self.config, option).await
    }

    /// 创建任务
    ///
    /// 创建新的异步任务，如文档导出、导入等。
    /// 支持设置任务类型和参数。
    ///
    /// # 参数
    /// * `request` - 创建任务请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回创建的任务信息，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::wiki::v2::task::{TaskService, CreateTaskRequest};
    /// use serde_json::json;
    ///
    /// let service = TaskService::new(config);
    /// let request = CreateTaskRequest::new("export", "导出文档")
    ///     .params(json!({
    ///         "doc_token": "doc_123",
    ///         "format": "pdf"
    ///     }));
    ///
    /// let response = service.create(request, None).await?;
    /// println!("任务创建成功，ID: {}", response.data.unwrap().task_id);
    /// ```
    pub async fn create(
        &self,
        request: CreateTaskRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> openlark_core::SDKResult<openlark_core::api::Response<CreateTaskResponse>> {
        create_task(request, &self.config, option).await
    }
}

impl openlark_core::trait_system::service::Service for TaskService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "task"
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_service() -> TaskService {
        let config = openlark_core::config::Config::new("test_app_id", "test_app_secret");
        TaskService::new(config)
    }

    #[test]
    fn test_task_service_creation() {
        let config = openlark_core::config::Config::new("test_app_id", "test_app_secret");
        let service = TaskService::new(config);

        assert_eq!(service.config().app_id(), "test_app_id");
        assert_eq!(service.config().app_secret(), "test_app_secret");
    }

    #[test]
    fn test_task_service_clone() {
        let service = create_test_service();
        let cloned_service = service.clone();

        assert_eq!(service.config().app_id(), cloned_service.config().app_id());
        assert_eq!(service.config().app_secret(), cloned_service.config().app_secret());
    }

    #[test]
    fn test_service_trait_implementation() {
        let service = create_test_service();

        // 测试Service trait的实现
        let config_ref = service.config();
        assert_eq!(config_ref.app_id(), "test_app_id");
    }

    #[test]
    fn test_create_task_builder() {
        let request = CreateTaskRequest::new("export", "导出文档")
            .params(serde_json::json!({"format": "pdf"}));

        assert_eq!(request.task_type, "export");
        assert_eq!(request.title, "导出文档");
        assert!(request.params.is_some());
    }

    #[test]
    fn test_module_reexports() {
        // 验证所有模块都正确重新导出
        // 这个测试主要是编译时检查，确保所有公共类型都可以访问
        let _list_request: ListTasksRequest = ListTasksRequest {
            task_type: Some("export".to_string()),
            status: None,
            page_size: Some(20),
            page_token: None,
        };

        let _create_request: CreateTaskRequest = CreateTaskRequest::new("export", "导出文档")
            .params(serde_json::json!({"format": "pdf"}));

        // 如果编译通过，说明所有导出都正确
        assert!(true);
}
}
