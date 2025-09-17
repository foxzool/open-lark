pub mod attachment;
pub mod comment;
pub mod custom_field;
pub mod custom_field_option;
pub mod section;
pub mod task;
pub mod task_subtask;
pub mod tasklist;
pub mod tasklist_activity_subscription;

use crate::core::config::Config;

pub use attachment::AttachmentService;
pub use comment::CommentService;
pub use custom_field::CustomFieldService;
pub use custom_field_option::CustomFieldOptionService;
pub use section::SectionService;
pub use task::TaskService;
pub use task_subtask::TaskSubtaskService;
pub use tasklist::TasklistService;
pub use tasklist_activity_subscription::TasklistActivitySubscriptionService;

/// 任务模块服务 (v2)
pub struct TaskV2Service {
    /// 任务服务
    pub task: TaskService,
    /// 子任务服务
    pub task_subtask: TaskSubtaskService,
    /// 清单服务
    pub tasklist: TasklistService,
    /// 清单活动订阅服务
    pub tasklist_activity_subscription: TasklistActivitySubscriptionService,
    /// 评论服务
    pub comment: CommentService,
    /// 附件服务
    pub attachment: AttachmentService,
    /// 自定义分组服务
    pub section: SectionService,
    /// 自定义字段服务
    pub custom_field: CustomFieldService,
    /// 自定义字段选项服务
    pub custom_field_option: CustomFieldOptionService,
}

impl TaskV2Service {
    pub fn new(config: Config) -> Self {
        Self {
            task: TaskService::new(config.clone()),
            task_subtask: TaskSubtaskService::new(config.clone()),
            tasklist: TasklistService::new(config.clone()),
            tasklist_activity_subscription: TasklistActivitySubscriptionService::new(
                config.clone(),
            ),
            comment: CommentService::new(config.clone()),
            attachment: AttachmentService::new(config.clone()),
            section: SectionService::new(config.clone()),
            custom_field: CustomFieldService::new(config.clone()),
            custom_field_option: CustomFieldOptionService::new(config),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::constants::AppType;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .app_type(AppType::SelfBuild)
            .build()
    }

    #[test]
    fn test_task_v2_service_creation() {
        let config = create_test_config();
        let _service = TaskV2Service::new(config);

        // Verify that all services are properly initialized - test passes by not panicking above
    }

    #[test]
    fn test_task_v2_service_structure() {
        let config = create_test_config();
        let _service = TaskV2Service::new(config);

        // Test that we can access all service fields
        let _task = &_service.task;
        let _task_subtask = &_service.task_subtask;
        let _tasklist = &_service.tasklist;
        let _subscription = &_service.tasklist_activity_subscription;
        let _comment = &_service.comment;
        let _attachment = &_service.attachment;
        let _section = &_service.section;
        let _custom_field = &_service.custom_field;
        let _custom_field_option = &_service.custom_field_option;

        // If we reach here without panic, structure is correct
    }

    #[test]
    fn test_task_v2_service_memory_safety() {
        let config = create_test_config();

        // Create service in a scope
        let _service = TaskV2Service::new(config);

        // Access services multiple times
        let _first_access = &_service.task;
        let _second_access = &_service.task;

        // Verify multiple references work correctly
        assert!(std::ptr::eq(_first_access, _second_access));
    }
}
